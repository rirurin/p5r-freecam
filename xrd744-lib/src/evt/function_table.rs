use std::{
    ffi::CStr,
    ptr::NonNull
};

pub const EVT_COMMAND_COUNT: usize = 121;

#[repr(C)]
#[derive(Debug)]
pub struct EvtCommandTable {
    name: *const i8,
    execute: NonNull<EvtCommandExecuteTable>,
    serial: Option<NonNull<EvtCommandSerialTable>>
}

impl EvtCommandTable {
    pub fn get_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap() }
    }
    pub fn get_execute_table(&self) -> &EvtCommandExecuteTable {
        unsafe { self.execute.as_ref() }
    }
    pub fn get_serial_table(&self) -> Option<&EvtCommandSerialTable> {
        unsafe { self.serial.map(|v| v.as_ref()) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EvtCommandExecuteTable {
    funcs: [*mut u8; 6],
    field30: u32,
    field34: u32,
    struct_size: u32,
    field3c: u32,
    field40: u32,
    field44: u32
}

impl EvtCommandExecuteTable {
    pub fn get_func_0(&self) -> *mut u8 {
        self.funcs[0]
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EvtCommandSerialTable {
    get_asset: fn(),
    stream_read: fn(),
    stream_write: fn()
}