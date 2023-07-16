#![crate_name = "lenia"]
#![allow(dead_code)]
#![allow(unused_imports)]

mod gol;
use gol::*;

use uuid::Uuid;
use std::{thread, time};

use image::*;
use image::Rgba;

use rand::Rng;

use piston::{
    event_loop::{EventLoop, EventSettings, Events},
    input::{Button, ButtonState, Key, MouseButton},
    Motion::MouseCursor,
    WindowSettings,
    ButtonEvent, Input, Motion, MouseCursorEvent, RenderEvent,
    IdleEvent, MouseScrollEvent
};

use graphics::{
    character::CharacterCache,
    Transformed
};

use glutin_window::GlutinWindow;

use opengl_graphics::{
    GlGraphics, OpenGL, Filter, GlyphCache, TextureSettings, Texture
};


type Color = [f32; 4];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const WHITE: Color = [1.0; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

const WINDOW_SIZE: [i32; 2] = [640, 480];
const WORLD_SIZE: [i32; 2] = [80, 60];

//const SCALE_FACTOR: f64 = 32.0;
//const GRID_SIZE: i32 = WINDOW_SIZE / SCALE_FACTOR as i32;
const INPUT_FILE: &str = "patterns/2673baaa-0393-4540-90e3-05699881a02c.lenia";


fn main() {
    //render vals
    let mut mouse_pos = [0.0, 0.0];
    let mut scale_factor: f32 = 8.0;

    //piston init
    let opengl = OpenGL::V3_2;
    //let settings = WindowSettings::new(  "Lenia", [WINDOW_SIZE as f64; 2]  ).exit_on_esc(true);
    let settings = WindowSettings::new(  "Lenia", [WINDOW_SIZE[1] as f64, WINDOW_SIZE[0] as f64]  ).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    //GOL init
    let mut saves: String = String::from(INPUT_FILE);
    //println!("World starting point: {}", saves);
    //let mut world: World = World::new_from_rle(INPUT_FILE);
    let mut world: World = World::new_random();

    //init texture
    /*
    let mut canvas = ImageBuffer::new(WINDOW_SIZE as u32, WINDOW_SIZE as u32);
    let val:u32 = canvas.width();

    let mut texture = Texture::from_image(
            &canvas,
            &TextureSettings::new()
        );
    */

    //event loop vars
    let mut loop_: bool = false;
    let mut event_settings = EventSettings::new();
    event_settings.lazy = false; // enable idle events
    //event_settings.ups = 2;

    //event loop
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {
        // idle events
        if let Some(_i) = e.idle_args() {
            if loop_ {
                world.step_forward();

                thread::sleep(time::Duration::from_millis(5)); // see issue 23

            }
        }

        // button press responses
        if let Some(k) = e.button_args() {
            //println!("{:?}", k );
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::H) => println!("\nLenia User Manual:\nPress 'b' to create a blank world\nPress 'n' to create a random world\nPress 'o' to create a blank world with a random creature\nPress 's' to save current pattern.\nPress 'r' to reset to last saved pattern (input pattern if no saves made)\nPress 'Right_Arrow' to step forward\nPress 'Space' to toggle continuous stepping\nPress 'esc' to quit"),
                    Button::Keyboard(Key::N) => {
                        world = World::new_random();
                    }
                    Button::Keyboard(Key::B) => {
                        world = World::new_empty();
                    }
                    Button::Keyboard(Key::O) => {
                        world = World::new_creature(6);
                    }
                    Button::Keyboard(Key::Space) => {
                        match loop_ {
                            true => loop_ = false,
                            false => loop_ = true,
                        };
                    },
                    Button::Mouse(MouseButton::Left) => {

                        //let grid_pos_row = (mouse_coords[1]).floor() as usize;
                        //let grid_pos_col = (mouse_coords[0]).floor() as usize;
                        //println!("row: {0}  col: {1}", grid_pos_row, grid_pos_col);
                        //world.update_cell(grid_pos_row, grid_pos_col);

                        //let grid_pos_row = (mouse_coords[1] / SCALE_FACTOR).floor() as usize;
                        //let grid_pos_col = (mouse_coords[0] / SCALE_FACTOR).floor() as usize;
                        //world.update_cell(grid_pos_row, grid_pos_col);

                    }
                    Button::Keyboard(Key::Right) => world.step_forward(),
                    Button::Keyboard(Key::R) => {
                        world = World::new_from_rle(&saves);
                    }
                    Button::Keyboard(Key::S) => {
                        let uuid: Uuid = Uuid::new_v4();
                        let path: String = format!("patterns/{}.lenia", uuid.hyphenated().to_string());
                        saves = path.clone();
                        world.save(&path);
                        println!("World save point: {}", saves);
                    }
                    _ => (),
                }
            }
            //add functionality for holding down mouse or button on hold
        }

        // mouse events
        #[allow(unused)]
        if let Some(m) = e.mouse_cursor_args() {
            //println!("{:?}", m );
            mouse_pos = m;
        }

        if let Some(s) = e.mouse_scroll_args() {
            //println!("{:?}", s );
            match s[1] {
                -1.0 => {
                    scale_factor -= 0.1_f32;
                    if scale_factor < 1.0 {
                        scale_factor = 1.0;
                    }
                 },

                 1.0 => {scale_factor += 0.1_f32;},

                 _ => ()
            }
            //println!("before: {:?}", scale_factor );
            scale_factor *= 100.0_f32;
            scale_factor = scale_factor.round();
            scale_factor = scale_factor / 100.0_f32;
            //println!("after: {:?}\n", scale_factor );
        }

        //render loop
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |_c, g| {
                graphics::clear(BLUE, g);

                let view = [ (WINDOW_SIZE[0] as f32 / scale_factor).floor() , (WINDOW_SIZE[1] as f32 / scale_factor).floor() ];
                let mut x0 = (mouse_pos[0] / scale_factor as f64) - (view[0] / 2.0) as f64;
                let mut x1 = (mouse_pos[0] / scale_factor as f64) + (view[0] / 2.0) as f64;
                let mut y0 = (mouse_pos[1] / scale_factor as f64) - (view[1] / 2.0) as f64;
                let mut y1 = (mouse_pos[1] / scale_factor as f64) + (view[1] / 2.0) as f64;

                if x0 < 0.0 {
                    x0 = 0.0;
                    x1 = view[0] as f64;
                } else if x1 > WORLD_SIZE[0] as f64 {
                    x0 = (WORLD_SIZE[0] as f32 - view[0]) as f64;
                    x1 = WORLD_SIZE[0] as f64;
                };

                if y0 < 0.0 {
                    y0 = 0.0;
                    y1 = view[1] as f64;
                } else if y1 > WORLD_SIZE[1].into() {
                    y0 = (WORLD_SIZE[1] as f32 - view[1]) as f64;
                    y1 = WORLD_SIZE[1] as f64;
                };

                //todo collect data from world



            });
        }
    }
}
