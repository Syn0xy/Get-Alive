use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;

pub struct InputSystem {
    stdin_channel: Receiver<String>,
}

impl Default for InputSystem {
    fn default() -> Self {
        Self {
            stdin_channel: spawn_stdin_channel(),
        }
    }
}

impl InputSystem {
    pub fn get_input(&self) -> Result<String, String> {
        match self.stdin_channel.try_recv() {
            Ok(key) => Ok(format!("Received: {}", key)),
            Err(TryRecvError::Empty) => Err(format!("Channel empty")),
            Err(TryRecvError::Disconnected) => Err(format!("Channel disconnected")),
        }
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            tx.send(buffer).unwrap();
        }
    });
    rx
}
