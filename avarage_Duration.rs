use std::time::Instant;
use std::time::Duration;

fn main() {
    let mut times: Vec<Duration> = Vec::new();
    //let duration = Duration::ZERO;
    let mut xx: Duration = Duration::ZERO;
    for i in (1..10) {
        //println!("Iteration new mine {i}");
        xx = maxop4(i, 1000);
        times.push(xx);
    }
    let mut avrage : Duration = Duration::ZERO;
    let mut total = 0;
    for i in times {
        avrage += i;
        total += 1
    }
    let mut avarage = avrage.div_f64(total as f64);
    println!("{:?}", avarage);
    
}

fn maxop4(i: u8, val :u32) -> Duration  {}