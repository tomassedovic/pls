mod show;
mod state;
mod util;
mod window;

struct Pls {
    state: state::State,
}

impl epi::App for Pls {
    fn name(&self) -> &str {
        "pls"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = egui::FontDefinitions::default();
        let font_name = "OpenSans";
        fonts.font_data.insert(
            font_name.to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/OpenSans-Bold.ttf",)),
        );
        fonts
            .fonts_for_family
            .insert(egui::FontFamily::Proportional, vec![font_name.to_owned()]);

        fonts.family_and_size.insert(
            egui::TextStyle::Button,
            (egui::FontFamily::Proportional, 18.0),
        );
        fonts.family_and_size.insert(
            egui::TextStyle::Heading,
            (egui::FontFamily::Proportional, 28.0),
        );
        ctx.set_fonts(fonts);

        let mut theme = egui::Visuals::light();
        theme.widgets.inactive.fg_stroke.color = egui::Color32::BLACK;
        theme.widgets.inactive.bg_stroke.color = egui::Color32::from_gray(192);
        theme.widgets.inactive.bg_stroke.width = 2.0;
        ctx.set_visuals(theme);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            window::show(&mut self.state, ui);
        });
    }
}

fn main() -> anyhow::Result<()> {
    let qualifier = ""; // NOTE: something like com.mydomain
    let organisation = ""; // NOTE: Try Jumping
    let application = "pls";

    println!("Hostname: {:?}", hostname::get());

    let test_config_dir = std::path::PathBuf::from("test/pls").canonicalize()?;
    let config_dir = if cfg!(feature = "test") {
        test_config_dir
    } else {
        directories::ProjectDirs::from(qualifier, organisation, application)
            .map(|d| d.config_dir().to_owned())
            .unwrap_or(test_config_dir)
    };
    let config_path = config_dir.join("pls.toml");

    println!("Config location: {:?}", config_path);
    let state = state::State::new(&config_path)?;

    let app = Pls { state };
    let native_options = egui_glow::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(600.0, 800.0)),
        ..egui_glow::NativeOptions::default()
    };
    egui_glow::run(Box::new(app), &native_options)
}
