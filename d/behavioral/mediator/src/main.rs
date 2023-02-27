use std::collections::HashMap;

pub trait HasNotify {
    fn notify(&self, event: String);
}

pub struct Mediator {
    components: HashMap<String, Box<dyn Component>>,
}

impl Mediator {
    pub fn new() -> Self {
        Mediator {
            components: HashMap::<String, Box<dyn Component>>::default(),
        }
    }

    pub fn accept(&mut self, component: impl Component + 'static) {
        self.components
            .insert(component.name(), Box::new(component));
    }

    pub fn invoke(&mut self, name: String) {
        let component = self.components.get(&name);
        if let Some(c) = component {
            c.operation(self);
        }
    }
}

impl HasNotify for Mediator {
    fn notify(&self, event: String) {
        if event == "A" {
            print!("Mediator: ");
            let component = self.components.get(&String::from("Second"));
            if let Some(second_component) = component {
                second_component.operation(self);
            }
        }
    }
}

pub trait Component {
    fn name(&self) -> String;
    fn operation(&self, mediator: &Mediator);
}

pub struct FirstComponent {
    name: String,
}

impl Component for FirstComponent {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn operation(&self, mediator: &Mediator) {
        println!("FirstComponent does A.");
        mediator.notify(String::from("A"));
    }
}

impl FirstComponent {
    pub fn new(name: String) -> Self {
        FirstComponent { name }
    }
}

pub struct SecondComponent {
    name: String,
}

impl Component for SecondComponent {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn operation(&self, mediator: &Mediator) {
        println!("SecondComponent does B.");
        mediator.notify(String::from("B"));
    }
}

impl SecondComponent {
    pub fn new(name: String) -> Self {
        SecondComponent { name }
    }
}

fn main() {
    let mut mediator = Mediator::new();
    mediator.accept(FirstComponent::new(String::from("First")));
    mediator.accept(SecondComponent::new(String::from("Second")));

    print!("Client: ");
    mediator.invoke(String::from("First"));
}
