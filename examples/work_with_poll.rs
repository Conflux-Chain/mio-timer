use log::*;
use mio::{Events, Interest, Poll, Token};
use mio_timer::{Builder, Timer};
use std::time::Duration;

const TIMER: Token = Token(1);

fn main() -> std::io::Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();
    
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let mut timer: Timer<usize> = Builder::default().build();

    poll.registry()
        .register(&mut timer, TIMER, Interest::READABLE)?;

    timer.set_timeout(Duration::new(1, 0), 1);
    timer.set_timeout(Duration::new(10, 0), 100);

    'outer: loop {
        poll.poll(&mut events, Some(Duration::from_millis(100)))?;

        for event in events.iter() {
            info!("Got event: {:?}", event);
            match event.token() {
                TIMER => {
                    let state = timer.poll();
                    match state {
                        Some(s) => {
                            if s == 100 {
                                break 'outer;
                            }
                            info!("Got state from timer: {}", s);
                        }
                        None => {
                            info!("Not get state from timer");
                        }
                    }
                    let state = timer.poll();
                    info!("Got state from timer: {:?}", state);
                }
                _ => {
                    assert!(false, "Unexpected token: {:?}", event.token());
                }
            }
        }
    }

    assert!(true, "Timer should have triggered");

    Ok(())
}
