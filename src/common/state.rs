use actix_web::web::Data;
use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<MySql>,
}

pub trait DbConnection {
    fn get_connection(&self) -> &Pool<MySql>;
}

impl DbConnection for AppState {
    fn get_connection(&self) -> &Pool<MySql> {
        &self.db
    }
}

impl DbConnection for Data<AppState> {
    fn get_connection(&self) -> &Pool<MySql> {
        &self.db
    }
}
