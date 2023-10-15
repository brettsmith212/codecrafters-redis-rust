use std::collections::HashMap;

#[derive(Debug)]
pub struct RedisInternalState {
  key_value_store: HashMap<String, String>,
}

impl RedisInternalState {
  pub fn new() -> Self {
    Self {
      key_value_store: HashMap::new(),
    }
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    self.key_value_store.get(key)
  }

  pub fn set(&mut self, key: &str, value: &str) -> Result<String, anyhow::Error> {
    self.key_value_store.insert(key.to_string(), value.to_string());
    Ok(String::from("SUCCESS"))
  }
}
  