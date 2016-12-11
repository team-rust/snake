extern crate piston_window;

use piston_window::*;

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
