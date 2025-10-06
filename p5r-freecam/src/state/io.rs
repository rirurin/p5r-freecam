use glam::Vec3A;
use glam::swizzles::Vec4Swizzles;
use imgui::Ui;
use opengfd::kernel::graphics::GraphicsGlobal;
use riri_file_dialog::dialog::{FileDialogManager, FileTypeFilter, OpenDialog, SaveDialog};
use riri_mod_tools_rt::logln;
use rkyv::rancor::ResultExt;
use rkyv::util::AlignedVec;
use crate::state::camera::{Freecam, FreecamFlags};
use crate::state::node::{u32_ne, ArchivedFreecamNode, FreecamNode};
use rkyv::rancor::Error as RkyvError;
use crate::gui::app::APP_GLB;

const FREECAM_FILE_EXT: &'static str = "p5path";

impl Freecam {
    fn read(buf: &[u8]) -> Result<Vec<FreecamNode>, RkyvError> {
        let head = &buf[..size_of::<u32>()];
        let head = rkyv::access::<u32_ne, RkyvError>(head).into_error()?;
        let head = rkyv::deserialize::<u32, RkyvError>(head).into_error()?;
        let mut nodes = vec![];
        for i in 0..head as usize {
            let start = size_of::<u32>() + (size_of::<ArchivedFreecamNode>() * i);
            let end = size_of::<u32>() + (size_of::<ArchivedFreecamNode>() * (i + 1));
            let body = rkyv::access::<ArchivedFreecamNode, RkyvError>(&buf[start..end]).into_error()?;
            nodes.push(rkyv::deserialize::<FreecamNode, RkyvError>(body).into_error()?);
        }
        Ok(nodes)
    }

    fn write(&self) -> Result<AlignedVec, RkyvError> {
        let mut buf = AlignedVec::new();
        let _ = rkyv::util::with_arena(|arena| {
            // length
            rkyv::api::high::to_bytes_in_with_alloc::<&mut AlignedVec, _, RkyvError>(
                &(self.nodes.len() as u32), &mut buf, arena.acquire()
            ).into_error()?;
            // nodes
            for node in &self.nodes {
                rkyv::api::high::to_bytes_in_with_alloc::<_, _, RkyvError>(
                    node, &mut buf, arena.acquire()
                ).into_error()?;
            }
            Ok(())
        })?;
        Ok(buf)
    }

    pub(crate) fn draw_contents_topbar(&mut self, ui: &Ui) {
        let max_width = ui.content_region_avail()[0];
        match self.flags.contains(FreecamFlags::ACTIVE) {
            true => if ui.button("Disable##ForFreecamWindow") { self.disable_freecam_mode() },
            false => if ui.button("Enable##ForFreecamWindow") { self.enable_freecam_mode() },
        }
        ui.same_line_with_spacing(0., 10.);
        if ui.button("Load Path##ForFreecamWindow") {
            let mut dlg_lock = FileDialogManager::get();
            if let Some(v) = OpenDialog::new(dlg_lock.as_mut().unwrap()).unwrap().open(
                Some(&[FileTypeFilter::new(FREECAM_FILE_EXT.to_owned(), "P5R Freecam Path".to_owned())]),
                Some("Open camera path")
            ).unwrap() {
                match std::fs::read(v.as_path()) {
                    Ok(buf) => {
                        logln!(Verbose, "Read file {} ({} bytes)", v.as_path().to_str().unwrap(), buf.len());
                        match Self::read(&buf) {
                            Ok(v) => self.nodes = v,
                            Err(e) => logln!(Verbose, "Error while parsing file: {}", e),
                        }
                    },
                    Err(e) => logln!(Verbose, "Error while opening file: {}", e),
                }
            }
        }
        ui.same_line_with_spacing(0., 10.);
        if ui.button("Save Path##ForFreecamWindow") {
            let mut dlg_lock = FileDialogManager::get();
            if let Some(v) = SaveDialog::new(dlg_lock.as_mut().unwrap()).unwrap().save(
                Some(&[FileTypeFilter::new(FREECAM_FILE_EXT.to_owned(), "P5R Freecam Path".to_owned())]),
                Some("Save camera path")
            ).unwrap() {
                match self.write() {
                    Ok(buf) => match std::fs::write(v.as_path(), buf.as_slice()) {
                        Ok(_) => logln!(Verbose, "File saved to {}", v.to_str().unwrap()),
                        Err(e) => logln!(Verbose, "Couldn't save file: {}", e),
                    },
                    Err(e) => logln!(Verbose, "Error while writing file: {}", e),
                }
            }
        }
        ui.same_line_with_spacing(0., 10.);
        if ui.button("Add Node##ForFreecamWindow") {  self.add_camera_node(); }
        let scene = GraphicsGlobal::get_gfd_graphics_global_mut();
        let mut scene_cam = scene.get_scene_mut(0)
            .and_then(|scn| scn.get_current_camera_mut());
        let mut cam_pos = scene_cam.as_mut().map_or([0.; 3],
        |cam| Into::<[f32; 3]>::into(cam.get_view_transform().inverse().w_axis.xyz()));
        let font_data = ui.fonts().get_font(APP_GLB.lock().unwrap().font).unwrap();
        match self.flags.contains(FreecamFlags::ACTIVE) {
            true => {
                let text_length = "Camera Position".chars().map(|c| font_data.get_glyph(c).advance_x).sum::<f32>();
                ui.same_line_with_spacing((max_width / 2.) - text_length, 0.);
                ui.set_next_item_width(max_width / 2.);
                if ui.input_float3("Camera Position##ForFreecamWindow", &mut cam_pos).build() {
                    if let Some(_) = scene_cam {
                        // self.camera_pos = cam_pos.into();
                        // cam.set_translate(self.camera_pos);
                    }
                }
            },
            false => {
                let cam_pos_text = format!("Camera position: {:?}", cam_pos);
                let cam_length = cam_pos_text.chars().map(|c| font_data.get_glyph(c).advance_x).sum::<f32>();
                ui.same_line_with_spacing(max_width - cam_length, 0.);
                ui.text(cam_pos_text);
            }
        }
    }
}