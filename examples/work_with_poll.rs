use log::*;
use mio::{Events, Poll, Token, Waker};
use mio_misc::{NotificationId, queue::NotificationQueue};
use mio_timer::{Builder, Timer};
use std::sync::Arc;
use std::time::Duration;

const TIMER: Token = Token(1);

fn main() -> std::io::Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let mut timer: Timer<usize> = Builder::default().build();

    let waker = Arc::new(Waker::new(poll.registry(), TIMER)?);
    let queue = Arc::new(NotificationQueue::new(waker));

    let noti_id = NotificationId::gen_next();

    timer.register(queue.clone(), noti_id)?;

    timer.set_timeout(Duration::new(1, 0), 1);
    timer.set_timeout(Duration::new(2, 0), 100);

    'outer: loop {
        poll.poll(&mut events, Some(Duration::from_millis(100)))?;

        for event in events.iter() {
            info!("Got event: {:?}", event);
            match event.token() {
                TIMER => {
                    let noti_id = queue.pop();
                    info!("Notification ID popped: {:?}", noti_id);
                    while let Some(state) = timer.poll() {
                        info!("Got state from timer: {}", state);
                        if state == 100 {
                            break 'outer;
                        }
                    }
                }
                _ => {
                    assert!(false, "Unexpected token: {:?}", event.token());
                }
            }
        }
    }

    Ok(())
}
