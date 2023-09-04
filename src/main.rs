mod args_helper;
mod command_helper;
mod config_helper;
mod serial_helper;

use args_helper::{ArgCli, Command, SetConf};
use config_helper::Config;
use serial_helper::Serial;

use clap::Parser;
use std::time::Instant;

fn show_conf() {
    let mut config = Config::default();
    let conf_res = config.read_config();
    if conf_res.is_ok() {
        println!("PORT: {}", config.get_port());
        println!("BAUD: {}", config.get_baud());
        return;
    }
    println!("{}", conf_res.unwrap_err());
    println!("Creating config files...");
    let init_res = config.init_config();
    if init_res.is_err() {
        println!("{}", init_res.unwrap_err());
    }
    let conf_retry_res = config.read_config();
    if conf_retry_res.is_ok() {
        println!("PORT: {}", config.get_port());
        println!("BAUD: {}", config.get_baud());
        return;
    }
    println!("Fetch file failed 2 times. Aborting...")
}

fn list_port() {
    let serial = Serial::new();
    let ports = serial.get_avail_port_name();
    for p in ports.to_owned() {
        println!("{}", p);
    }
}

fn set_conf(val: SetConf) {
    let mut config = Config::default();
    let read_res = config.read_config();
    if read_res.is_err() {
        println!("{}", read_res.unwrap_err());
        return;
    }
    match val {
        SetConf::Baud(val) => {
            if val.baud <= 0 {
                return;
            }
            let set_res = config.set_baud(val.baud);
            if set_res.is_err() {
                println!("{}", set_res.unwrap_err());
            } else {
                println!("Update baudrate success");
            }
        }
        SetConf::Port(val) => {
            if val.port == "" {
                return;
            }
            let set_res = config.set_port(val.port);
            if set_res.is_err() {
                println!("{}", set_res.unwrap_err());
            } else {
                println!("Update port success");
            }
        }
    }
}

fn search_port(port: String, baud: u32, time_till_timeout: u128) -> bool {
    let serial = Serial::new();
    let mut port_con = serial.connection(port.to_owned(), baud);
    let mut last_time = 0;
    let mut delta_time = 0;
    let mut loop_ms = 0;
    let mut timeout_ms = 0;
    let timer = Instant::now();

    while timeout_ms < time_till_timeout {
        let mut str_read = String::new();
        let mut _err = port_con.read_to_string(&mut str_read).unwrap_err();

        if str_read.trim() == "ALIVE" {
            return true;
        }

        if loop_ms > 1000 {
            loop_ms = 0;
            port_con.write("STATUS".as_bytes()).expect("Write failed!");
            port_con.flush().expect("Error flush");
        }

        delta_time = timer.elapsed().as_millis() - last_time;
        last_time = timer.elapsed().as_millis();

        loop_ms += delta_time;
        timeout_ms += delta_time;
    }
    return false;
}

fn auto_search() {
    let serial = Serial::new();
    let list_port = serial.get_avail_port_name();
    let mut config = Config::default();
    let mut baud = 0;
    let mut port_list: Vec<String> = vec![];
    let mut port_list_str = String::new();
    let read_res = config.read_config();
    if read_res.is_err() {
        println!("{}", read_res.unwrap_err());
        return;
    }
    if config.get_baud() == 0 {
        println!("Baudrate config is 0, using 19200 as default");
        baud = 19200;
    }else{
        println!("Current baudrate: {}", config.get_baud());
        baud = config.get_baud();
    }
    println!("Search for port...");
    for port in list_port.to_owned() {
        let result = search_port(port.to_owned(), baud, 5000);
        if result == true {
            port_list.push(port.to_owned());
            port_list_str += port.as_str();
            port_list_str += " ";
        }
    }
    if port_list.len() == 0 {
        println!("Client device not found!");
        return;
    }
    println!("Found {} client available at: {}", port_list.len(), port_list_str.to_owned());
}

fn reset() {
    let mut config = Config::default();
    let write_res = config.init_config();
    if write_res.is_err() {
        println!("{}", write_res.unwrap_err());
    }
    println!("Reset done");
}

fn main() {
    let args = ArgCli::parse();

    match args.command {
        Command::Show => show_conf(),
        Command::List => list_port(),
        Command::Set(val) => set_conf(val),
        Command::Auto => auto_search(),
        Command::Reset => reset(),
    }
}
