pub mod application_handler;
pub mod entity;
pub mod input;
pub mod renderer;
pub mod runtime;

use std::sync::Arc;

use entity::{EntityBuilder, EntityManager};
use input::InputSystem;
use renderer::{DrawCommandBuffer, GraphicsColor, RendererManager};
use runtime::RuntimeManager;
use winit::{
    dpi::PhysicalSize,
    event::KeyEvent,
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes},
};

use crate::model::{entity::PlayerController, transform::Transform};

pub struct App<R: RuntimeManager, I: InputSystem, E: EntityManager, D: RendererManager> {
    window: Option<Arc<Window>>,
    window_attributes: WindowAttributes,
    runtime_manager: R,
    input_system: I,
    entity_manager: E,
    renderer_manager: Option<D>,
    draw_command_buffer: DrawCommandBuffer,
}

impl<E: EntityManager, I: InputSystem, R: RuntimeManager, D: RendererManager> App<R, I, E, D> {
    pub fn new(
        window_attributes: WindowAttributes,
        runtime_manager: R,
        input_system: I,
        entity_manager: E,
    ) -> Self {
        Self {
            window: None,
            window_attributes,
            runtime_manager,
            input_system,
            entity_manager,
            renderer_manager: None,
            draw_command_buffer: DrawCommandBuffer::new(),
        }
    }

    pub fn handle_resize(&mut self, inner_size: PhysicalSize<u32>) {
        if let Some(renderer_manager) = self.renderer_manager.as_mut() {
            renderer_manager.resize(inner_size).ok();
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        if let PhysicalKey::Code(key_code) = event.physical_key {
            self.input_system.dispatch(key_code, &event.state);
        }
    }

    pub fn start(&mut self) {
        let player_id = self
            .entity_manager
            .build_entity()
            .with_component(Transform::default())
            .with_component(PlayerController::default())
            .build();

        if let Some(transform) = self
            .entity_manager
            .entity_component_mut::<Transform>(player_id)
        {
            self.input_system.subscribe("Up", move || {
                transform.position.y += 1.0;
                println!("transform: {:?}", transform);
            });
        }
    }

    pub fn handle_redraw(&mut self) {
        self.paint();

        if let Some(renderer_manager) = self.renderer_manager.as_mut() {
            if let Err(error) = renderer_manager.draw(&mut self.draw_command_buffer) {
                eprintln!("[ ERROR ] Renderer Manager : {}", error);
            }
        }

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    pub fn paint(&mut self) {
        self.draw_command_buffer.push(|graphics| {
            graphics.set_color(GraphicsColor::RED);
            graphics.full_fill();
            graphics.set_color(GraphicsColor::GREEN);
            graphics.pixel(10, 10);
            graphics.set_color(GraphicsColor::BLUE);
            graphics.fill_rect(20, 20, 100, 100);
            graphics.fill_rect(600, 300, 100, 100);
        });
    }
}
