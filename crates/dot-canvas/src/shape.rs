pub trait Shape<'a> {
    /// Returns an iterator over all points of the shape
    fn points(&'a self) -> Box<dyn Iterator<Item = (f32, f32)> + 'a>;
}

pub struct Dot {
    pub x: f32,
    pub y: f32,
}

impl<'a> Shape<'a> for Dot {
    fn points(&'a self) -> Box<dyn Iterator<Item = (f32, f32)> + 'a> {
        Box::new(std::iter::once((self.x, self.y)))
    }
}
