pub trait State: std::fmt::Debug {
    fn operation_a(&self) -> Box<dyn State>;
    fn operation_b(&self) -> Box<dyn State>;
}

#[derive(Debug)]
pub struct FirstState;
impl State for FirstState {
    fn operation_a(&self) -> Box<dyn State> {
        Box::new(SecondState)
    }

    fn operation_b(&self) -> Box<dyn State> {
        Box::new(Self)
    }
}

#[derive(Debug)]
pub struct SecondState;
impl State for SecondState {
    fn operation_a(&self) -> Box<dyn State> {
        Box::new(Self)
    }

    fn operation_b(&self) -> Box<dyn State> {
        Box::new(FirstState)
    }
}

pub struct Context {
    state: Box<dyn State>,
}
impl Context {
    pub fn new(state: Box<dyn State>) -> Self {
        println!("Context: {:?}", state);
        Context { state: state }
    }

    pub fn request_a(&mut self) {
        self.state = self.state.operation_a();
        println!("Context: {:?}", self.state);
    }

    pub fn request_b(&mut self) {
        self.state = self.state.operation_b();
        println!("Context: {:?}", self.state);
    }
}

fn main() {
    let mut context = Context::new(Box::new(FirstState));
    context.request_a();
    context.request_b();
}
