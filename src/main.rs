// there are exsamples

mod motor;
use motor::Motor;
use std::collections::HashMap;
use std::{thread, time};


const FREQUENCY: f64 = 1000.;
const START_DUTY_CYCLE: f64 = 0.1;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let setup_motors = [
        Motor::new(16, 1, START_DUTY_CYCLE, FREQUENCY),
        Motor::new(26, 2, START_DUTY_CYCLE, FREQUENCY),
        Motor::new(6, 3, START_DUTY_CYCLE, FREQUENCY),
    ];

    let motors = HashMap::from([
        ("L", setup_motors[0].duty_cycle.clone()),
        ("R", setup_motors[1].duty_cycle.clone()),
        ("U", setup_motors[2].duty_cycle.clone()),
    ]);

    // start
    {
        for mut setup_motor in setup_motors {
            thread::spawn(move || {
                setup_motor.start();
            });
        }
    }

    // change
    {
        let some = time::Duration::from_millis(5000);
        thread::sleep(some);

        for i in ["L", "R", "U"] {
            let mut duty_cycle = motors[i].lock().unwrap();
            *duty_cycle = 1.;
        }

        thread::sleep(some);

        for i in ["L", "R", "U"] {
            let mut duty_cycle = motors[i].lock().unwrap();
            *duty_cycle = 0.;
        }

        // なぜか少し待たないと暴走する
        let some = time::Duration::from_millis(1);
        thread::sleep(some);
    }

    Ok(())
}
