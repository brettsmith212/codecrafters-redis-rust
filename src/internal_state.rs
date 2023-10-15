use std::{collections::HashMap, time::SystemTime};

#[derive(Debug, Clone)]
pub struct RedisStoredValue {
    expiration: Option<SystemTime>,
    value: String,
}

impl RedisStoredValue {
    pub fn new(value:String, expiration: Option<SystemTime>) -> RedisStoredValue {
      RedisStoredValue {
        expiration,
        value,
      }
    }
  
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn expiration(&self) -> Option<&SystemTime> {
        self.expiration.as_ref()
    }
}

#[derive(Debug)]
pub struct RedisInternalState {
    key_value_store: HashMap<String, RedisStoredValue>,
}

impl RedisInternalState {
    pub fn new() -> Self {
        Self {
            key_value_store: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&RedisStoredValue> {
        if let Some(stored_value) = self.key_value_store.get(key) {
            if let Some(expiration) = stored_value.expiration() {
                println!("expiration: {:?}", expiration);
                println!("current time: {:?}", SystemTime::now());
                if expiration < &SystemTime::now() {
                    return None;
                }
            }
            return Some(stored_value);
        }

        return None;
    }

    pub fn set(&mut self, key: &str, value: &RedisStoredValue) -> Result<String, anyhow::Error> {
        self.key_value_store
            .insert(key.to_string(), value.clone()); 
        Ok(String::from("OK"))
    }
}
