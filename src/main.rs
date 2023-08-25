mod args_helper;
mod config_helper;
mod serial_helper;
mod command_helper;

use args_helper::{ArgCli, Command, SetConf};
use config_helper::Config;
use serial_helper::Serial;

use clap::Parser;

fn show_conf() {
    let mut config = Config::default();
    let conf_res = config.init_config();
    if conf_res.is_err() {
        println!("{}", conf_res.unwrap_err());
        println!("Creating config files...");
        let write_res = config.write_file("".to_owned(), 19200);
        if write_res.is_err() {
            println!("{}", write_res.unwrap_err());
        }
        return;
    }
    println!("PORT: {}", config.get_port());
    println!("BAUD: {}", config.get_baud());
}

fn list_port() {
    let serial = Serial::new();
    let ports = serial.get_avail_port_name();
    for p in ports.to_owned() {
        println!("{}", p);
    }
}

fn set_conf(val: SetConf) {
    match val {
        SetConf::Baud(val) => {
            if val.baud != 0 {}
            println!("Work in progress!");
            return;
        }
        SetConf::Port(val) => {
            if val.port != "" {}
            println!("Work in progress!");
            return;
        }
    }
}

fn auto_search() {
    let serial = Serial::new();
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
