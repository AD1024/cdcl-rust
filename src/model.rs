use std::collections::HashMap;

pub struct Model {
    var_data_map : HashMap<String, bool>
}

impl Model {
    pub fn new() -> Self {
        Model { var_data_map : HashMap::new() }
    }

    pub fn get_value(&self, name : &str) -> Option<&bool> {
        self.var_data_map.get(name)
    }
}