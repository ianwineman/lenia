#![crate_name = "lenia"]

mod gol;
use gol::*;

extern crate glutin_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::WindowSettings;

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::RenderEvent;

use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Lenia", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    let RED = [1.0, 0.0, 0.0, 1.0];
    let GREEN = [0.0, 1.0, 0.0, 1.0];
    let BLUE = [0.0, 0.0, 1.0, 1.0];
    let WHITE = [1.0; 4];

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |_c, g| {
                graphics::clear(BLUE, g);
            });
        }
    }
  
    //let world1: World = World::new_empty_world();
    //println!("{}", world1);

    /*
    let mut world2: World = World::new_world(
        [
            [0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 1, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0]
        ]
    );

    println!("{}", world2);
    for i in 1..5 {
        world2.step_forward();
        println!("{}", world2);
    }
    */
}