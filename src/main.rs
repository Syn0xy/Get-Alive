use get_alive::input::InputSystem;

fn main() {
    let input_system = &InputSystem::default();

    loop {
        if let Ok(input) = input_system.get_input() {
            println!("{input}");
            break;
        }
        println!("oui");
    }
}
