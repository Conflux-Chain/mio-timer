use std::time::Duration;

use log::*;
use mio_timer::{EventLoop, EventLoopBuilder, Handler};

struct MyHandler;

impl Handler for MyHandler {
    type TimeoutState = usize;
    type Message = String;

    fn notify(&mut self, _event_loop: &mut EventLoop<Self>, msg: Self::Message) {
        println!("Notification received: {}", msg);
    }

    fn timeout(&mut self, _event_loop: &mut EventLoop<Self>, timeout: Self::TimeoutState) {
        println!("Timeout occurred {timeout:?}");
    }
}

fn main() -> std::io::Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut my_handler = MyHandler;

    let mut event_loop: EventLoop<MyHandler> = EventLoopBuilder::default().build()?;

    let channel = event_loop.channel();

    let _ = event_loop.timeout(1, Duration::from_secs(1));
    let _ = event_loop.timeout(3, Duration::from_secs(3));
    let _ = event_loop.timeout(5, Duration::from_secs(5));

    std::thread::spawn(move || {
        for i in 0..10 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            channel.send(format!("Message {}", i)).unwrap();
        }
    });

    let _ = event_loop.run(&mut my_handler);

    Ok(())
}
