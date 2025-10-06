use crate::gui::app::AppGlobalFlags;
use glam::Vec2;
use imgui::{
    BackendFlags,
    Context as ImContext,
    // FotnCon
    // FontSource,
    Key,
    MouseButton
};
use std::{
    mem::MaybeUninit,
    time::Instant
};
use riri_mod_tools_rt::logln;
use windows::{
    core::{ BOOL, PCWSTR },
    Win32::{
        Foundation::{ HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM },
        Graphics::Gdi::ScreenToClient,
        System::LibraryLoader::GetModuleHandleA,
        UI::{
            Input::KeyboardAndMouse::VIRTUAL_KEY,
            WindowsAndMessaging::{
                CreateWindowExW,
                DefWindowProcW,
                DestroyWindow,
                GetClientRect,
                GetCursorPos,
                GetForegroundWindow,
                HICON,
                ICON_SMALL,
                RegisterClassW,
                SetWindowTextW,
                SendMessageW,
                ShowWindow,
                SHOW_WINDOW_CMD,
                SIZE_MINIMIZED,
                WHEEL_DELTA,
                WM_CHAR,
                WM_CLOSE,
                WM_MOUSEMOVE,
                WM_NCMOUSEMOVE,
                WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
                WM_LBUTTONDOWN, WM_LBUTTONDBLCLK,
                WM_RBUTTONDOWN, WM_RBUTTONDBLCLK,
                WM_MBUTTONDOWN, WM_MBUTTONDBLCLK,
                WM_XBUTTONDOWN, WM_XBUTTONDBLCLK,
                WM_MOUSEWHEEL, WM_MOUSEHWHEEL,
                WM_LBUTTONUP,
                WM_RBUTTONUP,
                WM_MBUTTONUP,
                WM_XBUTTONUP,
                WM_SETFOCUS, WM_KILLFOCUS,
                WM_SETICON,
                WM_SIZE,
                WM_SIZING,
                WNDCLASSW,
                WINDOW_EX_STYLE,
                WS_OVERLAPPEDWINDOW,
                UnregisterClassW,
                XBUTTON1,
            },
        }
    },
};

const WINDOW_CLASS: &'static str = "P5R_FREECAM_GUI\0";
const WINDOW_NAME: &'static str = "P5R Freecam GUI\0";

#[derive(Debug)]
pub struct Window {
    class_name: Vec<u16>,
    window_name: Vec<u16>,
    handle: HWND,
    instance: HINSTANCE,
}

type WndProc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

// Adapted from imgui D3D11 + Win32 example:
// https://github.com/ocornut/imgui/blob/master/examples/example_win32_directx11/main.cpp

impl Window {
    pub unsafe fn new(wnd_proc: WndProc, pos: Vec2, size: Vec2) -> windows::core::Result<Self> {
        let mut class = WNDCLASSW::default();
        let class_name: Vec<u16> = WINDOW_CLASS.encode_utf16().collect();
        let window_name: Vec<u16> = WINDOW_NAME.encode_utf16().collect();
        let instance = GetModuleHandleA(None)?.into();
        class.lpfnWndProc = Some(wnd_proc);
        class.hInstance = instance;
        class.lpszClassName = PCWSTR(class_name.as_ptr());
        RegisterClassW(&raw const class);

        let handle = CreateWindowExW(WINDOW_EX_STYLE(0),
                                     PCWSTR(class_name.as_ptr()), PCWSTR(class_name.as_ptr()), WS_OVERLAPPEDWINDOW,
                                     pos.x as i32, pos.y as i32, size.x as i32, size.y as i32, None, None, Some(instance), None)?;
        Ok(Self { class_name, window_name, handle, instance })
    }

    pub fn show(&mut self) -> bool {
        unsafe {
            SetWindowTextW(self.handle, PCWSTR(self.window_name.as_ptr())).unwrap();
            ShowWindow(self.handle, SHOW_WINDOW_CMD(1)).into()
        }
    }

    pub fn set_icon(&mut self, icon: HICON) {
        unsafe {
            SendMessageW(self.get_handle(), WM_SETICON, Some(WPARAM(ICON_SMALL as usize)), Some(LPARAM(icon.0 as isize)));
        }
    }

    pub fn get_handle(&self) -> HWND { self.handle }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.handle).unwrap();
            UnregisterClassW(PCWSTR(self.class_name.as_ptr()), Some(self.instance)).unwrap();
        }
    }
}

// From riri-imgui-hook

#[allow(dead_code)]
pub(crate) static XINPUT_DLL: [&'static str; 5] = [
    "xinput1_4.dll\0", // Windows 8+
    "xinput1_3.dll\0", // DirectX SDK
    "xinput9_1_0.dll\0", // Windows Vista/Windows 7
    "xinput1_2.dll\0", // DirectX SDK
    "xinput1_1.dll\0" // DirectX SDK
];

#[allow(unused)]
fn wparam_get_low_word(wparam: WPARAM) -> u16 {
    (wparam.0 & u16::MAX as usize) as u16
}
#[allow(unused)]
fn wparam_get_high_word(wparam: WPARAM) -> u16 {
    ((wparam.0 >> u16::BITS as usize) & u16::MAX as usize) as u16
}
#[allow(unused)]
fn lparam_get_low_word(lparam: LPARAM) -> i16 {
    (lparam.0 & u16::MAX as isize) as i16
}
fn lparam_get_high_word(lparam: LPARAM) -> i16 {
    ((lparam.0 >> u16::BITS as isize) & u16::MAX as isize) as i16
}

#[derive(Debug)]
pub struct Win32Impl {
    last_frame: Instant,
    hwnd: HWND,
}

unsafe impl Send for Win32Impl {}
unsafe impl Sync for Win32Impl {}

impl Win32Impl {
    pub fn new(ctx: &mut ImContext, hwnd: HWND) -> Self {
        let platform_name = format!("riri-imgui-hook-win32");
        ctx.set_platform_name(Some(platform_name));
        let io = ctx.io_mut();
        io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
        Self { last_frame: Instant::now(), hwnd }
    }

    pub fn new_frame(&mut self, ctx: &mut ImContext) {
        let io = ctx.io_mut();
        // Set display size
        let mut rect = MaybeUninit::uninit();
        unsafe { GetClientRect(self.hwnd, rect.as_mut_ptr()).unwrap() };
        let rect = unsafe { rect.assume_init() };
        io.display_size = [(rect.right - rect.left) as f32, (rect.bottom - rect.top) as f32];

        // Set time
        let new_time = Instant::now();
        io.delta_time = new_time.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = new_time;

        // Update IO
        self.update_mouse_pos(ctx);
    }

    fn update_mouse_pos(&mut self, ctx: &mut ImContext) {
        if unsafe { GetForegroundWindow() } == self.hwnd {
            let io = ctx.io_mut();
            let mut point = MaybeUninit::uninit();
            unsafe {
                GetCursorPos(point.as_mut_ptr()).unwrap();
                if ScreenToClient(self.hwnd, point.as_mut_ptr()).into() {
                    let point = point.assume_init();
                    let point_pos = [point.x as f32, point.y as f32];
                    io.add_mouse_pos_event(point_pos);
                }
            }
        }
    }

    // Map VK_xxx to ImGuiKey_xxx.
    fn from_key_event(wparam: WPARAM, lparam: LPARAM) -> Option<Key> {
        // There is no distinct VK_xxx for keypad enter, instead it is VK_RETURN + KF_EXTENDED.
        match VIRTUAL_KEY(wparam.0 as u16) {
            windows::Win32::UI::Input::KeyboardAndMouse::VK_TAB => Some(Key::Tab),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_LEFT => Some(Key::LeftArrow),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_RIGHT => Some(Key::RightArrow),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_UP => Some(Key::UpArrow),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_DOWN => Some(Key::DownArrow),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_PRIOR => Some(Key::PageUp),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NEXT => Some(Key::PageDown),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_HOME => Some(Key::Home),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_END => Some(Key::End),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_INSERT => Some(Key::Insert),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_DELETE => Some(Key::Delete),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_BACK => Some(Key::Backspace),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_SPACE => Some(Key::Space),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_RETURN => Some(Key::Enter),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE => Some(Key::Escape),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_OEM_COMMA => Some(Key::Comma),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_OEM_PERIOD => Some(Key::Period),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_CAPITAL => Some(Key::CapsLock),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_SCROLL => Some(Key::ScrollLock),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMLOCK => Some(Key::NumLock),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_SNAPSHOT => Some(Key::PrintScreen),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_PAUSE => Some(Key::Pause),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD0 => Some(Key::Keypad0),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD1 => Some(Key::Keypad1),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD2 => Some(Key::Keypad2),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD3 => Some(Key::Keypad3),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD4 => Some(Key::Keypad4),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD5 => Some(Key::Keypad5),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD6 => Some(Key::Keypad6),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD7 => Some(Key::Keypad7),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD8 => Some(Key::Keypad8),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_NUMPAD9 => Some(Key::Keypad9),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_DECIMAL => Some(Key::KeypadDecimal),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_DIVIDE => Some(Key::KeypadDivide),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_MULTIPLY => Some(Key::KeypadMultiply),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_SUBTRACT => Some(Key::KeypadSubtract),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_ADD => Some(Key::KeypadAdd),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_LSHIFT => Some(Key::LeftShift),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_LCONTROL => Some(Key::LeftCtrl),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_LMENU => Some(Key::LeftAlt),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_LWIN => Some(Key::LeftSuper),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_RSHIFT => Some(Key::RightShift),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_RCONTROL => Some(Key::RightCtrl),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_RMENU => Some(Key::RightAlt),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_RWIN => Some(Key::RightSuper),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_APPS => Some(Key::Menu),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_0 => Some(Key::Alpha0),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_1 => Some(Key::Alpha1),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_2 => Some(Key::Alpha2),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_3 => Some(Key::Alpha3),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_4 => Some(Key::Alpha4),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_5 => Some(Key::Alpha5),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_6 => Some(Key::Alpha6),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_7 => Some(Key::Alpha7),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_8 => Some(Key::Alpha8),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_9 => Some(Key::Alpha9),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_A => Some(Key::A),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_B => Some(Key::B),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_C => Some(Key::C),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_D => Some(Key::D),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_E => Some(Key::E),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F => Some(Key::F),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_G => Some(Key::G),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_H => Some(Key::H),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_I => Some(Key::I),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_J => Some(Key::J),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_K => Some(Key::K),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_L => Some(Key::L),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_M => Some(Key::M),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_N => Some(Key::N),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_O => Some(Key::O),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_P => Some(Key::P),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_Q => Some(Key::Q),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_R => Some(Key::R),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_S => Some(Key::S),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_T => Some(Key::T),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_U => Some(Key::U),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_V => Some(Key::V),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_W => Some(Key::W),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_X => Some(Key::X),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_Y => Some(Key::Y),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_Z => Some(Key::Z),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F1 => Some(Key::F1),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F2 => Some(Key::F2),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F3 => Some(Key::F3),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F4 => Some(Key::F4),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F5 => Some(Key::F5),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F6 => Some(Key::F6),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F7 => Some(Key::F7),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F8 => Some(Key::F8),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F9 => Some(Key::F9),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F10 => Some(Key::F10),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F11 => Some(Key::F11),
            windows::Win32::UI::Input::KeyboardAndMouse::VK_F12 => Some(Key::F12),
            _ => {
                // Fallback to scancode
                // https://handmade.network/forums/t/2011-keyboard_inputs_-_scancodes,_raw_input,_text_input,_key_names
                let scancode = lparam_get_high_word(lparam) as u16 & u8::MAX as u16;
                match scancode {
                    41 => Some(Key::GraveAccent), // VK_OEM_8 in EN-UK, VK_OEM_3 in EN-US, VK_OEM_7 in FR, VK_OEM_5 in DE, etc.
                    12 => Some(Key::Minus),
                    13 => Some(Key::Equal),
                    26 => Some(Key::LeftBracket),
                    27 => Some(Key::RightBracket),
                    43 => Some(Key::Backslash),
                    39 => Some(Key::Semicolon),
                    40 => Some(Key::Apostrophe),
                    51 => Some(Key::Comma),
                    52 => Some(Key::Period),
                    53 => Some(Key::Slash),
                    _ => None
                }
            }
        }
    }

    pub(crate) unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        // window events can get called after imgui structures are dropped
        if (*crate::gui::app::APP_GLB.lock().unwrap()).flags.contains(AppGlobalFlags::WINDOW_CLOSED) {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        } else if msg == WM_CLOSE {
            (*crate::gui::app::APP_GLB.lock().unwrap()).flags |= AppGlobalFlags::WINDOW_CLOSED;
            return LRESULT(0);
        }
        if let Some(p) = crate::gui::app::PLATFORM_GLB.get() {
            let platform = unsafe { &mut *p.as_ptr() };
            if let Some(i) = crate::gui::app::IMGUI_GLB.get() {
                let imgui = unsafe { &mut *i.as_ptr() };
                return match platform.wnd_proc_managed(imgui, msg, wparam, lparam) {
                    Some(res) => res,
                    None => DefWindowProcW(hwnd, msg, wparam, lparam)
                }
            }
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    pub unsafe fn wnd_proc_managed(&mut self, ctx: &mut ImContext, umsg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<LRESULT> {
        match umsg {
            WM_MOUSEMOVE | WM_NCMOUSEMOVE => {
                // We need to call TrackMouseEvent in order to receive WM_MOUSELEAVE events
                // let area = if umsg == WM_MOUSEMOVE { 1 } else { 2 };
                // ...
                let mut mouse_pos = POINT {
                    x: (lparam.0 & u16::MAX as isize) as i32,
                    y: ((lparam.0 >> u16::BITS as isize) & u16::MAX as isize) as i32,
                };
                if umsg != WM_NCMOUSEMOVE || ScreenToClient(self.hwnd, &raw mut mouse_pos) == BOOL(1) {
                    let io = ctx.io_mut();
                    io.add_mouse_pos_event([mouse_pos.x as f32, mouse_pos.y as f32]);
                }
                None
            },
            WM_LBUTTONDOWN | WM_LBUTTONDBLCLK |
            WM_RBUTTONDOWN | WM_RBUTTONDBLCLK |
            WM_MBUTTONDOWN | WM_MBUTTONDBLCLK |
            WM_XBUTTONDOWN | WM_XBUTTONDBLCLK => {
                let io = ctx.io_mut();
                let mouse_button = match umsg {
                    WM_LBUTTONDOWN | WM_LBUTTONDBLCLK => MouseButton::Left,
                    WM_RBUTTONDOWN | WM_RBUTTONDBLCLK => MouseButton::Right,
                    WM_MBUTTONDOWN | WM_MBUTTONDBLCLK => MouseButton::Middle,
                    WM_XBUTTONDOWN | WM_XBUTTONDBLCLK => {
                        let xbutton_param = ((wparam.0 >> u16::BITS as usize) & u16::MAX as usize) as u16;
                        if xbutton_param == XBUTTON1 { MouseButton::Extra1 } else { MouseButton::Extra2 }
                    },
                    _ => todo!()
                };
                io.add_mouse_button_event(mouse_button, true);
                if io.want_capture_mouse {
                    Some(LRESULT(0))
                } else { None }
            },
            WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => {
                let io = ctx.io_mut();
                let mouse_button = match umsg {
                    WM_LBUTTONUP => MouseButton::Left,
                    WM_RBUTTONUP => MouseButton::Right,
                    WM_MBUTTONUP => MouseButton::Middle,
                    WM_XBUTTONUP => {
                        let xbutton_param = ((wparam.0 >> u16::BITS as usize) & u16::MAX as usize) as u16;
                        if xbutton_param == XBUTTON1 { MouseButton::Extra1 } else { MouseButton::Extra2 }
                    },
                    _ => todo!()
                };
                io.add_mouse_button_event(mouse_button, false);
                if io.want_capture_mouse {
                    Some(LRESULT(0))
                } else { None }
            },
            WM_MOUSEWHEEL => {
                let io = ctx.io_mut();
                let delta = wparam_get_high_word(wparam) as i16 as f32 / WHEEL_DELTA as f32;
                io.add_mouse_wheel_event([0.0, delta]);
                if io.want_capture_mouse {
                    Some(LRESULT(0))
                } else { None }
            },
            WM_MOUSEHWHEEL => {
                let io = ctx.io_mut();
                let delta = wparam_get_high_word(wparam) as i16 as f32 / WHEEL_DELTA as f32;
                io.add_mouse_wheel_event([-delta, 0.0]);
                if io.want_capture_mouse {
                    Some(LRESULT(0))
                } else { None }
            },
            WM_SETFOCUS | WM_KILLFOCUS => {
                let io = ctx.io_mut();
                io.app_focus_lost = umsg == WM_KILLFOCUS;
                None
            },
            WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => {
                let is_key_down = umsg == WM_KEYDOWN || umsg == WM_SYSKEYDOWN;
                if wparam.0 < 256 {
                    // self.update_key_modifiers(ctx);
                    if let Some(key) = Self::from_key_event(wparam, lparam) {
                        let io = ctx.io_mut();
                        io.add_key_event(key, is_key_down);
                    }
                }
                let io = ctx.io_mut();
                if io.want_capture_keyboard {
                    Some(LRESULT(0))
                } else { None }
            },
            WM_CHAR => {
                let io = ctx.io_mut();
                if wparam.0 > 0 && wparam.0 <= u16::MAX as usize {
                    let as_utf8: Vec<char> = std::char::decode_utf16([wparam.0 as u16])
                        .map(|c| c.unwrap_or(std::char::REPLACEMENT_CHARACTER))
                        .collect();
                    io.add_input_character(as_utf8[0]);
                }
                if io.want_text_input {
                    Some(LRESULT(0))
                } else { None }
            },
            // resize window
            WM_SIZE => {
                if wparam.0 as u32 == SIZE_MINIMIZED { None }
                else {
                    let mut glb_lock = crate::gui::app::APP_GLB.lock().unwrap();
                    let tgtw = lparam_get_low_word(lparam) as u32;
                    let tgth = lparam_get_high_word(lparam) as u32;
                    glb_lock.set_resize(tgtw, tgth);
                    drop(glb_lock);
                    None
                }
            },
            WM_SIZING => {
                // let rect = &mut *(lparam.0 as *mut RECT);
                let mut glb_lock = crate::gui::app::APP_GLB.lock().unwrap();
                let mut rect: MaybeUninit<RECT> = MaybeUninit::uninit();
                GetClientRect(self.hwnd, rect.assume_init_mut()).unwrap();
                let rect = rect.assume_init();
                let tgtw = (rect.right - rect.left) as u32;
                let tgth = (rect.bottom - rect.top) as u32;
                glb_lock.set_resize(tgtw, tgth);
                drop(glb_lock);
                if let Err(e) = crate::gui::app::App::frame(self.hwnd, 0.) {
                    logln!(Verbose, "Error while drawing moving window: {}", e);
                }
                // logln!(Verbose, "resizing: {:?}", rect);
                Some(LRESULT(0))
            },
            // unhandled events
            _u => None
        }
    }
}