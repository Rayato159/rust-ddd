use diesel::PgConnection;

pub trait Database {
    fn get_conn(&self) -> &PgConnection;    
}