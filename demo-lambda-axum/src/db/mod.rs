use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn read_entries() -> HashMap<String, String> {
    DB.lock().unwrap().clone()
}

pub fn update_entry(key: &String, value: &String) {
    DB.lock().unwrap().insert(key.clone(), value.clone());
}

pub fn delete_entry(key: &String) {
    DB.lock().unwrap().remove(key);
}
