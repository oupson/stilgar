use serde::{ser::SerializeStruct, Serialize, Serializer};
use sqlx::types::mac_address::MacAddress;

pub(crate) struct Sensor {
    mac: MacAddress,
    name: Option<String>,
}

impl Sensor {
    pub(crate) fn new(mac: MacAddress, name: Option<String>) -> Self {
        Self { mac, name }
    }
}

impl Serialize for Sensor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Sensor", 2)?;
        state.serialize_field("mac", &self.mac.to_string())?;
        state.serialize_field("name", &self.name)?;

        state.end()
    }
}
