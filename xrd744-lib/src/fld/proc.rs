use crate::fld::main::Main as FldMain;

type ProcFn = Option<fn(&FldMain) -> bool>;

#[repr(C)]
pub struct ProcTable {
    exec_func: ProcFn,
    check_func: ProcFn,
    ret_state: u32,
    flag: u32
}

impl ProcTable {
    pub fn get_exec_func(&self) -> ProcFn {
        self.exec_func
    }
    pub fn get_check_func(&self) -> ProcFn {
        self.check_func
    }
    pub fn get_return_state(&self) -> u32 {
        self.ret_state
    }
    pub fn get_flag(&self) -> u32 {
        self.flag
    }
}