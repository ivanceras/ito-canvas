use crate::Shape;

/// Shape to draw a line from (x1, y1) to (x2, y2) with the given color
pub struct Line {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Line { x1, y1, x2, y2 }
    }
}

pub struct LineIterator {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    dir_x: f32,
    dir_y: f32,
    current: f32,
    end: f32,
}

impl Iterator for LineIterator {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let pos = (
                self.x + (self.current * self.dx) / self.end * self.dir_x,
                self.y + (self.current * self.dy) / self.end * self.dir_y,
            );
            //self.current += 1.0;
            self.current += 0.25;
            Some(pos)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = (f32, f32);
    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        let dx = self.x1.max(self.x2) - self.x1.min(self.x2);
        let dy = self.y1.max(self.y2) - self.y1.min(self.y2);
        let dir_x = if self.x1 <= self.x2 { 1.0 } else { -1.0 };
        let dir_y = if self.y1 <= self.y2 { 1.0 } else { -1.0 };
        let end = dx.max(dy);
        LineIterator {
            x: self.x1,
            y: self.y1,
            dx,
            dy,
            dir_x,
            dir_y,
            current: 0.0,
            end,
        }
    }
}

impl<'a> Shape<'a> for Line {
    fn points(&'a self) -> Box<dyn Iterator<Item = (f32, f32)> + 'a> {
        Box::new(self.into_iter())
    }
}
