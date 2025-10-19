#![windows_subsystem = "windows"]

mod app;
mod config;
mod file;
mod tray;

use crate::app::App;
use crate::config::AppConfig;

fn main() {
    dotenvy::dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default()).init();

    let config = envy::from_env::<AppConfig>().unwrap();

    log::info!("Initializing application");

    App::new(config).listen_files().run_tray();
}
