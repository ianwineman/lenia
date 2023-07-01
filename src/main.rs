#![crate_name = "lenia"]
#![allow(dead_code)]
#![allow(unused_imports)]

mod gol;
use gol::*;

use image::*;
use image::Rgba;

use rand::Rng;

extern crate piston;
use piston::WindowSettings;
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{Button, ButtonState, Key, MouseButton};
use piston::{ButtonEvent, RenderEvent, Input, Motion, MouseCursorEvent};
use piston::Motion::MouseCursor;

use graphics::character::CharacterCache;
use graphics::Transformed;

extern crate glutin_window;
use glutin_window::GlutinWindow;

extern crate graphics;

extern crate opengl_graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::{Filter, GlyphCache, TextureSettings, Texture};

type Color = [f32; 4];
const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const WHITE: Color = [1.0; 4];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

const WINDOW_SIZE: i32 = 512;
//const SCALE_FACTOR: f64 = 32.0;
//const GRID_SIZE: i32 = WINDOW_SIZE / SCALE_FACTOR as i32; 

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Lenia", [WINDOW_SIZE as f64; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    let mut mouse_coords = [0.0; 2];
    let mut world: World = World::new_random();
    //world.save("patterns/test.txt");

    //init texture
    let mut canvas = ImageBuffer::new(WINDOW_SIZE as u32, WINDOW_SIZE as u32);
    let val:u32 = canvas.width();

    let mut rng = rand::thread_rng(); //remove if necessary 

    let mut texture = Texture::from_image(
            &canvas,
            &TextureSettings::new()
        );

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // button press responses
        if let Some(k) = e.button_args(){
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Space) => world.step_forward(),
                    Button::Mouse(MouseButton::Left) => {
                        //let grid_pos_row = (mouse_coords[1]).floor() as usize;
                        //let grid_pos_col = (mouse_coords[0]).floor() as usize;
                        //println!("row: {0}  col: {1}", grid_pos_row, grid_pos_col);
                        //world.update_cell(grid_pos_row, grid_pos_col);
                    }
                    _ => ()
                }
            }
            //add functionality for holding down mouse or button on hold
        }
        // mouse events
        if let Some(m) = e.mouse_cursor_args() {
            mouse_coords = m;
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
            });
        }
    }
}