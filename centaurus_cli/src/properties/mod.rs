use centaurus_core::Scene;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Properties {
    pub scene: Scene,
}
