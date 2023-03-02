pub trait Strategy {
    fn operation(&self, data: &mut Vec<&str>);
}

pub struct FirstStrategy;
impl Strategy for FirstStrategy {
    fn operation(&self, data: &mut Vec<&str>) {
        data.sort();
    }
}

pub struct SecondStrategy;
impl Strategy for SecondStrategy {
    fn operation(&self, data: &mut Vec<&str>) {
        data.sort();
        data.reverse();
    }
}

pub struct Context<'a> {
    strategy: Option<Box<dyn Strategy>>,
    data: Vec<&'a str>,
}

impl Context<'_> {
    pub fn new() -> Self {
        Context {
            strategy: None,
            data: vec!["c", "d", "a", "b", "e"],
        }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = Some(strategy);
    }

    pub fn operation(&mut self) {
        if let Some(strategy) = &self.strategy {
            strategy.operation(&mut self.data);
        }
        println!("Context: {:?}", self.data);
    }
}

fn main() {
    let mut context = Context::new();
    context.operation();

    context.set_strategy(Box::new(FirstStrategy));
    context.operation();

    context.set_strategy(Box::new(SecondStrategy));
    context.operation();
}
