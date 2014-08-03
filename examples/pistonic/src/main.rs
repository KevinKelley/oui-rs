#![feature(globs)]
#![feature(struct_variant)]
#![feature(unsafe_destructor)]
//#![allow(unused_variable)]

extern crate piston;
extern crate glfw_game_window;

extern crate nanovg;
extern crate blendish;
extern crate oui;

use std::gc::{Gc,GC};
use std::cell::Cell;

pub use Window = glfw_game_window::GameWindowGLFW;

use piston::{
    Game, GameWindow, GameIteratorSettings, GameWindowSettings,
    UpdateArgs, RenderArgs,
    MousePressArgs, MouseReleaseArgs,
    MouseMoveArgs,
};

use nanovg::{Ctx, ANTIALIAS,STENCIL_STROKES};
use blendish::ThemedContext;
use oui::Context;

use resources::Resources;
use ui::Widget;

mod ui;
mod resources;

///////////////////////////////////////////////////////////////////////
// AppData (some simulated state for UI to modify

#[deriving(Show)]
pub struct AppData {
    // some persistent variables for demonstration
    pub enum1:     Gc<Cell<i32>>,
    pub progress1: Gc<Cell<f32>>,
    pub progress2: Gc<Cell<f32>>,
    pub option1:  Gc<Cell<bool>>,
    pub option2:  Gc<Cell<bool>>,
    pub option3:  Gc<Cell<bool>>,
}
pub fn init_app_data() -> AppData {
    // fake load-from-storage
    AppData {
        enum1:     box (GC) Cell::new(0),
        progress1: box (GC) Cell::new(0.25),
        progress2: box (GC) Cell::new(0.75),
        option1:   box (GC) Cell::new(true),
        option2:   box (GC) Cell::new(false),
        option3:   box (GC) Cell::new(false),
    }
}
#[unsafe_destructor]
impl Drop for AppData {
    fn drop(&mut self) {
        // fake save-to-storage
        println!("drop appdata {}", self);
    }
}
///////////////////////////////////////////////////////////////////////

pub struct App<'a> {
    mouse: (i32,i32),           // current mouse pos
    button: bool,               // is mousebutton pressed
    elapsed_time: f64,          // seconds since app start
    data: AppData,
    themed: ThemedContext<'a>,  // wrap nvg ctx w/ themed-draw fns
    ui: Context<Widget>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let themed = {
            let nvg = Ctx::create_gl3(ANTIALIAS|STENCIL_STROKES);
            let resources = Resources::load(&nvg, "../../res");
            let font = resources.fontNormal;
            let icons = resources.iconsheet;
            // move nvg & resources into the ThemedContext
            ThemedContext::wrap(nvg, icons, font)
        };
        App {
            mouse: (0,0),
            button: false,
            elapsed_time: 0.0,         // time since app start
            data: init_app_data(),
            themed: themed,
            ui: ui::create(),
        }
    }
    fn nvg(&mut self) -> &mut Ctx { self.themed.nvg() }
}

impl<'a, W: GameWindow> Game<W> for App<'a>
{
    fn load(&mut self, _window: &mut W) {
        ui::init(self);
    }

    fn update(&mut self, _window: &mut W, args: &UpdateArgs) {
        self.elapsed_time += args.dt;
        ui::update(&mut self.ui, self.mouse, self.button, self.elapsed_time as f32);
    }

    fn render(&mut self, _window: &mut W, args: &RenderArgs) {
        let (w,  h) = (args.width as f32, args.height as f32);
        let pxRatio = 1.0;

        self.nvg().begin_frame(w as i32, h as i32, pxRatio);

        ui::draw(&mut self.ui, &mut self.themed, w,h);

        self.nvg().end_frame();
    }

    // capture events, for forwarding to ui in its update cycle
    fn mouse_press(&mut self, _window: &mut W, _args: &MousePressArgs) {
        self.button = true;
    }
    fn mouse_release(&mut self, _window: &mut W, _args: &MouseReleaseArgs) {
        self.button = false;
    }
    fn mouse_move(&mut self, _window: &mut W, args: &MouseMoveArgs) {
        self.mouse = (args.x as i32, args.y as i32);
    }
}


fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "OUI demo".to_string(),
            size: [800,600],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let mut app = App::new();

    let game_iter_settings = GameIteratorSettings {
            // sim and ui can run at different rates
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    app.run(&mut window, &game_iter_settings);
}
