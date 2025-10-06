use imgui::{Condition, Ui};
use windows::Win32::Foundation::RECT;
use crate::state::camera::Freecam;

impl Freecam {
    pub fn draw_contents(&mut self, ui: &Ui, area: RECT) {
        let width = (area.right - area.left) as f32;
        let height = (area.bottom - area.top) as f32;

        ui.window("Freecam Window")
            .size([width, height], Condition::Always)
            .position([0., 0.], Condition::Always)
            .resizable(false)
            .title_bar(false)
            .build(|| {
                self.draw_contents_topbar(ui);
                ui.separator();
                self.draw_contents_keyframes(ui);
                ui.separator();
                self.draw_contents_controls(ui);
                // add shortcuts if not already
                /*
                if self.shortcuts.is_empty() {
                    self.shortcuts = vec![
                        Shortcut::new(vec![Key::F4], Self::toggle_freecam_mode),
                        Shortcut::new(vec![Key::Escape], Self::disable_freecam_mode_sp),
                        // Shortcut::new(vec![Key::Minus], Self::decrease_scene_frequency),
                        // Shortcut::new(vec![Key::KeypadAdd], Self::increase_scene_frequency),
                        // Shortcut::new(vec![Key::Equal], Self::increase_scene_frequency),
                        Shortcut::new(vec![Key::Alpha0], Self::lock_camera_position),
                        Shortcut::new(vec![Key::Alpha1], Self::add_camera_node),
                        // Shortcut::new(vec![Key::Alpha2], Self::toggle_playback),
                        // Shortcut::new(vec![Key::Alpha3], Self::stop_playback),
                        // Shortcut::new(vec![Key::Alpha8], Self::decrease_node_path_time),
                        // Shortcut::new(vec![Key::Alpha9], Self::increase_node_path_time),
                        // Shortcut::new(vec![Key::Backspace], Self::remove_last_node),
                        // Shortcut::new(vec![Key::Delete], Self::clear_all_nodes),
                    ];
                }
                */
            });
    }
}