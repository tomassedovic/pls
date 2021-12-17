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
        if let Some(show) = state.shows.get_mut(&state.selected_key) {
            println!("Selected: {:?}", show);
            println!("{}", show.current_episode().display());
            // TODO: handle errors
            opener::open(show.current_episode());
            println!("Opened: {:?}", show.current_episode().display());
            println!("Returning control back to pls");
            //show.next();
        }
    };

    if ui.button("About").clicked() {
        println!("Clicked: About");
    };

    if ui.button("Settings").clicked() {
        println!("Clicked: Settings");
    };
}
