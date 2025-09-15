use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use glam::Vec3A;
use crate::gfw::camera::Camera as GfwCamera;
use opengfd::{
    gfw::{
        list::List,
        smartpointer::SmartPointer
    },
    kernel::allocator::GfdAllocator
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CameraControlFlags : u32 {
        const FLAG0 = 1 << 0;
        const FLAG1 = 1 << 1;
        const FLAG2 = 1 << 2;
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

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CameraType {
    Idle	= 0,
    Overview	= 1,
    Chantview	= 2,
    Skillview	= 3,
    Attackview	= 4,
    Pursuitview	= 5,
    Formationview	= 6,
    Encountview	= 7,
    Personaview	= 8,
    Commandview	= 9,
    Scriptview	= 10,
    Targetview	= 11,
    Approachview	= 12,
    Exchangeview	= 13,
    Victoryview	= 14,
    Wholeview	= 15,
    Moveview	= 16,
    Combinationview	= 17,
    PlayerSingleTargetview	= 18,
    PlayerPartyTargetview	= 19,
    PlayerSingleSkillview	= 20,
    SingleTargetSkillview	= 21,
    IncludeCameraSkillview	= 22,
    Talkview	= 23,
    Reinforcementview	= 24,
    PinchEscapeview	= 25,
    PinchTransitionview	= 26,
    HoldUpview	= 27,
    Criticalview	= 28,
    OneShotview	= 29,
    Changeview	= 30,
    RandomFireview	= 31,
    Statusview	= 32,
    Prepareview	= 33,
}
impl TryFrom<u32> for CameraType {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= CameraType::Prepareview as u32 {
            Ok(unsafe { std::mem::transmute(value )})
        } else {
            Err(())
        }
    }
}

#[repr(C)]
pub struct CameraController<A = GfdAllocator>
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    field8: *const u8,
    flags: CameraControlFlags,
    camera: SmartPointer<GfwCamera<A>, A>,
    cam_type: CameraType,
    field3c: u32,
    skill_id: u32,
    field44: u32,
    current_view: Vec3A,
    current_up: Vec3A,
    current_rad: f32,
    current_time: f32,
    current_seed: u32,
    field7c: f32,
    current_aspect: f32,
    current_near_clip: f32,
    current_far_clip: f32,
    chant_action: SmartPointer<usize, A>,
    thinking_action: SmartPointer<usize, A>,
    lock_on_action: SmartPointer<usize, A>,
    persona_unit: SmartPointer<usize, A>,
    persona_unit_id: u16,
    target_list: List<SmartPointer<usize, A>, A>,
    field138: u32,
    camera_sequence_anim: SmartPointer<usize, A>,
    eye_start: Vec3A,
    eye_end: Vec3A,
    target_start: Vec3A,
    target_end: Vec3A,
    new_view: Vec3A,
    new_up: Vec3A,
    end_time: f32,
    interpolate_type: u32,
    rotate_enable: bool,
    rotate_start: f32,
    rotate_end: f32,
    fov_enable: bool,
    fov_start: f32,
    fov_end: f32,
    dof_setting: SmartPointer<usize, A>,
    target_fov: f32,
    targett_animation_rate: f32,
    pause: bool,
    field209: [u8; 0x47],
    _allocator: A
}

impl<A> CameraController<A>
where A: Allocator + Clone
{
    pub fn get_camera_flags(&self) -> CameraControlFlags { self.flags }
    pub fn get_camera_flags_ptr(&self) -> *const CameraControlFlags { &raw const self.flags }
    pub fn get_camera_type(&self) -> CameraType { self.cam_type }

    pub fn get_current_view(&self) -> Vec3A { self.current_view }
    pub fn get_current_up(&self) -> Vec3A { self.current_up }
    pub fn get_current_time(&self) -> f32 { self.current_time }
    pub fn get_eye_pos(&self) -> Vec3A { self.eye_start }
    pub fn get_target_pos(&self) -> Vec3A { self.target_start }

    pub fn set_camera_flags(&mut self, value: CameraControlFlags) { self.flags = value }
    pub fn set_camera_type(&mut self, value: CameraType) { self.cam_type = value }

    pub fn get_camera(&self) -> &GfwCamera<A> { self.camera.get_data() }
    pub fn get_camera_mut(&mut self) -> &mut GfwCamera<A> { self.camera.get_data_mut() }
}