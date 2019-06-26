use std::sync::Mutex;

use dom::domutil::browser_time;

lazy_static! {
    static ref INSTANCE_ID: Mutex<Option<String>> = Mutex::new(None);
}

fn generate_instance_id() -> String {
    let inst_bytes = (browser_time() as i64).to_be_bytes();
    let mut inst_id = base64::encode_config(&inst_bytes,base64::URL_SAFE_NO_PAD);
    let len = inst_id.len();
    inst_id.split_off(len-6)
}

pub fn set_instance_id() {
    let id = generate_instance_id();
    INSTANCE_ID.lock().unwrap().replace(id.to_string());
}

pub fn get_instance_id() -> String {
    INSTANCE_ID.lock().unwrap().as_ref().unwrap_or(&String::new()).to_string()
}
