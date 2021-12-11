#[derive(Default)]
pub struct State {
    selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn show(state: &mut State, ui: &mut egui::Ui) {
    ui.heading("Hello World!");

    egui::ScrollArea::vertical()
        .max_height(200.0)
        .auto_shrink([false, false])
        .always_show_scroll(true)
        .show(ui, |ui| {
            for i in 0..20 {
                let selected = i == state.selected_index;
                if ui
                    .selectable_label(selected, format!("{} item", i))
                    .clicked()
                {
                    println!("Clicked item: {}", i);
                    state.selected_index = i;
                };
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
