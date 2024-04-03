use atomic_interval::AtomicIntervalLight;
use std::time::Duration;
use std::time::Instant;

fn main() {

    let period = Duration::from_secs(1);
    let atomic_interval = AtomicIntervalLight::new(period);

    let time_start = Instant::now();
    let mut counter = 0;
    let elapsed = loop {
        if atomic_interval.is_ticked() {
            println!("{}", counter);
            counter+=1;
        }
        if counter> 5 {
            break time_start.elapsed();
        }
    };

    println!("Elapsed: {:?}", elapsed);
}
