use std::sync::{Arc, Mutex};

use std::net::UdpSocket;

use serde::{Deserialize, Serialize};
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct JsonData {
    id: usize,
    power: f64,
}
const OWN_ADDR: &str = "0.0.0.0";

pub fn recv_pwm_udp(motors: &mut Vec<Arc<Mutex<f64>>>, port: &str) {
    match UdpSocket::bind(OWN_ADDR.to_owned() + ":" + port) {
        Ok(sock) => {
            let mut buff = [0; 200];
            match sock.recv_from(&mut buff) {
                Ok((recv_size, _src)) => match String::from_utf8(buff[..recv_size].to_vec()) {
                    Ok(recv_json) => judge(recv_json, motors),
                    Err(recv_json) => {
                        println!("failed to convert to string from u8 array:{}", recv_json)
                    }
                },
                Err(_) => println!("failed to receive message"),
            }
        }
        Err(recv_json) => {
            println!("failed to start udp receiver:{}", recv_json);
        }
    }
}

fn judge(recv_json: String, motors: &mut Vec<Arc<Mutex<f64>>>) {
    let data: JsonData = serde_json::from_str(&recv_json.clone()).unwrap();

    let mut duty_cycle = motors[data.id].lock().unwrap();
    *duty_cycle = data.power;
}

// {"id": 3,"power": -0.1}
pub fn send_pwm_udp(own_port: &str, broadcast_addr: &str, id: usize, power: f64) {
    let send_data = JsonData { id, power };

    match UdpSocket::bind(OWN_ADDR.to_owned() + ":" + own_port) {
        Ok(sock) => {
            sock.set_broadcast(true).expect("failed to set broadcast");
            let input = serde_json::to_string(&send_data).unwrap();

            match sock.send_to(input.as_bytes(), broadcast_addr) {
                Ok(v) => println!("send message : {}", &input[..v]),
                Err(v) => println!("failed to send message:{}", v),
            }
        }
        Err(v) => println!("failed to start sender:{}", v),
    }
}
