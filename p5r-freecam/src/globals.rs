#![allow(dead_code, improper_ctypes)]
// This file was automatically generated from opengfd-globals.
use opengfd :: { device :: hedge :: hid :: keyboard :: KeyboardManager , io :: controller :: ControllerPlatformManager , kernel :: global :: Global , platform :: global :: PlatformGlobal , } ;
#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of GFD_GLOBAL.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_gfd_global(ptr: *mut Global);
   /// Get a possible reference to GFD_GLOBAL. This checks to see if `set_gfd_global`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_gfd_global() -> Option<& 'static Global>;
   /// Like `get_gfd_global_mut`, but a mutable reference is created instead.
    pub(crate) fn get_gfd_global_mut() -> Option<& 'static mut Global>;
   /// An unchecked version of `get_gfd_global`. This assumes that GFD_GLOBAL
    /// is set and it's initialized.
    pub(crate) fn get_gfd_global_unchecked() -> & 'static Global;
   /// An unchecked version of `get_gfd_global_mut`. This assumes that GFD_GLOBAL
    /// is set and it's initialized.
    pub(crate) fn get_gfd_global_unchecked_mut() -> & 'static mut Global;
   /// Change the value of `GFD_GLOBAL`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_gfd_global(new: Global);
}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of PLATFORM_GLOBAL.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_platform_global(ptr: *mut PlatformGlobal);
   /// Get a possible reference to PLATFORM_GLOBAL. This checks to see if `set_platform_global`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_platform_global() -> Option<& 'static PlatformGlobal>;
   /// Like `get_platform_global_mut`, but a mutable reference is created instead.
    pub(crate) fn get_platform_global_mut() -> Option<& 'static mut PlatformGlobal>;
   /// An unchecked version of `get_platform_global`. This assumes that PLATFORM_GLOBAL
    /// is set and it's initialized.
    pub(crate) fn get_platform_global_unchecked() -> & 'static PlatformGlobal;
   /// An unchecked version of `get_platform_global_mut`. This assumes that PLATFORM_GLOBAL
    /// is set and it's initialized.
    pub(crate) fn get_platform_global_unchecked_mut() -> & 'static mut PlatformGlobal;
   /// Change the value of `PLATFORM_GLOBAL`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_platform_global(new: PlatformGlobal);
}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to KEYBOARD_INSTANCE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_keyboard_instance(ptr: *mut * mut KeyboardManager);
   /// Get a possible reference to KEYBOARD_INSTANCE. This checks to see if `set_keyboard_instance`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_keyboard_instance() -> Option<& 'static KeyboardManager>;
   /// Like `get_keyboard_instance_mut`, but a mutable reference is created instead.
    pub(crate) fn get_keyboard_instance_mut() -> Option<& 'static mut KeyboardManager>;
   /// An unchecked version of `get_keyboard_instance`. This assumes that KEYBOARD_INSTANCE
    /// is set and it's initialized.
    pub(crate) fn get_keyboard_instance_unchecked() -> & 'static KeyboardManager;
   /// An unchecked version of `get_keyboard_instance_mut`. This assumes that KEYBOARD_INSTANCE
    /// is set and it's initialized.
    pub(crate) fn get_keyboard_instance_unchecked_mut() -> & 'static mut KeyboardManager;
   /// Change the value of `KEYBOARD_INSTANCE`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_keyboard_instance(new: * mut KeyboardManager);
}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to PAD_INSTANCE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_pad_instance(ptr: *mut * mut ControllerPlatformManager);
   /// Get a possible reference to PAD_INSTANCE. This checks to see if `set_pad_instance`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_pad_instance() -> Option<& 'static ControllerPlatformManager>;
   /// Like `get_pad_instance_mut`, but a mutable reference is created instead.
    pub(crate) fn get_pad_instance_mut() -> Option<& 'static mut ControllerPlatformManager>;
   /// An unchecked version of `get_pad_instance`. This assumes that PAD_INSTANCE
    /// is set and it's initialized.
    pub(crate) fn get_pad_instance_unchecked() -> & 'static ControllerPlatformManager;
   /// An unchecked version of `get_pad_instance_mut`. This assumes that PAD_INSTANCE
    /// is set and it's initialized.
    pub(crate) fn get_pad_instance_unchecked_mut() -> & 'static mut ControllerPlatformManager;
   /// Change the value of `PAD_INSTANCE`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_pad_instance(new: * mut ControllerPlatformManager);
}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of IS_STEAM.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_is_steam(ptr: *mut bool);
   /// Get a possible reference to IS_STEAM. This checks to see if `set_is_steam`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_is_steam() -> Option<& 'static bool>;
   /// Like `get_is_steam_mut`, but a mutable reference is created instead.
    pub(crate) fn get_is_steam_mut() -> Option<& 'static mut bool>;
   /// An unchecked version of `get_is_steam`. This assumes that IS_STEAM
    /// is set and it's initialized.
    pub(crate) fn get_is_steam_unchecked() -> & 'static bool;
   /// An unchecked version of `get_is_steam_mut`. This assumes that IS_STEAM
    /// is set and it's initialized.
    pub(crate) fn get_is_steam_unchecked_mut() -> & 'static mut bool;
   /// Change the value of `IS_STEAM`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_is_steam(new: bool);
}

