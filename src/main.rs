mod logger;

use logger::init_logger;
use tracing::debug;

#[tokio::main]
async fn main() {
    load_env_vars();
    init_logger();

    debug!("testing 123");
    a_plus_b(1, 2);
}

#[tracing::instrument]
fn a_plus_b(a: u8, b: u8) -> u8 {
    let result: u8 = a + b;
    debug!("a plus b equals to {:?}", result);
    return result;
}

fn load_env_vars() {
    match dotenvy::dotenv() {
        Ok(_) => println!("Successfully loaded environment variables"),
        Err(_) => panic!("Failed to load environment variables from '.env' file"),
    }
}
