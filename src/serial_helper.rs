use std::time::Duration;

use serialport::{SerialPort, SerialPortType};

pub struct Serial {}

impl Serial {

    pub fn new() -> Serial {
        return Serial{};
    }

    pub fn get_avail_port_name(&self) -> Vec<String> {
        let ports = serialport::available_ports().expect("No ports found!");
        let mut port_name_list = Vec::new();
        for p in ports {
            port_name_list.push(p.port_name);
        }
        return port_name_list;
    }

    pub fn get_avail_port_type(&self) -> Vec<SerialPortType> {
        let ports = serialport::available_ports().expect("No ports found!");
        let mut port_type_list = Vec::new();
        for p in ports {
            port_type_list.push(p.port_type);
        }
        return port_type_list;
    }

    pub fn connection(&self, portname: String, baudrate: u32) -> Box<dyn SerialPort> {
        let open_port = serialport::new(portname, baudrate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");
        return open_port;
    }
}
