use std::time::Duration;

use super::DisplayEvent;

const CARRIAGE_RETURN: char = '\n';
const HEIGHT: u32 = 10;
const WIDTH: u32 = HEIGHT * 2;

pub fn handle_render(event: &DisplayEvent) {
    println!("oui");
    match event {
        DisplayEvent::RedrawRequested { elapsed_time } => render(elapsed_time),
    }
}

fn render(elapsed_time: &Duration) {
    let mut string_buffer = String::new();

    string_buffer.push(CARRIAGE_RETURN);

    for _ in 0..HEIGHT {
        for _ in 0..WIDTH {
            string_buffer.push_str(".");
        }
        string_buffer.push(CARRIAGE_RETURN);
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("fps: {:?}", 1.0 / elapsed_time.as_secs_f32());
    print!("{string_buffer}");
}
