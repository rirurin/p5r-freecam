use std::error::Error;
use std::num::NonZeroUsize;
use bitflags::bitflags;
use glam::{EulerRot, Mat4, Quat, Vec3A, Vec4};
use opengfd::io::controller::ControllerButton;
use opengfd::kernel::allocator::GfdAllocator;
use opengfd::kernel::graphics::GraphicsGlobal;
use opengfd::object::camera::Camera as GfdCamera;
use riri_mod_tools_rt::logln;
use xrd744_lib::btl::package::Package;
use crate::gui::utils::Shortcut;
use crate::state::node::FreecamNode;
use opengfd::kernel::task::{InitTask, Task as GfdTask, TaskFunctionReturn, UpdateTask};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_0, VK_ADD, VK_F4, VK_NUMPAD0, VK_OEM_MINUS, VK_OEM_PLUS, VK_SUBTRACT};
use xrd744_lib::fld::camera::Camera as FldCamera;
use glam::Vec4Swizzles;

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
        const OPENED_DEBUG_WINDOW = 1 << 2;
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
        const HOOKED_CASINO_COIN = 1 << 0x15;
        const CLOSED_DEBUG_WINDOW = 1 << 0x1f;
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
    pub(crate) return_node: FreecamNode,

    // GUI
    pub(crate) shortcuts: Vec<Shortcut<Self>>,
}

pub(crate) const FREQUENCY_SPEED_TICK: f32 = 0.1;
pub(crate) const NODE_PATH_DEFAULT_TIME: f32 = 3.0;
pub(crate) const NODE_PATH_STEP: f32 = 0.25;

pub(crate) const BSPLINE_DEGREE_QUADRATIC: usize = 2;

impl Freecam {

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

    fn toggle_freecam_mode(&mut self) -> Result<bool, Box<dyn Error>> {
        match self.flags.contains(FreecamFlags::ACTIVE) {
            true => self.disable_freecam_mode(),
            false => self.enable_freecam_mode(),
        };
        Ok(true)
    }

    fn disable_freecam_mode_sp(&mut self)  -> Result<bool, Box<dyn Error>> {
        self.disable_freecam_mode();
        Ok(true)
    }

    pub fn update_scene_speed(&mut self) {
        if Self::check_key_pressed(VK_OEM_MINUS) || Self::check_key_pressed(VK_SUBTRACT) {
            self.change_frequency_speed(true);
        } else if Self::check_key_pressed(VK_OEM_PLUS) || Self::check_key_pressed(VK_ADD) {
            self.change_frequency_speed(false);
        }
    }

    pub fn lock_camera_position(&mut self) {
        if Self::check_key_pressed(VK_0) || Self::check_key_pressed(VK_NUMPAD0) {
            self.flags ^= FreecamFlags::LOCK_CAMERA_MOVEMENT;
            match self.flags.contains(FreecamFlags::LOCK_CAMERA_MOVEMENT) {
                true => logln!(Verbose, "Camera is locked"),
                false => logln!(Verbose, "Camera is unlocked"),
            };
        }
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

    pub fn get_active() -> Option<&'static mut Freecam> {
        GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME).and_then(|task| {
            let ctx = task.get_main_work_mut().unwrap();
            match ctx.flags.contains(FreecamFlags::ACTIVE) {
                true => Some(ctx), false => None
            }
        })
    }

    pub fn change_frequency_speed(&self, slow: bool) {
        let glb = GraphicsGlobal::get_gfd_graphics_global_mut();
        if let Some(btl) = GfdTask::<GfdAllocator, Package>::find_by_str_mut("battle") {
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
    }

    pub fn change_node_path_time(&mut self, slow: bool) {
        let new = (self.node_path_time + if slow { -NODE_PATH_STEP } else { NODE_PATH_STEP }).max(NODE_PATH_STEP);
        logln!(Verbose, "New node path time: {:.02} sec", new);
        self.node_path_time = new;
    }

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
}

impl Freecam {
    fn update_ui_element_visibility(&mut self) {
        // hooked UI elements
        if !self.flags.contains(FreecamFlags::HOOKED_PANEL_MAP) {
            if crate::hooks::field::try_hook_panel_map() {
                self.flags |= FreecamFlags::HOOKED_PANEL_MAP;
            }
        }
        if !self.flags.contains(FreecamFlags::HOOKED_DATE_DRAW) {
            if crate::hooks::field::try_hook_date_draw() {
                self.flags |= FreecamFlags::HOOKED_DATE_DRAW;
            }
        }
        if !self.flags.contains(FreecamFlags::HOOKED_MISSION_DRAW) {
            if crate::hooks::field::try_hook_mission_draw() {
                self.flags |= FreecamFlags::HOOKED_MISSION_DRAW;
            }
        }
        if !self.flags.contains(FreecamFlags::HOOKED_BATTLE_PARTY_PANEL) {
            if crate::hooks::field::try_hook_party_panel() {
                self.flags |= FreecamFlags::HOOKED_BATTLE_PARTY_PANEL;
            }
        }
        if !self.flags.contains(FreecamFlags::HOOKED_ROADMAP) {
            if crate::hooks::field::try_hook_roadmap() {
                self.flags |= FreecamFlags::HOOKED_ROADMAP;
            }
        }
        if !self.flags.contains(FreecamFlags::HOOKED_CASINO_COIN) {
            if crate::hooks::field::try_hook_casino_coin() {
                self.flags |= FreecamFlags::HOOKED_CASINO_COIN;
            }
        }
    }

    pub(crate) fn get_scene_camera() -> Option<&'static GfdCamera> {
        let graphics = GraphicsGlobal::get_gfd_graphics_global();
        graphics.get_current_scene()
                .and_then(|scn| scn.get_current_camera())
    }

    pub(crate) fn get_scene_camera_mut() -> Option<&'static mut GfdCamera> {
        let graphics = GraphicsGlobal::get_gfd_graphics_global_mut();
        graphics.get_current_scene_mut()
            .and_then(|scn| scn.get_current_camera_mut())
    }
}

impl UpdateTask for Freecam {
    const NAME: &'static str = "Rirurin Freecam";
    fn update(task: &mut GfdTask<GfdAllocator, Self>, delta: f32)
              -> TaskFunctionReturn where Self: Sized {
        let ctx = task.get_main_work_mut().unwrap();
        // enable/disable freecam
        if Self::check_key_pressed(VK_F4) {
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
            if let Some(cam) = Self::get_scene_camera() {
                let inv = cam.get_view_transform().inverse();
                (ctx.pan, ctx.pitch, ctx.roll) = inv.to_euler(EulerRot::YXZEx);
                ctx.pan = match ctx.pan >= 0. {
                    true => -(std::f32::consts::PI - ctx.pan),
                    false => ctx.pan + std::f32::consts::PI
                };
                ctx.camera_pos = inv.w_axis.xyz().into();
                let ret_rot = Quat::from_euler(EulerRot::YXZEx, ctx.pan, ctx.pitch, ctx.roll);
                ctx.return_node = FreecamNode::new(ctx.camera_pos, ret_rot);
            }
            ctx.flags &= !FreecamFlags::SET_INITIAL_STATE;
        }
        if ctx.flags.contains(FreecamFlags::ACTIVE) {
            if let Some(cam) = Self::get_scene_camera_mut() {
                cam.set_view_transform(ctx.update_view_matrix());
                cam.set_roll(ctx.roll);
            }
        }
        // open window
        if !ctx.flags.contains(FreecamFlags::OPENED_DEBUG_WINDOW) {
            if let Err(e) = crate::gui::app::init_debug_window() {
                logln!(Error, "An error occurred while initializing the debugger GUI: {}", e);
            }
            ctx.flags |= FreecamFlags::OPENED_DEBUG_WINDOW;
        } else if !ctx.flags.contains(FreecamFlags::CLOSED_DEBUG_WINDOW) {
            match crate::gui::app::update_debug_window(delta) {
                Ok(s) => if !s {
                    crate::gui::app::shutdown_debug_window();
                    ctx.flags |= FreecamFlags::CLOSED_DEBUG_WINDOW;
                },
                Err(e) => logln!(Error, "An error occurred while executing the debugger GUI: {}", e),
            }
        }
        ctx.update_ui_element_visibility();
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
            return_node: FreecamNode::default(),
            shortcuts: vec![],
        }
    }
}