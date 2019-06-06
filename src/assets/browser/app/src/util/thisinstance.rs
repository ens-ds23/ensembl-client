use std::sync::Mutex;

lazy_static! {
    static ref INSTANCE_ID: Mutex<Option<String>> = Mutex::new(None);
}

pub fn set_instance_id(id: &str) {
    INSTANCE_ID.lock().unwrap().replace(id.to_string());
}

pub fn get_instance_id() -> String {
    INSTANCE_ID.lock().unwrap().as_ref().unwrap_or(&String::new()).to_string()
}
