pub trait Component {
    fn operation(&self) -> String;
}

pub struct FirstComponent;
impl Component for FirstComponent {
    fn operation(&self) -> String {
        String::from("FirstComponent")
    }
}

pub trait Decorator: Component {
    fn new(component: Box<dyn Component>) -> Self;
}

pub struct FirstDecorator {
    component: Box<dyn Component>,
}

impl Component for FirstDecorator {
    fn operation(&self) -> String {
        format!("FirstDecorator({})", self.component.operation())
    }
}

impl Decorator for FirstDecorator {
    fn new(component: Box<dyn Component>) -> Self {
        FirstDecorator {
            component: component,
        }
    }
}

pub struct SecondDecorator {
    component: Box<dyn Component>,
}

impl Component for SecondDecorator {
    fn operation(&self) -> String {
        format!("SecondDecorator({})", self.component.operation())
    }
}

impl Decorator for SecondDecorator {
    fn new(component: Box<dyn Component>) -> Self {
        SecondDecorator {
            component: component,
        }
    }
}

fn main() {
    let component = FirstComponent;
    println!("{}", component.operation());

    let first_decorator = FirstDecorator::new(Box::new(component));
    let second_decorator = SecondDecorator::new(Box::new(first_decorator));
    println!("{}", second_decorator.operation());
}
