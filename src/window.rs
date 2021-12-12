use crate::state::State;

pub fn show(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Hello World!");

    egui::ScrollArea::vertical()
        .max_height(200.0)
        .auto_shrink([false, false])
        .always_show_scroll(true)
        .show(ui, |ui| {
            for (index, (_key, series)) in state.config.iter().enumerate() {
                let selected = index == state.selected_index;
                let name = series.get("name").map(|v| v.as_str()).flatten();
                if let Some(name) = name {
                    if ui.selectable_label(selected, name).clicked() {
                        state.selected_index = index;
                    }
                }
            }
        });

    if ui.button("Play Next").clicked() {
        println!("Playing next");
    };

    if ui.button("About").clicked() {
        println!("About");
    };

    if ui.button("Settings").clicked() {
        println!("Settings");
    };
}
