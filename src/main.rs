extern crate rand;
extern crate piston_window;

use piston_window::*;
use rand::Rng;

struct Fruit {
    x: f64,
    y: f64,
    size: f64,
    color: [f32; 4]
}

impl Fruit {
     fn new() -> Fruit {
        let mut r = rand::thread_rng();
        let x = r.gen_range(0, 640);
        let y = r.gen_range(0, 480);

        Fruit {x: x as f64, y: y as f64, size: 10.0, color: [1.0, 0.0, 0.0, 1.0]}
     }
     
     fn draw(&self, c: piston_window::Context, g: &mut G2d) {
        rectangle(self.color,
                  [self.x, self.y, self.size, self.size],
                  c.transform, g);
     }

     fn pos(&self) -> (f64, f64) {
         (self.x, self.y)
     }
}

struct Snake {
    x: f64,
    y: f64,
    size: f64,
    speed_x: f64,
    speed_y: f64,
    color: [f32; 4]
}

impl Snake {
     fn new(x: f64, y: f64) -> Snake {
        Snake {x: x, y: y, size: 10.0, speed_x: 0.5, speed_y: 0.0, color: [1.0; 4]}
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

     fn pos(&self) -> (f64, f64) {
         (self.x, self.y)
     }

     fn draw(&self, c: piston_window::Context, g: &mut G2d) {
        rectangle(self.color,
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
    let mut fruit = Fruit::new();

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

        let (sx, sy) = snake.pos();
        let (fx, fy) = fruit.pos();
        if sx - fx < 10.0 && sy - fy < 10.0 {
            fruit = Fruit::new();
        }

        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g); // black
            snake.draw(c, g);
            fruit.draw(c, g);
        });
    }
}
