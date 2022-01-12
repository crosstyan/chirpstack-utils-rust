use std::io::BufReader;
use std::io::BufRead;
use log::debug;
use log::info;
use serialport::{SerialPort};
use crate::utils::{escape_string};
pub mod at;


fn send_read(mut serial: Box<dyn SerialPort>, command: &str) -> String {
  let w_buf = command;
  let mut r_buf = String::new();
  serial.write(w_buf.as_bytes()).expect("Failed to write");
  debug!("Send Content: {}", escape_string(w_buf.to_string()));
  // https://stackoverflow.com/questions/67520106/how-to-use-read-line-function-with-rusts-serialport-crate
  let mut reader = BufReader::new(serial);
  reader.read_line(&mut r_buf).expect("Failed to read");
  debug!("Response: {}", escape_string(r_buf.clone()));
  if r_buf.to_ascii_uppercase().contains("ERROR"){
    panic!("Error from device: {}", r_buf);
  }
  return r_buf;
}

pub fn at_version(serial: Box<dyn SerialPort>){
  send_read(serial, "at+version\r\n");
}

pub fn at_join(serial: Box<dyn SerialPort>){
  send_read(serial, "at+join\r\n");
}

/// This function won't check the DevEUI. 
/// Use with caution.
/// at+set_config=lora:dev_eui:<dev_eui>\r\n
pub fn at_dev_eui(serial: Box<dyn SerialPort>, dev_eui: &str){
  let w_buf = format!("at+set_config=lora:dev_eui:{}\r\n", dev_eui);
  send_read(serial, &w_buf);
}

/// This function won't check the AppKey. 
/// Use with caution.
pub fn at_app_key(serial: Box<dyn SerialPort>, app_key: &str){
  let w_buf = format!("at+set_config=lora:app_key:{}\r\n", app_key);
  send_read(serial, &w_buf);
}
