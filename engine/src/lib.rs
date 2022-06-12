pub mod renderer;

use std::sync::mpsc::Receiver;
use glfw::{Context, Glfw};
use crate::renderer::Pixel;


pub mod engine {
    pub struct Pixel;
}

pub struct GsnEngine {
    glfw: Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64,glfw::WindowEvent)>,
    pub renderer: renderer::GsnRenderer,
    pub actions: Vec<GsnAction>
}

pub enum GsnAction {
    Init,
    Update,
    Draw,
    Exit,
    KeyPress(GsnKey)
}


pub enum GsnKey {
    Escape
}

pub enum GsnWindowMode {
    WINDOWED
}

impl GsnEngine {
    pub fn new(title: &str, width: u32, height: u32, mode: GsnWindowMode) -> GsnEngine {
        let actions = vec![GsnAction::Init];
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw.create_window(
            width,
            height,
            title,
            match mode {
                GsnWindowMode::WINDOWED => glfw::WindowMode::Windowed
            }).expect("Failed to create GLFW window.");
        gl::load_with(|s| window.get_proc_address(s));
        let mut renderer = renderer::new_gsn_renderer();
        renderer.initialize(width,height);
        window.make_current();
        window.set_key_polling(true);

        GsnEngine {
            glfw,
            window,
            events,
            renderer,
            actions
        }
    }

    pub fn render(&mut self) {
        self.actions.push(GsnAction::Draw);
        self.renderer.render();
        self.window.swap_buffers();
    }

    fn set_pixel(&mut self, x: u32, y: u32, p: Pixel) ->  bool {
        self.renderer.set_pixel(x,y,p)
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, p: Pixel) {
        let x = x.clamp(0,self.width());
        let y = y.clamp(0, self.height());
        let x2 = (x + w).clamp( 0,self.width());
        let y2 = (y + h).clamp(0,self.height());

        for dx in x..x2 {
            for dy in y..y2 {
                self.set_pixel(dx,dy,p);
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.renderer.buffer.width
    }

    pub fn height(&self) -> u32 {
        self.renderer.buffer.height
    }

    pub fn exit(&mut self) {
        self.actions.push(GsnAction::Exit);
        self.window.set_should_close(true);
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.actions.push(GsnAction::KeyPress(GsnKey::Escape));
                },
                _ => {},
            }
        }
        self.actions.push(GsnAction::Update);
        self.actions.push(GsnAction::Draw);
        self.render();
    }

}

pub fn new_pixel(r: u8, g: u8, b: u8) -> Pixel {
    Pixel::from_rgb(r,g,b)
}