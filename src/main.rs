pub mod app;
pub mod constants;
pub mod event;

use std::process;

use app::{
    input::{InputEvent, InputKey},
    main::MainEvent,
    render::handle_event,
};
use event::{BasicEventLoop, Dispatcher, EventLoop};

fn main() {
    let mut event_loop = BasicEventLoop::default();

    event_loop.subscribe(handle_event);
    event_loop.subscribe(input);
    event_loop.subscribe(exit);
    event_loop.run();
}

fn input(event: &InputEvent) {
    match event {
        InputEvent::KeyboardInput { key } => {
            println!("oui: {key:?}");
            if matches!(key, &InputKey::A) {
                println!("Exiting");
                // dispatch(&MainEvent::LoopExiting);
            }
        }
    }
}

fn exit(event: &MainEvent) {
    match event {
        MainEvent::Tick => tick(),
        MainEvent::LoopExiting => process::exit(0),
    }
}

fn tick() {
    // println!("tick");
}

/*

-> EventLoop

    -> HashMap [ TypeId ; Dispatcher ]

        -> Dispatcher

            -> List [ FnOnce / FnMut ]

        # subscribe [ FnOnce / FnMut ]
        # emit [ CustomEvent ]

    # subscribe [ CustomEvent ; Fnonce / FnMut ]
    # emit [ CustomEvent ]

    Main

*/
