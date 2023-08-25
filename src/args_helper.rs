use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ArgCli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Command {
    /// Show current configuration
    Show,

    /// List all available serial port
    List,

    /// Update current configuration
    #[clap(subcommand)]
    Set(SetConf),

    /// Auto search for connected netra client, should return port name -- !! work in progress !!
    Auto,

    /// Reset the configuration -- !! work in progress !!
    Reset,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum SetConf {
    /// Set the baudrate
    Baud(SetBaud),
    /// Set the serial port
    Port(SetPort),
}

#[derive(Debug, Args, PartialEq)]
pub struct SetPort {
    /// Serial port based on list command
    pub port: String,
}

#[derive(Debug, Args, PartialEq)]
pub struct SetBaud {
    /// Baudrate for the serial communication, default is 19200
    pub baud: u32,
}