use dotenv::dotenv;

mod config_init;

fn main() {
    dotenv().ok();
    config_init::init_from_env();
}
