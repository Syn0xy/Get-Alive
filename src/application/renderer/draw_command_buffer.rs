use super::Graphics;

pub struct DrawCommandBuffer {
    commands: Vec<Box<dyn FnOnce(&mut dyn Graphics)>>,
}

impl DrawCommandBuffer {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn push<F>(&mut self, func: F)
    where
        F: FnOnce(&mut dyn Graphics) + 'static,
    {
        self.commands.push(Box::new(func));
    }

    pub fn consume(&mut self, graphics: &mut dyn Graphics) {
        for func in self.commands.drain(..) {
            func(graphics);
        }
    }
}
