use dotenv::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
}
// this is ight
pub fn get_mysql_password() -> String {
    env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set.")
}
// so is this
pub fn get_ip() -> String {
    env::var("IP").expect("IP must be set.")
}
// this one needs to be an int
pub fn get_port() -> u16 {
    let get = env::var("PORT").expect("PORT must be set.");
    let port = get.parse::<u16>().unwrap();
    return port;
}
// this one for some reason wants a &str
// currently stuck, whole 24 line underlined
pub fn get_api_route<'a>() -> &'a str {
    env::var("API_ROUTE").expect("API_ROUTE must be set.").as_str()
}