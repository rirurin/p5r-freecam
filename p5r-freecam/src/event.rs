use crate::camera::{Freecam, FreecamFlags, FreecamNode};
use opengfd::kernel::{
    allocator::GfdAllocator,
    task::{ Task as GfdTask, UpdateTask }
};
use riri_mod_tools_proc::{create_hook, riri_hook_fn, riri_hook_static, riri_static};
use riri_mod_tools_rt::{ logln, sigscan_resolver };
use std::ptr::NonNull;
use glam::{EulerRot, Quat, Vec3A, Vec4};
use xrd744_lib::evt::function_table::{EvtCommandTable, EVT_COMMAND_COUNT};
use xrd744_lib::evt::task::EvtTask;

#[no_mangle]
pub unsafe extern "C" fn set_event_command_name(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v, None => return None
    };
    let get_cmd_table = addr.as_ptr() as *mut *mut u8;
    for i in 0..EVT_COMMAND_COUNT {
        let table_addr = sigscan_resolver::get_indirect_address_long_abs(*get_cmd_table.add(i));
        match table_addr {
            Some(v) => {
                let table = &*(v.as_ptr() as *mut EvtCommandTable);
                let ptr = table.get_execute_table().get_func_0() as usize;
                let ptr = match riri_mod_tools_rt::sigscan_resolver::get_address_may_thunk_absolute(ptr) {
                    Some(v) => unsafe { v.as_ptr() as usize }, None => 0
                };
                match table.get_name() {
                    "CMD_" => {
                        logln!(Verbose, "Created hook for event command CAMERA MOVE DIRECT: 0x{:x}", ptr);
                        create_hook!(ptr, camera_move_direct_func_1_0);
                    },
                    "CSA_" => {
                        logln!(Verbose, "Created hook for event command CAMERA SET ASSET: 0x{:x}", ptr);
                        create_hook!(ptr, camera_set_asset_func_1_0);
                    },
                    "CSD_" => {
                        logln!(Verbose, "Created hook for event command CAMERA SET DIRECT: 0x{:x}", ptr);
                        create_hook!(ptr, camera_set_direct_func_1_0);
                    },
                    /*
                    "CShk" => {
                        logln!(Verbose, "Created hook for event command CAMERA SHAKE: 0x{:x}", ptr);
                        create_hook!(ptr, camera_shake_func_1_0);
                    },
                    */
                    /*
                    "CSF_" => {
                        logln!(Verbose, "Created hook for event command CAMERA SET FIELD: 0x{:x}", ptr);
                        create_hook!(ptr, camera_set_field_func_1_0);
                    },
                    */
                    _ => ()
                }
            },
            None => {
                logln!(Error, "TABLE FAILED: Entry {}", i);
            }
        }
    }
    logln!(Information, "got evtCommandTable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

// app::fld::fldWorld::CreateWorld
#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "4C 8D 2D ?? ?? ?? ?? 48 89 5C 24 ?? 66 66 66 0F 1F 84 ?? 00 00 00 00",
    resolve_type = set_event_command_name,
    calling_convention = "microsoft",
))]
riri_static!(EVENT_COMMAND_TYPES_HOOK, usize);

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn camera_move_direct_func_1_0(p_work: *mut u8) -> bool {
    logln!(Verbose, "evtCommandRequest: CAMERA MOVE DIRECT");
    match GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME) {
        Some(t) => {
            let work = t.get_main_work_mut().unwrap();
            match work.flags.contains(FreecamFlags::ACTIVE) {
                true => true,
                false => original_function!(p_work)
            }
        },
        None => original_function!(p_work)
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn camera_set_asset_func_1_0(p_work: *mut u8) -> bool {
    logln!(Verbose, "evtCommandRequest: CAMERA SET ASSET");
    match GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME) {
        Some(t) => {
            let work = t.get_main_work_mut().unwrap();
            match work.flags.contains(FreecamFlags::ACTIVE) {
                true => true,
                false => original_function!(p_work)
            }
        },
        None => original_function!(p_work)
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn camera_set_direct_func_1_0(p_work: *mut u8) -> bool {
    logln!(Verbose, "evtCommandRequest: CAMERA SET DIRECT");
    match GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME) {
        Some(t) => {
            let work = t.get_main_work_mut().unwrap();
            match work.flags.contains(FreecamFlags::ACTIVE) {
                true => true,
                false => original_function!(p_work)
            }
        },
        None => original_function!(p_work)
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn camera_shake_func_1_0(p_work: *mut u8) -> bool {
    logln!(Verbose, "evtCommandRequest: CAMERA SHAKE");
    match GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME) {
        Some(t) => {
            let work = t.get_main_work_mut().unwrap();
            match work.flags.contains(FreecamFlags::ACTIVE) {
                true => true,
                false => original_function!(p_work)
            }
        },
        None => original_function!(p_work)
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn camera_set_field_func_1_0(p_work: *mut u8) -> bool {
    logln!(Verbose, "evtCommandRequest: CAMERA SET FIELD");
    match GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME) {
        Some(t) => {
            let work = t.get_main_work_mut().unwrap();
            match work.flags.contains(FreecamFlags::ACTIVE) {
                true => true,
                false => original_function!(p_work)
            }
        },
        None => original_function!(p_work)
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_evt_state_loop(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got EvtStateLoop: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_fn(dynamic_offset(
    signature = "48 89 4C 24 ?? 53 56 57 41 54 48 81 EC A8 0C 00 00",
    resolve_type = set_evt_state_loop,
    calling_convention = "microsoft",
))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn EvtStateLoop(p_work: *mut u8, delta: f32) -> u64 {
    let work = &mut *(p_work as *mut EvtTask);
    if let Some(ec) = work.get_ctrl_mut() {
        if let Some(cam) = ec.get_camera_mut() {
            if let Some(free) = GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME) {
                let ctx = free.get_main_work_mut().unwrap();
                if ctx.flags.contains(FreecamFlags::ACTIVE) {
                    if !ctx.flags.contains(FreecamFlags::EVT_SET_INITIAL_PARAMS) {
                        let inv = cam.get_view_transform().inverse();
                        (ctx.pan, ctx.pitch, ctx.roll) = inv.to_euler(EulerRot::YXZEx);
                        ctx.pan = if ctx.pan >= 0. { -(std::f32::consts::PI - ctx.pan) }
                        else { ctx.pan + std::f32::consts::PI };
                        let ret_rot = Quat::from_euler(EulerRot::YXZEx, ctx.pan, ctx.pitch, ctx.roll);
                        ctx.camera_pos = Vec3A::from_vec4(inv.mul_vec4(Vec4::new(0., 0., 0., 1.)));
                        ctx.evt_return = FreecamNode::new(ctx.camera_pos, ret_rot);
                        ctx.flags |= FreecamFlags::EVT_SET_INITIAL_PARAMS;
                    }
                    if ctx.flags.contains(FreecamFlags::PLAYING_PATH) {
                        ctx.set_position_from_last_interp(cam);
                    }
                    cam.set_view_transform(ctx.update_view_matrix());
                    cam.set_roll(ctx.get_roll());
                } else {
                    if ctx.flags.contains(FreecamFlags::EVT_SET_INITIAL_PARAMS) {
                        (ctx.pan, ctx.pitch, ctx.roll) = ctx.evt_return.rot.to_euler(EulerRot::YXZEx);
                        ctx.camera_pos = ctx.evt_return.trans;
                        cam.set_view_transform(ctx.update_view_matrix());
                        ctx.flags &= !FreecamFlags::EVT_SET_INITIAL_PARAMS;
                    }
                }
            }
        }
    }
    original_function!(p_work, delta)
}