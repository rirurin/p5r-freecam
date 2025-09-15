use std::{
    ptr::NonNull,
    sync::OnceLock
};
use opengfd::kernel::{
    allocator::GfdAllocator,
    task::{
        Task as GfdTask,
        UpdateTask
    }
};
use opengfd::kernel::task::TaskFunctionReturn;
use riri_mod_tools_proc::{ create_hook, riri_hook_fn, riri_hook_static, riri_static };
use riri_mod_tools_rt::{logln, sigscan_resolver};
use crate::camera::{Freecam, FreecamFlags};
use xrd744_lib::fld::proc::ProcTable;

#[no_mangle]
pub unsafe extern "C" fn setfldPCMoveUpdate(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v, None => return None
    };
    logln!(Information, "got fldPCMoveUpdate: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_fn(dynamic_offset(
    signature = "40 53 48 83 EC 50 48 8B 59 ?? 0F 29 74 24 ?? 0F 28 F1",
    resolve_type = setfldPCMoveUpdate,
    calling_convention = "microsoft",
))]
pub unsafe extern "C" fn fldPCMoveUpdate(p_task: *mut u8, delta: f32) {
    if !Freecam::check_active() {
        let _ = original_function!(p_task, delta);
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn wholeExec(p_fldmain: *mut u8) -> bool {
    if !Freecam::check_active() {
        original_function!(p_fldmain)
    } else {
        false
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn saveExec(p_fldmain: *mut u8) -> bool {
    if !Freecam::check_active() {
        original_function!(p_fldmain)
    } else {
        false
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn logExec(p_fldmain: *mut u8) -> bool {
    if !Freecam::check_active() {
        original_function!(p_fldmain)
    } else {
        false
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn campExec(p_fldmain: *mut u8) -> bool {
    if !Freecam::check_active() {
        original_function!(p_fldmain)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn setGfdPlatformHook(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v, None => return None
    };
    crate::globals::set_platform_global(addr.as_ptr() as *mut _);
    logln!(Information, "got GfdPlatform: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    signature = "48 8D 0D ?? ?? ?? ?? BA 90 56 00 00",
    resolve_type = setGfdPlatformHook,
    calling_convention = "microsoft",
))]
riri_static!(GFD_PLATFORM_HOOK, usize);

#[no_mangle]
pub unsafe extern "C" fn setfldProcTable(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v, None => return None
    };
    let table_start = addr.as_ptr() as *const ProcTable;
    let camp = std::mem::transmute::<_, usize>((&*table_start.add(0xf)).get_exec_func());
    let whole = std::mem::transmute::<_, usize>((&*table_start.add(0x10)).get_exec_func());
    let save = std::mem::transmute::<_, usize>((&*table_start.add(0x11)).get_exec_func());
    let log = std::mem::transmute::<_, usize>((&*table_start.add(0x16)).get_exec_func());
    logln!(Information, "got fldProcTable: 0x{:x}", addr.as_ptr() as usize);
    logln!(Debug, "campExec: 0x{:x}, wholeExec: 0x{:x}, saveExec: 0x{:x}, logExec: 0x{:x}", camp, whole, save, log);
    create_hook!(camp, campExec);
    create_hook!(whole, wholeExec);
    create_hook!(save, saveExec);
    create_hook!(log, logExec);
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    signature = "48 8D 2D ?? ?? ?? ?? 66 21 83 ?? ?? ?? ??",
    resolve_type = setfldProcTable,
    calling_convention = "microsoft",
))]
riri_static!(FLD_PROC_TABLE, usize);

pub(crate) fn try_hook_panel_map() -> bool {
    // let task_name = "fld_panel_map";
    // let task_name = "road map(FLD)";
    let task_name = "fld_panel";
    if let Some(task) = GfdTask::<GfdAllocator, ()>::find_by_str_mut(task_name) {
        let update_fn = task.get_update_ptr() as usize;
        logln!(Verbose, "Hooked fldPanelMapUpdate: 0x{:x}", update_fn);
        create_hook!(update_fn, fldPanelMapUpdate);
        true
    } else {
        false
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn fldPanelMapUpdate(p_this: *mut u8, delta: f32) -> u64 {
    if !Freecam::check_active() {
        original_function!(p_this, delta)
    } else {
        TaskFunctionReturn::Continue as u64
    }
}

pub(crate) fn try_hook_date_draw() -> bool {
    if let Some(task) = GfdTask::<GfdAllocator, ()>::find_by_str_mut("date draw") {
        let update_fn = task.get_update_ptr() as usize;
        logln!(Verbose, "Hooked dateDrawUpdate: 0x{:x}", update_fn);
        create_hook!(update_fn, dateDrawUpdate);
        true
    } else {
        false
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn dateDrawUpdate(p_this: *mut u8, delta: f32) -> u64 {
    if !Freecam::check_active() {
        original_function!(p_this, delta)
    } else {
        TaskFunctionReturn::Continue as u64
    }
}

pub(crate) fn try_hook_mission_draw() -> bool {
    if let Some(task) = GfdTask::<GfdAllocator, ()>::find_by_str_mut("draw mission list(FLD)") {
        let update_fn = task.get_update_ptr() as usize;
        logln!(Verbose, "Hooked missionDrawUpdate: 0x{:x}", update_fn);
        create_hook!(update_fn, missionDrawUpdate);
        true
    } else {
        false
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn missionDrawUpdate(p_this: *mut u8, delta: f32) -> u64 {
    if !Freecam::check_active() {
        original_function!(p_this, delta)
    } else {
        TaskFunctionReturn::Continue as u64
    }
}

pub(crate) fn try_hook_party_panel() -> bool {
    if let Some(task) = GfdTask::<GfdAllocator, ()>::find_by_str_mut("btlPartyPanel") {
        let update_fn = task.get_update_ptr() as usize;
        logln!(Verbose, "Hooked btlPartyPanelUpdate: 0x{:x}", update_fn);
        create_hook!(update_fn, btlPartyPanelUpdate);
        true
    } else {
        false
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn btlPartyPanelUpdate(p_this: *mut u8, delta: f32) -> u64 {
    if !Freecam::check_active() {
        original_function!(p_this, delta)
    } else {
        TaskFunctionReturn::Continue as u64
    }
}

pub(crate) fn try_hook_roadmap() -> bool {
    if let Some(task) = GfdTask::<GfdAllocator, ()>::find_by_str_mut("road map(FLD)") {
        let update_fn = task.get_update_ptr() as usize;
        logln!(Verbose, "Hooked fldRoadmapUpdate: 0x{:x}", update_fn);
        create_hook!(update_fn, fldRoadmapUpdate);
        true
    } else {
        false
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn fldRoadmapUpdate(p_this: *mut u8, delta: f32) -> u64 {
    if !Freecam::check_active() {
        original_function!(p_this, delta)
    } else {
        TaskFunctionReturn::Continue as u64
    }
}

// fldPanelTipUpdate

pub(crate) fn try_hook_casino_coin() -> bool {
    if let Some(task) = GfdTask::<GfdAllocator, ()>::find_by_str_mut("fldPanelTipUpdate") {
        let update_fn = task.get_update_ptr() as usize;
        logln!(Verbose, "Hooked fldPanelTipUpdate: 0x{:x}", update_fn);
        create_hook!(update_fn, fldPanelTipUpdate);
        true
    } else {
        false
    }
}

#[riri_hook_fn(user_defined())]
pub unsafe extern "C" fn fldPanelTipUpdate(p_this: *mut u8, delta: f32) -> u64 {
    if !Freecam::check_active() {
        original_function!(p_this, delta)
    } else {
        TaskFunctionReturn::Continue as u64
    }
}