// samples

mod motor;
mod udp_communication;
use motor::{setup_motors, DualPwm, PwmDir};

const FREQUENCY: f64 = 1000.;
const START_DUTY_CYCLE: f64 = 0.0;
fn main() {
    let pwm_dir_motors = vec![
        PwmDir::new(17, 18, START_DUTY_CYCLE, FREQUENCY), //　motor id 0
        PwmDir::new(27, 22, START_DUTY_CYCLE, FREQUENCY), // 1
        PwmDir::new(23, 24, START_DUTY_CYCLE, FREQUENCY), // 2
    ];

    let dualpwm_motors = vec![
        DualPwm::new(25, 32, START_DUTY_CYCLE, FREQUENCY), // 3
        DualPwm::new(16, 26, START_DUTY_CYCLE, FREQUENCY), // 4
        DualPwm::new(13, 2, START_DUTY_CYCLE, FREQUENCY),  // 5
    ];

    let mut motors = setup_motors(pwm_dir_motors);
    motors.extend(setup_motors(dualpwm_motors));

    //  Changing motors changes the rotation speed of the motor Maximum value 1 Minimum value -1

    // With send_pwm_udp and recv_pwm_udp, it can be operated remotely.
    loop {
        udp_communication::recv_pwm_udp(&mut motors, "8080");
    }

    // Note that the following line is not executed because it stops at loop.
    // Execute the following from another program
    udp_communication::send_pwm_udp("8081", "0.0.0.0:8080", 0, 0.5);
}
