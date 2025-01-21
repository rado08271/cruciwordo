use skytable::{ClientResult, Config, Connection};

pub mod get_puzzle;
pub mod create_puzzle;

pub fn get_database () -> ClientResult<Connection> {
    let SKYTABLE_DATABASE_ROOT_USER: String = std::env::var("SKYTABLE_DATABASE_ROOT_USER").expect("SKYTABLE_DATABASE_ROOT_USER must be set");
    let SKYTABLE_DATABASE_ROOT_PASS: String = std::env::var("SKYTABLE_DATABASE_ROOT_PASS").expect("SKYTABLE_DATABASE_ROOT_PASS must be set");
    let SKYTABLE_DATABASE_ROOT_HOST: String = std::env::var("SKYTABLE_DATABASE_ROOT_HOST").expect("SKYTABLE_DATABASE_ROOT_HOST must be set");
    let SKYTABLE_DATABASE_ROOT_PORT: u16  = std::env::var("SKYTABLE_DATABASE_ROOT_PORT").expect("SKYTABLE_DATABASE_ROOT_PORT must be set").parse::<u16>().expect("Cannot parse SKYTABLE_DATABASE_ROOT_PORT to number");

    let mut db = Config::new(
        SKYTABLE_DATABASE_ROOT_HOST.as_str(),
        SKYTABLE_DATABASE_ROOT_PORT,
        SKYTABLE_DATABASE_ROOT_USER.as_str(),
        SKYTABLE_DATABASE_ROOT_PASS.as_str()
    ).connect();

    return db;
}
