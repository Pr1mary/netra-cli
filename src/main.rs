mod args_helper;
mod command_helper;
mod config_helper;
mod serial_helper;

use args_helper::{ArgCli, Command, SetConf};
use config_helper::Config;
use serial_helper::Serial;

use clap::Parser;

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
    let write_res = config.init_config();
    if write_res.is_err() {
        println!("{}", write_res.unwrap_err());
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

fn auto_search() {
    // let serial = Serial::new();
    println!("Work in progress!");
}

fn reset() {
    println!("Work in progress!");
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
