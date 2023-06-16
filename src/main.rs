#![crate_name = "lenia"]

mod gol;
use gol::*;

extern crate piston;
use piston::WindowSettings;
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent};

use graphics::character::CharacterCache;
use graphics::Transformed;

extern crate glutin_window;
use glutin_window::GlutinWindow;

extern crate graphics;

extern crate opengl_graphics;
use opengl_graphics::{GlGraphics, OpenGL};
//use opengl_graphics::{Filter, GlyphCache, TextureSettings};

type Color = [f32; 4];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const WHITE: Color = [1.0; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

const WINDOW_SIZE: i32 = 512;
const SCALE_FACTOR: f64 = 32.0;
const GRID_SIZE: i32 = WINDOW_SIZE / SCALE_FACTOR as i32; //16 w current vals

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Lenia", [WINDOW_SIZE as f64; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    //define world:
    let mut world: World = World::new_world(
        [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    );




    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(k) = e.button_args(){
            
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Space) => world.step_forward(),
                    _ => ()
                }
            }
            
        }
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |_c, g| {
                graphics::clear(BLUE, g);

                for i in 0..WORLD_SIZE {
                    for j in 0..WORLD_SIZE {
                        let tile_pos: [f64; 4] = [
                            SCALE_FACTOR * i as f64,
                            SCALE_FACTOR * j as f64,
                            SCALE_FACTOR * (i+1) as f64,
                            SCALE_FACTOR * (j+1) as f64
                        ];

                        if world.map[j as usize][i as usize] == 1 {
                            graphics::Rectangle::new(WHITE).draw(
                                tile_pos,
                                &_c.draw_state,
                                _c.transform,
                                g);
                        } else {
                            graphics::Rectangle::new(BLACK).draw(
                                tile_pos,
                                &_c.draw_state,
                                _c.transform,
                                g)
                        }
                    }
                }

            });
        }
    }
  
    //let world1: World = World::new_empty_world();
    //println!("{}", world1);

    /*
    let mut world: World = World::new_world(
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