/*  
    @Author Julien LE THENO
    @mod Data Manager : centralizes the data around the application by giving API to store and get data around
*/

use std::collections::HashMap;

pub struct DataStore {
    data: HashMap<String, String>,
    subscribers : HashMap<String, Vec<Box<dyn Fn(&str) + Send>>>,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            data: HashMap::new(),
            subscribers: HashMap::new(),
        }
    }

    pub fn write(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_owned(), value.to_owned());
        if let Some(subscribers) = self.subscribers.get(key) {
            for subscriber in subscribers {
                subscriber(value);
            }
        }
    }

    pub fn read(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub  fn subscribe<F> (&mut self, key: &str, callback: F)
    where 
        F: Fn(&str) + Send + 'static,
    {
        if let Some(subscribers) = self.subscribers.get_mut(key) {
            subscribers.push(Box::new(callback));
        } else {
            self.subscribers.insert(key.to_owned(), vec![Box::new(callback)]);
        }
    }
}