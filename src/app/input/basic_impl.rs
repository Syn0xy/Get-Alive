use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use super::InputError;
use super::InputKey;
use super::InputSystem;

pub struct BasicInputSystem {
    stdin_channel: Receiver<char>,
}

impl Default for BasicInputSystem {
    fn default() -> Self {
        Self {
            stdin_channel: spawn_stdin_channel(),
        }
    }
}

impl InputSystem for BasicInputSystem {
    fn get_input(&self) -> Result<InputKey, InputError> {
        self.stdin_channel
            .try_recv()
            .map_or(Err(InputError::NoInput), InputKey::from_char)
    }
}

fn spawn_stdin_channel() -> Receiver<char> {
    let (tx, rx) = mpsc::channel::<char>();

    thread::spawn(move || {
        let mut string_buffer = String::default();

        while io::stdin().read_line(&mut string_buffer).is_ok() {
            if let Some(first_char) = string_buffer.chars().next() {
                tx.send(first_char).unwrap()
            }

            string_buffer.clear();
        }
    });
    rx
}
