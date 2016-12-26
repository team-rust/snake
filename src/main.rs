extern crate rand;
extern crate piston_window;

use piston_window::*;
use rand::Rng;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;
const TILE_SIZE: f64 = 10.0;

struct Game {
    window: PistonWindow,
    snake: Snake,
    fruit: Fruit,
}

impl Game {
    fn new(window: PistonWindow) -> Game {
        let snake = Snake::new(0.0, 0.0);
        let fruit = Fruit::new();
        Game {
            window: window,
            snake: snake,
            fruit: fruit,
        }
    }

    fn handle_event(&mut self, e: &Event) {
        match *e {
            Event::Input(ref inp) => {
                match *inp {
                    Input::Press(but) => {
                        match but {
                            Button::Keyboard(Key::Up) => self.snake.dir(Direction::Up),
                            Button::Keyboard(Key::Down) => self.snake.dir(Direction::Down),
                            Button::Keyboard(Key::Left) => self.snake.dir(Direction::Left),
                            Button::Keyboard(Key::Right) => self.snake.dir(Direction::Right),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn run(&mut self) {
        while let Some(e) = self.window.next() {
            self.handle_event(&e);
            self.snake.mv();

            let (sx, sy) = self.snake.pos();
            let (fx, fy) = self.fruit.pos();
            if sx == fx && sy == fy {
                self.fruit = Fruit::new();
            }

            let ref mut snake = self.snake;
            let ref mut fruit = self.fruit;
            self.window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g); // black
                snake.draw(c, g);
                fruit.draw(c, g);
            });
        }
    }
}

struct Fruit {
    x: f64,
    y: f64,
    size: f64,
    color: [f32; 4],
}

impl Fruit {
    fn new() -> Fruit {
        let mut r = rand::thread_rng();
        let x = r.gen_range(0, (WIDTH / TILE_SIZE) as i64) * TILE_SIZE as i64;
        let y = r.gen_range(0, (HEIGHT / TILE_SIZE) as i64) * TILE_SIZE as i64;

        Fruit {
            x: x as f64,
            y: y as f64,
            size: 10.0,
            color: [1.0, 0.0, 0.0, 1.0],
        }
    }

    fn draw(&self, c: piston_window::Context, g: &mut G2d) {
        rectangle(self.color,
                  rectangle::square(self.x, self.y, self.size),
                  c.transform,
                  g);
    }

    fn pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Straight,
}
struct Snake {
    x: f64,
    y: f64,
    size: f64,
    speed_x: f64,
    speed_y: f64,
    dir: Direction,
    color: [f32; 4],
}

impl Snake {
    fn new(x: f64, y: f64) -> Snake {
        Snake {
            x: x,
            y: y,
            size: 10.0,
            speed_x: 0.5,
            speed_y: 0.0,
            dir: Direction::Straight,
            color: [1.0; 4],
        }
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

    fn dir(&mut self, dir: Direction) {
        self.dir = dir
    }

    fn mv(&mut self) {
        if self.x % TILE_SIZE == 0.0 {
            match self.dir {
                Direction::Up => self.up(),
                Direction::Down => self.down(),
                _ => {}
            }
        }
        if self.y % TILE_SIZE == 0.0 {
            match self.dir {
                Direction::Left => self.left(),
                Direction::Right => self.right(),
                _ => {}
            }
        }
        self.x += self.speed_x;
        self.y += self.speed_y;
    }

    fn pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn draw(&mut self, c: piston_window::Context, g: &mut G2d) {
        rectangle(self.color,
                  rectangle::square(self.x, self.y, self.size),
                  c.transform,
                  g);
    }
}

fn main() {


    let window: PistonWindow = WindowSettings::new("Snake", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(window);
    game.run();
}
