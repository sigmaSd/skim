use std::io::Write;
use super::Backend;
use super::CatchAll;

use tuikit::event::Event;
use tuikit::draw::Draw;

pub struct TuikitBackend<W: Write> {
    buffer: W,
    term: tuikit::term::Term,
}

impl<W> TuikitBackend<W>
where
    W: Write,
{
    pub fn new(buffer: W) -> TuikitBackend<W> {
        TuikitBackend { buffer , term: tuikit::term::Term::new().unwrap()}
    }
}

impl <W>Backend for TuikitBackend<W> 
where W: Write
{
    fn poll_event(&self) -> super::CatchAll<Event> {
        self.term.poll_event()
    }

    fn send_event(&self, event: Event) -> super::CatchAll<()> {
        self.term.send_event(event)
    }

    fn pause(&self) -> super::CatchAll<()> {
        self.term.pause()
    }

    fn restart(&self) -> CatchAll<()> {
        self.term.restart()
    }
    fn term_size(&self) -> CatchAll<(usize, usize)> {
        self.term.term_size()
    }
    fn draw(&self, draw: &dyn Draw) -> CatchAll<()> {
        self.term.draw(draw)
    }
    fn present(&self) -> CatchAll<()> {
        self.term.present()
    }
}
