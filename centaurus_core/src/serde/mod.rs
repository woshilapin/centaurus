use image::Rgb;
use serde::de::value::SeqAccessDeserializer;
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

pub fn deserialize_rgb<'de, D>(deserializer: D) -> Result<Rgb<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(RgbVisitor)
}

struct RgbVisitor;

impl<'de> Visitor<'de> for RgbVisitor {
    type Value = Rgb<f64>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a floating number between 0.0 and 1.1")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        Deserialize::deserialize(SeqAccessDeserializer::new(seq))
            .map(Rgb)
            .map_err(|_| Error::invalid_length(3, &"Need Red, Green and Blue colors"))
    }
}
