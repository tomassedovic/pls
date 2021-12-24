use crate::state::State;

pub fn show(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Hello World!");

    egui::ScrollArea::vertical()
        .max_height(200.0)
        .auto_shrink([false, false])
        .always_show_scroll(true)
        .show(ui, |ui| {
            for key in &state.ordered_keys {
                if let Some(show) = &state.shows.get(key) {
                    ui.selectable_value(&mut state.selected_key, key.to_string(), &show.name);
                }
            }
        });

    if ui.button("Play Next").clicked() {
        println!("Clicked: Playing next");
        if let Some(show) = state.shows.get_mut(&state.selected_key) {
            println!("Selected: {:?}", show);
            println!("{}", show.current_episode().display());
            // TODO: handle errors
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
            //show.next();
        }
    };

    if ui.button("About").clicked() {
        println!("Clicked: About");
    };

    if ui.button("Settings").clicked() {
        println!("Clicked: Settings");
    };

    let mut window_is_open = state.error.is_some();
    if let Some(message) = state.error.as_ref() {
        let result = egui::Window::new("Error")
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
