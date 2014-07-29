#![feature(globs)]
#![allow(unused_imports)]
#![allow(unused_variable)]
#![allow(dead_code)]

extern crate graphics;
extern crate piston;
extern crate glfw_game_window;

extern crate nanovg;
extern crate blendish;
extern crate oui;

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
pub use oui::Context;
pub use oui::oui::*;

mod ui;
mod resources;

///////////////////////////////////////////////////////////////////////
// shorthand fns
//fn rgb(r:u8, g:u8, b:u8) -> Color { Color::rgb(r,g,b) }
fn rgba(r:u8, g:u8, b:u8, a:u8) -> Color { Color::rgba(r,g,b, a) }
// just testing
fn draw_bg(vg: &mut Ctx, x:f32,y:f32,w:f32,h:f32) {
    let paint = vg.linear_gradient(x,y,x,y+15.0, rgba(255,255,255,8), rgba(0,0,0,16));
    vg.begin_path();
    vg.rect(x,y,w,h);
    vg.fill_paint(paint);
    vg.fill();
}
///////////////////////////////////////////////////////////////////////


pub struct App<'a> {
    //resources: Resources,
    mouse: (i32,i32),           // current mouse pos
    elapsed_time: f64,          // seconds since app start
    oui: Context,
    themed: ThemedContext<'a>   // wrap nvg ctx w/ themed-draw fns
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let nvg = Ctx::create_gl3(ANTIALIAS|STENCIL_STROKES);
        let resources = Resources::load(&nvg, "../../res");
        let font = resources.fontNormal;
        let icons = resources.iconsheet;    // move resources into the ThemedContext
        App {
            //resources: resources,
            mouse: (0,0),
            elapsed_time: 0.0,         // time since app start
            oui: Context::create_context(),
            themed: ThemedContext::wrap(nvg, icons, font)
        }
    }
    fn nvg(&mut self) -> &mut Ctx { self.themed.nvg() }

    fn theme(&self) -> &Theme { self.themed.theme() }
}

impl<'a, W: GameWindow> Game<W> for App<'a>
{
    fn load(&mut self, _window: &mut W) {
    }

    fn update(&mut self, window: &mut W, args: &UpdateArgs) {
        self.elapsed_time += args.dt;
        //let (mx, my) = window.get_cursor_pos();
        //self.oui.set_cursor(mx, my);
    }

    #[allow(unused_variable)]
    fn render(&mut self, _window: &mut W, args: &RenderArgs) {
        //let (winWidth, winHeight) = window.get_size();
        //let (fbWidth, fbHeight) = window.get_framebuffer_size();
        //// Calculate pixel ration for hi-dpi devices.
        //let pxRatio = fbWidth as f32 / winWidth as f32;

        let (w,  h) = (args.width as f32, args.height as f32);
        let pxRatio = 1.0;
        let t       = self.elapsed_time as f32;
        let dt      = args.ext_dt as f32;
        let (mx,my) = self.mouse;
        let bg      = self.theme().backgroundColor;

        self.nvg().begin_frame(w as i32, h as i32, pxRatio);

        draw_bg(self.nvg(), 0.0,0.0, w,h);
        ui::draw(&mut self.oui, &mut self.themed, w,h, t);

        self.nvg().end_frame();
    }

    //fn key_press(&mut self, _window: &mut W,  _args: &KeyPressArgs) {}
    //fn key_release(&mut self, _window: &mut W, _args: &KeyReleaseArgs) {}

    fn mouse_press(&mut self, _window: &mut W, args: &MousePressArgs) {
//        self.oui.set_button(args.button, true);
    }
    fn mouse_release(&mut self, _window: &mut W, args: &MouseReleaseArgs) {
//        self.oui.set_button(args.button, false);
    }
    fn mouse_move(&mut self, _window: &mut W, args: &MouseMoveArgs) {
        self.mouse = (args.x as i32, args.y as i32);
        self.oui.set_cursor(args.x as i32, args.y as i32);
    }
    ///// Moved mouse relative, not bounded by cursor.
    //fn mouse_relative_move(&mut self, _window: &mut W, _args: &MouseRelativeMoveArgs) {}
    //fn mouse_scroll(&mut self, _window: &mut W, _args: &MouseScrollArgs) {}
}


fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "Blendish/NanoVG UI demo".to_string(),
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
