use std::{sync::mpsc, thread};
use std::time::Duration;

use anyhow::{anyhow, Result};
use rand::prelude::*;

const NUM_PRODUCERS: usize = 4;

#[derive(Debug)]
struct Msg {
    id: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
    });

    consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time as _));
    }
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}
