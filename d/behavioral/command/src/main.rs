pub trait Command {
    fn execute(&self);
}

pub struct FirstCommand;
impl Command for FirstCommand {
    fn execute(&self) {
        println!("FirstCommand")
    }
}

pub struct Receiver;
impl Receiver {
    pub fn operation(&self, param: String) {
        println!("Receiver {}", param)
    }
}

pub struct SecondCommand {
    receiver: Receiver,
    param: String,
}

impl SecondCommand {
    pub fn new(receiver: Receiver, param: String) -> Self {
        SecondCommand {
            receiver: receiver,
            param: param,
        }
    }
}

impl Command for SecondCommand {
    fn execute(&self) {
        print!("SecondCommand, ");
        self.receiver.operation(self.param.clone())
    }
}

pub struct Invoker {
    on_start: Option<Box<dyn Command>>,
    on_finish: Option<Box<dyn Command>>,
}
impl Invoker {
    pub fn new(on_start: Option<Box<dyn Command>>, on_finish: Option<Box<dyn Command>>) -> Self {
        Invoker {
            on_start: on_start,
            on_finish: on_finish,
        }
    }

    pub fn execute_command(&self) {
        print!("OnStart: ");
        match &self.on_start {
            Some(command) => command.execute(),
            _ => (),
        }

        print!("OnFinish: ");
        match &self.on_finish {
            Some(command) => command.execute(),
            _ => (),
        }
    }
}

fn main() {
    let first_command = FirstCommand;

    let receiver = Receiver;
    let second_command = SecondCommand::new(receiver, String::from("Send email"));

    let invoker = Invoker::new(
        Some(Box::new(first_command)),
        Some(Box::new(second_command)),
    );
    invoker.execute_command()
}
