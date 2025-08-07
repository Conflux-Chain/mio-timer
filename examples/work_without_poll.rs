use log::*;
use mio_timer::{Builder, Timer};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut timer: Timer<usize> = Builder::default().build();

    timer.set_timeout(Duration::new(1, 0), 1);
    timer.set_timeout(Duration::new(2, 0), 100);

    loop {
        let state = timer.poll();
        info!("Polled state: {:?}", state);

        if state == Some(100) {
            break;
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
