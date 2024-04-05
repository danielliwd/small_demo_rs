use atomic_interval::AtomicInterval;
use std::time::Duration;
use std::time::Instant;
use std::thread;
use std::sync::atomic::Ordering;
use std::env;

fn main() {
    let choice: u32 = env::args().nth(1).unwrap_or_else(|| "1".to_string()).parse().unwrap();
    match choice{
        1=> atomic_tick(),
        2 => thread_tick(),
        _=> return,
    };
}

fn thread_tick(){
    let period = 1;
    let mut counter = 5;
    // counter for loop
    let scheduler = thread::spawn(move || loop{
        if counter <= 0 {
            break;
        }
        let wait_time = Duration::from_secs(period);

        let start = Instant::now();
        thread::sleep(Duration::from_millis(300));
        counter-=1;


        let runtime = start.elapsed();

        if let Some(remaining) = wait_time.checked_sub(runtime) {
            eprintln!(
                "schedule slice has time left over; sleeping for {:?}",
                remaining
            );
            thread::sleep(remaining);
        }
    });

    scheduler.join().expect("Scheduler panicked");
}

fn atomic_tick(){
    let period = Duration::from_secs(1);
    let atomic_interval = AtomicInterval::new(period);

    let time_start = Instant::now();
    let mut counter = 0;
    let elapsed = loop {
        if atomic_interval.is_ticked(Ordering::Relaxed, Ordering::Relaxed) {
            println!("{}", counter);
            counter+=1;
        }
        if counter> 5 {
            break time_start.elapsed();
        }
    };

    println!("Elapsed: {:?}", elapsed);

}
