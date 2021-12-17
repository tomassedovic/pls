use crate::state::State;

pub fn show(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Hello World!");

    egui::ScrollArea::vertical()
        .max_height(200.0)
        .auto_shrink([false, false])
        .always_show_scroll(true)
        .show(ui, |ui| {
            for (key, show) in &state.shows {
                ui.selectable_value(&mut state.selected_key, key.to_string(), &show.name);
            }
        });

    if ui.button("Play Next").clicked() {
        println!("Clicked: Playing next");
        // TODO: https://crates.io/crates/opener
        // This should open a file on windows/linux/macos
        if let Some(show) = state.shows.get(&state.selected_key) {
            println!("Selected: {:?}", show);
        }
    };

    if ui.button("About").clicked() {
        println!("Clicked: About");
    };

    if ui.button("Settings").clicked() {
        println!("Clicked: Settings");
    };
}
