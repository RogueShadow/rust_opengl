pub mod renderer;

use std::sync::mpsc::Receiver;
use glfw::{Context, Glfw, Key};
use crate::renderer::{GsnSprite, Pixel};


pub mod engine {
    pub struct Pixel;
}

pub struct GsnEngine {
    glfw: Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64,glfw::WindowEvent)>,
    pub renderer: renderer::GsnRenderer,
    pub actions: Vec<GsnEvent>
}

//noinspection ALL
pub struct GsnModifiers {
    shift: bool,
    alt: bool,
    control: bool,
    _super: bool,
    capslock: bool,
    numlock: bool
}

pub enum GsnEvent {
    Init,
    Draw,
    KeyPress(GsnKey, GsnAction),
    MousePress(GsnButton, GsnAction, MousePos)
}

pub struct MousePos {
    pub x: u32,
    pub y: u32
}

pub enum GsnAction {
    Press,
    Release,
    Repeat
}

pub enum GsnKey {
    Escape,Space,Up,Down,Left,Right,Apostrophe,Comma,Minus,Period,Slash,
    A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
    Unknown,
    Num0,Num1,Num2,Num3,Num4,Num5,Num6,Num7,Num8,Num9,
    Kp0,Kp1,Kp2,Kp3,Kp4,Kp5,Kp6,Kp7,Kp8,Kp9,
    F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,F13,F14,F15,F16,F17,F18,F19,F20,F21,F22,F23,F24,F25,
    Enter,Grave,LeftBracket,RightBracket,SemiColon,Backslash,
    Equal,World1,World2,Tab,Backspace,Insert,Delete,PageUp,PageDown,
    Home,End,CapsLock,ScrollLock,NumLock,PrintScreen,Pause,
    KpDecimal,KpDivide,KpMultiply,KpSubtract,KpAdd,KpEnter,KpEqual,
    LeftShift,LeftControl,LeftAlt,LeftSuper,RightShift,RightControl,
    RightAlt,RightSuper,Menu
}

type GsnButton = u8;

pub enum GsnWindowMode {
    WINDOWED
}

impl GsnEngine {
    pub fn new(title: &str, width: u32, height: u32, mode: GsnWindowMode) -> GsnEngine {
        let actions = vec![GsnEvent::Init];
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
        window.set_mouse_button_polling(true);


        GsnEngine {
            glfw,
            window,
            events,
            renderer,
            actions
        }
    }

    pub fn render(&mut self) {
        self.actions.push(GsnEvent::Draw);
        self.renderer.render();
        self.window.swap_buffers();
    }

    pub fn buffer(&mut self) -> &mut GsnSprite {
        &mut self.renderer.buffer
    }

    pub fn width(&self) -> u32 {
        self.renderer.buffer.width
    }

    pub fn height(&self) -> u32 {
        self.renderer.buffer.height
    }

    pub fn exit(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => {
                    let gsn_key = map_keys(key);
                    let gsn_action = map_action(action);
                    self.actions.push(GsnEvent::KeyPress(gsn_key, gsn_action));
                },
                glfw::WindowEvent::MouseButton(button,action, modifiers) => {
                    let (mx,my) = self.window.get_cursor_pos();
                    let gsn_button: GsnButton = map_mouse_button(button);
                    let gsn_action = map_action(action);
                    self.actions.push(GsnEvent::MousePress(gsn_button,gsn_action,MousePos{x: mx as u32,y: my as u32}));
                },
                _ => {},
            }
        }
        self.actions.push(GsnEvent::Draw);
        self.render();
    }

}

pub fn new_pixel(r: u8, g: u8, b: u8) -> Pixel {
    Pixel::from_rgb(r,g,b)
}

fn map_action(glfw_action: glfw::Action) -> GsnAction {
    match glfw_action {
        glfw::Action::Press => GsnAction::Press,
        glfw::Action::Release => GsnAction::Release,
        glfw::Action::Repeat => GsnAction::Repeat
    }
}
fn map_mouse_button(glfw_mouse_button: glfw::MouseButton) -> GsnButton {
    match glfw_mouse_button {
        glfw::MouseButton::Button1 => 1,
        glfw::MouseButton::Button2 => 2,
        glfw::MouseButton::Button3 => 3,
        glfw::MouseButton::Button4 => 4,
        glfw::MouseButton::Button5 => 5,
        glfw::MouseButton::Button6 => 6,
        glfw::MouseButton::Button7 => 7,
        glfw::MouseButton::Button8 => 8
    }
}
fn map_keys(glfw_key: glfw::Key) -> GsnKey {
    match glfw_key {
        Key::Space => {GsnKey::Space}
        Key::Apostrophe => {GsnKey::Apostrophe}
        Key::Comma => {GsnKey::Comma}
        Key::Minus => {GsnKey::Minus}
        Key::Period => {GsnKey::Period}
        Key::Slash => {GsnKey::Slash}
        Key::Num0 => {GsnKey::Num0}
        Key::Num1 => {GsnKey::Num1}
        Key::Num2 => {GsnKey::Num2}
        Key::Num3 => {GsnKey::Num3}
        Key::Num4 => {GsnKey::Num4}
        Key::Num5 => {GsnKey::Num5}
        Key::Num6 => {GsnKey::Num6}
        Key::Num7 => {GsnKey::Num7}
        Key::Num8 => {GsnKey::Num8}
        Key::Num9 => {GsnKey::Num9}
        Key::Semicolon => {GsnKey::SemiColon}
        Key::Equal => {GsnKey::Equal}
        Key::A => {GsnKey::A}
        Key::B => {GsnKey::B}
        Key::C => {GsnKey::C}
        Key::D => {GsnKey::D}
        Key::E => {GsnKey::E}
        Key::F => {GsnKey::F}
        Key::G => {GsnKey::G}
        Key::H => {GsnKey::H}
        Key::I => {GsnKey::I}
        Key::J => {GsnKey::J}
        Key::K => {GsnKey::K}
        Key::L => {GsnKey::L}
        Key::M => {GsnKey::M}
        Key::N => {GsnKey::N}
        Key::O => {GsnKey::O}
        Key::P => {GsnKey::P}
        Key::Q => {GsnKey::Q}
        Key::R => {GsnKey::R}
        Key::S => {GsnKey::S}
        Key::T => {GsnKey::T}
        Key::U => {GsnKey::U}
        Key::V => {GsnKey::V}
        Key::W => {GsnKey::W}
        Key::X => {GsnKey::X}
        Key::Y => {GsnKey::Y}
        Key::Z => {GsnKey::Z}
        Key::LeftBracket => {GsnKey::LeftBracket}
        Key::Backslash => {GsnKey::Backslash}
        Key::RightBracket => {GsnKey::RightBracket}
        Key::GraveAccent => {GsnKey::Grave}
        Key::World1 => {GsnKey::World1}
        Key::World2 => {GsnKey::World2}
        Key::Escape => {GsnKey::Escape}
        Key::Enter => {GsnKey::Enter}
        Key::Tab => {GsnKey::Tab}
        Key::Backspace => {GsnKey::Backspace}
        Key::Insert => {GsnKey::Insert}
        Key::Delete => {GsnKey::Delete}
        Key::Right => {GsnKey::Right}
        Key::Left => {GsnKey::Left}
        Key::Down => {GsnKey::Down}
        Key::Up => {GsnKey::Up}
        Key::PageUp => {GsnKey::PageUp}
        Key::PageDown => {GsnKey::PageDown}
        Key::Home => {GsnKey::Home}
        Key::End => {GsnKey::End}
        Key::CapsLock => {GsnKey::CapsLock}
        Key::ScrollLock => {GsnKey::ScrollLock}
        Key::NumLock => {GsnKey::NumLock}
        Key::PrintScreen => {GsnKey::PrintScreen}
        Key::Pause => {GsnKey::Pause}
        Key::F1 => {GsnKey::F1}
        Key::F2 => {GsnKey::F2}
        Key::F3 => {GsnKey::F3}
        Key::F4 => {GsnKey::F4}
        Key::F5 => {GsnKey::F5}
        Key::F6 => {GsnKey::F6}
        Key::F7 => {GsnKey::F7}
        Key::F8 => {GsnKey::F8}
        Key::F9 => {GsnKey::F9}
        Key::F10 => {GsnKey::F10}
        Key::F11 => {GsnKey::F11}
        Key::F12 => {GsnKey::F12}
        Key::F13 => {GsnKey::F13}
        Key::F14 => {GsnKey::F14}
        Key::F15 => {GsnKey::F15}
        Key::F16 => {GsnKey::F16}
        Key::F17 => {GsnKey::F17}
        Key::F18 => {GsnKey::F18}
        Key::F19 => {GsnKey::F19}
        Key::F20 => {GsnKey::F20}
        Key::F21 => {GsnKey::F21}
        Key::F22 => {GsnKey::F22}
        Key::F23 => {GsnKey::F23}
        Key::F24 => {GsnKey::F24}
        Key::F25 => {GsnKey::F25}
        Key::Kp0 => {GsnKey::Kp0}
        Key::Kp1 => {GsnKey::Kp1}
        Key::Kp2 => {GsnKey::Kp2}
        Key::Kp3 => {GsnKey::Kp3}
        Key::Kp4 => {GsnKey::Kp4}
        Key::Kp5 => {GsnKey::Kp5}
        Key::Kp6 => {GsnKey::Kp6}
        Key::Kp7 => {GsnKey::Kp7}
        Key::Kp8 => {GsnKey::Kp8}
        Key::Kp9 => {GsnKey::Kp9}
        Key::KpDecimal => {GsnKey::KpDecimal}
        Key::KpDivide => {GsnKey::KpDivide}
        Key::KpMultiply => {GsnKey::KpMultiply}
        Key::KpSubtract => {GsnKey::KpSubtract}
        Key::KpAdd => {GsnKey::KpAdd}
        Key::KpEnter => {GsnKey::KpEnter}
        Key::KpEqual => {GsnKey::KpEqual}
        Key::LeftShift => {GsnKey::LeftShift}
        Key::LeftControl => {GsnKey::LeftControl}
        Key::LeftAlt => {GsnKey::LeftAlt}
        Key::LeftSuper => {GsnKey::LeftSuper}
        Key::RightShift => {GsnKey::RightShift}
        Key::RightControl => {GsnKey::RightControl}
        Key::RightAlt => {GsnKey::RightAlt}
        Key::RightSuper => {GsnKey::RightSuper}
        Key::Menu => {GsnKey::Menu}
        Key::Unknown => {GsnKey::Unknown}
    }
}