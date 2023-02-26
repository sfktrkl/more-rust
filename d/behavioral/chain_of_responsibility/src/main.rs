pub trait Handler {
    fn next(&self) -> &Option<Box<dyn Handler>>;
    fn handle(&self, request: String) -> String;
}

pub struct FirstHandler {
    handler: Option<Box<dyn Handler>>,
}
impl FirstHandler {
    pub fn new(handler: Option<Box<dyn Handler>>) -> Self {
        FirstHandler { handler: handler }
    }
}
impl Handler for FirstHandler {
    fn next(&self) -> &Option<Box<dyn Handler>> {
        &self.handler
    }

    fn handle(&self, request: String) -> String {
        if request.contains(&String::from("First")) {
            String::from("FirstHandler")
        } else {
            let r = request + &String::from("FirstHandler > ");
            match &self.handler {
                Some(handler) => handler.handle(r),
                _ => r,
            }
        }
    }
}

pub struct SecondHandler {
    handler: Option<Box<dyn Handler>>,
}
impl SecondHandler {
    pub fn new(handler: Option<Box<dyn Handler>>) -> Self {
        SecondHandler { handler: handler }
    }
}
impl Handler for SecondHandler {
    fn next(&self) -> &Option<Box<dyn Handler>> {
        &self.handler
    }

    fn handle(&self, request: String) -> String {
        if request.contains(&String::from("Second")) {
            String::from("SecondHandler")
        } else {
            let r = request + &String::from("SecondHandler > ");
            match &self.handler {
                Some(handler) => handler.handle(r),
                _ => r,
            }
        }
    }
}

pub struct ThirdHandler {
    handler: Option<Box<dyn Handler>>,
}
impl ThirdHandler {
    pub fn new(handler: Option<Box<dyn Handler>>) -> Self {
        ThirdHandler { handler: handler }
    }
}
impl Handler for ThirdHandler {
    fn next(&self) -> &Option<Box<dyn Handler>> {
        &self.handler
    }

    fn handle(&self, request: String) -> String {
        if request.contains(&String::from("Third")) {
            String::from("ThirdHandler")
        } else {
            let r = request + &String::from("ThirdHandler");
            match &self.handler {
                Some(handler) => handler.handle(r),
                _ => r,
            }
        }
    }
}

fn main() {
    let third = ThirdHandler::new(None);
    let second = SecondHandler::new(Some(Box::new(third)));
    let first = FirstHandler::new(Some(Box::new(second)));
    println!("{}", first.handle(String::from("Chain: ")));
    match &first.next() {
        Some(handler) => println!("{}", handler.handle(String::from("Subchain: "))),
        _ => (),
    };
}
