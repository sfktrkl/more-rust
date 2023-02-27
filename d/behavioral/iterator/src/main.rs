pub struct Collection {
    users: Vec<String>,
}

impl Collection {
    pub fn new(users: Vec<String>) -> Self {
        Collection { users: users }
    }

    pub fn iter(&self) -> CollectionIterator {
        CollectionIterator {
            index: 0,
            user_collection: self,
        }
    }
}

pub struct CollectionIterator<'a> {
    index: usize,
    user_collection: &'a Collection,
}

impl Iterator for CollectionIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.user_collection.users.len() {
            let user = Some(self.user_collection.users[self.index].clone());
            self.index += 1;
            return user;
        }

        None
    }
}

fn main() {
    let users = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Carl"),
    ];
    let collection = Collection::new(users);
    print!("All elements in collection: ");
    collection.iter().for_each(|e| print!("{} ", e));
    println!("");
}
