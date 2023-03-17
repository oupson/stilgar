use std::{borrow::Cow, num::ParseFloatError};

use serde::{ser::SerializeStruct, Serialize, Serializer};
use sqlx::types::mac_address::MacAddress;

#[derive(Debug, Clone)]
pub(crate) struct Message {
    mac: MacAddress,
    temperature: f32,
    humidity: f32,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Message", 3)?;
        state.serialize_field("mac", &self.mac.to_string())?;
        state.serialize_field("temperature", &self.temperature)?;
        state.serialize_field("humidity", &self.humidity)?;

        state.end()
    }
}

impl Message {
    pub(crate) fn mac(&self) -> &MacAddress {
        &self.mac
    }

    pub(crate) fn temperature(&self) -> f32 {
        self.temperature
    }

    pub(crate) fn humidity(&self) -> f32 {
        self.humidity
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let msg = String::from_utf8_lossy(buf);

        let mut splitted = msg.split(';');

        let msg = Message {
            mac: splitted
                .next()
                .ok_or_else(|| ParseError::MissingField(0))?
                .try_into()
                .map_err(|_| ParseError::MacParseError())?,
            temperature: splitted
                .next()
                .ok_or_else(|| ParseError::MissingField(1))?
                .parse::<f32>()?,
            humidity: splitted
                .next()
                .ok_or_else(|| ParseError::MissingField(2))?
                .parse::<f32>()?,
        };

        Ok(msg)
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ParseError {
    #[error("missing field in message : index {0}")]
    MissingField(usize),
    #[error(transparent)]
    NumberParseError(#[from] ParseFloatError),
    #[error("failed to parse mac adresse")]
    MacParseError(),
}
