#![crate_name = "lenia"]
#![allow(dead_code)]
#![allow(unused_imports)]

mod gol;
use gol::*;

use image::*;
use image::Rgba;

use rand::Rng;

extern crate piston;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, ButtonState, Key, MouseButton};
use piston::Motion::MouseCursor;
use piston::WindowSettings;
use piston::{ButtonEvent, Input, Motion, MouseCursorEvent, RenderEvent};
use piston::IdleEvent;

use graphics::character::CharacterCache;
use graphics::Transformed;

extern crate glutin_window;
use glutin_window::GlutinWindow;

extern crate graphics;

extern crate opengl_graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::{Filter, GlyphCache, TextureSettings, Texture};
use uuid::Uuid;

type Color = [f32; 4];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const WHITE: Color = [1.0; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

const USER_MANUAL: &str = "Lenia User Manual:
Press 'b' to create a blank world
Press 'c' to toggle the speed controls for continuous stepping
Press 'esc' to quit
Press 'h' to print User Manual
Press 'n' to create a random world
Press 'o' to create a blank world with a random creature
Press 'r' to reset to last saved pattern (input pattern if no saves made)
Press 'Right_Arrow' to step forward
Press 's' to save current pattern.
Press 'Space' to toggle continuous stepping
Press '+' to increase the speed of continuous stepping
Press '-' to decrease the speed of continuous stepping";

const WINDOW_SIZE: i32 = 512;

//const SCALE_FACTOR: f64 = 32.0;
//const GRID_SIZE: i32 = WINDOW_SIZE / SCALE_FACTOR as i32;
const INPUT_FILE: &str = "patterns/2673baaa-0393-4540-90e3-05699881a02c.lenia";


fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Lenia", [WINDOW_SIZE as f64; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    //let mut mouse_coords = [0.0; 2];

    let mut saves: String = String::from(INPUT_FILE);
    println!("World starting point: {}", saves);
    let mut world: World = World::new_from_rle(INPUT_FILE);

    //init texture
    let mut canvas = ImageBuffer::new(WINDOW_SIZE as u32, WINDOW_SIZE as u32);
    let val:u32 = canvas.width();

    let mut texture = Texture::from_image(
            &canvas,
            &TextureSettings::new()
        );

    let mut loop_: bool = false;
    let mut use_counter = true;
    let mut counter: i32 = 0;
    let mut counter_max: i32 = 10;

    let mut event_settings = EventSettings::new();
    event_settings.lazy = false; // enable idle events
    //event_settings.ups = 2;

    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {
        // idle events
        if let Some(_i) = e.idle_args() {
            if loop_ {
                if use_counter {
                    if counter < counter_max {
                        counter += 1;
                    }
                    else {
                        counter = 0;
                        world.step_forward();
                    }
                }
                else {
                    world.step_forward(); // step as fast as possible
                }
            }
        }

        // button press responses
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::H) => println!("{}", USER_MANUAL),
                    Button::Keyboard(Key::C) => {
                        match use_counter {
                            true => use_counter = false,
                            false => {
                                use_counter = true;
                                counter = 0;
                            },
                        }
                    }
                    Button::Keyboard(Key::Equals) => {
                        if counter > 0 {
                            counter_max -= 1;
                        }
                    },
                    Button::Keyboard(Key::Minus) => {
                        counter_max += 1;
                    },
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
            //mouse_coords = m;
        }

        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |_c, g| {
                graphics::clear(BLUE, g);
                for i in 0..val {
                    for j in 0..val {


                        match world.map[i as usize][j as usize] {
                            1 => {canvas.put_pixel(i, j, Rgba([255, 255, 255, 255])  ); },
                            0 => {canvas.put_pixel(i, j, Rgba([0, 0, 0, 255])  ); },
                            _ => ()
                        }
                    };
                };

                texture.update(&canvas);

                graphics::image(&texture, _c.transform, g);

                /*for i in 0..WORLD_SIZE {
                    for j in 0..WORLD_SIZE {
                        let tile_pos: [f64; 4] = [
                            SCALE_FACTOR * i as f64,
                            SCALE_FACTOR * j as f64,
                            SCALE_FACTOR * (i + 1) as f64,
                            SCALE_FACTOR * (j + 1) as f64,
                        ];

                        if world.map[j as usize][i as usize] == 1 {
                            graphics::Rectangle::new(WHITE).draw(
                                tile_pos,
                                &_c.draw_state,
                                _c.transform,
                                g,
                            );
                        } else {
                            graphics::Rectangle::new(BLACK).draw(
                                tile_pos,
                                &_c.draw_state,
                                _c.transform,
                                g,
                            )
                        }
                    }
                }*/
            });
        }
    }
}
