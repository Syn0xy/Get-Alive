use get_alive::input::{InputError, InputSystem};

fn main() {
    let input_system = &InputSystem::default();

    loop {
        match input_system.get_input() {
            Ok(input) => {
                println!("Received: {:?}", input);
                break;
            }
            Err(InputError::InvalidInput(invalid_input)) => {
                println!("Input invalid: {:?}", invalid_input)
            }
            Err(InputError::NoInput) => {}
        }
    }
}
