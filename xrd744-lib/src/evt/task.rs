use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use opengfd::{
    kernel::allocator::GfdAllocator,
    object::camera::Camera as GfdCamera,
    pak::file::PakFile
};

#[repr(C)]
pub struct EvtTask<A = GfdAllocator>
where A: Allocator + Clone {
    addr_id: u32,
    file_major_no: i32,
    file_minor_no: i32,
    file_path: [u8; 0x400],
    file_work: Option<NonNull<EvtFile>>,
    field418: *mut u8,
    ctrl: Option<NonNull<EvtCtrl<A>>>,
    current_frame: i32,
    start_frame: i32,
    task_phase: i32,
    audio_flags: u32,
    field438: u32,
    field43c: i32,
    field440: i32,
    field444: u32,
    file_flags: u32,
    field44c: u32,
    field450: [u8; 0x48],
    _allocator: A
}

impl<A> EvtTask<A>
where A: Allocator + Clone {
    pub fn get_file_work(&self) -> Option<&EvtFile> {
        self.file_work.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_file_work_mut(&mut self) -> Option<&mut EvtFile> {
        self.file_work.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_ctrl(&self) -> Option<&EvtCtrl<A>> {
        self.ctrl.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_ctrl_mut(&mut self) -> Option<&mut EvtCtrl<A>> {
        self.ctrl.map(|mut v| unsafe { v.as_mut() })
    }
}

#[repr(C)]
pub struct EvtFile {
    next: NonNull<Self>,
    flags: u32,
    listing_10: Option<NonNull<PakFile>>,
    evt_file_path: [u8; 0x400],
    evt_file_handle: Option<NonNull<PakFile>>,
    // ...
}

#[repr(C)]
pub struct EvtCtrl<A = GfdAllocator>
where A: Allocator + Clone {
    next: NonNull<Self>,
    camera: Option<NonNull<GfdCamera<A>>>,
    file_handle: NonNull<EvtFile>,
    command_list: *mut u8,
    flags: u32,
    field24: f32,
    frame_count: u32,
    // ...
    _allocator: A
}

impl<A> EvtCtrl<A>
where A: Allocator + Clone {
    pub fn get_camera_mut(&mut self) -> Option<&mut GfdCamera<A>> {
        self.camera.map(|mut v| unsafe { v.as_mut() })
    }
}