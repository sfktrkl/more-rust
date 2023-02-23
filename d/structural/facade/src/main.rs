pub struct FirstSubsystem;
impl FirstSubsystem {
    pub fn operation(&self) -> String {
        String::from("FirstSubsystem")
    }
}

pub struct SecondSubsystem;
impl SecondSubsystem {
    pub fn operation(&self) -> String {
        String::from("SecondSubsystem")
    }
}

pub struct Facade {
    first_subsystem: FirstSubsystem,
    second_subystem: SecondSubsystem,
}
impl Facade {
    pub fn new(first_subsystem: FirstSubsystem, second_subystem: SecondSubsystem) -> Self {
        Facade {
            first_subsystem: first_subsystem,
            second_subystem: second_subystem,
        }
    }

    pub fn operation(&self) {
        println!(
            "Facade: {}, {}",
            self.first_subsystem.operation(),
            self.second_subystem.operation()
        )
    }
}

fn main() {
    let first_subsystem = FirstSubsystem;
    let second_subystem = SecondSubsystem;
    let facade = Facade::new(first_subsystem, second_subystem);
    facade.operation();
}
