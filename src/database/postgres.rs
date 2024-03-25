
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::settings::settings::Database as DatabaseSetting;

use super::database::Database;

pub struct PostgresDatabase {
    conn: PgConnection
}

impl PostgresDatabase {
    pub fn new(setting: &DatabaseSetting) -> Self {
        let url = &setting.url;

        let conn = PgConnection::establish(&url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", url));

        Self {
            conn
        }
    }
}

impl Database for PostgresDatabase {
    fn get_conn(&self) -> &PgConnection {
        &self.conn
    }
}