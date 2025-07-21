# mio-timer

A simple timer library for mio 1.0, which is originally part of the mio 0.6. From mio 0.7, the timer functionality has been removed from the mio crate.

This crate is the timer component from mio 0.6, adapted for mio 1.0.
It provides the same basic functionality and usage patterns as the timer module in mio 0.6.

## Usage

```Rust
use log::*;
use mio::{Events, Interest, Poll, Token};
use mio_timer::{Builder, Timer};
use std::time::Duration;

const TIMER: Token = Token(1);

let mut poll = Poll::new()?;
let mut events = Events::with_capacity(128);

let mut timer: Timer<usize> = Builder::default().build();

poll.registry().register(&mut timer, TIMER, Interest::READABLE)?;

timer.set_timeout(Duration::new(1, 0), 1);

loop {
    poll.poll(&mut events, Some(Duration::from_millis(100)))?;

    for event in events.iter() {
        match event.token() {
            TIMER => {
                let state = timer.poll();
                match state {
                    Some(s) => {
                        info!("Got state from timer: {}", s);
                    }
                    None => {
                        info!("Not get state from timer");
                    }
                }
            }
            _ => {
                assert!(false, "Unexpected token: {:?}", event.token());
            }
        }
    }
}
```

Check the [examples](./examples/) directory for detail usage examples.

## Related Projects

- [mio-channel](https://github.com/oh-jinsu/mio-channel) compatible with mio 1.0
- [mio-misc](https://github.com/onurzdg/mio-misc) compatible with mio 0.8
- [mio-extras](https://github.com/dimbleby/mio-extras) compatible with mio 0.6