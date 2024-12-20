#![windows_subsystem = "windows"]

mod show;
mod state;
mod util;
mod window;

use simplelog::{CombinedLogger, Config, LevelFilter, SharedLogger, SimpleLogger, WriteLogger};

struct Pls {
    state: state::State,
}

impl epi::App for Pls {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            window::show(&mut self.state, ui);
        });
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

    fn name(&self) -> &str {
        "pls"
    }
}

fn main() -> anyhow::Result<()> {
    // Set up logging
    let log_level = LevelFilter::Trace;
    let mut loggers =
        vec![SimpleLogger::new(log_level, Config::default()) as Box<dyn SharedLogger>];

    if let Ok(logfile) = std::fs::File::create("pls.log") {
        loggers.push(WriteLogger::new(log_level, Config::default(), logfile));
    }

    // NOTE: ignore the loggers if we can't initialise them. The app
    // should still be able to function.
    let _ = CombinedLogger::init(loggers);

    log_panics::init();

    let qualifier = ""; // NOTE: something like com.mydomain
    let organisation = ""; // NOTE: Try Jumping
    let application = "pls";

    log::debug!("Hostname: {:?}", hostname::get());

    let test_config_dir = std::path::PathBuf::from("test/pls");
    let config_dir = if cfg!(feature = "test") {
        test_config_dir
    } else {
        directories::ProjectDirs::from(qualifier, organisation, application)
            .map(|d| d.config_dir().to_owned())
            .unwrap_or(test_config_dir)
    };
    let config_path = config_dir.join("pls.toml");

    log::debug!("Current directory: {:?}", std::env::current_dir());
    log::info!("Config location: {:?}", config_path);
    let full_config_path = config_path.canonicalize()?;
    let state = state::State::new(&full_config_path)?;
    log::info!("Config version: {}", state.config_version);

    let app = Pls { state };
    let native_options = egui_glow::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(600.0, 800.0)),
        ..egui_glow::NativeOptions::default()
    };
    egui_glow::run(Box::new(app), &native_options)
}
