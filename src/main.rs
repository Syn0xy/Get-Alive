pub mod app;
pub mod constants;
pub mod event;

use std::time::Instant;

use app::main::{BasicEntityManager, EntityManager};

fn main() {
    // let dispatcher = BasicDispatcher::default();
    // let entity_manager = BasicEntityManager::default();
    // let input_system = BasicInputSystem::default();
    // let app = AppState::new(entity_manager, Box::new(input_system));

    // BasicEventLoop::new(dispatcher, app).run();

    init(&mut BasicEntityManager::default());
}

fn init<T: EntityManager>(entity_manager: &mut T) {
    let player_id = entity_manager.create_entity();

    entity_manager.attach_component(player_id, Player::default());
    for _ in 0..500 {
        let c_id = entity_manager.create_entity();
        entity_manager.attach_component(c_id, Collider);
    }

    if let Some(player) = entity_manager.borrow_single_entity_component_mut::<Player>(player_id) {
        let controller = &mut player.controller;

        controller.direction.x = 1;
        controller.direction.y = 1;

        controller.direction = IVec2::ZERO;

        player.controller = PlayerController::default();
    }

    let colliders = entity_manager.borrow_component::<Collider>();

    let start_time = Instant::now();
    let mut last_time = start_time;
    let mut total = 0_f32;
    let mut i = 0;

    for &(_id_a, _collider_a) in colliders.iter() {
        for &(_id_b, _collider_b) in colliders.iter() {
            total += last_time.elapsed().as_nanos() as f32;
            last_time = Instant::now();
            i += 1;
        }
    }

    let elapsed = start_time.elapsed().as_secs_f32();
    let average = total / i as f32;
    let tickrate = 1.0 / elapsed;

    println!("total: {elapsed} secs");
    println!("avg: {average} nanosecs");
    println!("tickrate: {tickrate}");
}

#[derive(Debug, Default)]
struct Player {
    controller: PlayerController,
}

#[derive(Debug, Default)]
struct PlayerController {
    direction: Vec2<i32>,
}

#[derive(Debug, Default)]
struct Vec2<T> {
    x: T,
    y: T,
}

type FVec2 = Vec2<f32>;
type IVec2 = Vec2<i32>;

#[derive(Debug, Default)]
struct Collider;

impl FVec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl IVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
}
