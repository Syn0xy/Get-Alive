use std::process;

use crate::{app::AppState, event::dispatcher::Dispatcher};

use super::{EntityManager, MainEvent};

pub fn handle_main_event<D: Dispatcher, S: EntityManager>(event: &MainEvent<D, S>) {
    match event {
        MainEvent::Tick {
            app_state: _app_state,
            dispatcher: _dispatcher,
        } => {
            // if let Some(dispatcher) = dispatcher.clone().upgrade() {
            //     if let Some(app_state) = app_state.clone().upgrade() {
            //         handle_tick(dispatcher.get_mut(), app_state.get_mut())
            //     }
            // }
        }
        MainEvent::LoopExiting => process::exit(0),
    }
}

fn _handle_tick<D: Dispatcher, S: EntityManager>(
    _dispatcher: &mut D,
    _app_state: &mut AppState<S>,
) {
    println!("tick");
    // let input_system = app_state.get_input_system();

    // match input_system.get_input() {
    //     Ok(key) => dispatcher.dispatch(&InputEvent::KeyboardInput { key }),
    //     Err(InputError::InvalidInput(invalid_input)) => {
    //         eprintln!("[ERROR]: invalid input {:?}", invalid_input)
    //     }
    //     Err(InputError::NoInput) => {}
    // }

    // let crnt_time = Instant::now();
    // let tickrate = app_state.get_tickrate();

    // match app_state.get_last_tick() {
    //     Some(last_tick) => {
    //         let elapsed = crnt_time - *last_tick;

    //         if elapsed.as_secs_f32() >= 1.0 / tickrate as f32 {
    //             app_state.set_last_tick(crnt_time);

    //             dispatcher.dispatch(&DisplayEvent::RedrawRequested {
    //                 elapsed_time: elapsed,
    //             });
    //         }
    //     }
    //     None => app_state.set_last_tick(crnt_time),
    // }
}
