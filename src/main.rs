mod weather;

use clap::{Parser, Subcommand};
use eframe::{egui, NativeOptions};
use weather::{fetch_weather, WeatherSummary};

#[derive(Subcommand)]
enum Command {
    /// Launch the weather GUI
    Gui,
}

#[derive(Parser)]
#[command(author, version, about = "Weather CLI with optional GUI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// City name to fetch weather for
    city: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Gui) => launch_gui(),
        None => {
            if let Some(city) = cli.city {
                match fetch_weather(&city) {
                    Ok(summary) => println!("{}", summary.format_ansi()),
                    Err(error) => eprintln!("Error: {}", error),
                }
            } else {
                launch_gui();
            }
        }
    }
}

fn launch_gui() {
    let options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "Weather CLI GUI",
        options,
        Box::new(|_cc| Box::new(WeatherApp::default())),
    ) {
        eprintln!("Failed to launch GUI: {}", err);
    }
}

struct WeatherApp {
    city: String,
    status: String,
    summary: Option<WeatherSummary>,
}

impl Default for WeatherApp {
    fn default() -> Self {
        Self {
            city: "San Francisco".into(),
            status: "Enter a city name and click Fetch.".into(),
            summary: None,
        }
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Weather CLI with GUI");
            ui.label("Type a city name and fetch the current weather.");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("City:");
                ui.text_edit_singleline(&mut self.city);
                if ui.button("Fetch").clicked() {
                    self.fetch_weather();
                }
            });

            ui.add_space(8.0);
            if ui.button("Refresh").clicked() {
                self.fetch_weather();
            }

            ui.separator();
            ui.add_space(6.0);

            if let Some(summary) = &self.summary {
                ui.label(egui::RichText::new(format!("{}", summary.city)).heading());
                ui.label(format!("Country: {}", summary.country));
                ui.label(format!("Condition: {}", summary.description));
                ui.label(format!("Temperature: {} °C", summary.temperature_c));
                ui.label(format!("Feels like: {} °C", summary.feels_like_c));
                ui.label(format!("Humidity: {}%", summary.humidity));
                ui.label(format!("Wind: {} km/h", summary.wind_kmph));

                ui.separator();
                ui.label(egui::RichText::new("5-Day Forecast").heading());
                for day in &summary.forecast {
                    ui.label(format!("{}: {}°C / {}°C - {}", day.date, day.max_temp_c, day.min_temp_c, day.description));
                }

                ui.separator();
                ui.hyperlink_to("View Satellite Image", format!("https://wttr.in/{}.png", self.city.replace(" ", "+")));
            } else {
                ui.colored_label(egui::Color32::LIGHT_BLUE, &self.status);
            }
        });
    }
}

impl WeatherApp {
    fn fetch_weather(&mut self) {
        self.status = "Fetching weather...".into();
        self.summary = None;

        match fetch_weather(&self.city) {
            Ok(summary) => {
                self.summary = Some(summary);
                self.status = "Weather loaded successfully.".into();
            }
            Err(error) => {
                self.status = format!("Error: {}", error);
            }
        }
    }
}
