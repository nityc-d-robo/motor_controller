use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[allow(dead_code)]
pub trait PwmControl {
    fn get_duty_cycle(&self) -> Arc<Mutex<f64>>;
    fn get_frequency(&self) -> f64;
    fn start(&mut self);
}
impl PwmControl for PwmDir {
    fn get_duty_cycle(&self) -> Arc<Mutex<f64>> {
        Arc::clone(&self.duty_cycle)
    }

    fn get_frequency(&self) -> f64 {
        self.frequency
    }

    fn start(&mut self) {
        let period = Duration::from_secs_f64(1.0 / self.frequency);
        let start = Instant::now();

        loop {
            if *self.duty_cycle.lock().unwrap() > 0. {
                self.dir_pin.set_high();
            } else {
                self.dir_pin.set_low();
            }

            let mut duty_cycle = (*self.duty_cycle.lock().unwrap()).abs();

            duty_cycle = duty_cycle.max(0.);
            duty_cycle = duty_cycle.min(1.);

            let high_time = period.mul_f64(duty_cycle);
            let low_time = period - high_time;

            self.pwm_pin.set_high();
            thread::sleep(high_time);

            self.pwm_pin.set_low();
            thread::sleep(low_time);

            // Check if we've overshot our period and adjust sleep accordingly
            let elapsed = start.elapsed();
            if elapsed < period {
                thread::sleep(period - elapsed);
            }
        }
    }
}

impl PwmControl for DualPwm {
    fn get_duty_cycle(&self) -> Arc<Mutex<f64>> {
        Arc::clone(&self.duty_cycle)
    }

    fn get_frequency(&self) -> f64 {
        self.frequency
    }

    fn start(&mut self) {
        let period = Duration::from_secs_f64(1.0 / self.frequency);
        let start = Instant::now();

        loop {
            let pwm_pin_pointer = if *self.duty_cycle.lock().unwrap() > 0. {
                self.pwm_pin2.set_low();
                &mut self.pwm_pin1
            } else {
                self.pwm_pin1.set_low();
                &mut self.pwm_pin2
            };

            let mut duty_cycle = (*self.duty_cycle.lock().unwrap()).abs();

            duty_cycle = duty_cycle.max(0.);
            duty_cycle = duty_cycle.min(1.);

            let high_time = period.mul_f64(duty_cycle);
            let low_time = period - high_time;

            pwm_pin_pointer.set_high();
            thread::sleep(high_time);

            pwm_pin_pointer.set_low();
            thread::sleep(low_time);

            // Check if we've overshot our period and adjust sleep accordingly
            let elapsed = start.elapsed();
            if elapsed < period {
                thread::sleep(period - elapsed);
            }
        }
    }
}

pub struct PwmDir {
    pub pwm_pin: OutputPin,
    pub dir_pin: OutputPin,
    pub duty_cycle: Arc<Mutex<f64>>, // 0.0 to 1.0
    pub frequency: f64,              // Hz
}

impl PwmDir {
    pub fn new(pwm_pin: u8, dir_pin: u8, duty_cycle: f64, frequency: f64) -> Self {
        let gpio = Gpio::new().unwrap();
        Self {
            pwm_pin: gpio.get(pwm_pin).unwrap().into_output(),
            dir_pin: gpio.get(dir_pin).unwrap().into_output(),
            duty_cycle: Arc::new(Mutex::new(duty_cycle)),
            frequency,
        }
    }
}

pub struct DualPwm {
    pub pwm_pin1: OutputPin,
    pub pwm_pin2: OutputPin,
    pub duty_cycle: Arc<Mutex<f64>>, // 0.0 to 1.0
    pub frequency: f64,              // Hz
}

impl DualPwm {
    pub fn new(pwm_pin: u8, dir_pin: u8, duty_cycle: f64, frequency: f64) -> Self {
        let gpio = Gpio::new().unwrap();
        Self {
            pwm_pin1: gpio.get(pwm_pin).unwrap().into_output(),
            pwm_pin2: gpio.get(dir_pin).unwrap().into_output(),
            duty_cycle: Arc::new(Mutex::new(duty_cycle)),
            frequency,
        }
    }
}

pub fn setup_motors<T: PwmControl + std::marker::Send + 'static>(
    setup_motors: Vec<T>,
) -> Vec<Arc<Mutex<f64>>> {
    let motors = setup_motors
        .iter()
        .map(|pwm| pwm.get_duty_cycle().clone())
        .collect::<Vec<_>>();

    // start
    {
        for mut setup_motor in setup_motors {
            thread::spawn(move || {
                setup_motor.start();
            });
        }
    }

    motors
}
