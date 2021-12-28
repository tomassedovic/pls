use crate::state::State;

use egui::{Align, Layout, Widget};

pub fn show(state: &mut State, ui: &mut egui::Ui) {
    ui.style_mut().spacing.button_padding = [10.0, 10.0].into();
    ui.heading("Select a show");
    ui.add_space(5.0);

    ui.with_layout(
        Layout::bottom_up(Align::Min).with_cross_justify(true),
        |ui| {
            ui.add_space(5.0);
            ui.allocate_ui_with_layout(
                egui::Vec2::new(200.0, 30.0),
                Layout::left_to_right(),
                |ui| {
                    ui.columns(2, |c| {
                        if c[0].button("About").clicked() {
                            println!("Clicked: About");
                        };

                        if c[1].button("Settings").clicked() {
                            println!("Clicked: Settings");
                            if let Err(error) = opener::open(&state.config_path) {
                                state.error =
                                    Some(format!("Error opening the config file:\n{:?}", error));
                            }
                        };
                    });
                },
            );

            ui.separator();

            let play_next_text = state
                .shows
                .get(&state.selected_key)
                .map(|show| {
                    show.current_episode()
                        .file_name()
                        .map(|f| f.to_string_lossy().into_owned())
                })
                .flatten()
                .unwrap_or_else(|| "No episode available".into());

            let play_next_button = egui::Button::new(play_next_text)
                .text_style(egui::TextStyle::Heading)
                .ui(ui);
            if play_next_button.clicked() {
                println!("Clicked: Playing next");
                if let Some(show) = state.shows.get_mut(&state.selected_key) {
                    println!("Selected: {:?}", show);
                    println!("{}", show.current_episode().display());
                    let current_episode = show.current_episode();
                    if !current_episode.exists() {
                        state.error = Some(format!(
                            "Episode file doesn't exist: {}",
                            current_episode.display()
                        ));
                    } else if !current_episode.is_file() {
                        state.error = Some(format!(
                            "Episode path is not a file: {}",
                            current_episode.display()
                        ));
                    } else if let Err(error) = opener::open(&current_episode) {
                        state.error = Some(format!("Error opening file:\n{:?}", error));
                    }
                    println!("Opened: {:?}", current_episode.display());
                    println!("Returning control back to pls");
                    show.advance_to_next_episode();
                    if let Some(table) = state
                        .config
                        .get_mut(&state.selected_key)
                        .map(toml_edit::Item::as_table_mut)
                        .flatten()
                    {
                        table.insert("next", toml_edit::value(show.next.display().to_string()));
                    }
                    println!("{}", state.config.to_string());
                    state.save_config();
                }
            };
            ui.heading("Play next episode:");

            if ui.button("Replay last watched").clicked() {
                println!("Clicked: Replay last watched");
            }
            ui.label("Replay last watched:");

            ui.separator();

            ui.add_space(5.0);

            if let Some(show) = state.shows.get_mut(&state.selected_key) {
                ui.label(format!("Location: {}", show.dir.display()));
            }

            ui.add_space(10.0);

            egui::ScrollArea::vertical()
                .always_show_scroll(true)
                .show(ui, |ui| {
                    ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                        for key in &state.ordered_keys {
                            if let Some(show) = &state.shows.get(key) {
                                ui.selectable_value(
                                    &mut state.selected_key,
                                    key.to_string(),
                                    &show.name,
                                );
                            }
                        }
                    });
                });
        },
    );

    let mut window_is_open = state.error.is_some();
    if let Some(message) = state.error.as_ref() {
        egui::Window::new("Error")
            .open(&mut window_is_open)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.heading("Error occured");
                ui.label(message);
            });
        if !window_is_open {
            state.error = None;
        }
    }
}
