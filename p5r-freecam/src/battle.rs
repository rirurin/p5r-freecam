use crate::camera::{ Freecam, FreecamFlags };
use opengfd::kernel::{
    allocator::GfdAllocator,
    task::{ Task as GfdTask, UpdateTask }
};
use riri_mod_tools_proc::{ create_hook, riri_hook_fn, riri_mods_loaded_fn };
use riri_mod_tools_rt::{ logln, sigscan_resolver };
use std::ptr::NonNull;
use xrd744_lib::btl::camera::CameraController;

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn btl_camera_update(p_this: *mut u8, p_pkg: *mut u8, delta: f32) {
    match GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME) {
        Some(task) => {
            let ctx = task.get_main_work_mut().unwrap();
            if ctx.flags.contains(FreecamFlags::ACTIVE) {
                let this = &mut *(p_this as *mut CameraController<GfdAllocator>);
                if let Some(cam) = this.get_camera_mut().get_entity_mut() {
                    cam.set_view_transform(ctx.update_view_matrix());
                    cam.set_roll(ctx.get_roll());
                }
            } else {
                original_function!(p_this, p_pkg, delta)
            }
        },
        None => original_function!(p_this, p_pkg, delta)
    }
}
#[riri_mods_loaded_fn]
fn setup_battle_hooks() {
    let usually_vtable = riri_mod_tools_rt::vtable::get_vtable("Usually@camera@btl@@") as *const usize;
    let usually_tick = *usually_vtable.add(2);
    logln!(Information, "Got btl::camera::Usually::update at 0x{:x}", usually_tick);
    create_hook!(usually_tick, btl_camera_update);
}