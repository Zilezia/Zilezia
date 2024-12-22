use dotenv::from_filename;

pub fn load_env() {
    from_filename("./common/.env").ok();
}
