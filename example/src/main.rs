use std::collections::HashMap;
use engine;
use rand;
use rand::Rng;
use engine::{GsnEvent, GsnEngine, GsnKey, GsnAction};
use engine::renderer::{BLACK, GsnSprite, Pixel, pixel_rgb};

#[derive(Debug,Copy,Clone)]
pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub color: Pixel
}
impl Entity {
    fn left(&mut self) {
        self.x -= self.speed;
    }
    fn right(&mut self) {
        self.x += self.speed;
    }
    fn up(&mut  self) {
        self.y += self.speed;
    }
    fn down(&mut self) {
        self.y -= self.speed;
    }
    fn draw(&mut self,buffer: &mut GsnSprite) {
        buffer.fill_rect(self.x as u32,self.y as u32,16,16,self.color);
    }
}

fn main() {
    // Setting up the window.
    let mut game = GsnEngine::new(
        "This is a window",
        1024,
        786,
        engine::GsnWindowMode::WINDOWED);


    let mut player = Entity {
        x: 0.0,
        y: 0.0,
        speed: 8.0,
        color: pixel_rgb(128,128,0)
    };

    // The looping.
    'running: loop {
        game.update();
        while !game.actions.is_empty() {
            match game.actions.pop().unwrap() {
                GsnEvent::Draw => {
                    if game.key_held(GsnKey::A) {player.left()}
                    if game.key_held(GsnKey::D){player.right()}
                    if game.key_held(GsnKey::W) {player.up()}
                    if game.key_held(GsnKey::S) {player.down()}

                    game.buffer().clear(BLACK);

                    player.draw(game.buffer());
                }
                GsnEvent::KeyPress(k, GsnAction::Press) => {
                    match k {
                        GsnKey::Escape => {break 'running}
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
