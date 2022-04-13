mod config_init;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    config_init::init_from_env();
}
