use crate::Shape;

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Circle { x, y, radius }
    }
}

impl<'a> Shape<'a> for Circle {
    fn points(&'a self) -> Box<dyn Iterator<Item = (f32, f32)> + 'a> {
        let mut x = self.radius;
        let mut y = 0.0;
        let mut err = 0.0;

        let inc = 0.25;

        let mut points = vec![];

        while (x >= y) {
            points.push((self.x + x, self.y + y));
            points.push((self.x + y, self.y + x));
            points.push((self.x - y, self.y + x));
            points.push((self.x - x, self.y + y));
            points.push((self.x - x, self.y - y));
            points.push((self.x - y, self.y - x));
            points.push((self.x + y, self.y - x));
            points.push((self.x + x, self.y - y));

            if (err <= 0.0) {
                y += inc;
                err += 2.0 * y + inc;
            }

            if (err > 0.0) {
                x -= inc;
                err -= 2.0 * x + inc;
            }
        }
        Box::new(points.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn draw_circle4() {
        let width = 10.0;
        let height = 10.0;
        let mut context = Context::new(width, height);

        context.draw(&Circle {
            x: 5.0,
            y: 5.0,
            radius: 4.0,
        });
        let result = context.to_string();
        println!("{}", result);

        let expected = [
            "                    ",
            "      ⣀⠤⠒⠒⠑⠒⠢⢄⡀     ",
            "    ⡠⠊        ⠈⠢⡀   ",
            "   ⡜            ⠘⡄  ",
            "  ⢸              ⢸  ",
            "  ⢱              ⢰⠁ ",
            "  ⠈⢆            ⢀⠎  ",
            "   ⠈⠢⡀         ⡠⠊   ",
            "     ⠈⠒⠤⣀⣀⢀⣀⡠⠔⠊     ",
            "          ⠁         ",
        ];
        assert_eq!(result, expected.join("\n"));
    }

    #[test]
    fn draw_dynamic() {
        let radius = 10.0;
        let mut context = Context::new(radius * 2.0, radius * 2.0);

        context.draw(&Circle {
            x: radius,
            y: radius,
            radius,
        });
        let result = context.to_string();
        println!("{}", result);

        let expected = [
            "             ⣀⡠⠤⠔⠒⠒⠒⠑⠒⠒⠒⠤⠤⣀⡀            ",
            "         ⢀⠤⠒⠉              ⠈⠑⠢⢄         ",
            "       ⡠⠊⠁                     ⠉⠢⡀      ",
            "     ⡠⠊                          ⠈⠢⡀    ",
            "   ⢀⠎                              ⠈⢆   ",
            "  ⢠⠃                                 ⢣  ",
            " ⢠⠃                                   ⢣ ",
            " ⡎                                    ⠈⡆",
            "⢰⠁                                     ⢱",
            "⢸                                      ⢸",
            "⢱                                      ⢰",
            "⢸                                      ⢸",
            " ⡇                                     ⡇",
            " ⠸⡀                                   ⡸ ",
            "  ⠱⡀                                 ⡰⠁ ",
            "   ⠑⡄                               ⡔⠁  ",
            "    ⠈⠢⡀                           ⡠⠊    ",
            "      ⠈⠢⡀                       ⡠⠊      ",
            "        ⠈⠑⠤⣀                ⢀⡠⠔⠉        ",
            "            ⠉⠒⠢⠤⢄⣀⣀⣀⢀⣀⣀⣀⠤⠤⠒⠊⠁           ",
        ];
        assert_eq!(result, expected.join("\n"));
    }
}
