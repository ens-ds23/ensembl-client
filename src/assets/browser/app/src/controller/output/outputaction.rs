use serde_json::Value as JSONValue;

use controller::global::AppRunner;
use dom::domutil;

#[derive(Clone)]
pub enum OutputAction {
    SendCustomEvent(String,JSONValue),
    Destroy
}

impl OutputAction {
    pub fn run(&self, ar: &mut AppRunner) {
        match self {
            OutputAction::SendCustomEvent(name,details) => {
                domutil::send_custom_event(&ar.get_browser_el().into(),name,details);
            },
            OutputAction::Destroy => {
                ar.destroy()
            }
        }
    }
}
