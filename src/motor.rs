//use std::error::Error;
//use std::thread;
//use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin};
//use rppal::pwm::{Channel, Polarity, Pwm};
use std::clone::Clone;

const PWMA: u8 = 16;
const AIN1: u8 = 21;
const AIN2: u8 = 20;

const PWMB: u8 = 26;
const BIN1: u8 = 13;
const BIN2: u8 = 19;

const STBY: u8 = 6;


pub struct Motors {
    gpio: Gpio,
    pwma: OutputPin,
    ain1: OutputPin,
    ain2: OutputPin,
    pwmb: OutputPin,
    bin1: OutputPin,
    bin2: OutputPin,
    stby: OutputPin,
}

pub fn new_motors() -> Motors {
    let gpio = Gpio::new().unwrap();
    let ain1 = gpio.get(AIN1).unwrap().into_output();
    let ain2 = gpio.get(AIN2).unwrap().into_output();
    let pwma = gpio.get(PWMA).unwrap().into_output();
    let bin1 = gpio.get(BIN1).unwrap().into_output();
    let bin2 = gpio.get(BIN2).unwrap().into_output();
    let stby = gpio.get(STBY).unwrap().into_output();
    let pwmb = gpio.get(PWMB).unwrap().into_output();
    Motors { gpio: gpio, pwma: pwma, ain1: ain1, ain2: ain2, pwmb: pwmb, bin1: bin1, bin2: bin2, stby: stby }
}

pub fn stop(motors: &mut Motors) {
    motors.ain1.set_low();
    motors.ain2.set_low();
    motors.bin1.set_low();
    motors.bin2.set_low();
    println!("STOP @inside");
}    

pub fn right(motors: &mut Motors) {
    motors.ain1.set_high();
    motors.ain2.set_low();
    motors.bin1.set_high();
    motors.bin2.set_low();
    println!("RIGHT @inside");
}

pub fn left(motors: &mut Motors) {
    motors.ain1.set_low();
    motors.ain2.set_high();
    motors.bin1.set_low();
    motors.bin2.set_high();
    println!("LEFT @inside");
}

pub fn front(motors: &mut Motors) {
    motors.ain1.set_high();
    motors.ain2.set_low();
    motors.bin1.set_low();
    motors.bin2.set_high();
    println!("FRONT @inside");
}

pub fn back(motors: &mut Motors) {
    motors.ain1.set_low();
    motors.ain2.set_high();
    motors.bin1.set_high();
    motors.bin2.set_low();
    println!("BACK @inside");
}

pub fn finish(motors: &mut Motors) {
    _ = motors.pwma.clear_pwm();
    _ = motors.pwmb.clear_pwm();
    motors.stby.set_low();
    println!("FINISH @inside");
}

pub fn prepare(motors: &mut Motors) {
    _ = motors.pwma.set_pwm_frequency(50.0, 1.0);
    _ = motors.pwmb.set_pwm_frequency(50.0, 1.0);
    motors.ain1.set_low();
    motors.ain2.set_low();
    motors.bin1.set_low();
    motors.bin2.set_low();
    motors.stby.set_high();
    println!("PREPARE @inside");
}

//#[derive(Clone)]
/* pub struct Motors {
    gpio: Gpio,
    pwma: OutputPin,
    ain1: OutputPin,
    ain2: OutputPin,
    pwmb: OutputPin,
    bin1: OutputPin,
    bin2: OutputPin,
    stby: OutputPin,
}

impl Motors {
    pub fn new() -> Motors {
        let gpio = Gpio::new().unwrap();
        let ain1 = gpio.get(AIN1).unwrap().into_output();
        let ain2 = gpio.get(AIN2).unwrap().into_output();
        let pwma = gpio.get(PWMA).unwrap().into_output();
        let bin1 = gpio.get(BIN1).unwrap().into_output();
        let bin2 = gpio.get(BIN2).unwrap().into_output();
        let stby = gpio.get(STBY).unwrap().into_output();
        let pwmb = gpio.get(PWMB).unwrap().into_output();
        Motors { gpio: gpio, pwma: pwma, ain1: ain1, ain2: ain2, pwmb: pwmb, bin1: bin1, bin2: bin2, stby: stby }
    }

    pub fn prepare(&mut self) {
        _ = self.pwma.set_pwm_frequency(50.0, 1.0);
        _ = self.pwmb.set_pwm_frequency(50.0, 1.0);
        self.ain1.set_low();
        self.ain2.set_low();
        self.bin1.set_low();
        self.bin2.set_low();
        self.stby.set_low();
    }

    pub fn stop(&mut self) {
        println!("stop inside");
        self.ain1.set_low();
        self.ain2.set_low();
        self.bin1.set_low();
        self.bin2.set_low();
    }    

    pub fn front(&mut self) {
        println!("front inside");
        self.ain1.set_high();
        self.ain2.set_low();
        self.bin1.set_high();
        self.bin2.set_low();
    }

    pub fn back(&mut self) {
        self.ain1.set_low();
        self.ain2.set_high();
        self.bin1.set_low();
        self.bin2.set_high();
    }

    pub fn left(&mut self) {
        self.ain1.set_high();
        self.ain2.set_low();
        self.bin1.set_low();
        self.bin2.set_high();
    }

    pub fn right(&mut self) {
        self.ain1.set_low();
        self.ain2.set_high();
        self.bin1.set_high();
        self.bin2.set_low();
    }

    pub fn finish(&mut self) {
        self.pwma.clear_pwm();
        self.pwmb.clear_pwm();
        self.stby.set_low();
    }


} */