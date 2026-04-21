use super::terminal::{Position, Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod buffer;
use buffer::Buffer;

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size
}

impl View {
    pub fn resize(&mut self, to: Size){
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn render_welcome_screen(&self) -> Result<(), Error> {
        let Size { height, .. } = self.size;

        Terminal::clear_row()?;
        for current_row in 0..height {
            Terminal::clear_row()?;

            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                self.draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::move_caret_to(Position {
                    col: 0,
                    row: current_row + 1,
                })?;
            }
        }
        Ok(())
    }

    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size { height, width } = self.size;

        for current_row in 0..height {
            Terminal::clear_row()?;
            if let Some(line) = self.buffer.lines.get(current_row) {
                let sliced = line.get(0..width).unwrap_or(line);
                Terminal::print(sliced)?;
                Terminal::move_caret_to(Position {col:0, row: current_row + 1})?;
            } else {
                Self::draw_empty_row()?;
                Terminal::move_caret_to(Position { col: 0, row: current_row + 1 })?;
            }
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        if self.buffer.is_empty() {
            self.render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        self.needs_redraw = false;
        Ok(())
    }

    fn draw_welcome_message(&self) -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = self.size.width;
        let len = welcome_message.len();
        #[allow(clippy::integer_division)]
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn load(&mut self, _filename: &str) {
        if let Ok(buffer) = Buffer::load(_filename) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
