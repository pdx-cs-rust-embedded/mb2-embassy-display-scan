#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use microbit_bsp::Microbit;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Microbit::default();
    let display = board.display;
    let (mut rows, mut cols) = display.into_inner();

    for r in &mut rows {
        r.set_low();
    }
    for c in &mut cols {
        c.set_low();
    }

    loop {
        for r in &mut rows {
            r.set_high();
            for c in &mut cols {
                c.set_high();
                Timer::after_millis(100).await;
                c.set_low();
            }
            r.set_low();
        }
    }
}
