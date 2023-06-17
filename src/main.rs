use rand::Rng;
mod trade_struct;
mod calc_struct;

use trade_struct::Trade;
use calc_struct::{FIFO, LIFO, Core};


fn main() {

    let mut rng = rand::thread_rng();
    let mut trades: Vec<Trade> = vec![];

    for _i in 1..5001{
        let price:f32 = rng.gen_range(5..10) as f32;
        let qty:f32 = rng.gen_range(-7..10) as f32;
        let qty = match qty {
            _ if qty == 0.0 => 1.0,
            _ => qty 
        };

        trades.push(Trade{qty: qty, price: price});
    }

    println!("Number of trades: {}\r\n\r\n", trades.len());

    let mut trades: Vec<Trade> = vec![];

    trades.push(Trade{qty: 1.0, price: 10.0});
    trades.push(Trade{qty: 2.0, price: 12.0});
    trades.push(Trade{qty: 9.0, price: 15.0});
    trades.push(Trade{qty: -5.0, price: 12.0});
    trades.push(Trade{qty: -1.0, price: 11.0});
    trades.push(Trade{qty: 2.0, price: 12.0});


    // Data

    use std::time::Instant;
    let now = Instant::now();

    println!("FIFO:\r\n----");
    let mut cls = FIFO::new();
    cls.run(&trades);

    for ut in cls.unwind_trades.iter() {
        println!{"{}", ut.get_pnl()};
    }

    println!("\r\n\r\nLIFO:\r\n----");
    let mut cls = LIFO::new();
    cls.run(&trades);

    for ut in cls.unwind_trades.iter() {
        println!{"{}", ut.get_pnl()};
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
