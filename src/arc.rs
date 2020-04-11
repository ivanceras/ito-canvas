use crate::Shape;

pub struct Arc {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub radius: f32,
    pub sweep_flag: bool,
}

impl Arc {
    fn center(&self) -> (f32, f32) {
        let q = ((self.x2 - self.x1).powf(2.0) + (self.y2 - self.y1).powf(2.0)).sqrt();
        dbg!(q);
        let y3 = (self.y1 + self.y2) / 2.0;
        let x3 = (self.x1 + self.x2) / 2.0;

        let rr_q22 = (self.radius.powf(2.0) - (q / 2.0).powf(2.0)).sqrt();

        let base_x = rr_q22 * (self.y1 - self.y2) / q;
        let base_y = rr_q22 * (self.x2 - self.x1) / q;

        if self.sweep_flag {
            let cx = x3 + base_x;
            let cy = y3 + base_y;
            (cx, cy)
        } else {
            let cx = x3 - base_x;
            let cy = y3 - base_y;
            (cx, cy)
        }
    }

    fn octant(&self) -> (u8, u8) {
        let (cx, cy) = self.center();
        let o1 = Self::line_octant(cx, cy, self.x1, self.y1);
        let o2 = Self::line_octant(cx, cy, self.x2, self.y2);
        (o1 + 1, o2)
    }

    // calculate the octant of a line
    fn line_octant(x1: f32, y1: f32, x2: f32, y2: f32) -> u8 {
        println!("{},{} -> {},{}", x1, y1, x2, y2);
        let mut dx = x2 - x1;
        let mut dy = -(y2 * 2.0 - y1 * 2.0);

        let mut octant = 0;

        if dy < 0.0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0.0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2
        }

        if dx < dy {
            octant += 1
        }
        dbg!(octant);
        octant
    }
}

// Note: This is not a lazy iterator
// since we are computing multiple points at each iteration
pub struct ArcIterator {
    points: Vec<(f32, f32)>,
    current: usize,
}

impl<'a> IntoIterator for &'a Arc {
    type Item = (f32, f32);
    type IntoIter = ArcIterator;

    fn into_iter(self) -> Self::IntoIter {
        let mut x = self.radius;
        let mut y = 0.0;
        let mut err = 0.0;

        let inc = 0.25;

        let mut points = vec![];

        let (cx, cy) = self.center();
        let (o1, o2) = self.octant();
        dbg!(o1);
        dbg!(o2);

        while (x >= y) {
            if (o1..=o2).contains(&7) {
                points.push((cx + x, cy + y));
            }
            if (o1..=o2).contains(&6) {
                points.push((cx + y, cy + x));
            }
            if (o1..=o2).contains(&5) {
                points.push((cx - y, cy + x));
            }
            if (o1..=o2).contains(&4) {
                points.push((cx - x, cy + y));
            }
            if (o1..=o2).contains(&3) {
                points.push((cx - x, cy - y));
            }
            if (o1..=o2).contains(&2) {
                points.push((cx - y, cy - x));
            }
            if (o1..=o2).contains(&1) {
                points.push((cx + y, cy - x));
            }
            if (o1..=o2).contains(&0) {
                points.push((cx + x, cy - y));
            }

            if (err <= 0.0) {
                y += inc;
                err += 2.0 * y + inc;
            }

            if (err > 0.0) {
                x -= inc;
                err -= 2.0 * x + inc;
            }
        }
        ArcIterator { points, current: 0 }
    }
}

impl Iterator for ArcIterator {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.points.get(self.current);
        self.current += 1;
        point.cloned()
    }
}

impl<'a> Shape<'a> for Arc {
    fn points(&'a self) -> Box<dyn Iterator<Item = (f32, f32)> + 'a> {
        Box::new(self.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn arc_center() {
        let arc = Arc {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 10.0,
            radius: 10.0,
            sweep_flag: false,
        };

        let center = arc.center();
        assert_eq!(center, (10.0, 0.0));
    }

    #[test]
    fn arc_center2() {
        let arc = Arc {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 10.0,
            radius: 10.0,
            sweep_flag: true,
        };

        let center = arc.center();
        assert_eq!(center, (0.0, 10.0));
    }

    #[test]
    fn draw_arc() {
        let width = 11.0;
        let height = 11.0;
        let mut context = Context::new(width, height);
        let arc = Arc {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 10.0,
            radius: 10.0,
            sweep_flag: false,
        };

        context.draw(&arc);

        let center = arc.center();
        dbg!(center);

        let result = context.to_string();
        println!("{}", result);
        let expected = [
            "⢱                     ",
            "⢸                     ",
            " ⡇                    ",
            " ⠸⡀                   ",
            "  ⠱⡀                  ",
            "   ⠑⡄                 ",
            "    ⠈⠢⡀               ",
            "      ⠈⠢⡀             ",
            "        ⠈⠑⠤⣀          ",
            "            ⠉⠒⠢⠤⢄⣀⣀⣀  ",
            "                    ⠁ ",
        ];
        let expected = expected.join("\n");
        assert_eq!(result, expected);
    }

    #[test]
    fn draw_arc2() {
        let width = 22.0;
        let height = 22.0;
        let mut context = Context::new(width, height);
        let arc = Arc {
            x1: 10.0,
            y1: 0.0,
            x2: 10.0,
            y2: 20.0,
            radius: 10.0,
            sweep_flag: false,
        };

        context.draw(&arc);

        let center = arc.center();
        dbg!(center);

        let result = context.to_string();
        println!("{}", result);
        let expected = [
            "             ⣀⡠⠤⠔⠒⠒⠒⠁                       ",
            "         ⢀⠤⠒⠉                               ",
            "       ⡠⠊⠁                                  ",
            "     ⡠⠊                                     ",
            "   ⢀⠎                                       ",
            "  ⢠⠃                                        ",
            " ⢠⠃                                         ",
            " ⡎                                          ",
            "⢰⠁                                          ",
            "⢸                                           ",
            "⢱                                           ",
            "⢸                                           ",
            " ⡇                                          ",
            " ⠸⡀                                         ",
            "  ⠱⡀                                        ",
            "   ⠑⡄                                       ",
            "    ⠈⠢⡀                                     ",
            "      ⠈⠢⡀                                   ",
            "        ⠈⠑⠤⣀                                ",
            "            ⠉⠒⠢⠤⢄⣀⣀⣀                        ",
            "                    ⠁                       ",
            "                                            ",
        ];
        let expected = expected.join("\n");
        assert_eq!(result, expected);
    }
}
