extern crate piston_window;

use piston_window::*;


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

     fn up() {

     }
}







fn main() {

    const WIDTH : f64 = 640.0;
    const HEIGHT: f64 = 480.0;

    let mut window: PistonWindow = 
        WindowSettings::new("Snake", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true).build().unwrap();

    let size = 10.0;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut speed_x = 0.5;
    let mut speed_y = 0.5;

    while let Some(e) = window.next() {

        match e {
           Event::Input(ref inp) => match *inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        speed_x = 0.0;
                        speed_y = -0.5;
                    }
                    Button::Keyboard(Key::Down) => {
                        speed_x = 0.0;
                        speed_y = 0.5;
                    }
                    Button::Keyboard(Key::Left) => {
                        speed_x = -0.5;
                        speed_y = 0.0;
                    }
                    Button::Keyboard(Key::Right) => {
                        speed_x = 0.5;
                        speed_y = 0.0;
                    }
                    _ => {}
                }
             },
           _ => {}
           },
           _ => {}
        }

        x += speed_x;
        y += speed_y;

        if x + size >= WIDTH || x < 0.0 {
            speed_x *= -1.0;
        }
        if y + size > HEIGHT || y < 0.0 {
            speed_y *= -1.0;
        }

        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g); // black
            rectangle([1.0, 1.0, 1.0, 1.0], // white
                      [x, y, size, size],
                      c.transform, g);
        });
    }
}
