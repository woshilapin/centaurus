use serde_derive::Deserialize;
use centaurus::Scene;

#[derive(Deserialize)]
pub struct Properties {
    pub scene: Scene,
}