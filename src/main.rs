mod config_helper;
mod serial_helper;

use config_helper::Config;
use serial_helper::Serial;

fn main() {
    let serial = Serial::new();
    // let mut config = Config::default();

    // config.init_config().expect("config init fail");
    
    // let client_port = config.get_port();
    // let client_baud = config.get_baud();

    let ports = serial.get_avail_port_name();

    println!("Avail port list:");
    for p in ports.to_owned() {
        println!("{}", p);
    }


}
