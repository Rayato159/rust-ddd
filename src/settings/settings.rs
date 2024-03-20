use config::Config;
use tracing::log::info;

pub struct Settings {
    pub port: i64,
    pub timeout: i64,
    pub limit: i64
}

impl Settings {
    pub fn new() -> Self {
        let file = config::File::with_name("Settings");

        let settings = Config::builder()
            // Add in `./Settings.toml`
            .add_source(file)
            .build()
            .unwrap();

        info!("Setting has been loaded.");

        Self {
            port: settings.get_int("server.port").unwrap(),
            timeout: settings.get_int("server.timeout").unwrap(),
            limit: settings.get_int("server.limit").unwrap()
        }
    }
}