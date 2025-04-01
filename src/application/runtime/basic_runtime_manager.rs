use std::time::Instant;

use super::RuntimeManager;

#[derive(Default)]
pub struct BasicRuntimeManager {
    framecount: u32,
    last_frame_time: Option<Instant>,
}

impl RuntimeManager for BasicRuntimeManager {
    fn get_framerate(&mut self) -> u32 {
        self.framecount += 1;

        if let Some(last_frame_time) = self.last_frame_time {
            let elapsed_time = last_frame_time.elapsed();

            if elapsed_time.as_secs_f32() > 1.0 {
                println!("framecount: {}", self.framecount);
                self.framecount = 0;
                self.last_frame_time.replace(Instant::now());
            }
        } else {
            self.framecount = 0;
            self.last_frame_time.replace(Instant::now());
        }

        self.framecount
    }
}
