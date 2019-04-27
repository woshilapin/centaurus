use image::Rgba;
use serde::de::value::SeqAccessDeserializer;
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

pub fn deserialize_rgba<'de, D>(deserializer: D) -> Result<Rgba<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(RgbaVisitor)
}

struct RgbaVisitor;

impl<'de> Visitor<'de> for RgbaVisitor {
    type Value = Rgba<u8>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between 0 and 255")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let data: [u8; 4] = Deserialize::deserialize(SeqAccessDeserializer::new(seq)).unwrap();
        if data.len() != 4 {
            Err(Error::invalid_length(4, &"Need 4 values for an Rgba color"))
        } else {
            Ok(Rgba(data))
        }
    }
}
