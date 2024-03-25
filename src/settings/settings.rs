use config::Config;
use tracing::log::info;

pub struct Settings {
    pub server: Server,
    pub database: Database
}

pub struct Server {
    pub port: i64,
    pub timeout: i64,
    pub body_limit: i64
}

pub struct Database {
    pub url: String
}

impl Settings {
    pub fn new() -> Self {
        let file = config::File::with_name("Settings");

        let settings = Config::builder()
            // Add in `./Settings.toml`
            .add_source(file)
            .build()
            .unwrap();

        info!("Settings has been loaded.");

        Self {
            server: Server {
                port: settings.get("server.port").unwrap(),
                timeout: settings.get("server.timeout").unwrap(),
                body_limit: settings.get("server.limit").unwrap()
            },
            database: Database {
                url: settings.get("database.url").unwrap()
            }
        }
    }
}