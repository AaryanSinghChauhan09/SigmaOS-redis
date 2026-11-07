//! redis model — in-memory key-value store.

#[derive(Debug, Clone)]
pub struct Key {
    pub name: String,
    pub type_: String,
    pub value: String,
    pub ttl: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub keys: Vec<Key>,
}

impl Default for Model {
    fn default() -> Self { Self { keys: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, name: &str, type_: &str, value: &str, ttl: Option<u64>) {
        if let Some(k) = self.keys.iter_mut().find(|k| k.name == name) {
            k.type_ = type_.into();
            k.value = value.into();
            k.ttl = ttl;
        } else {
            self.keys.push(Key { name: name.into(), type_: type_.into(), value: value.into(), ttl });
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.keys.iter().find(|k| k.name == name).map(|k| k.value.as_str())
    }

    pub fn del(&mut self, name: &str) -> bool {
        if let Some(pos) = self.keys.iter().position(|k| k.name == name) { self.keys.remove(pos); true } else { false }
    }

    pub fn keys(&self) -> Vec<&str> {
        self.keys.iter().map(|k| k.name.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_get_del() {
        let mut r = Model::new();
        r.set("user:1", "string", "alice", Some(3600));
        assert_eq!(r.get("user:1"), Some("alice"));
        assert!(r.del("user:1"));
        assert!(r.get("user:1").is_none());
    }
}
