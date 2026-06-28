use std::collections::HashMap;

pub struct Database {
    name: String,
    data: HashMap<String, String>, // key-value pair
}

impl Database {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        // Take a key and a value, and insert them into the HashMap
        self.data.insert(key, value);

        /*
        self is mutable here because we want to modify
        the already existing HashMap.
        */
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        // Takes a string slice for the key and returns the value
        // if it exists
        self.data.get(key) // expression

        /*
        Return an Option because the key might not exist, so we
        match it. Also, it is a reference to the String because we
        want to look at the data, not remove it from the database.
        */
    }

    pub fn delete(&mut self, key: &str) -> bool {
        // Removes the key-value pair from the HashMap
        self.data.remove(key).is_some()
        // is_some() returns true if it contains a value
        // and false if it is None

        /*
        Returns true if a key was removed.
        Returns false if the key did not exist in the first place.
        */
    }

    // Testing method to check current Database contents
    pub fn print_all(&self) {
        for (key, value) in &self.data {
            println!("{}: {}", key, value);
        }
    }
}
