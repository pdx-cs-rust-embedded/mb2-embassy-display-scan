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
        c.set_high();
    }

    loop {
        let frame_rates = (1..9).step_by(2).chain((10..60).step_by(5));

        for fr in frame_rates {
            let delay_time = 1_000_000 / (fr * 25);
            for _ in 0..5 * fr {
                for r in &mut rows {
                    r.set_high();
                    for c in &mut cols {
                        c.set_low();
                        Timer::after_micros(delay_time).await;
                        c.set_high();
                    }
                    r.set_low();
                }
            }
            Timer::after_millis(250).await;
        }
        Timer::after_millis(2000).await;
    }
}
