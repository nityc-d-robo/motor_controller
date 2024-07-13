mod motor;
mod udp_communication;

use motor::Motor;

const FREQUENCY: f64 = 1000.;
const START_DUTY_CYCLE: f64 = 0.1;
fn main() {
    let setup_motors = vec![
        Motor::new(16, 1, START_DUTY_CYCLE, FREQUENCY),
        Motor::new(26, 2, START_DUTY_CYCLE, FREQUENCY),
        Motor::new(6, 3, START_DUTY_CYCLE, FREQUENCY),
    ];

    udp_communication::run_motor(setup_motors, "8080");

    udp_communication::send_pwm_udp("8081", "0.0.0.0:8080", 0, 0.5);
}
