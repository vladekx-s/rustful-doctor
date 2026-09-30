use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub db_url: String,
    pub port: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            db_url: env::var("DATABASE_URL").expect("Ошибка адреса БД"),
            port: env::var("PORT").expect("Ошибка переменной PORT"),
        }
    }
}
