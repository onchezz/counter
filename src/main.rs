use std::io;

#[derive(Debug)]
struct Counter {
    val: i8,
}
enum CurrentState {
    Adding,
    Removig,
    Stop,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            val: Default::default(),
        }
    }
}
impl Counter {
    fn incremnt(&mut self) {
        self.val += 1;
    }
    fn decrement(&mut self) {
        self.val -= 1;
    }
    fn state(var: &mut Counter, input: String) -> CurrentState {
        if input.trim() == "add" {
            Counter::incremnt(var);
            return CurrentState::Adding;
        } else if input.trim() == "sub" {
            Counter::decrement(var);
            return CurrentState::Removig;
        } else if input.trim() == "exit" {
            return CurrentState::Stop;
        }
        return CurrentState::Stop;
    }
}

fn main() {
    let mut val = Counter { val: 0 };
    loop {
        println!("Enter add or sub");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to get user input");

        match Counter::state(&mut val, user_input) {
            CurrentState::Adding => {
                println!("The value is {}", val.val);
                continue;
            }
            CurrentState::Removig => {
                println!("The value is {}", val.val);
                continue;
            }
            CurrentState::Stop => {
                println!("Exxing >>>>");
                break;
            }
        }
    }
}
