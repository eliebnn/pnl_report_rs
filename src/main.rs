use rand::Rng;
mod trade_struct;
mod core_struct;


use trade_struct::Trade;
use core_struct::Core;


fn main() {

    let mut rng = rand::thread_rng();
    let mut trades: Vec<Trade> = vec![];

    for _i in 1..10001{
        let price:f32 = rng.gen_range(5..10) as f32;
        let qty:f32 = rng.gen_range(-7..10) as f32;
        let qty = match qty {
            _ if qty == 0.0 => 1.0,
            _ => qty 
        };

        trades.push(Trade{qty: qty, price: price});
    }

    println!("{}", trades.len());

    // Data

    use std::time::Instant;
    let now = Instant::now();

    let mut core = Core::new(trades);
    core.run();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}



