use centaurus::Scene;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Properties {
    pub scene: Scene,
}
