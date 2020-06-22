use crate::Shape;

/// ```ignore
///      0 1 2 3 4           B C D          BRAILLE ⣿
///     0┌─┬─┬─┬─┐        A┌─┬─┬─┬─┐E       ┌       ┐
///     1├─┼─┼─┼─┤         │ │ │ │ │         ─▮───▮─
///     2├─┼─┼─┼─┤        F├─G─H─I─┤J         │   │
///     3├─┼─┼─┼─┤         │ │ │ │ │         ─▮───▮─
///     4├─┼─┼─┼─┤        K├─L─M─N─┤O         │   │
///     5├─┼─┼─┼─┤         │ │ │ │ │         ─▮───▮─
///     6├─┼─┼─┼─┤        P├─Q─R─S─┤T         │   │
///     7├─┼─┼─┼─┤         │ │ │ │ │         ─▮───▮─
///     8└─┴─┴─┴─┘        U└─┴─┴─┴─┘Y       └       ┘
/// ```                      V W X

pub const DOTS: [[u16; 2]; 4] = [
    [0x0001, 0x0008],
    [0x0002, 0x0010],
    [0x0004, 0x0020],
    [0x0040, 0x0080],
];
pub const BRAILLE_OFFSET: u16 = 0x2800;

pub struct Grid {
    width: usize,
    cells: Vec<u16>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            cells: vec![BRAILLE_OFFSET; width * height],
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        for (i, cell) in self.cells.iter().enumerate() {
            if i != 0 && i % self.width == 0 {
                buf.push('\n');
            }
            let ch = String::from_utf16(&[*cell]).unwrap();
            if ch == "\u{2800}" {
                buf.push(' ');
            } else {
                buf.push_str(&ch);
            }
        }
        buf
    }

    pub fn reset(&mut self) {
        for c in &mut self.cells {
            *c = BRAILLE_OFFSET;
        }
    }
}

/// Holds the state of the Canvas when painting to it.
/// width of 1 cell text is 0.5 and height is 1.0
pub struct Context {
    width: f32,
    height: f32,
    x_bounds: (f32, f32),
    y_bounds: (f32, f32),
    grid: Grid,
}

impl Context {
    pub fn new(width: f32, height: f32) -> Self {
        let width = width * 2.0;
        Context {
            width,
            height,
            x_bounds: (0.0, width),
            y_bounds: (0.0, height),
            grid: Grid::new(width as usize, height as usize),
        }
    }

    pub fn to_string(&self) -> String {
        self.grid.to_string()
    }

    /// Draw any object that may implement the Shape trait
    pub fn draw<'b, S>(&mut self, shape: &'b S)
    where
        S: Shape<'b>,
    {
        let (left, right) = self.x_bounds;
        let (top, bottom) = self.y_bounds;
        for (x, y) in shape
            .points()
            .map(|(x, y)| (2.0 * x, y))
            .filter(|&(x, y)| x >= left && x < right && y >= top && y < bottom)
        {
            let dy =
                ((top - y) * (self.height) * 4.0 / (top - bottom)) as usize;
            let dx =
                ((x - left) * (self.width) * 2.0 / (right - left)) as usize;
            let index = dy / 4 * self.width as usize + dx / 2;
            let dy_index = dy % 4;
            let dx_index = dx % 2;
            let braille = DOTS[dy_index][dx_index];
            self.grid.cells[index] |= braille;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::Line;

    #[test]
    fn draw_cell_horizontal_line() {
        let mut context = Context::new(0.5, 1.0);

        context.draw(&Line {
            x1: 0.0,
            y1: 0.5,
            x2: 1.0,
            y2: 0.5,
        });
        let result = context.to_string();
        assert_eq!(result, "⠤");
    }

    #[test]
    fn draw_cell_vertical_line() {
        let mut context = Context::new(0.5, 1.0);

        context.draw(&Line {
            x1: 0.125,
            y1: 0.0,
            x2: 0.125,
            y2: 1.0,
        });
        let result = context.to_string();
        assert_eq!(result, "⡇");
    }

    #[test]
    fn draw_cell_slant_line1() {
        let mut context = Context::new(0.5, 1.0);

        context.draw(&Line {
            x1: 0.0,
            y1: 0.0,
            x2: 0.5,
            y2: 1.0,
        });
        let result = context.to_string();
        assert_eq!(result, "⢣");
    }
    #[test]
    fn draw_cell_slant_line2() {
        let mut context = Context::new(0.5, 1.0);

        context.draw(&Line {
            x1: 0.0,
            y1: 1.0,
            x2: 0.5,
            y2: 0.0,
        });
        let result = context.to_string();
        assert_eq!(result, "⡰");
    }

    #[test]
    fn draw_cell_slant_line3() {
        let mut context = Context::new(0.5, 1.0);

        context.draw(&Line {
            x1: 0.0,
            y1: 0.75,
            x2: 0.5,
            y2: 0.375,
        });
        let result = context.to_string();
        assert_eq!(result, "⡠");
    }

    #[test]
    fn draw_horizontal_lines() {
        let width = 10;
        let height = 1;
        let mut context = Context::new(width as f32, height as f32);

        context.draw(&Line {
            x1: 0.0,
            y1: 0.5,
            x2: width as f32,
            y2: 0.5,
        });
        let result = context.to_string();
        assert_eq!(result, "⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤");
    }

    #[test]
    fn draw_vertical_lines() {
        let width = 1.0;
        let height = 10.0;
        let mut context = Context::new(width as f32, height as f32);

        context.draw(&Line {
            x1: 0.5,
            y1: 0.0,
            x2: 0.5,
            y2: height,
        });
        let result = context.to_string();
        let expected =
            [" ⡇", " ⡇", " ⡇", " ⡇", " ⡇", " ⡇", " ⡇", " ⡇", " ⡇", " ⡇"];
        assert_eq!(result, expected.join("\n"));
    }

    #[test]
    fn draw_slash_lines() {
        let width = 10.0;
        let height = 10.0;

        let mut context = Context::new(width as f32, height as f32);
        context.draw(&Line {
            x1: 0.0,
            y1: 0.0,
            x2: width,
            y2: height,
        });
        let result = context.to_string();

        let expected = [
            "⠑⢄                  ",
            "  ⠑⢄                ",
            "    ⠑⢄              ",
            "      ⠑⢄            ",
            "        ⠑⢄          ",
            "          ⠑⢄        ",
            "            ⠑⢄      ",
            "              ⠑⢄    ",
            "                ⠑⢄  ",
            "                  ⠑⢄",
        ];

        assert_eq!(result, expected.join("\n"));
    }

    #[test]
    fn draw_slash_lines2() {
        let width = 10.0;
        let height = 10.0;
        let mut context = Context::new(width as f32, height as f32);

        context.draw(&Line {
            x1: width,
            y1: height,
            x2: 0.0,
            y2: 0.0,
        });
        let result = context.to_string();
        println!("{}", result);
        let expected = [
            "⠐⢄                  ",
            "  ⠑⢄                ",
            "    ⠑⢄              ",
            "      ⠑⢄            ",
            "        ⠑⢄          ",
            "          ⠑⢄        ",
            "            ⠑⢄      ",
            "              ⠑⢄    ",
            "                ⠑⢄  ",
            "                  ⠑⢄",
        ];
        assert_eq!(result, expected.join("\n"));
    }

    #[test]
    fn draw_slant_lines1() {
        let width = 10.0;
        let height = 10.0;
        let mut context = Context::new(width as f32, height as f32);

        context.draw(&Line {
            x1: 0.0,
            y1: height,
            x2: width,
            y2: 0.0,
        });
        let result = context.to_string();
        println!("{}", result);

        let expected = [
            "                  ⢀⠔",
            "                ⢀⠔⠁ ",
            "              ⢀⠔⠁   ",
            "            ⢀⠔⠁     ",
            "          ⢀⠔⠁       ",
            "        ⢀⠔⠁         ",
            "      ⢀⠔⠁           ",
            "    ⢀⠔⠁             ",
            "  ⢀⠔⠁               ",
            "⢀⠔⠁                 ",
        ];

        assert_eq!(result, expected.join("\n"));
    }

    #[test]
    fn draw_slant_lines2() {
        let width = 10.0;
        let height = 10.0;
        let mut context = Context::new(width as f32, height as f32);

        context.draw(&Line {
            x1: width - 0.5,
            y1: 0.0,
            x2: 0.0,
            y2: height,
        });
        let result = context.to_string();
        println!("{}", result);

        let expected = [
            "                 ⢀⠔⠁",
            "               ⢀⠔⠁  ",
            "             ⢀⠔⠁    ",
            "           ⢀⠔⠁      ",
            "         ⢀⠔⠁        ",
            "        ⡠⠊          ",
            "      ⡠⠊            ",
            "    ⡠⠊              ",
            "  ⡠⠊                ",
            "⡠⠊                  ",
        ];

        assert_eq!(result, expected.join("\n"));
    }
}
