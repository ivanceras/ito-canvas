use crate::fragment::arc;
use crate::fragment::line;
use crate::fragment::thick;
use crate::fragment::Cell;
use crate::fragment::Fragment;
use crate::string_buffer::StringBuffer;
use crate::unicode_map::FRAGMENT_CHAR;
pub(crate) use border::Border;
use std::collections::HashMap;

mod border;

#[derive(Debug)]
pub struct Canvas {
    cells: HashMap<(usize, usize), Vec<Fragment>>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            cells: HashMap::new(),
        }
    }

    pub fn draw_horizontal_line(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        use_thick: bool,
    ) {
        let (x1, y1) = start;
        let (x2, y2) = end;
        assert_eq!(y1, y2, "horizontal line must have the same y1 and y2");
        //swap the points if x1 is greater than x2
        let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let k = Cell::K;
        let m = Cell::M;
        let o = Cell::O;
        let mo = if use_thick { thick(m, o) } else { line(m, o) };
        let km = if use_thick { thick(k, m) } else { line(k, m) };

        let width = x2 - x1 + 1;
        for i in 0..width {
            if let Some(existing) = self.cells.get_mut(&(x1 + i, y1)) {
                if i == 0 {
                    existing.push(mo);
                } else if i == width - 1 {
                    existing.push(km);
                } else {
                    existing.push(km);
                    existing.push(mo);
                }
            } else {
                if i == 0 {
                    self.cells.insert((x1 + i, y1), vec![mo]);
                } else if i == width - 1 {
                    self.cells.insert((x1 + i, y1), vec![km]);
                } else {
                    self.cells.insert((x1 + i, y1), vec![mo, km]);
                }
            }
        }
    }

    pub fn draw_vertical_line(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        use_thick: bool,
    ) {
        let (x1, y1) = start;
        let (x2, y2) = end;
        assert_eq!(x1, x2, "veritcal line must have the same x1 and x2");
        //swap the points if y1 is greater than y2
        let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };

        let c = Cell::C;
        let m = Cell::M;
        let w = Cell::W;

        let mw = if use_thick { thick(m, w) } else { line(m, w) };
        let cm = if use_thick { thick(c, m) } else { line(c, m) };

        let height = y2 - y1 + 1;
        for j in 0..height {
            if let Some(existing) = self.cells.get_mut(&(x1, y1 + j)) {
                if j == 0 {
                    existing.push(mw);
                } else if j == height - 1 {
                    existing.push(cm);
                } else {
                    existing.push(cm);
                    existing.push(mw);
                }
            } else {
                if j == 0 {
                    self.cells.insert((x1, y1 + j), vec![mw]);
                } else if j == height - 1 {
                    self.cells.insert((x1, y1 + j), vec![cm]);
                } else {
                    self.cells.insert((x1, y1 + j), vec![mw, cm]);
                }
            }
        }
    }

    pub fn draw_rect(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        border: Border,
    ) {
        let (x1, y1) = start;
        let (x2, y2) = end;
        self.draw_horizontal_line((x1, y1), (x2, y1), border.use_thick_border);
        self.draw_horizontal_line((x1, y2), (x2, y2), border.use_thick_border);

        self.draw_vertical_line((x1, y1), (x1, y2), border.use_thick_border);
        self.draw_vertical_line((x2, y1), (x2, y2), border.use_thick_border);

        if !border.use_thick_border {
            let o = Cell::O;
            let w = Cell::W;
            let k = Cell::K;
            let c = Cell::C;
            if border.is_top_left_rounded {
                self.cells.insert((x1, y1), vec![arc(o, w)]);
            }
            if border.is_top_right_rounded {
                self.cells.insert((x2, y1), vec![arc(w, k)]);
            }
            if border.is_bottom_left_rounded {
                self.cells.insert((x1, y2), vec![arc(c, o)]);
            }
            if border.is_bottom_right_rounded {
                self.cells.insert((x2, y2), vec![arc(k, c)]);
            }
        }
    }

    fn resolve(fragments: &[Fragment]) -> Option<char> {
        //TODO: put this in lazy static
        let mut fragments = fragments.to_owned();
        fragments.sort();
        FRAGMENT_CHAR.get(&fragments).map(|c| *c)
    }

    pub fn get_cells<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (usize, usize, char)> + 'a> {
        let mut cells = self.cells.iter().collect::<Vec<_>>();
        cells.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        Box::new(cells.into_iter().flat_map(|((x, y), frags)| {
            Self::resolve(frags).map(|ch| (*x, *y, ch))
        }))
    }

    pub fn dump(&self) -> String {
        let mut sb = StringBuffer::new();
        let cells = self.get_cells();
        cells.for_each(|(x, y, ch)| sb.add_char(x as i32, y as i32, ch));
        sb.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rect1() {
        let mut canvas = Canvas::new();
        canvas.draw_rect((0, 0), (2, 2), Border::thin());
        let mut cells = canvas
            .cells
            .iter()
            .map(|((x, y), frag)| (x, y, frag))
            .collect::<Vec<_>>();
        cells.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
        println!("cells: {:#?}", cells);
        assert_eq!(cells.len(), 8);
        let char_cells: Vec<(usize, usize, char)> =
            canvas.get_cells().collect();
        println!("char cells: {:#?}", char_cells);
        println!("dump: \n{}", canvas.dump());
        assert_eq!(
            "┌─┐\n\
             │ │\n\
             └─┘",
            canvas.dump()
        );
    }
    #[test]
    fn rect2() {
        let mut canvas = Canvas::new();
        canvas.draw_rect((0, 0), (4, 2), Border::thick());
        let mut cells = canvas
            .cells
            .iter()
            .map(|((x, y), frag)| (x, y, frag))
            .collect::<Vec<_>>();
        cells.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
        println!("cells: {:#?}", cells);
        assert_eq!(cells.len(), 12);
        let char_cells: Vec<(usize, usize, char)> =
            canvas.get_cells().collect();
        println!("char cells: {:#?}", char_cells);
        println!("dump: \n{}", canvas.dump());
        assert_eq!(
            "┏━━━┓\n\
             ┃   ┃\n\
             ┗━━━┛",
            canvas.dump()
        );
    }

    #[test]
    fn rect3() {
        let mut canvas = Canvas::new();
        canvas.draw_rect((0, 0), (6, 2), Border::rounded());
        let mut cells = canvas
            .cells
            .iter()
            .map(|((x, y), frag)| (x, y, frag))
            .collect::<Vec<_>>();
        cells.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
        println!("cells: {:#?}", cells);
        assert_eq!(cells.len(), 16);
        let char_cells: Vec<(usize, usize, char)> =
            canvas.get_cells().collect();
        println!("char cells: {:#?}", char_cells);
        println!("dump: \n{}", canvas.dump());
        assert_eq!(
            "╭─────╮\n\
             │     │\n\
             ╰─────╯",
            canvas.dump()
        );
    }

    #[test]
    fn crossing() {
        let mut canvas = Canvas::new();

        canvas.draw_rect((0, 0), (8, 4), Border::rounded());
        canvas.draw_horizontal_line((0, 2), (8, 2), true);
        canvas.draw_vertical_line((4, 0), (4, 4), false);
        let mut cells = canvas
            .cells
            .iter()
            .map(|((x, y), frag)| (x, y, frag))
            .collect::<Vec<_>>();
        cells.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
        let char_cells: Vec<(usize, usize, char)> =
            canvas.get_cells().collect();
        println!("dump: \n{}", canvas.dump());
        assert_eq!(
            "╭───┬───╮\n\
             │   │   │\n\
             ┝━━━┿━━━┥\n\
             │   │   │\n\
             ╰───┴───╯",
            canvas.dump()
        );
    }

    #[test]
    fn test_horizontal_line() {
        let mut canvas = Canvas::new();
        canvas.draw_horizontal_line((0, 0), (2, 0), false);
        let mut cells = canvas
            .cells
            .iter()
            .map(|((x, y), frag)| (x, y, frag))
            .collect::<Vec<_>>();
        cells.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
        println!("cells: {:#?}", cells);
        assert_eq!(cells.len(), 3);
    }

    #[test]
    fn test_vertical_line() {
        let mut canvas = Canvas::new();
        canvas.draw_vertical_line((0, 0), (0, 2), false);
        let mut cells = canvas
            .cells
            .iter()
            .map(|((x, y), frag)| (x, y, frag))
            .collect::<Vec<_>>();
        cells.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
        println!("cells: {:#?}", cells);
        assert_eq!(cells.len(), 3);
    }
}
