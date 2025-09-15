#[repr(C)]
pub struct MainParam {
    major: u16,
    minor: u16,
    env_major: u16,
    env_minor: u16,
    env_sub: u16,
    div_index: u16,
    pos_index: u16,
    mementos_section: u16,
    mementos_floor: u16,
    cl_total_day: u16,
    cl_time: u16,
    cl_weather: u16,
    field18: [u8; 0x10],
    field28: Option<fn()>
}

#[repr(C)]
pub struct Main {
    magic: [u8; 4], // FMWK
}