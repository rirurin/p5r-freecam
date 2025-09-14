use opengfd::{
    kernel::{
        allocator::GfdAllocator,
        global::Global,
        task::{
            InitTask,
            Task as GfdTask,
            TaskFunctionReturn,
            UpdateTask
        }
    },
    object::camera::Camera as GfdCamera,
};
use riri_mod_tools_proc::{ create_hook, riri_hook_fn };
use riri_mod_tools_rt::{ logln, sigscan_resolver };
use std::{
    ptr::NonNull,
    sync::OnceLock
};
use std::ops::{Add, Mul};
use bitflags::bitflags;
use glam::{EulerRot, Vec3A, Quat, Mat4, Vec4};
use opengfd::io::controller::ControllerButton;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState,
    VK_F4,
    VK_ADD,
    VK_SUBTRACT,
    VK_OEM_MINUS,
    VK_OEM_PLUS,
    VK_8,
    VK_9,
    VK_0,
    VK_1,
    VK_2,
    VK_NUMPAD8,
    VK_NUMPAD9,
    VK_NUMPAD0,
    VK_NUMPAD1,
    VK_NUMPAD2,
    VK_BACK,
    VK_DELETE
};
use xrd744_lib::fld::camera::Camera as FldCamera;

#[no_mangle]
pub unsafe extern "C" fn setTITLE_RES_PROC_LOOP(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v, None => return None
    };
    logln!(Information, "got TITLE_RES_PROC_LOOP: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

static INITIALIZED_FREECAM: OnceLock<()> = OnceLock::new();

#[riri_hook_fn(dynamic_offset(
    signature = "48 89 5C 24 ?? 48 89 6C 24 ?? 56 57 41 55 41 56 41 57 48 83 EC 30 48 8B 59 ??",
    resolve_type = setTITLE_RES_PROC_LOOP,
    calling_convention = "microsoft",
))]
pub unsafe extern "C" fn TITLE_RES_PROC_LOOP(p_task: *mut u8) -> u64 {
    if INITIALIZED_FREECAM.get().is_none() {
        let new_task = GfdTask::<GfdAllocator, Freecam>::new_update(10, 0, 0, 0, GfdAllocator);
        logln!(Information, "Freecam task: {}", new_task);
        let _ = INITIALIZED_FREECAM.set(());
    }
    original_function!(p_task)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CameraState {
    Field,
    Event,
    Battle
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FreecamFlags : u32 {
        const ACTIVE = 1 << 0;
        const SET_INITIAL_STATE = 1 << 1;
        // const HOOKED_PANEL_MAP = 1 << 2;
        // const IN_BATTLE = 1 << 1;
        // const FOUND_BTL_CAMERA_CONTROL = 1 << 2;
        const WAITING_FOR_PARTY_PANEL = 1 << 3;
        const LOCK_CAMERA_MOVEMENT = 1 << 4;
        const IN_EVENT = 1 << 5;
        const EVT_SET_INITIAL_PARAMS = 1 << 6;
        const DEBUG_HOOKED_FREE_CAM_LOOP = 1 << 7;
        const PLAYING_PATH = 1 << 8;
        const PLAYER_INPUT_LOCKED = 1 << 9;
        const HOOKED_PANEL_MAP = 1 << 0x10;
        const HOOKED_DATE_DRAW = 1 << 0x11;
        const HOOKED_MISSION_DRAW = 1 << 0x12;
        const HOOKED_BATTLE_PARTY_PANEL = 1 << 0x13;
        const HOOKED_ROADMAP = 1 << 0x14;
    }
}

// quaternion
#[derive(Debug, Clone)]
pub struct FreecamNode {
    pub(crate) trans: Vec3A,
    pub(crate) rot: Quat,
}

impl FreecamNode {
    pub fn new(trans: Vec3A, rot: Quat) -> Self {
        let (pan, pitch, roll) = rot.to_euler(EulerRot::YXZEx);
        Self { trans, rot }
    }
    pub fn new_euler(trans: Vec3A, pan: f32, pitch: f32, roll: f32) -> Self {
        let rot = Quat::from_euler(EulerRot::YXZEx, pan, pitch, roll);
        Self { trans, rot }
    }
}

impl Default for FreecamNode {
    fn default() -> Self {
        Self {
            trans: Vec3A::default(),
            rot: Quat::default()
        }
    }
}

#[derive(Debug)]
pub struct Freecam {
    pub(crate) flags: FreecamFlags,
    // state for custom event/battle freecam
    pub(crate) pan: f32,
    pub(crate) pitch: f32,
    pub(crate) roll: f32,
    pub(crate) camera_pos: Vec3A,
    pub(crate) lookat_pos: Vec3A,
    pub(crate) up_vec: Vec3A,
    // camera path
    pub(crate) nodes: Vec<FreecamNode>,
    pub(crate) node_path_time: f32,
    pub(crate) node_path_current: f32,
    pub(crate) node_path_percent: f32,
    // send to evt task
    pub(crate) last_interp: FreecamNode,
    pub(crate) evt_return: FreecamNode
}

const FREQUENCY_SPEED_TICK: f32 = 0.1;
const NODE_PATH_DEFAULT_TIME: f32 = 3.0;
const NODE_PATH_STEP: f32 = 0.25;

const BSPLINE_DEGREE_QUADRATIC: usize = 2;

impl Freecam {
    pub fn change_frequency_speed(&self, slow: bool) {
        /*
        let glb = GraphicsGlobal::get_gfd_graphics_global_mut();
        if self.flags.contains(FreecamFlags::IN_BATTLE) {
            let btl = GfdTask::<GfdAllocator, Package>::find_by_str_mut("battle").unwrap();
            let pkg = btl.get_main_work_mut().unwrap();
            if let Some(freq) = pkg.get_frequency_mut() {
                let new_freq = (freq.get_time() + if slow { -FREQUENCY_SPEED_TICK } else { FREQUENCY_SPEED_TICK }).max(0.);
                logln!(Verbose, "New game speed: {:.02}x", new_freq);
                freq.set_time(new_freq);
            }
        } else {
            if let Some(scn) = glb.get_current_scene_mut() {
                let new_freq = (scn.get_frequency() + if slow { -FREQUENCY_SPEED_TICK } else { FREQUENCY_SPEED_TICK }).max(0.);
                logln!(Verbose, "New game speed: {:.02}x", new_freq);
                scn.set_frequency(new_freq);
            }
        }
        */
    }

    pub fn change_node_path_time(&mut self, slow: bool) {
        let new = (self.node_path_time + if slow { -NODE_PATH_STEP } else { NODE_PATH_STEP }).max(NODE_PATH_STEP);
        logln!(Verbose, "New node path time: {:.02} sec", new);
        self.node_path_time = new;
    }

    pub fn get_roll(&self) -> f32 { self.roll }

    pub fn update_view_matrix(&mut self) -> Mat4 {
        // handle camera inputs
        let ctrl  = unsafe { crate::globals::get_pad_instance().unwrap() };
        let lstick = ctrl.get_current().get_lstick();
        let rstick = ctrl.get_current().get_rstick();
        let buttons = ctrl.get_current().get_hold_press();

        let (pan_speed, move_speed) = if buttons.contains(ControllerButton::LEFT_TRIGGER) { (2., 5.) }
        else if buttons.contains(ControllerButton::RIGHT_TRIGGER) { (0.5, 0.2) }
        else { (1., 1.) };


        self.pan -= rstick.get_horizontal() as f32 / (8000. / pan_speed);
        self.pitch += rstick.get_vertical() as f32 / (8000. / pan_speed);
        if !self.flags.contains(FreecamFlags::LOCK_CAMERA_MOVEMENT) {
            let lh = lstick.get_horizontal() as f32 / (10. / move_speed);
            let lv = lstick.get_vertical() as f32 / (10. / move_speed);
            let dir: Vec3A = self.camera_pos - self.lookat_pos; // front vector
            let r = Vec3A::Y.cross(dir).normalize_or_zero(); // right unit vector

            self.camera_pos += lh * r + lv * dir.normalize_or_zero();
            if buttons.contains(ControllerButton::LEFT_SHOULDER) { self.camera_pos.y += 5. * move_speed; }
            if buttons.contains(ControllerButton::RIGHT_SHOULDER) { self.camera_pos.y -= 5. * move_speed; }
        }
        if buttons.contains(ControllerButton::DPAD_UP) { self.roll += 0.1; }
        if buttons.contains(ControllerButton::DPAD_DOWN) { self.roll -= 0.1; }

        // set lookat and up vec
        self.lookat_pos = self.camera_pos - Vec3A::new(
            -(self.pan.sin() * self.pitch.cos()),
            self.pitch.sin(),
            -(self.pan.cos() * self.pitch.cos()),
        ) * 100.;

        let dir: Vec3A = self.camera_pos - self.lookat_pos;
        let r = Vec3A::Y.cross(dir).normalize_or_zero();
        self.up_vec = dir.cross(r).normalize_or_zero().into();

        Mat4::look_at_rh(self.camera_pos.into(), self.lookat_pos.into(), self.up_vec.into())
    }

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
                let (low, high) = (BSPLINE_DEGREE_QUADRATIC, k);
                let t = self.node_path_percent.min(0.999) * (high - low) as f32 + low as f32; // remap time
                let s = t as usize; // spline segment
                let mut nodes = self.nodes.clone();
                for l in 1..BSPLINE_DEGREE_QUADRATIC + 2 { // perform interpolation
                    for i in (s - BSPLINE_DEGREE_QUADRATIC + l..s + 1).rev() {
                        let alpha = (t - i as f32) / ((i + BSPLINE_DEGREE_QUADRATIC + 1 - l) - i) as f32;
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

    pub fn enable_freecam_mode(&mut self) {
        self.flags |= FreecamFlags::ACTIVE | FreecamFlags::SET_INITIAL_STATE;
        FldCamera::handle_freecam_onoff(true);
        logln!(Verbose, "Enable freecam");
    }

    pub fn disable_freecam_mode(&mut self) {
        self.flags &= !FreecamFlags::ACTIVE;
        FldCamera::handle_freecam_onoff(false);
        logln!(Verbose, "Disable freecam");
    }

    pub fn update_scene_speed(&mut self) {
        if unsafe { GetAsyncKeyState(VK_OEM_MINUS.0 as i32) & 1 != 0
            || GetAsyncKeyState(VK_SUBTRACT.0 as i32) & 1 != 0 } {
            self.change_frequency_speed(true);
        } else if unsafe { GetAsyncKeyState(VK_OEM_PLUS.0 as i32) & 1 != 0
            || GetAsyncKeyState(VK_ADD.0 as i32) & 1 != 0 } {
            self.change_frequency_speed(false);
        }
    }

    pub fn lock_camera_position(&mut self) {
        if unsafe { GetAsyncKeyState(VK_0.0 as i32) & 1 != 0
            || GetAsyncKeyState(VK_NUMPAD0.0 as i32) & 1 != 0 } {
            self.flags ^= FreecamFlags::LOCK_CAMERA_MOVEMENT;
            match self.flags.contains(FreecamFlags::LOCK_CAMERA_MOVEMENT) {
                true => logln!(Verbose, "Camera is locked"),
                false => logln!(Verbose, "Camera is unlocked"),
            };
        }
    }

    pub fn add_camera_node(&mut self) {
        let trans = self.camera_pos;
        let rot = Quat::from_euler(EulerRot::YXZEx, self.pan, self.pitch, self.roll);
        let new = FreecamNode::new(trans, rot);
        logln!(Verbose, "Add node #{} {:?} <pan: {}, pitch: {}, roll: {}>", self.nodes.len() + 1, new, self.pan, self.pitch, self.roll);
        self.nodes.push(new);
    }

    pub fn update_camera_path(&mut self, delta: f32) {
        // set camera path
        if unsafe { GetAsyncKeyState(VK_1.0 as i32) & 1 != 0
            || GetAsyncKeyState(VK_NUMPAD1.0 as i32) & 1 != 0 } {
            self.add_camera_node();
        }
        // update path speed
        if unsafe { GetAsyncKeyState(VK_8.0 as i32) & 1 != 0
            || GetAsyncKeyState(VK_NUMPAD8.0 as i32) & 1 != 0 }
        {
            self.change_node_path_time(true);
        } else if unsafe { GetAsyncKeyState(VK_9.0 as i32) & 1 != 0
            || GetAsyncKeyState(VK_NUMPAD9.0 as i32) & 1 != 0 } {
            self.change_node_path_time(false);
        }
        // remove nodes
        if unsafe { GetAsyncKeyState(VK_BACK.0 as i32) & 1 != 0 } {
            if self.nodes.len() > 0 {
                let old_id = self.nodes.len();
                let rem = self.nodes.pop().unwrap();
                logln!(Verbose, "Removed node #{} {:?}", old_id, rem);
            } else {
                logln!(Verbose, "Node list is already empty");
            }
        }
        if unsafe { GetAsyncKeyState(VK_DELETE.0 as i32) & 1 != 0 } {
            if self.nodes.len() > 0 {
                logln!(Verbose, "Cleared node list (had {} nodes)", self.nodes.len());
                self.nodes.clear();
            } else {
                logln!(Verbose, "Node list is already empty");
            }
        }
        // start playback
        if (unsafe { GetAsyncKeyState(VK_2.0 as i32) & 1 != 0
            || GetAsyncKeyState(VK_NUMPAD2.0 as i32) & 1 != 0
        }) && self.nodes.len() > 0 {
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

    pub fn check_active() -> bool {
        match GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME) {
            Some(task) => {
                let ctx = task.get_main_work_mut().unwrap();
                ctx.flags.contains(FreecamFlags::ACTIVE)
            },
            None => false
        }
    }
}

impl UpdateTask for Freecam {
    const NAME: &'static str = "Rirurin Freecam";
    fn update(task: &mut GfdTask<GfdAllocator, Self>, delta: f32)
              -> TaskFunctionReturn where Self: Sized {
        let ctx = task.get_main_work_mut().unwrap();
        // enable/disable freecam
        if unsafe { GetAsyncKeyState(VK_F4.0 as i32) & 1 != 0 } {
            match ctx.flags.contains(FreecamFlags::ACTIVE) {
                true => ctx.disable_freecam_mode(),
                false => ctx.enable_freecam_mode(),
            }
        }
        if ctx.flags.contains(FreecamFlags::ACTIVE) {
            ctx.update_scene_speed();
            ctx.lock_camera_position();
        }
        if ctx.flags.contains(FreecamFlags::ACTIVE) {
            ctx.update_camera_path(delta);
        }

        if ctx.flags.contains(FreecamFlags::SET_INITIAL_STATE) {
            if let Some(task) = GfdTask::<GfdAllocator, FldCamera>::find_by_str_mut("field camera CTRL") {
                let fldcam_ctx = task.get_main_work_mut().unwrap();
                let target_node = fldcam_ctx.get_target_node();
                /*
                ctx.camera_pos = target_node.map_or(Vec3A::ZERO, |n| n.get_translate());
                ctx.lookat_pos = ctx.camera_pos + fldcam_ctx.get_target_offset();
                */
                ctx.pitch = fldcam_ctx.get_pitch();
                ctx.pan = fldcam_ctx.get_yaw();
            }
            ctx.flags &= !FreecamFlags::SET_INITIAL_STATE;
        }
        if ctx.flags.contains(FreecamFlags::ACTIVE) {
            if let Some(task) = GfdTask::<GfdAllocator, FldCamera>::find_by_str_mut("field camera CTRL") {
                let fldcam_ctx = task.get_main_work_mut().unwrap();
                if let Some(fldcam) = fldcam_ctx.get_gfd_camera_mut() {
                    fldcam.set_view_transform(ctx.update_view_matrix());
                }
            }
        }
        if !ctx.flags.contains(FreecamFlags::HOOKED_PANEL_MAP) {
            if crate::field::try_hook_panel_map() {
                ctx.flags |= FreecamFlags::HOOKED_PANEL_MAP;
            }
        }
        if !ctx.flags.contains(FreecamFlags::HOOKED_DATE_DRAW) {
            if crate::field::try_hook_date_draw() {
                ctx.flags |= FreecamFlags::HOOKED_DATE_DRAW;
            }
        }
        if !ctx.flags.contains(FreecamFlags::HOOKED_MISSION_DRAW) {
            if crate::field::try_hook_mission_draw() {
                ctx.flags |= FreecamFlags::HOOKED_MISSION_DRAW;
            }
        }
        if !ctx.flags.contains(FreecamFlags::HOOKED_BATTLE_PARTY_PANEL) {
            if crate::field::try_hook_party_panel() {
                ctx.flags |= FreecamFlags::HOOKED_BATTLE_PARTY_PANEL;
            }
        }
        if !ctx.flags.contains(FreecamFlags::HOOKED_ROADMAP) {
            if crate::field::try_hook_roadmap() {
                ctx.flags |= FreecamFlags::HOOKED_ROADMAP;
            }
        }
        TaskFunctionReturn::Continue
    }
    fn shutdown(_task: &mut GfdTask<GfdAllocator, Self>) -> ()
    where Self: Sized {}
}

impl InitTask for Freecam {
    fn new() -> Self where Self: Sized {
        Self {
            flags: FreecamFlags::empty(),
            pan: 0.,
            pitch: 0.,
            roll: 0.,
            camera_pos: Vec3A::ZERO,
            lookat_pos: Vec3A::ZERO,
            up_vec: Vec3A::Y,
            nodes: vec![],
            node_path_time: NODE_PATH_DEFAULT_TIME,
            node_path_current: 0.,
            node_path_percent: 0.,
            last_interp: FreecamNode::default(),
            evt_return: FreecamNode::default(),
        }
    }
}