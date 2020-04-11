use txtcan::*;

fn main(){
        let mut context = Context::new(100.0, 40.0);

        for i in 0..20{
            context.draw(&Circle {
                x: 20.0,
                y: 20.0,
                radius: i as f32 + 1.0,
            });
        }

        context.draw(&Circle {
            x: 40.0,
            y: 10.0,
            radius: 5.0,
        });

        context.draw(&Line {
            x1: 0.0,
            y1: 0.5,
            x2: 40.0,
            y2: 0.5,
        });

        context.draw(&Line {
            x1: 0.0,
            y1: 0.0,
            x2: 100.0,
            y2: 40.0,
        });
        context.draw(&Line {
            x1: 0.0,
            y1: 0.0,
            x2: 40.0,
            y2: 40.0,
        });

        context.draw(&Line {
            x1: 0.5,
            y1: 0.0,
            x2: 0.5,
            y2: 40.0,
        });
        let result = context.to_string();
        println!("{}", result);
}