//crossterm
//#[cfg(feature = "crossterm")]
mod crossterm_i;
//#[cfg(feature = "crossterm")]
pub use self::crossterm_i::CrosstermBackend;

// tuikit
//#[cfg(feature = "tuikit")]
//pub mod tuikit_i;
//#[cfg(feature = "tuikit")]
//pub use self::tuikit_i::TuikitBackend;

//TODO Make these geric
use tuikit::draw::Draw as TermDraw;
use tuikit::event::Event as TermEvent;

pub type CatchAll<T> = Result<T, Box<dyn std::error::Error>>;

pub trait Backend {
    fn poll_event(&self) -> CatchAll<TermEvent>;
    fn send_event(&self, event: TermEvent) -> CatchAll<()>;
    fn pause(&self) -> CatchAll<()>;
    fn restart(&self) -> CatchAll<()>;
    fn term_size(&self) -> CatchAll<(usize, usize)>;
    fn draw(&self, draw: &dyn TermDraw) -> CatchAll<()>;
    fn present(&self) -> CatchAll<()>;
}

pub struct Term<B>
where
    B: Backend,
{
    backend: B,
    options: TermOptions,
}

impl<B> Term<B>
where
    B: Backend,
{
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            options: Default::default(),
        }
    }

    pub fn with_options(backend: B, options: TermOptions) -> CatchAll<Self> {
        Ok(Self { backend, options })
    }

    pub fn poll_event(&self) -> CatchAll<TermEvent> {
        self.backend.poll_event()
    }

    pub fn send_event(&self, event: TermEvent) -> CatchAll<()> {
        self.backend.send_event(event)
    }

    pub fn pause(&self) -> CatchAll<()> {
        self.backend.pause()
    }

    pub fn restart(&self) -> CatchAll<()> {
        self.backend.restart()
    }
    pub fn term_size(&self) -> CatchAll<(usize, usize)> {
        self.backend.term_size()
    }
    pub fn draw(&self, draw: &dyn TermDraw) -> CatchAll<()> {
        self.backend.draw(draw)
    }
    pub fn present(&self) -> CatchAll<()> {
        self.backend.present()
    }
}

#[derive(Default)]
pub struct TermOptions {
    max_height: TermHeight,
    min_height: TermHeight,
    height: TermHeight,
    clear_on_exit: bool,
    mouse_enabled: bool,
}

pub enum TermHeight {
    Fixed(usize),
    Percent(usize),
}

impl Default for TermHeight {
    fn default() -> Self {
        TermHeight::Percent(100)
    }
}

impl TermOptions {
    pub fn max_height(mut self, max_height: TermHeight) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn min_height(mut self, min_height: TermHeight) -> Self {
        self.min_height = min_height;
        self
    }

    pub fn height(mut self, height: TermHeight) -> Self {
        self.height = height;
        self
    }
}
