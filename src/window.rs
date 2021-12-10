pub fn show(ui: &mut egui::Ui) {
    ui.heading("Hello World!");

    egui::ScrollArea::vertical()
        .max_height(200.0)
        .auto_shrink([false, false])
        .always_show_scroll(true)
        .show(ui, |ui| {
            for i in 0..20 {
                if ui.selectable_label(i == 3, format!("{} item", i)).clicked() {
                    println!("Clicked item: {}", i);
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
