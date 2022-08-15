pub struct Database {
    data: Vec<(String, String)>
}

impl Database {
    pub fn new() -> Database {
        Database { data: Vec::new() }
    }

    pub fn insert(&mut self, key: String, value: String) {
        if let Some(index) = self.key_index(&key) {
            self.remove(index);
        }
        self.data.push((key, value));
    }

    pub fn read(&self, key: String) -> Option<String> {
        match self.key_index(&key) {
            Some(index) => {
                let value = self.data.get(index);
                Some(value.unwrap().1.clone())
            }
            None => None
        }
    }

    pub fn delete(&mut self, key: String) -> Option<String> {
        match self.key_index(&key) {
            Some(index) => self.remove(index),
            None => None
        }
    }

    fn remove(&mut self, index: usize) -> Option<String> {
        Some(self.data.remove(index).1)
    }

    fn key_index(&self, key: &String) -> Option<usize> {
        match &self.data.iter().position(|t| &t.0 == key ) {
            Some(index) => Some(index.clone()),
            None => None
        }
    }
}

#[test]
fn it_can_insert_read_and_delete_data() {
    let mut database = Database::new();
    database.insert("key".to_string(), "hello there".to_string());
    assert_eq!(Some("hello there".to_string()), database.read("key".to_string()));
    assert_eq!(Some("hello there".to_string()), database.delete("key".to_string()));
    assert_eq!(None, database.read("key".to_string()));
}

#[test]
fn it_will_overwrite_keys() {
    let mut database = Database::new();
    database.insert("key".to_string(), "hello there".to_string());
    assert_eq!(Some("hello there".to_string()), database.read("key".to_string()));
    database.insert("key".to_string(), "goodbye".to_string());
    assert_eq!(Some("goodbye".to_string()), database.read("key".to_string()));
}
