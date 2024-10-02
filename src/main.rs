use log::info;
use simple_logger::SimpleLogger;

mod sign_order;
mod starkex_messages;

fn main() {
    SimpleLogger::new().env().init().unwrap();

    let order_hash = sign_order::sign_order_example();

    info!("order_hash = {}", order_hash);
}
