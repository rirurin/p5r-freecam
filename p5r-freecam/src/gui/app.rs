use bitflags::bitflags;
use crate::gui::{
    d3d11::state::State as D3D11State,
    win32::{ Window, Win32Impl },
};
use glam::{ Vec2, Vec4 };
use imgui::{Context as ImContext, ConfigFlags, DrawData, FontGlyphRanges, FontId, Ui, Condition};
use implot::Context as ImPlotCtx;
use opengfd::utility::misc::RGBFloat;
use riri_mod_tools_rt::{address::ProcessInfo, mod_loader_data};
use std::{
    error::Error,
    mem::MaybeUninit,
    ops::{ Deref, DerefMut },
    path::PathBuf,
    ptr::NonNull,
    sync::{ Mutex, OnceLock },
    thread::sleep,
    time::Duration
};
use opengfd::kernel::{
    allocator::GfdAllocator,
    task::{ Task as GfdTask, UpdateTask }
};
use riri_file_dialog::dialog::FileDialogManager;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{ DXGI_STATUS_OCCLUDED, HWND },
        UI::WindowsAndMessaging::{
            DispatchMessageW,
            GetSystemMetrics,
            HICON,
            LoadIconA,
            MSG,
            PeekMessageW,
            PM_REMOVE,
            SM_CXSCREEN,
            SM_CYSCREEN,
            TranslateMessage,
        }
    }
};
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use crate::state::camera::Freecam;

#[derive(Debug)]
pub struct StatePointer<T>(NonNull<T>);
unsafe impl<T> Send for StatePointer<T> {}
unsafe impl<T> Sync for StatePointer<T> {}

impl<T> Deref for StatePointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl<T> DerefMut for StatePointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T> From<&mut T> for StatePointer<T> {
    fn from(value: &mut T) -> Self {
        Self(unsafe { NonNull::new_unchecked(&raw mut *value) })
    }
}

impl<T> From<*mut T> for StatePointer<T> {
    fn from(value: *mut T) -> Self {
        Self(unsafe { NonNull::new_unchecked(value) })
    }
}

impl<T> StatePointer<T> {
    pub fn as_ptr(&self) -> *mut T { self.0.as_ptr() }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct AppGlobalFlags : u8 {
        const WINDOW_CLOSED = 1 << 0;
        const WINDOW_OCCLUDED = 1 << 1;
    }
}

#[derive(Debug)]
pub struct App {
    // resize parameters
    width: u32,
    height: u32,
    // state bitflags
    pub(crate) flags: AppGlobalFlags,
    frame_count: usize,
    time_elapsed: f32,
    pub(crate) font: FontId,
}

unsafe impl Send for App {}
unsafe impl Sync for App {}

impl App {
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            flags: AppGlobalFlags::empty(),
            frame_count: 0,
            time_elapsed: 0.,
            font: unsafe { std::mem::transmute(std::ptr::null::<imgui::Font>()) }
        }
    }

    pub fn should_resize(&self) -> bool {
        self.width != 0 && self.height != 0
    }

    pub fn clear_resize(&mut self) {
        self.width = 0;
        self.height = 0;
    }

    pub fn set_resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn show_window(win_pos: Vec2, win_size: Vec2, icon: Option<HICON>) -> Result<(), Box<dyn Error>> {
        // Create window and imgui context
        let win = unsafe { &mut *Box::into_raw(Box::new(Window::new(Win32Impl::wnd_proc, win_pos, win_size)?)) };
        if ImContext::has_context() { // destroy any existing context
            ImContext::clear_context_raw();
        }

        let ctx = unsafe { &mut *Box::into_raw(Box::new(ImContext::create())) };
        let plot = unsafe { &mut *Box::into_raw(Box::new(ImPlotCtx::create())) };
        // Setup config
        let mod_dir: String = mod_loader_data::get_directory_for_mod().into();
        let mod_dir = PathBuf::from(mod_dir);
        ctx.set_ini_filename(mod_dir.join("debug_ui/imgui.ini"));
        ctx.set_log_filename(None);
        ctx.io_mut().config_flags |= ConfigFlags::NAV_ENABLE_KEYBOARD | ConfigFlags::DOCKING_ENABLE;

        // Tweak to ensure that platform windows look identical to regular ones
        ctx.style_mut().window_rounding = 0.;
        ctx.style_mut().colors[imgui::StyleColor::WindowBg as usize][3] = 1.;

        // Load fonts
        let mut app = APP_GLB.lock()?;
        app.font = crate::gui::utils::load_font(ctx, mod_dir.join("debug_ui/NotoSans-Medium.ttf"), FontGlyphRanges::japanese(), 15.)?;
        FileDialogManager::new(std::env::current_exe()?.parent().unwrap().to_path_buf(), win.get_handle());
        let platform = Box::into_raw(Box::new(Win32Impl::new(ctx, win.get_handle())));
        let renderer = unsafe { &mut *Box::into_raw(Box::new(D3D11State::new(ctx, win.get_handle())?)) };
        drop(app);
        let _ = IMGUI_GLB.set(ctx.into());
        let _ = PLATFORM_GLB.set(platform.into());
        let _ = RENDERER_GLB.set(renderer.into());
        let _ = WINDOW_GLB.set(win.into());
        let _ = IMPLOT_GLB.set(plot.into());
        if let Some(icon) = icon { win.set_icon(icon); }
        win.show();
        Ok(())
    }

    pub fn frame(hwnd: HWND, delta: f32) -> Result<bool, Box<dyn Error>> {
        // Handle window messages
        let mut msg: MaybeUninit<MSG> = MaybeUninit::uninit();
        unsafe {
            while PeekMessageW(msg.as_mut_ptr(), Some(hwnd), 0, 0, PM_REMOVE).into() {
                let _ = TranslateMessage(msg.as_ptr());
                DispatchMessageW(msg.as_ptr());
                if (*APP_GLB.lock()?).flags.contains(AppGlobalFlags::WINDOW_CLOSED) { return Ok(false) }
            }
        }
        let renderer = unsafe { &mut *RENDERER_GLB.get().unwrap().as_ptr() };

        let mut app = APP_GLB.lock()?;
        // Window state update
        app.frame_count += 1;
        app.time_elapsed += delta;
        let time_elapsed = app.time_elapsed;
        // Handle window being minimized or screen locked
        if app.flags.contains(AppGlobalFlags::WINDOW_OCCLUDED) && renderer.occulsion_test() {
            sleep(Duration::from_millis(10));
            drop(app); // drop app mutex before next iteration
            return Ok(true);
        }
        app.flags &= !AppGlobalFlags::WINDOW_OCCLUDED;

        // Handle window resizing
        if app.should_resize() {
            renderer.resize(app.width, app.height)?;
            app.clear_resize();
        }
        drop(app);

        // app update
        let imgui = unsafe { &mut *IMGUI_GLB.get().unwrap().as_ptr() };
        let platform = unsafe { &mut *PLATFORM_GLB.get().unwrap().as_ptr() };
        platform.new_frame(imgui);
        let ui = imgui.new_frame();

        let mut area: MaybeUninit<RECT> = MaybeUninit::uninit();
        unsafe { GetClientRect(hwnd, area.as_mut_ptr())?; }
        let area = unsafe { area.assume_init() };
        if let Some(task) = GfdTask::<GfdAllocator, Freecam>::find_by_str_mut(Freecam::NAME) {
            task.get_main_work_mut().unwrap().draw_contents(ui, area);
        }

        // draw window
        let draw_data = imgui.render();
        let bg_color = RGBFloat::from_hsv((time_elapsed / 30.) % 1., 0.25, 0.6);
        renderer.render(draw_data, Vec4::new(bg_color.get_red_f32(), bg_color.get_green_f32(), bg_color.get_blue_f32(), 1.))?;
        if let Err(e) = renderer.present() {
            if e.code() == DXGI_STATUS_OCCLUDED { (*APP_GLB.lock()?).flags |= AppGlobalFlags::WINDOW_OCCLUDED; }
        }
        Ok(true)
    }
}

// Global resources. These live for the lifetime of the program, so we won't worry about deallocating them.
pub(crate) static APP_GLB: Mutex<App> = Mutex::new(App::new());
pub(crate) static PLATFORM_GLB: OnceLock<StatePointer<Win32Impl>> = OnceLock::new();
pub(crate) static IMGUI_GLB: OnceLock<StatePointer<ImContext>> = OnceLock::new();
pub(crate) static RENDERER_GLB: OnceLock<StatePointer<D3D11State>> = OnceLock::new();
pub(crate) static WINDOW_GLB: OnceLock<StatePointer<Window>> = OnceLock::new();
pub(crate) static IMPLOT_GLB: OnceLock<StatePointer<ImPlotCtx>> = OnceLock::new();

pub fn init_debug_window() -> Result<(), Box<dyn Error>> {
    let proc = ProcessInfo::get_current_process().unwrap();
    let main_icon = unsafe { std::mem::transmute::<usize, PCSTR>(0x69) }; // icon ID defined in CreateWindow
    let main_icon = unsafe { LoadIconA(Some(proc.get_main_module().as_raw().into()), main_icon).ok() };
    // for now: always open in Display 1 at half resolution, near the top left
    let mut main_disp_res = Vec2::new(unsafe { GetSystemMetrics(SM_CXSCREEN) } as f32, unsafe { GetSystemMetrics(SM_CYSCREEN) } as f32 );
    let win_pos = Vec2::new(50., 50.);
    // create a square equal to 1/2 of height
    main_disp_res.x = main_disp_res.y;
    main_disp_res /= 2.;
    App::show_window(win_pos, main_disp_res, main_icon)
}

pub fn update_debug_window(delta: f32) -> Result<bool, Box<dyn Error>> {
    let window = unsafe { &mut *WINDOW_GLB.get().unwrap().as_ptr() };
    App::frame(window.get_handle(), delta)
}

pub fn shutdown_debug_window() {
    // Box all the global resources to call their destructors
    let _platform_box = unsafe { Box::from_raw(PLATFORM_GLB.get().unwrap().as_ptr()) };
    let _imgui_box = unsafe { Box::from_raw(IMGUI_GLB.get().unwrap().as_ptr()) };
    let _renderer_box = unsafe { Box::from_raw(RENDERER_GLB.get().unwrap().as_ptr()) };
    let _window_box = unsafe { Box::from_raw(WINDOW_GLB.get().unwrap().as_ptr()) };
}

pub fn get_hwnd() -> HWND {
    WINDOW_GLB.get().unwrap().get_handle()
}