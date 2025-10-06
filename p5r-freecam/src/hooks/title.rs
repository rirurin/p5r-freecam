use std::ptr::NonNull;
use std::sync::OnceLock;
use opengfd::kernel::allocator::GfdAllocator;
use riri_mod_tools_proc::riri_hook_fn;
use riri_mod_tools_rt::{logln, sigscan_resolver};
use crate::state::camera::Freecam;
use opengfd::kernel::task::Task as GfdTask;

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

#[no_mangle]
pub unsafe extern "C" fn setFUN_141475500(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v, None => return None
    };
    let addr = match sigscan_resolver::get_indirect_address_short_abs(addr.as_ptr().add(0x3f)) {
        Some(v) => v, None => return None
    };
    logln!(Information, "got FUN_141475500: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_fn(dynamic_offset(
    signature = "48 89 5C 24 ?? 57 48 83 EC 60 8B 1A",
    resolve_type = setFUN_141475500,
    calling_convention = "microsoft",
))]
pub unsafe extern "C" fn FUN_141475500(a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize, a7: usize, a8: usize, a9: usize) {
    if !Freecam::check_active() {
        original_function!(a1, a2, a3, a4, a5, a6, a7, a8, a9)
    }
}