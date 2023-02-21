pub trait HasImplementation {
    fn operation(&self);
}

pub struct Abstraction<I: Implementation> {
    implementation: I,
}

impl<I: Implementation> Abstraction<I> {
    pub fn new(implementation: I) -> Self {
        Abstraction {
            implementation: implementation,
        }
    }
}

impl<I: Implementation> HasImplementation for Abstraction<I> {
    fn operation(&self) {
        println!("Abstraction: {}", self.implementation.operation());
    }
}

pub struct RefinedAbstraction<I: Implementation> {
    implementation: I,
}

impl<I: Implementation> RefinedAbstraction<I> {
    pub fn new(implementation: I) -> Self {
        RefinedAbstraction {
            implementation: implementation,
        }
    }
}

impl<I: Implementation> HasImplementation for RefinedAbstraction<I> {
    fn operation(&self) {
        println!("RefinedAbstraction: {}", self.implementation.operation());
    }
}

pub trait Implementation {
    fn operation(&self) -> String;
}

pub struct FirstImplementation;
impl Implementation for FirstImplementation {
    fn operation(&self) -> String {
        String::from("FirstImplementation")
    }
}

pub struct SecondImplementation;
impl Implementation for SecondImplementation {
    fn operation(&self) -> String {
        String::from("SecondImplementation")
    }
}

fn main() {
    let first_implementation = FirstImplementation;
    let abstraction = Abstraction::new(first_implementation);
    abstraction.operation();

    let second_implementation = SecondImplementation;
    let refined_abstraction = RefinedAbstraction::new(second_implementation);
    refined_abstraction.operation();
}
