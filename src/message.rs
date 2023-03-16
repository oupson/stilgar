use std::num::ParseFloatError;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Message {
    mac: String,
    temperature: f32,
    humidity: f32,
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
                .to_owned(),
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
}
