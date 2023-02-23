use std::collections::HashMap;

#[derive(Clone)]
pub struct Context {
    pub name: String,
}
impl Context {
    pub fn new(name: &str) -> Self {
        Context {
            name: name.to_string(),
        }
    }
}

pub struct Flyweight {
    context: Context,
}
impl Flyweight {
    pub fn new(context: Context) -> Self {
        Flyweight { context: context }
    }

    pub fn operation(&self, unique: Context) {
        println!(
            "Flyweight shared: {}\nFlyweight unique: {}",
            self.context.name, unique.name
        )
    }
}

pub struct FlyweightFactory {
    flyweights: HashMap<String, Flyweight>,
}
impl FlyweightFactory {
    pub fn new(contexts: Vec<Context>) -> Self {
        let mut map = HashMap::<String, Flyweight>::new();
        for i in 0..contexts.len() {
            map.insert(
                contexts[i].name.clone(),
                Flyweight::new(contexts[i].clone()),
            );
        }
        FlyweightFactory { flyweights: map }
    }

    pub fn get_flyweight(&mut self, context: Context) -> &Flyweight {
        let name = context.name.clone();
        if !self.flyweights.contains_key(&context.name) {
            self.flyweights
                .insert(name.clone(), Flyweight::new(context));
        }
        self.flyweights.get(&name).unwrap()
    }
}

fn main() {
    let mut factory = FlyweightFactory::new(vec![
        Context::new("A"),
        Context::new("B"),
        Context::new("C"),
    ]);
    let flyweight = factory.get_flyweight(Context::new("A"));
    flyweight.operation(Context::new("D"));
}
