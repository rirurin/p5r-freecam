use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use opengfd::{
    kernel::{
        allocator::GfdAllocator,
        task::Task as GfdTask
    },
    object::{
        camera::Camera as GfdCamera,
        node::Node as GfdNode
    }
};
use std::ptr::NonNull;
use glam::{Mat4, Quat, Vec3, Vec3A};
use riri_mod_tools_rt::logln;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CameraFlags : u32 {
        const FLAG0 = 1 << 0;
        const FLAG1 = 1 << 1;
        const CAMERA_LOCKED = 1 << 2;
        const FLAG3 = 1 << 3;
        const FLAG4 = 1 << 4;
        const FLAG5 = 1 << 5;
        const FLAG6 = 1 << 6;
        const FLAG7 = 1 << 7;
        const FLAG8 = 1 << 8;
        const FLAG9 = 1 << 9;
        const FLAG10 = 1 << 10;
        const FLAG11 = 1 << 11;
        const FLAG12 = 1 << 12;
        const FLAG13 = 1 << 13;
        const FLAG14 = 1 << 14;
        const FLAG15 = 1 << 15;
        const FLAG16 = 1 << 16;
        const FLAG17 = 1 << 17;
        const FLAG18 = 1 << 18;
        const FLAG19 = 1 << 19;
        const FLAG20 = 1 << 20;
        const FLAG21 = 1 << 21;
        const FLAG22 = 1 << 22;
        const FLAG23 = 1 << 23;
        const FLAG24 = 1 << 24;
        const FLAG25 = 1 << 25;
        const FLAG26 = 1 << 26;
        const FLAG27 = 1 << 27;
        const FLAG28 = 1 << 28;
        const FLAG29 = 1 << 29;
        const FLAG30 = 1 << 30;
        const FLAG31 = 1 << 31;
    }
}

#[repr(C)]
pub struct CameraPreset {
    dist_min: f32,
    dist_max: f32,
    pitch: f32,
    ofs: Vec3,
    field18: f32,
    field1c: f32
}

#[repr(C)]
pub struct AnalogMovement {
    amount: f32,
    on_time: f32,
    off_time: f32
}

#[repr(C)]
pub struct Camera<A = GfdAllocator>
where A: Allocator + Clone {
    handle_parent: NonNull<GfdTask<A, ()>>,
    step: u32,
    flag: CameraFlags,
    mode: i32,
    kep_mode: i32,
    set_type: i32,
    field1c: u32,
    field20: u32,
    delay_count: u32,
    follow_speed: f32,
    delta_time: f32,
    preset: CameraPreset,
    camera: Option<NonNull<GfdCamera<A>>>,
    target_node: Option<NonNull<GfdNode<A>>>,
    target_offset: Vec3A,
    target_offset_mark: Vec3A,
    target_pos: Vec3A,
    target_pos_history: Vec3A,
    target_pos_real: Vec3A,
    target_move: Vec3A,
    eye_pos: Vec3A,
    lookat_pos: Vec3A,
    fielde0: Vec3A,
    fieldf0: Vec3A,
    field100: Vec3A,
    field110: Vec3A,
    distance_min: f32,
    distance_min_mark: f32,
    distance_max: f32,
    distance_max_mark: f32,
    distance: f32,
    distance_mark: f32,
    fovy: f32,
    fovy_mark: f32,
    now_fovy: f32,
    now_fovy_mark: f32,
    default_fovy: f32,
    ofs_fovy: f32,
    down_ratio: f32,
    down: f32,
    down_mark: f32,
    blur_timer: f32,
    blur_move_ratio: f32,
    blur_dot: f32,
    blur_mode_ratio: f32,
    blur_power: f32,
    blur_fall_off: f32,
    blur_power_mark: f32,
    blur_fall_off_mark: f32,
    blur_power_default: f32,
    blur_fall_off_default: f32,
    pitch: f32,
    yaw: f32,
    field18c: f32,
    pitch_rad: f32,
    pitch_mark: f32,
    pitch_min: f32,
    pitch_max: f32,
    pitch_swing: f32,
    default_pitch: f32,
    correct_yaw: f32,
    correct_yaw_sign: f32,
    correct_ratio: f32,
    correct_speed: f32,
    correct_timer: f32,
    correct_on_time: f32,
    correct_diff_deg: f32,
    point_dist: f32,
    point_range: f32,
    point_ofs_ratio: f32,
    point_delay_time: f32,
    behind_radian: f32,
    behind_rot: f32,
    target_radian: f32,
    current_radian: f32,
    behind_time: f32,
    behind_duration: f32,
    pitch_timer: f32,
    pitch_duration: f32,
    pitch_s_rot: f32,
    pitch_e_rot: f32,
    pitch_kep_rot: f32,
    swing_timer: f32,
    swing_duration: f32,
    swing_s_rot: f32,
    swing_kep_rot: f32,
    slope_rot_mark: f32,
    slope_rot: f32,
    slope_rot_kep: f32,
    slope_rot_timer: f32,
    roll: f32,
    rotate_speed: f32,
    rotate_on_time: f32,
    rotate_off_time: f32,
    rotate_x: f32,
    rotate_kep_x: f32,
    rotate_y: f32,
    rotate_kep_y: f32,
    rotate_spd_cp: f32,
    rotate_on_time_cp: f32,
    rotate_y_speed: f32,
    rotate_y_on_time: f32,
    rotate_y_off_time: f32,
    field254: [u8; 0x14],
    lock_ref_count: u32,
    input_lock_ref_count: u32,
    lock_view: Mat4,
    lock_fovy: f32,
    lock_near: f32,
    lock_far: f32,
    save_pos: Vec3A,
    save_tgt: Vec3A,
    save_up: Vec3A,
    save_rot: Quat,
    save_pitch: f32,
    save_yaw: f32,
    save_fovy: f32,
    axis_rx: AnalogMovement,
    axis_ry: AnalogMovement,
    // ...
}

impl Camera {
    pub fn handle_freecam_onoff(active: bool) -> bool {
        match GfdTask::<GfdAllocator, Self>::find_by_str_mut("field camera CTRL") {
            Some(task) => {
                let ctx = task.get_main_work_mut().unwrap();
                logln!(Verbose, "field camera CTRL: 0x{:x}", &raw const *ctx as usize);
                ctx.flag.set(CameraFlags::CAMERA_LOCKED, active);
                true
            },
            None => false
        }
    }
}

impl<A> Camera<A>
where A: Allocator + Clone {
    pub fn get_gfd_camera(&self) -> Option<&GfdCamera<A>> {
        self.camera.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_gfd_camera_mut(&mut self) -> Option<&mut GfdCamera<A>> {
        self.camera.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_target_node(&self) -> Option<&GfdNode<A>> {
        self.target_node.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_target_node_mut(&mut self) -> Option<&mut GfdNode<A>> {
        self.target_node.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_pitch(&self) -> f32 {
        self.pitch_rad
    }
    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }
    pub fn get_target_offset(&self) -> Vec3A {
        self.target_offset
    }
    pub fn get_target_pos(&self) -> Vec3A {
        self.target_pos
    }
    pub fn get_lookat_pos(&self) -> Vec3A {
        self.lookat_pos
    }
    pub fn get_eye_pos(&self) -> Vec3A {
        self.eye_pos
    }
}