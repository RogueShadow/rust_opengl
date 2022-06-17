use engine;
use rand;
use rand::Rng;
use engine::{GsnEvent, GsnEngine, GsnKey, GsnAction};

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
                GsnEvent::Init => {
                    println!("Init.");
                }
                GsnEvent::Draw => {
                    for x in (0..width).step_by(step as usize) {
                        for y in (0..height).step_by(step as usize) {
                            let p = engine::new_pixel(
                                rng.gen_range(0..255),
                                rng.gen_range(0..255),
                                rng.gen_range(0..255)
                            );
                            gsn_engine.buffer().fill_rect(x, y, step, step, p);
                        }
                    }
                }
                GsnEvent::KeyPress(k, GsnAction::Press) => {
                    match k {
                        GsnKey::Escape => {break 'running}
                        _ => {}
                    }
                }
                GsnEvent::MousePress(1,GsnAction::Press,pos) => {
                    let p = gsn_engine.buffer().get_pixel(pos.x,pos.y);
                    println!("{:?}",p);
                }
                GsnEvent::KeyPress(_, GsnAction::Release) => {}
                GsnEvent::KeyPress(_, GsnAction::Repeat) => {}
                GsnEvent::MousePress(_, _, _) => {}
            }
        }
    }
}
