use glam::{ Vec2, Vec3, Vec4, Vec4Swizzles };
use imgui::{
    Context as ImContext,
    FontConfig,
    FontId,
    FontGlyphRanges,
    FontSource,
    Key,
    Ui
};
use std::{
    error::Error,
    fmt::Display,
    mem::MaybeUninit,
    path::Path,
};
use windows::Win32::{
    Foundation::{ HWND, RECT },
    UI::WindowsAndMessaging::GetClientRect,
};

#[derive(Debug)]
pub struct AppError(String);
impl AppError {
    pub fn new<T>(text: T) -> Self where T: AsRef<str> { Self(text.as_ref().to_owned()) }
    pub fn new_owned(text: String) -> Self { Self(text) }
}
impl Error for AppError {}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError: {}", self.0)
    }
}

pub fn from_hsv(h: f32, s: f32, v: f32) -> Vec3 {
    let c = Vec3::new(h, s, v);
    // From Metaphor Refantazio HLSL shader source (45.HLSL)
    let k = Vec4::new(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p = ((c.x + k.xyz()).fract() * 6.0 - k.w).abs();
    c.z * k.xxx().lerp((p - k.x).clamp(Vec3::ZERO, Vec3::ONE), c.y)
}

pub fn load_font<P>(ctx: &mut ImContext, path: P, range: FontGlyphRanges, size: f32) -> Result<FontId, Box<dyn Error>>
where P: AsRef<Path> {
    let font_data = match std::fs::read(path.as_ref()) {
        Ok(f) => f, Err(_) => return Err(Box::new(AppError::new("Custom font is missing! Falling back to default font"))),
    };
    let mut font_config = FontConfig::default();
    font_config.glyph_ranges = range;
    Ok(ctx.fonts().add_font(&[FontSource::TtfData { data: font_data.as_slice(), size_pixels: size, config: Some(font_config) }]))
}

pub fn get_window_size(hwnd: HWND) -> Result<Vec2, Box<dyn Error>> {
    let mut area: MaybeUninit<RECT> = MaybeUninit::uninit();
    unsafe { GetClientRect(hwnd, area.as_mut_ptr())? };
    let area = unsafe { area.assume_init() };
    Ok(Vec2::new((area.right - area.left) as f32, (area.bottom - area.top) as f32))
}

#[derive(Debug)]
pub struct Shortcut<C> {
    sequence: Vec<Key>,
    point: usize,
    callback: fn(&mut C) -> Result<bool, Box<dyn Error>>
}

impl<C> Shortcut<C> {
    pub fn new(sequence: Vec<Key>, callback: fn(&mut C) -> Result<bool, Box<dyn Error>>) -> Self {
        Self { sequence, point: 0, callback }
    }

    pub fn frame(ctx: &mut C, ui: &Ui, shortcuts: &mut [Self]) -> Result<bool, Box<dyn Error>> {
        // Check keyboard shortcuts
        let mut from_shortcut = None;
        // let frame_num = app.frame_count;
        for s in shortcuts {
            let last_key = s.point + 1 == s.sequence.len();
            if match last_key {
                true => ui.is_key_released(s.sequence[s.point]),
                false => ui.is_key_down(s.sequence[s.point])
            } {
                match last_key {
                    true => {
                        from_shortcut = Some(s.callback);
                        s.point = 0;
                    },
                    false => s.point += 1
                };
            } else {
                s.point = 0;
            }
        }
        match from_shortcut {
            Some(cb) => cb(ctx),
            None => Ok(true)
        }
    }
}