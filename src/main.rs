use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

fn main() -> Result<(), Box<dyn Error>>  {
  // TODO:
  //  Configure a PWM to toggle at 1 kHz.
  //  Configure an input pin for edge triggering.
  //  Read the interrupts, and look at the stats for the intervals.
  // Experiment with partrt to see what difference that makes.

    println!("Hello, world!");

    let gpio = Gpio::new()?;
    let mut pin = gpio.get(17)?.into_output();
    pin.set_high();

    let start = std::time::Instant::now();
    // About 130 nsec per toggle.
    // Only 12 nsec per set_high/set_low.
    for _ in 0..1000 {
        pin.set_high();
        pin.set_low();
    }
    println!("Elapsed time: {:?}", start.elapsed());
    loop {
        pin.toggle();
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
