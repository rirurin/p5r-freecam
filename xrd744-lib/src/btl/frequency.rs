use opengfd::gfw::fade::FadeState;

#[repr(C)]
#[derive(Debug)]
pub struct Frequency {
    _cpp_vtable: *const u8,
    fade_state: FadeState,
    time_elapsed: f32,
    target_time: f32,
    target_time2: f32,
    to_value: f32,
    from_value: f32,
    target_value: f32,
    change_value_by: f32,
    default_value: f32
}

impl Frequency {
    pub fn get_time(&self) -> f32 { self.to_value }
    pub fn set_time(&mut self, time: f32) { self.to_value = time }
}