use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;

use super::InputError;
use super::InputKey;

pub struct InputSystem {
    stdin_channel: Receiver<InputKey>,
}

impl Default for InputSystem {
    fn default() -> Self {
        Self {
            stdin_channel: spawn_stdin_channel(),
        }
    }
}

impl InputSystem {
    pub fn get_input(&self) -> Result<InputKey, InputError> {
        match self.stdin_channel.try_recv() {
            Ok(key) => Ok(key),
            Err(error) => match error {
                TryRecvError::Empty => Err(InputError::NoInput),
                TryRecvError::Disconnected => Err(InputError::NoInput),
            },
        }
    }
}

fn spawn_stdin_channel() -> Receiver<InputKey> {
    let (tx, rx) = mpsc::channel::<InputKey>();
    thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            // tx.send(buffer).unwrap();
            tx.send(InputKey::A).unwrap();
        }
    });
    rx
}
