use crate::state::State;

pub fn show(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Hello World!");

    egui::ScrollArea::vertical()
        .max_height(200.0)
        .auto_shrink([false, false])
        .always_show_scroll(true)
        .show(ui, |ui| {
            for (key, series) in state.config.iter() {
                let name = series.get("name").map(|v| v.as_str()).flatten();
                if let Some(name) = name {
                    ui.selectable_value(&mut state.selected_key, key.to_string(), name);
                }
            }
        });

    if ui.button("Play Next").clicked() {
        println!("Clicked: Playing next");
        if let Some(series) = state.config.get(&state.selected_key) {
            println!("{}", series);
        }
    };

    if ui.button("About").clicked() {
        println!("Clicked: About");
    };

    if ui.button("Settings").clicked() {
        println!("Clicked: Settings");
    };
}
