#![feature(globs)]
#![feature(struct_variant)]
#![allow(unused_imports)]
#![allow(unused_variable)]
#![allow(dead_code)]

extern crate graphics;
extern crate piston;
extern crate glfw_game_window;

extern crate nanovg;
extern crate blendish;
extern crate oui;

use std::rc::Rc;
use std::cell::{RefCell,Cell};

pub use Window = glfw_game_window::GameWindowGLFW;

use piston::{
    Game, GameWindow, GameIteratorSettings, GameWindowSettings,
    UpdateArgs, RenderArgs,
    KeyPressArgs, KeyReleaseArgs,
    MousePressArgs, MouseReleaseArgs,
    MouseScrollArgs, MouseMoveArgs, MouseRelativeMoveArgs,
};
use nanovg::{Ctx, ANTIALIAS,STENCIL_STROKES, Font,Image };
use blendish::*;
use blendish::lowlevel_draw::LowLevelDraw;
use blendish::themed_draw::ThemedDraw;
use resources::Resources;

mod ui;
mod resources;

///////////////////////////////////////////////////////////////////////

pub struct App<'a> {
    mouse: (i32,i32),           // current mouse pos
    elapsed_time: f64,          // seconds since app start
    //oui: Context<Widget<'a>>,
    //appdata: ui::AppData<'a>,
    themed: ThemedContext<'a>   // wrap nvg ctx w/ themed-draw fns
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let nvg = Ctx::create_gl3(ANTIALIAS|STENCIL_STROKES);
        let resources = Resources::load(&nvg, "../../res");
        let font = resources.fontNormal;
        let icons = resources.iconsheet;    // move resources into the ThemedContext
        App {
            mouse: (0,0),
            elapsed_time: 0.0,         // time since app start
            //oui: Context::create_context(),
            //appdata: data,
            themed: ThemedContext::wrap(nvg, icons, font)
        }
    }
    fn nvg(&mut self) -> &mut Ctx { self.themed.nvg() }
    fn theme(&self) -> &Theme { self.themed.theme() }
}

impl<'a, W: GameWindow> Game<W> for App<'a>
{
    fn load(&mut self, _window: &mut W) {}

    fn update(&mut self, window: &mut W, args: &UpdateArgs) {
        self.elapsed_time += args.dt;
        //let (mx, my) = window.get_cursor_pos();
        //self.oui.set_cursor(mx, my);
    }

    #[allow(unused_variable)]
    fn render(&mut self, _window: &mut W, args: &RenderArgs) {
        let (w,  h) = (args.width as f32, args.height as f32);
        let pxRatio = 1.0;
        let t       = self.elapsed_time as f32;
        let dt      = args.ext_dt as f32;
        let (mx,my) = self.mouse;
        let bg      = self.theme().backgroundColor;

        self.nvg().begin_frame(w as i32, h as i32, pxRatio);

        ui::draw(&mut self.themed, w,h, t);

        self.nvg().end_frame();
    }
    //fn key_press(&mut self, _window: &mut W,  _args: &KeyPressArgs) {}
    //fn key_release(&mut self, _window: &mut W, _args: &KeyReleaseArgs) {}
    fn mouse_press(&mut self, _window: &mut W, args: &MousePressArgs) {/*self.oui.set_button(args.button, true);*/}
    fn mouse_release(&mut self, _window: &mut W, args: &MouseReleaseArgs) {}
    fn mouse_move(&mut self, _window: &mut W, args: &MouseMoveArgs) {
        self.mouse = (args.x as i32, args.y as i32);
        //self.oui.set_cursor(args.x as i32, args.y as i32);
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
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    app.run(&mut window, &game_iter_settings);
}
