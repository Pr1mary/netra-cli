use std::time::{Duration, Instant};

use serialport::{SerialPort, SerialPortType};

pub struct Serial {}

impl Serial {
    pub fn new() -> Serial {
        return Serial {};
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

    pub fn connection_v2(
        &self,
        portname: String,
        baudrate: u32,
    ) -> Result<Box<dyn SerialPort>, String> {
        let open_port = serialport::new(portname, baudrate)
            .timeout(Duration::from_millis(10))
            .open();
        match open_port {
            Ok(_) => return Ok(open_port.unwrap()),
            Err(_) => return Err(open_port.unwrap_err().to_string()),
        }
    }

    pub fn test_connection(
        &self,
        port: String,
        baud: u32,
        time_till_timeout: u128,
    ) -> (bool, String) {
        let port_con = self.connection_v2(port.to_owned(), baud);
        if port_con.is_err() {
            return (false, "Port not available".to_owned());
        }
        let mut port_con = port_con.unwrap();
        let mut last_time = 0;
        let mut delta_time = 0;
        let mut loop_ms = 0;
        let mut timeout_ms = 0;
        let timer = Instant::now();
        let mut started = false;
        let mut ask_for_status = 0;
        while timeout_ms < time_till_timeout {
            if !started || loop_ms > 1000 {
                started = true;
                loop_ms = 0;
                port_con.write("STATUS".as_bytes()).expect("Write failed!");
                port_con.flush().expect("Error flush");
            }
            let mut str_read = String::new();
            let mut _err = port_con.read_to_string(&mut str_read).unwrap_err();
            if str_read.trim().to_owned() == "ALIVE" {
                return (true, String::new());
            }
            else if str_read.trim().to_owned() == "STATUS" {
                ask_for_status+=1;
            }
            delta_time = timer.elapsed().as_millis() - last_time;
            last_time = timer.elapsed().as_millis();
            loop_ms += delta_time;
            timeout_ms += delta_time;
        }
        if ask_for_status > 3 {
            return (true, String::new());
        }
        return (false, "Client not found on current config".to_owned());
    }
}
