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

    let mut entity_manager = BasicEntityManager::default();

    entity_manager.create_entity();

    init(&mut entity_manager);
}

fn init<T: EntityManager>(entity_manager: &mut T) {
    let player_id = 0;

    entity_manager.attach_component(player_id, Player::default());
    for _ in 0..100 {
        let c_id = entity_manager.create_entity();
        entity_manager.attach_component(c_id, Collider);
    }

    if let Some(player) = entity_manager
        .borrow_entity_component_mut::<Player>(player_id)
        .first_mut()
    {
        let controller = &mut player.controller;

        controller.direction.x = 1.0;
        controller.direction.y = 1.0;

        player.controller = PlayerController::default();
    }

    let colliders = entity_manager.borrow_component::<Collider>();

    let start_time = Instant::now();
    let mut last_time = start_time;
    let mut total = 0_f32;
    let mut i = 0;

    for _ in colliders.iter() {
        for _ in colliders.iter() {
            // if i % 10 == 0 {
            //     println!("{:7}: {:?}", i, last_time.elapsed());
            // }
            total += last_time.elapsed().as_nanos() as f32;
            last_time = Instant::now();
            i += 1;
        }
    }

    println!("total: {} secs", start_time.elapsed().as_secs_f32());
    let avg = total / i as f32;
    println!("avg: {}", avg);
    println!("tickrate: {}", 1.0 / avg);
}

#[derive(Debug, Default)]
struct Player {
    controller: PlayerController,
}

#[derive(Debug, Default)]
struct PlayerController {
    direction: Vec2,
}

#[derive(Debug, Default)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Default)]
struct Collider;
