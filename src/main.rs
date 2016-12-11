extern crate piston_window;
extern crate gfx_graphics;

use piston_window::*;
use gfx_graphics::GfxGraphics;

struct Snake {
    x: f64,
    y: f64,
    size: f64,
    speed_x: f64,
    speed_y: f64,
}

impl Snake {
     fn new(x: f64, y: f64) -> Snake {
        Snake {x: x, y: y, size: 10.0, speed_x: 0.5, speed_y: 0.0}
     }

     fn up(&mut self) {
        self.speed_x = 0.0;
        self.speed_y = -0.5;
     }

     fn down(&mut self) {
        self.speed_x = 0.0;
        self.speed_y = 0.5;
     }

     fn left(&mut self) {
        self.speed_x = -0.5;
        self.speed_y = 0.0;
     }

     fn right(&mut self) {
        self.speed_x = 0.5;
        self.speed_y = 0.0;
     }

     fn movement(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;
     }

     fn draw(&self, c: piston_window::Context, g: &mut G2d) {
        rectangle([1.0, 1.0, 1.0, 1.0],
                    [self.x, self.y, self.size, self.size],
                    c.transform, g);
     }
}

fn main() {

    const WIDTH : f64 = 640.0;
    const HEIGHT: f64 = 480.0;

    let mut window: PistonWindow = 
        WindowSettings::new("Snake", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true).build().unwrap();

    let mut snake = Snake::new(0.0, 0.0);

    while let Some(e) = window.next() {

        match e {
           Event::Input(ref inp) => match *inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        snake.up()
                    }
                    Button::Keyboard(Key::Down) => {
                        snake.down()
                    }
                    Button::Keyboard(Key::Left) => {
                        snake.left()
                    }
                    Button::Keyboard(Key::Right) => {
                        snake.right()
                    }
                    _ => {}
                }
             },
           _ => {}
           },
           _ => {}
        }

        snake.movement();

        /*
        if x + size >= WIDTH || x < 0.0 {
            speed_x *= -1.0;
        }
        if y + size > HEIGHT || y < 0.0 {
            speed_y *= -1.0;
        }
        */

        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g); // black
            snake.draw(c, g)
        });
    }
}
