use super::Backend;
use super::CatchAll;
use crossterm::event::Event;
use std::io::Write;

use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};

type AM<T> = Arc<Mutex<T>>;

struct Crossterm {
    tx: AM<Sender<Event>>,
    rx: AM<Receiver<Event>>,
}

pub struct CrosstermBackend<W: Write> {
    buffer: W,
    term: Crossterm,
}

impl<W> CrosstermBackend<W>
where
    W: Write,
{
    pub fn new(buffer: W) -> CrosstermBackend<W> {
        let (tx, rx) = channel();
        CrosstermBackend {
            buffer,
            term: Crossterm {
                tx: Arc::new(Mutex::new(tx)),
                rx: Arc::new(Mutex::new(rx)),
            },
        }
    }
}

impl<W> Backend for CrosstermBackend<W>
where
    W: Write,
{
    fn poll_event(&self) -> super::CatchAll<Event> {
        use crossterm::{event::poll, Result};
        use std::time::Duration;

        // poll for events
        loop {
            if let Ok(true) = poll(Duration::from_millis(100)) {
                return Ok(crossterm::event::read()?);
            }
            if let Ok(ev) = self.term.rx.lock().unwrap().try_recv() {
                return Ok(ev);
            }
        }
    }

    fn send_event(&self, event: Event) -> super::CatchAll<()> {
        Ok(self.term.tx.lock().unwrap().send(event)?)
    }

    fn pause(&self) -> super::CatchAll<()> {
        //TODO
        Ok(())
    }

    fn restart(&self) -> CatchAll<()> {
        //TODO
        Ok(())
    }
    fn term_size(&self) -> CatchAll<(usize, usize)> {
        let (x, y) = crossterm::terminal::size()?;
        Ok((x.into(), y.into()))
    }
    ///XXX
    fn draw(&self, draw: &dyn Draw) -> CatchAll<()> {
        self.term.draw(draw)
    }
    fn present(&self) -> CatchAll<()> {
        self.term.present()
    }
}

///XXX
use tuikit::prelude::Draw;
