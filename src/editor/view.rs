use super::terminal::{Position, Size, Terminal};
use std::{io::Error, iter::repeat};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
struct ViewConfig {
    cell_width: usize,
    cell_height: usize,
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    config: ViewConfig,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { row: at, col: 0 })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        #[allow(clippy::integer_division)]
        // let vertical_center = height / 3;
        let mut tuple_iter = self.buffer.tuples.clone().into_iter();
        let cell_num = width / self.config.cell_width;
        let cell_padding = " ".repeat(self.config.cell_width);

        for current_row in 0..height {
            if current_row % (self.config.cell_height + 1) == 0 {
                Self::render_line(current_row, &Self::build_cell_seperator(width))?;
            } else {
                let line = if let Some(tuple) = tuple_iter.next() {
                    let element_cells = Self::build_formatted_tuple(&tuple, self.config.cell_width);
                    let padding_cells: Vec<String> = repeat(String::from(&cell_padding))
                        .take(cell_num - element_cells.len())
                        .collect();

                    format!("│{}│{}", element_cells.join("│"), padding_cells.join("│"))
                } else {
                    let padding_cells: Vec<String> =
                        repeat(String::from(&cell_padding)).take(cell_num).collect();
                    format!("│{}", padding_cells.join("│"))
                };

                Self::render_line(current_row, &line)?;
            }
        }
        self.needs_redraw = false;

        Ok(())
    }

    fn build_formatted_tuple(tuple: &Vec<String>, cell_width: usize) -> Vec<String> {
        let mut formatted_tuple = Vec::new();

        for current_col in 0..tuple.len() {
            if let Some(el) = tuple.get(current_col) {
                let truncated_el = if el.len() >= cell_width {
                    &el[0..cell_width]
                } else {
                    el
                };

                let padding = " ".repeat(cell_width - truncated_el.len());

                formatted_tuple.push(format!("{}{}", padding, truncated_el));
            } else {
                formatted_tuple.push(" ".repeat(cell_width));
            }
        }

        formatted_tuple
    }

    fn build_cell_seperator(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let seperator = "─".repeat(width);
        seperator
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();

        if width <= len {
            return "~".to_string();
        }

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;

        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
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
            config: ViewConfig {
                cell_width: 10,
                cell_height: 1,
            },
        }
    }
}
