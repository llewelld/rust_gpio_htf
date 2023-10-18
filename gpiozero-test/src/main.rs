// ex:set ts=4 et:

use rust_gpiozero::output_devices;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting");
    let mut servo: output_devices::Servo = output_devices::Servo::new(17);

    for value in [0.5, -0.5] {
        println!("Setting position to {}", value);
        servo.set_position(value);

        println!("Sleep 3 seconds");
        thread::sleep(Duration::from_millis(3000));

        println!("Detaching");
        //servo.detach();

        println!("Sleep 3 seconds");
        thread::sleep(Duration::from_millis(3000));
    }
    println!("Done");
}

