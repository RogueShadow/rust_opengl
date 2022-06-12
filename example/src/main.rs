use engine;
use rand;
use rand::Rng;
use engine::{GsnAction, GsnEngine, GsnKey};

fn main() {
    // Setting up the window.
    let mut rng = rand::thread_rng();

    let mut gsn_engine = GsnEngine::new(
        "This is a window",
        1024,
        786,
        engine::GsnWindowMode::WINDOWED);

    let width = gsn_engine.width();
    let height = gsn_engine.height();
    let step: u32 = 8;

    // The looping.
    'running: loop {
        gsn_engine.update();
        while !gsn_engine.actions.is_empty() {
            match gsn_engine.actions.pop().unwrap() {
                GsnAction::Init => {
                    println!("Init.");
                }
                GsnAction::Update => {

                }
                GsnAction::Draw => {
                    for x in (0..width).step_by(step as usize) {
                        for y in (0..height).step_by(step as usize) {
                            let p = engine::new_pixel(
                                rng.gen_range(0..255),
                                rng.gen_range(0..255),
                                rng.gen_range(0..255)
                            );
                            gsn_engine.draw_rect(x,y,step,step,p);
                        }
                    }
                }
                GsnAction::Exit => {println!("Exit.");break 'running}
                GsnAction::KeyPress(k) => {
                    match k {
                        GsnKey::Escape => {gsn_engine.actions.push(GsnAction::Exit)}
                    }
                }
            }
        }
    }
}

