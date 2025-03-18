pub mod game;
pub mod input;

use std::time::Instant;

use game::Game;
use input::InputError;

const HEIGHT: i32 = 20;
const WIDTH: i32 = HEIGHT * 4;
const TICKS_PER_SECOND: i32 = 20;
const TICK_RATE: f32 = 1_f32 / TICKS_PER_SECOND as f32;

fn main() {
    let game = Game::default();
    let input_system = game.get_input_system();

    let mut last_time = Instant::now();

    loop {
        match input_system.get_input() {
            Ok(input) => {
                println!("Input received: {:?}", input);
                break;
            }
            Err(InputError::InvalidInput(invalid_input)) => {
                println!("Input invalid: {:?}", invalid_input)
            }
            Err(InputError::NoInput) => {}
        }

        let crnt_time = Instant::now();
        let diff_time = (crnt_time - last_time).as_secs_f32();

        if diff_time > TICK_RATE {
            last_time = crnt_time;

            print!("\x1B[2J\x1B[1;1H");
            println!("fps: {:?}", 1.0 / diff_time);
            display();
        }
    }
}

fn display() {
    let mut buffer = String::default();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buffer.push_str(".");
        }
        buffer.push_str("\n");
    }
    print!("{buffer}");
}
