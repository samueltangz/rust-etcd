use crate::proto::kv;

#[derive(Debug)]
pub struct KeyValue {
    key_value: kv::KeyValue,
}

impl KeyValue {
    pub fn key(&self) -> &[u8] {
        self.key_value.get_key()
    }

    pub fn value(&self) -> &[u8] {
        self.key_value.get_value()
    }

    pub fn version(&self) -> i64 {
        self.key_value.get_version()
    }

    pub fn create_revision(&self) -> i64 {
        self.key_value.get_create_revision()
    }

    pub fn mod_revision(&self) -> i64 {
        self.key_value.get_mod_revision()
    }

    pub fn lease(&self) -> i64 {
        self.key_value.get_lease()
    }
}

impl From<kv::KeyValue> for KeyValue {
    fn from(kv: kv::KeyValue) -> Self {
        Self { key_value: kv }
    }
}
