use dotenv::from_filename;
use std::env::var;

pub fn load_env() { // change to dotenv.ok?
    from_filename("./common/.env").ok();
}
pub fn get_mysql_url() -> String {
    let user = var("DB_USER").expect("DB_USER must be set.");
    let password = var("DB_PASSWORD").expect("DB_PASSWORD must be set.");
    let host = var("DB_HOST").expect("DB_HOST must be set.");
    let port = var("DB_PORT").expect("DB_PORT must be set.");
    let db_name = var("DB_NAME").expect("DB must be set.");
    
    format!(
        "mysql://{}:{}@{}:{}/{}", 
        user, password, host, port, db_name
    )
}

// pub fn get_table() -> String {
//     var("TABLE").expect("BASE_URL must be set.")
// }

pub fn get_table() -> String {
    match var("TABLE") {
        Ok(table) => table,
        Err(_) => "default_table".to_string(),
    }
}

pub fn get_base_url() -> String {
    var("BASE_URL").expect("BASE_URL must be set.")
}
pub fn get_ip() -> String {
    var("LIP").expect("LIP must be set.")
}
// this one needs to be an uint, a bit
pub fn get_port() -> String {
    var("PORT").expect("PORT must be set.")
}
