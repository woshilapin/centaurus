
use nalgebra::{Point3, Vector3};
use serde::de::{Deserializer, Error, SeqAccess, Visitor};
use serde_derive::Deserialize;
use std::fmt;

struct Vector3Visitor;

impl<'de> Visitor<'de> for Vector3Visitor {
    type Value = Vector3<f64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a list of 3 values")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Vector3<f64>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let x = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;
        let y = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(1, &self))?;
        let z = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(2, &self))?;
        Ok(Vector3::new(x, y, z))
    }
}

fn deserialize_vector3<'de, D>(deserializer: D) -> Result<Vector3<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    const FIELDS: &'static [&'static str] = &["data"];
    deserializer.deserialize_struct("Vector3", FIELDS, Vector3Visitor)
}

struct Point3Visitor;

impl<'de> Visitor<'de> for Point3Visitor {
    type Value = Point3<f64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a list of 3 values")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Point3<f64>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let x = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(0, &self))?;
        let y = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(1, &self))?;
        let z = seq
            .next_element()?
            .ok_or_else(|| Error::invalid_length(2, &self))?;
        Ok(Point3::new(x, y, z))
    }
}

fn deserialize_point3<'de, D>(deserializer: D) -> Result<Point3<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    const FIELDS: &'static [&'static str] = &["data"];
    deserializer.deserialize_struct("Point3", FIELDS, Point3Visitor)
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Camera {
    #[serde(deserialize_with = "deserialize_point3")]
    pub position: Point3<f64>,
    #[serde(deserialize_with = "deserialize_vector3")]
    pub direction: Vector3<f64>,
    pub focal_length: f64,
    pub upper_bound: f64,
    pub lower_bound: f64,
    pub left_bound: f64,
    pub right_bound: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Point3::new(0f64, 0f64, -1f64),
            direction: Vector3::new(0f64, 0f64, 1f64),
            focal_length: 1f64,
            upper_bound: 0.5,
            lower_bound: -0.5,
            left_bound: -0.5,
            right_bound: 0.5,
        }
    }
}

impl Camera {
    pub fn new(
        position: Point3<f64>,
        direction: Vector3<f64>,
        focal_length: f64,
        bounds: [f64; 4],
    ) -> Camera {
        Camera {
            position,
            direction,
            focal_length,
            upper_bound: bounds[0],
            lower_bound: bounds[1],
            left_bound: bounds[2],
            right_bound: bounds[3],
        }
    }
}
