use crate::state::State;

use egui::{
    Align, Button, Color32, Layout, Rect, RichText, ScrollArea, Stroke, TextStyle, Ui, Vec2,
    Widget, Window,
};

pub fn show(state: &mut State, ui: &mut Ui) {
    ui.style_mut().spacing.button_padding = [10.0, 10.0].into();
    ui.heading("Select a show:");
    ui.add_space(5.0);

    ui.with_layout(
        Layout::bottom_up(Align::Min).with_cross_justify(true),
        |ui| {
            ui.add_space(5.0);
            ui.allocate_ui_with_layout(Vec2::new(200.0, 30.0), Layout::left_to_right(), |ui| {
                ui.columns(3, |c| {
                    if c[0].button("About").clicked() {
                        state.about_window_is_open = true;
                    };

                    if c[1].button("Config").clicked() {
                        println!("Clicked: Config");
                        if let Err(error) = opener::open(&state.config_path) {
                            state.error =
                                Some(format!("Error opening the config file:\n{:?}", error));
                        }
                    };

                    if c[2].button("Reload").clicked() {
                        if let Err(error) = state.reload_config() {
                            state.error = Some(format!("Error reloading the config:\n{}", error));
                        }
                    }
                });
            });

            ui.separator();

            let replay_last_text = state
                .shows
                .get(&state.selected_key)
                .and_then(|show| show.previous_episode())
                .and_then(|e| e.file_name().map(|f| f.to_string_lossy().into_owned()))
                .unwrap_or_else(|| "No episode available".into());

            if ui.button(replay_last_text).clicked() {
                println!("Clicked: Replay last watched");
                if let Some(episode) = state
                    .shows
                    .get_mut(&state.selected_key)
                    .and_then(|show| show.previous_episode())
                {
                    println!("{}", episode.display());
                    if !episode.exists() {
                        state.error =
                            Some(format!("Episode file doesn't exist: {}", episode.display()));
                    } else if !episode.is_file() {
                        state.error =
                            Some(format!("Episode path is not a file: {}", episode.display()));
                    } else if let Err(error) = opener::open(&episode) {
                        state.error = Some(format!("Error opening file:\n{:?}", error));
                    }
                    println!("Opened: {:?}", episode.display());
                    println!("Returning control back to pls");
                }
            }
            ui.label("Replay last watched:");

            let play_next_text = state
                .shows
                .get(&state.selected_key)
                .map(|show| show.current_episode())
                .and_then(|e| e.file_name().map(|f| f.to_string_lossy().into_owned()))
                .unwrap_or_else(|| "No episode available".into());

            let play_next_label = RichText::new(play_next_text)
                .color(Color32::BLUE)
                .text_style(TextStyle::Heading);
            let play_next_button = Button::new(play_next_label).ui(ui);
            if play_next_button.clicked() {
                println!("Clicked: Playing next");
                if let Some(show) = state.shows.get_mut(&state.selected_key) {
                    println!("Selected: {:?}", show);
                    let current_episode = show.current_episode();
                    println!("{}", current_episode.display());
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
                        .and_then(toml_edit::Item::as_table_mut)
                    {
                        table.insert("next", toml_edit::value(show.next.display().to_string()));
                    }
                    println!("{}", state.config);
                    state.save_config();
                }
            };
            ui.label("Play next episode:");

            ui.separator();

            ui.add_space(5.0);

            if let Some(show) = state.shows.get_mut(&state.selected_key) {
                ui.label(format!("Location: {}", show.dir.display()));
            }

            ui.add_space(10.0);

            ScrollArea::vertical()
                .always_show_scroll(true)
                .show(ui, |ui| {
                    ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                        ui.painter_at(Rect::EVERYTHING).rect(
                            Rect::EVERYTHING,
                            0.0,
                            Color32::WHITE,
                            Stroke::default(),
                        );
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

    let mut error_window_is_open = state.error.is_some();
    if let Some(message) = state.error.as_ref() {
        Window::new("Error")
            .open(&mut error_window_is_open)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label(message);
            });
        if !error_window_is_open {
            state.error = None;
        }
    }

    Window::new("About pls")
        .open(&mut state.about_window_is_open)
        .collapsible(false)
        .show(ui.ctx(), |ui| {
            ScrollArea::vertical()
                .always_show_scroll(true)
                .show(ui, |ui| {
                    ui.label(ABOUT_TEXT);
                });
        });
}

const ABOUT_TEXT: &str = "pls is a program that lets you load up series of files (typically video files) and play those files one after another. For example, if you've got the episodes of a TV show in a directory, it will play them one by one without you having to remember where left off.

Copyright (C) 2019-2022 Tomas Sedovic <tomas@sedovic.cz>

Program license:

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

Icons:
The \"TV Show\" application icon comes from Icons8:

https://icons8.com/icon/46904/cute-color

It is provided free of charge under the condition of showing the link above in the About dialog of the app that uses it.
";
