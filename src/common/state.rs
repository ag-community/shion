use actix_web::web::Data;
use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct State {
    pub db: Pool<MySql>,
}

pub trait DatabaseState {
    fn db(&self) -> &Pool<MySql>;
}

impl DatabaseState for State {
    fn db(&self) -> &Pool<MySql> {
        &self.db
    }
}

impl DatabaseState for Data<State> {
    fn db(&self) -> &Pool<MySql> {
        &self.db
    }
}
