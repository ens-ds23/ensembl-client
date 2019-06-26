use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct ZMenuData {
    template: Option<String>,
    data: HashMap<String,String>
}
    
impl ZMenuData {
    pub fn new() -> ZMenuData {
        ZMenuData {
            template: None,
            data: HashMap::new()
        }
    }
    
    pub fn set_template(&mut self, template: &str) {
        self.template = Some(template.to_string());
    }
    
    pub fn get_template(&self) -> &Option<String> { &self.template }
    
    pub fn set_value(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
    
    pub fn get_values(&self) -> &HashMap<String,String> {
        &self.data
    }
}
