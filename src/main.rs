use dotenv::dotenv;

mod config;

fn main() {
    dotenv().ok();
    config::init_from_env();
}
