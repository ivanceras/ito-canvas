mod line;

pub trait Shape<'a> {
    /// Returns an iterator over all points of the shape
    fn points(&'a self) -> Box<dyn Iterator<Item = (f64, f64)> + 'a>;
}

pub const DOTS: [[u16; 2]; 4] = [
    [0x0001, 0x0008],
    [0x0002, 0x0010],
    [0x0004, 0x0020],
    [0x0040, 0x0080],
];
pub const BRAILLE_OFFSET: u16 = 0x2800;
pub const BRAILLE_BLANK: char = ' ';

struct Grid {
    cells: Vec<u16>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            cells: vec![BRAILLE_OFFSET; width * height - 1],
        }
    }

    fn to_string(&self, width: usize) -> String {
        let mut buf = String::new();
        for (i, cell) in self.cells.iter().enumerate() {
            if i != 0 && i % width == 0 {
                buf.push('\n');
            }
            let ch = String::from_utf16(&[*cell]).unwrap();
            println!("ch: {} [{}]", ch.chars().next().unwrap().escape_unicode(), ch);
            if ch == "\u{2800}"{
                buf.push(' ');
            }else{
                buf.push_str(&ch);
            }
        }
        buf
    }

    fn reset(&mut self) {
        for c in &mut self.cells {
            *c = BRAILLE_OFFSET;
        }
    }
}

/// Holds the state of the Canvas when painting to it.
pub struct Context {
    width: u16,
    height: u16,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    grid: Grid,
}

impl Context {
    /// Draw any object that may implement the Shape trait
    pub fn draw<'b, S>(&mut self, shape: &'b S)
    where
        S: Shape<'b>,
    {
        let left = self.x_bounds[0];
        let right = self.x_bounds[1];
        let bottom = self.y_bounds[1];
        let top = self.y_bounds[0];
        let mut n = 0;
        for (x, y) in shape
            .points()
            .filter(|&(x, y)| x >= left && x <= right && y >= top && y <= bottom)
        {
            println!("n: {}", n);
            n += 1;
            let dy = ((top - y) * f64::from(self.height - 1) * 4.0 / (top - bottom)) as usize;
            let dx = ((x - left) * f64::from(self.width - 1) * 2.0 / (right - left)) as usize;
            let index = dy / 4 * self.width as usize + dx / 2;
            let dy_index = dy % 4;
            let dx_index = dx % 2;
            dbg!(dy_index);
            dbg!(dx_index);
            let braille = DOTS[dy_index][dx_index];
            dbg!(braille);
            let existing = self.grid.cells[index];
            dbg!(existing);
            let new_braille = existing | braille;
            let ch = String::from_utf16(&[new_braille]).unwrap();
            dbg!(ch);
            dbg!(new_braille);
            self.grid.cells[index] = new_braille;
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::Line;

    #[test]
    fn draw_horizontal_lines() {
        let width = 10;
        let height = 1;
        let mut context = Context {
            width: width as u16,
            height: height as u16,
            x_bounds: [0.0, width as f64],
            y_bounds: [0.0, height as f64],
            grid: Grid::new(width, height),
        };

        context.draw(&Line {
            x1: 0.0,
            y1: 0.5,
            x2: width as f64,
            y2: 0.5,
        });
        let result = context.grid.to_string(width as usize);
        assert_eq!(result, "⠉⠉⠉⠉⠉⠉⠉⠉⠉");
    }

    #[test]
    fn draw_vertical_lines() {
        let width = 1.0;
        let height = 10.0;
        let mut context = Context {
            width: width as u16,
            height: height as u16,
            x_bounds: [0.0, width],
            y_bounds: [0.0, height],
            grid: Grid::new(width as usize, height as usize),
        };

        context.draw(&Line {
            x1: 0.5,
            y1: 0.0,
            x2: 0.5,
            y2: height,
        });
        let result = context.grid.to_string(width as usize);
        println!("{}", result);
        let expected = "⡇\n\
                        ⡇\n\
                        ⡇\n\
                        ⡇\n\
                        ⡇\n\
                        ⡇\n\
                        ⡇\n\
                        ⡇\n\
                        ⡇";
        assert_eq!(result, expected);
    }

    #[test]
    fn draw_slanted_lines() {
        let width = 10.0;
        let height = 10.0;
        let mut context = Context {
            width: width as u16,
            height: height as u16,
            x_bounds: [0.0, width],
            y_bounds: [0.0, height],
            grid: Grid::new(width as usize, height as usize),
        };

        context.draw(&Line {
            x1: 0.5,
            y1: 0.0,
            x2: width,
            y2: height,
        });
        let result = context.grid.to_string(width as usize);
        println!("{}", result);
        for ch in result.chars(){
            println!("ch: [{}] {}",ch, ch.escape_unicode());
        }


        let expected = "⠙⡄        \n \
                         ⠸⡀       \n  \
                          ⠑⡄      \n   \
                           ⠱⡀     \n    \
                            ⠱⡀    \n     \
                             ⠱⡀   \n      \
                              ⢣⡀  \n       \
                               ⢱  \n        \
                                ⢣ \n         ";
        assert_eq!(result.chars().count(), expected.chars().count());
        assert_eq!(result, expected);
    }
}
