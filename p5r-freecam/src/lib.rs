pub mod globals;
pub mod gui {
    pub mod app;
    pub mod d3d11 {
        pub mod backup;
        pub mod buffer;
        pub mod devices;
        pub mod font;
        pub mod shader;
        pub mod state;
    }
    pub mod utils;
    pub mod win32;
}
pub mod hooks {
    pub mod battle;
    pub mod event;
    pub mod field;
    pub mod title;
}
pub mod state {
    pub mod camera;
    pub mod controls;
    pub mod io;
    pub mod node;
    pub mod path;
    pub mod window;
}
pub mod version;