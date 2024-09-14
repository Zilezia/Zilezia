use dotenv::from_filename;
use std::env;

pub fn load_env() { // change to dotenv.ok?
    from_filename("./common/.env").ok();
}
pub fn get_mysql_url() -> String {
    let user = env::var("DB_USER").expect("DB_USER must be set.");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set.");
    let host = env::var("DB_HOST").expect("DB_HOST must be set.");
    let port = env::var("DB_PORT").expect("DB_PORT must be set.");
    let db_name = env::var("DB_NAME").expect("DB must be set.");
    
    format!(
        "mysql://{}:{}@{}:{}/{}", 
        user, password, host, port, db_name
    )
}

pub fn get_table() -> String {
    env::var("TABLE").expect("TABLE must be set.")
}
pub fn get_base_url() -> String {
    env::var("BASE_URL").expect("BASE_URL must be set.")
}
pub fn get_ip() -> String {
    env::var("IPH").expect("IPH must be set.")
}
// this one needs to be an uint, a bit
pub fn get_port() -> String {
    env::var("PORT").expect("PORT must be set.")
}