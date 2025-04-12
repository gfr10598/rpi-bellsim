use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::pwm::{Channel, Polarity, Pwm};

fn main() -> Result<(), Box<dyn Error>> {
    // TODO:
    //  Configure a PWM to toggle at 1 kHz.
    //  Configure an input pin for edge triggering.
    //  Read the interrupts, and look at the stats for the intervals.
    // Experiment with partrt to see what difference that makes.

    println!("Remember to sudo dtoverlay pwm-2chan!");
    let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, false)?;
    let pwm = Pwm::with_frequency(Channel::Pwm1, 1.0, 0.25, Polarity::Normal, false)?;

    let gpio = Gpio::new()?;
    let mut pin = gpio.get(16)?.into_output();

    let start = std::time::Instant::now();
    // About 130 nsec per toggle.
    // Only 6 nsec per set_high/set_low, even when compiling other code.
    for _ in 0..1000 {
        pin.set_high();
        pin.set_low();
    }
    println!("Elapsed time: {:?}", start.elapsed());

    let mut out = gpio.get(15)?.into_input();
    out.set_async_interrupt(rppal::gpio::Trigger::Both, None, |event| {
        println!("Interrupt at {:?}", event.timestamp);
    })?;

    loop {
        pin.toggle();
        thread::sleep(Duration::from_millis(500));
    }
}
