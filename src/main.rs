use std::error::Error;
use std::sync::Mutex;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::pwm::{Channel, Polarity, Pwm};

#[derive(Debug)]
struct Stats {
    x: f64,
    x2: f64,
    count: u64,
    count_over: u64,
    threshold: f64,
}

impl Stats {
    fn new(threshold: f64) -> Self {
        Stats {
            x: 0.0,
            x2: 0.0,
            count: 0,
            count_over: 0,
            threshold,
        }
    }

    fn add(&mut self, value: f64) -> Option<(f32, f32, f32)> {
        self.x += value;
        self.x2 += value * value;
        self.count += 1;
        if self.count > 10 {
            if value - (self.mean().abs() as f64) > self.threshold {
                println!("Outlier detected: {:?}", value / 1000.0);
                self.count_over += 1;
            }
        }
        if self.count % 1000 == 0 {
            Some((
                self.mean(),
                self.stdev(),
                (self.count_over as f64 / self.count as f64) as f32,
            ))
        } else {
            None
        }
    }

    fn mean(&self) -> f32 {
        (self.x / self.count as f64) as f32
    }

    fn stdev(&self) -> f32 {
        let variance = (self.x2 - self.x * self.x / self.count as f64) / self.count as f64;
        if variance < 0.0 {
            println!("Variance is negative: {:?}", self);
        }
        variance.sqrt() as f32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO:
    //  Configure a PWM to toggle at 1 kHz.
    //  Configure an input pin for edge triggering.
    //  Read the interrupts, and look at the stats for the intervals.
    // Experiment with partrt to see what difference that makes.

    println!("Remember to sudo dtoverlay pwm-2chan!");
    let _pwm = Pwm::with_frequency(Channel::Pwm0, 1000.0, 0.25, Polarity::Normal, true)?;
    let _pwm2 = Pwm::with_frequency(Channel::Pwm1, 1.0, 0.25, Polarity::Normal, true)?;

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

    let mut input = gpio.get(17)?.into_input();
    let last = Mutex::new(Duration::new(0, 0));
    // Use partrt run rt  nice -n -10 ./target/release/rpi-bellsim
    // This runs at around 0.03% outlier rate with async_interrupt.
    let up = Mutex::new(Stats::new(500.0)); // microseconds
    let down = Mutex::new(Stats::new(500.0));

    // out.set_async_interrupt(rppal::gpio::Trigger::Both, None, move |event| {
    //     let local = *last.lock().unwrap();
    //     let delta = event.timestamp - local;
    //     let micros = 1e6 * delta.as_secs_f64();
    //     *last.lock().unwrap() = event.timestamp;
    //     if micros > 10000.0 {
    //         return;
    //     }
    //     match event.trigger {
    //         rppal::gpio::Trigger::RisingEdge => {
    //             let maybe = up.lock().unwrap().add(micros);
    //             if let Some((mean, stdev, fraction_bad)) = maybe {
    //                 print!("Up:   {:.1}", mean);
    //                 print!("    \t+/- {:.1}", stdev);
    //                 println!("  \tpercent bad: {:.3}", fraction_bad * 100.0);
    //             }
    //         }
    //         rppal::gpio::Trigger::FallingEdge => {
    //             let maybe = down.lock().unwrap().add(micros);
    //             if let Some((mean, stdev, fraction_bad)) = maybe {
    //                 print!("Down: {:.1}", mean);
    //                 print!("    \t+/- {:.1}", stdev);
    //                 println!("  \tpercent bad: {:.3}", fraction_bad * 100.0);
    //             }
    //         }
    //         _ => unreachable!(),
    //     }
    // })?;

    input.set_interrupt(rppal::gpio::Trigger::Both, None)?;
    loop {
        //pin.toggle();
        let e = gpio.poll_interrupts(&[&input], false, None)?;
        if let Some((pin, event)) = e {
            if pin.pin() != 17 {
                continue;
            }
            let local = *last.lock().unwrap();
            let delta = event.timestamp - local;
            let micros = 1e6 * delta.as_secs_f64();
            *last.lock().unwrap() = event.timestamp;
            if micros > 10000.0 {
                continue;
            }
            match event.trigger {
                rppal::gpio::Trigger::RisingEdge => {
                    let maybe = up.lock().unwrap().add(micros);
                    if let Some((mean, stdev, fraction_bad)) = maybe {
                        print!("Up:   {:.1}", mean);
                        print!("    \t+/- {:.1}", stdev);
                        println!("  \tpercent bad: {:.3}", fraction_bad * 100.0);
                    }
                }
                rppal::gpio::Trigger::FallingEdge => {
                    let maybe = down.lock().unwrap().add(micros);
                    if let Some((mean, stdev, fraction_bad)) = maybe {
                        print!("Down: {:.1}", mean);
                        print!("    \t+/- {:.1}", stdev);
                        println!("  \tpercent bad: {:.3}", fraction_bad * 100.0);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
