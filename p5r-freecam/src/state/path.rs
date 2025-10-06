use std::error::Error;
use std::ops::{Add, Mul};
use glam::{EulerRot, Mat4, Quat, Vec3A};
use imgui::Ui;
use implot::{Axis, Plot, PlotScatter};
use opengfd::kernel::allocator::GfdAllocator;
use crate::state::camera::Freecam;
use opengfd::object::camera::Camera as GfdCamera;
use riri_mod_tools_rt::logln;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_1, VK_2, VK_8, VK_9, VK_BACK, VK_DELETE, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD8, VK_NUMPAD9};
use crate::state::camera::FreecamFlags;
use crate::state::node::{FreecamNode, FreecamNodeEntry};
use opengfd::kernel::task::Task as GfdTask;
use riri_inspector_components::table::InspectorTable;
use xrd744_lib::fld::camera::Camera as FldCamera;
// use crate::gui::app::IMPLOT_GLB;

impl Freecam {
    pub fn lerp<T>(&self, from: T, to: T) -> T
    where T: Mul<f32, Output = T> + Add<Output = T>
    { (from * (1. - self.node_path_percent)) + (to * self.node_path_percent) }

    pub fn bezier_quadratic<T>(&self, nodes: Vec<T>) -> T
    where T: Mul<f32, Output = T> + Add<Output = T> + Copy
    {
        let t = self.node_path_percent;
        let t2 = t * t;
        let mt = 1. - t;
        let mt2 = mt * mt;
        (nodes[0] * mt2) + (nodes[1] * 2. * mt * t) + (nodes[2] * t2)
    }

    pub fn bezier_cubic<T>(&self, nodes: Vec<T>) -> T
    where T: Mul<f32, Output = T> + Add<Output = T> + Copy
    {
        let t = self.node_path_percent;
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1. - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        (nodes[0] * mt3) + (nodes[1] * 3. * mt2 * t) + (nodes[2] * 3. * mt * t2) + (nodes[3] * t3)
    }

    pub fn set_position_from_interp(&mut self, cam: &mut GfdCamera, payload: FreecamNode) {
        (self.pan, self.pitch, self.roll) = payload.rot.to_euler(EulerRot::YXZEx);
        // logln!(Verbose, "{}: <pan: {}, pitch: {}, roll: {}>", self.node_path_percent, self.pan, self.pitch, self.roll);
        self.camera_pos = payload.trans;
        // set lookat and up vec
        self.lookat_pos = self.camera_pos - Vec3A::new(
            -(self.pan.sin() * self.pitch.cos()),
            self.pitch.sin(),
            -(self.pan.cos() * self.pitch.cos()),
        ) * 100.;

        let dir: Vec3A = self.camera_pos - self.lookat_pos;
        let r = Vec3A::Y.cross(dir).normalize_or_zero();
        self.up_vec = dir.cross(r).normalize_or_zero().into();
        cam.set_view_transform(Mat4::look_at_rh(self.camera_pos.into(), self.lookat_pos.into(), self.up_vec.into()));
    }

    pub fn set_position_from_last_interp(&mut self, cam: &mut GfdCamera) {
        (self.pan, self.pitch, self.roll) = self.last_interp.rot.to_euler(EulerRot::YXZEx);
        // logln!(Verbose, "{}: <pan: {}, pitch: {}, roll: {}>", self.node_path_percent, self.pan, self.pitch, self.roll);
        self.camera_pos = self.last_interp.trans;
        // set lookat and up vec
        self.lookat_pos = self.camera_pos - Vec3A::new(
            -(self.pan.sin() * self.pitch.cos()),
            self.pitch.sin(),
            -(self.pan.cos() * self.pitch.cos()),
        ) * 100.;

        let dir: Vec3A = self.camera_pos - self.lookat_pos;
        let r = Vec3A::Y.cross(dir).normalize_or_zero();
        self.up_vec = dir.cross(r).normalize_or_zero().into();
        cam.set_view_transform(Mat4::look_at_rh(self.camera_pos.into(), self.lookat_pos.into(), self.up_vec.into()));
    }

    pub fn add_camera_node(&mut self) {
        let trans = self.camera_pos;
        let rot = Quat::from_euler(EulerRot::YXZEx, self.pan, self.pitch, self.roll);
        let new = FreecamNode::new(trans, rot);
        logln!(Verbose, "Add node #{} {:?} <pan: {}, pitch: {}, roll: {}>", self.nodes.len() + 1, new, self.pan, self.pitch, self.roll);
        self.nodes.push(new);
    }

    pub fn camera_path_tick(&mut self, delta: f32) {
        self.node_path_current = (self.node_path_current + delta).min(self.node_path_time);
        self.node_path_percent = self.node_path_current / self.node_path_time;
        if let Some(payload) = match self.nodes.len() {
            0 => {
                logln!(Verbose, "No nodes have been set for camera path!");
                self.flags &= !FreecamFlags::PLAYING_PATH;
                None
            },
            1 => { // single point
                let node = self.nodes.first().unwrap();
                self.flags &= !FreecamFlags::PLAYING_PATH; // stop immediately
                Some(FreecamNode::new(node.trans, node.rot))
            },
            2 => { // lerp
                let first = self.nodes.first().unwrap();
                let last = self.nodes.last().unwrap();
                Some(FreecamNode::new(
                    self.lerp(first.trans, last.trans),
                    self.lerp(first.rot, last.rot)
                ))
            },
            3 => Some(FreecamNode::new( // bezier quadratic
                                        self.bezier_quadratic(self.nodes.iter().map(|v| v.trans).collect()),
                                        self.bezier_quadratic(self.nodes.iter().map(|v| v.rot).collect()),
            )),
            k => { // b-spline (De Boor's algorithm)
                let (low, high) = (crate::state::camera::BSPLINE_DEGREE_QUADRATIC, k);
                let t = self.node_path_percent.min(0.999) * (high - low) as f32 + low as f32; // remap time
                let s = t as usize; // spline segment
                let mut nodes = self.nodes.clone();
                for l in 1..crate::state::camera::BSPLINE_DEGREE_QUADRATIC + 2 { // perform interpolation
                    for i in (s - crate::state::camera::BSPLINE_DEGREE_QUADRATIC + l..s + 1).rev() {
                        let alpha = (t - i as f32) / ((i + crate::state::camera::BSPLINE_DEGREE_QUADRATIC + 1 - l) - i) as f32;
                        nodes[i].trans = nodes[i - 1].trans * (1. - alpha) + nodes[i].trans * alpha;
                        nodes[i].rot = nodes[i - 1].rot * (1. - alpha) + nodes[i].rot * alpha;
                    }
                }
                Some(FreecamNode::new(nodes[s].trans, nodes[s].rot))
            },
        } {
            if let Some(task) = GfdTask::<GfdAllocator, FldCamera>::find_by_str_mut("field camera CTRL") {
                let fldcam_ctx = task.get_main_work_mut().unwrap();
                if let Some(fldcam) = fldcam_ctx.get_gfd_camera_mut() {
                    self.set_position_from_interp(fldcam, payload);
                }
            }

        }
        if self.node_path_current >= self.node_path_time {
            self.flags &= !FreecamFlags::PLAYING_PATH;
        }
    }

    pub fn update_camera_path(&mut self, delta: f32) {
        // set camera path
        if Self::check_key_pressed(VK_1) || Self::check_key_pressed(VK_NUMPAD1) {
            self.add_camera_node();
        }
        // update path speed
        if Self::check_key_pressed(VK_8) || Self::check_key_pressed(VK_NUMPAD8) {
            self.change_node_path_time(true);
        } else if Self::check_key_pressed(VK_9) || Self::check_key_pressed(VK_NUMPAD9) {
            self.change_node_path_time(false);
        }
        // remove nodes
        if Self::check_key_pressed(VK_BACK) {
            if self.nodes.len() > 0 {
                let old_id = self.nodes.len();
                let rem = self.nodes.pop().unwrap();
                logln!(Verbose, "Removed node #{} {:?}", old_id, rem);
            } else {
                logln!(Verbose, "Node list is already empty");
            }
        }
        if Self::check_key_pressed(VK_DELETE) {
            if self.nodes.len() > 0 {
                logln!(Verbose, "Cleared node list (had {} nodes)", self.nodes.len());
                self.nodes.clear();
            } else {
                logln!(Verbose, "Node list is already empty");
            }
        }
        // start playback
        if (Self::check_key_pressed(VK_2) || Self::check_key_pressed(VK_NUMPAD2)) && self.nodes.len() > 0 {
            let first = &self.nodes[0];
            logln!(Verbose, "Start playing ({} sec)", self.node_path_time);
            self.node_path_current = 0.;
            // flip quarternion to avoid rotating the wrong way (if applicable)
            match self.nodes.len() {
                k => for i in 1..k {
                    let val = self.nodes[i].rot;
                    self.nodes[i].rot = if self.nodes[i - 1].rot.dot(val) < 0. { -val } else { val };
                },
                _ => ()
            }
            self.flags |= FreecamFlags::PLAYING_PATH;
        }
        if self.flags.contains(FreecamFlags::PLAYING_PATH) { self.camera_path_tick(delta); }
    }

    fn correct_node_rotation(&mut self) {
        // flip quarternion to avoid rotating the wrong way (if applicable)
        match self.nodes.len() {
            k => for i in 1..k {
                let val = self.nodes[i].rot;
                self.nodes[i].rot = if self.nodes[i - 1].rot.dot(val) < 0. { -val } else { val };
            },
            _ => ()
        }
    }

    pub(crate) fn toggle_playback(&mut self) -> Result<bool, Box<dyn Error>> {
        if self.nodes.len() > 0 {
            match self.flags.contains(FreecamFlags::PLAYING_PATH) {
                true => self.flags &= !FreecamFlags::PLAYING_PATH,
                false => {
                    if self.node_path_current >= self.node_path_time {
                        self.node_path_current = 0.;
                    }
                    self.correct_node_rotation();
                    self.flags |= FreecamFlags::PLAYING_PATH;
                }
            }
        }
        Ok(true)
    }

    pub(crate) fn stop_playback(&mut self) -> Result<bool, Box<dyn Error>> {
        if self.flags.contains(FreecamFlags::PLAYING_PATH) {
            self.flags &= !FreecamFlags::PLAYING_PATH;
        }
        self.node_path_current = 0.;
        if self.nodes.len() > 0 {
            self.camera_path_tick(0.);
        }
        Ok(true)
    }

    pub(crate) fn draw_contents_keyframes(&mut self, ui: &Ui) {
        // path nodes
        let content_area = ui.content_region_avail();
        let mut table: InspectorTable<FreecamNodeEntry<'_>, Self, 4> = InspectorTable::new(
            "Freecam Nodes",  Some([ "Index", "Translation", "Rotation", "Actions" ]),
            riri_inspector_components::table::default_flags(),
            content_area[1] * 2. / 3.,
        );
        let contents: Vec<_> = self.nodes.iter().enumerate().map(|(i, n)| FreecamNodeEntry::new(n, i)).collect();
        let self_ptr = unsafe { &mut *(&raw const *self as *mut Self) };
        table.draw_table(ui, self_ptr, &contents);
        // graph nodes
        /*
        let node_plot = Plot::new("Camera Nodes")
            .size([content_area[0], content_area[1] / 3.]);
        let plot = unsafe { &mut *IMPLOT_GLB.get().unwrap().as_ptr() };
        let plot_ui = plot.get_plot_ui();
        node_plot.build(&plot_ui, || {
            PlotScatter::new("Nodes").plot(&[0.], &[0.]);
            PlotLi
        });
        */
    }
}