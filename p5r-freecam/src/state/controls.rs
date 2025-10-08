use glam::Vec3A;
use imgui::Ui;
use opengfd::kernel::allocator::GfdAllocator;
use opengfd::kernel::graphics::GraphicsGlobal;
use riri_mod_tools_rt::logln;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, GetFocus, VIRTUAL_KEY};
use xrd744_lib::btl::package::Package;
use crate::gui::app::APP_GLB;
use crate::state::camera::{Freecam, FreecamFlags, FREQUENCY_SPEED_TICK};
use opengfd::kernel::task::Task as GfdTask;

impl Freecam {
    pub fn check_key_pressed(vk: VIRTUAL_KEY) -> bool {
        let platform = unsafe { crate::globals::get_platform_global().unwrap() };
        match unsafe { GetFocus() } == platform.get_hwnd() {
            true => unsafe { GetAsyncKeyState(vk.0 as i32) & 1 != 0 },
            false => false
        }
    }

    pub(crate) fn draw_contents_controls(&mut self, ui: &Ui) {
        let max_width = ui.content_region_avail()[0];
        let font_data = ui.fonts().get_font(APP_GLB.lock().unwrap().font).unwrap();
        let fov_length = "FOV".chars().map(|c| font_data.get_glyph(c).advance_x).sum::<f32>();
        ui.set_next_item_width((max_width / 2.) - (fov_length + 10.));
        let graphics = GraphicsGlobal::get_gfd_graphics_global_mut();
        let scene_cam = graphics.get_current_scene_mut()
            .and_then(|scn| scn.get_current_camera_mut());
        if let Some(cam) = scene_cam {
            let mut fovy = cam.get_fovy();
            if ui.slider_config("FOV##ForFreecamWindow", 5., 175.).build(&mut fovy) {
                cam.set_fovy(fovy);
            }
        }
        ui.same_line_with_spacing(0., 10.);
        let time_length = "Speed".chars().map(|c| font_data.get_glyph(c).advance_x).sum::<f32>();
        ui.set_next_item_width((max_width / 2.) - (time_length + 10.));
        let glb = GraphicsGlobal::get_gfd_graphics_global_mut();
        if let Some(btl) = GfdTask::<GfdAllocator, Package>::find_by_str_mut("battle") {
            let pkg = btl.get_main_work_mut().unwrap();
            if let Some(freq) = pkg.get_frequency_mut() {
                let mut time = freq.get_time();
                if ui.input_float("Speed##ForFreecamWindow", &mut time).step(0.1).build() {
                    freq.set_time(time.max(0.));
                }
            }
        } else {
            if let Some(scn) = glb.get_current_scene_mut() {
                let mut time = scn.get_frequency();
                if ui.input_float("Speed##ForFreecamWindow", &mut time).step(0.1).build() {
                    scn.set_frequency(time.max(0.));
                }
            }
        }
        /*
        let time_length = "Time".chars().map(|c| font_data.get_glyph(c).advance_x).sum::<f32>();
        ui.set_next_item_width((max_width / 2.) - (time_length + 10.));
        if ui.input_float("Time##ForFreecamWindow", &mut self.node_path_time).step(1.).build() {}
        */

        // ui.text(format!("{:.02} / {:.02} sec", self.node_path_current, self.node_path_time));
        ui.text(format!("{:.02} / ", self.node_path_current));
        ui.same_line_with_spacing(0., 10.);
        ui.set_next_item_width(50.);
        if ui.input_float("##NodePathTimeForFreecamWindow", &mut self.node_path_time).display_format("%.2f").build() {
        }
        ui.same_line_with_spacing(0., 10.);
        ui.text("sec");
        ui.disabled(self.nodes.is_empty(), || {
            ui.same_line_with_spacing(0., 10.);
            ui.set_next_item_width(ui.content_region_avail()[0] - (unsafe { ui.style().window_padding[0] - 10.}));
            if ui.slider_config("##TimelineForFreecamWindow", 0., self.node_path_time).build(&mut self.node_path_current) {
                self.flags &= !FreecamFlags::PLAYING_PATH;
                self.camera_path_tick(0.);
            }
        });
        let play_pause = match self.flags.contains(FreecamFlags::PLAYING_PATH) {
            true => "Pause##ForFreecamWindow", false => "Play##ForFreecamWindow"
        };
        if ui.button(play_pause) { self.toggle_playback().unwrap(); }
        ui.same_line_with_spacing(0., 10.);
        if ui.button("Stop##ForFreecamWindow") { self.stop_playback().unwrap(); }
    }
}