pub struct Scene {
    dimension: u8,
}

/// Some nice structure
/// ```
/// # use centaurus_core::SceneBuilder;
/// let scene = SceneBuilder::new().dimension(3).build();
/// assert_eq!(scene.render(), 3);
/// ```
pub struct SceneBuilder {
    dimension: u8,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            dimension: 3,
            ..Default::default()
        }
    }

    pub fn with_dimension(&mut self, dimension: u8) -> &mut Self {
        self.dimension = dimension;
        self
    }

    pub fn build(&self) -> Scene {
        Scene {
            dimension: self.dimension,
        }
    }
}

impl Scene {
    pub fn new(dimension: u8) -> Scene {
        Scene { dimension }
    }
}

impl Scene {
    pub fn render(&self) -> u8 {
        self.dimension
    }
}

impl Default for SceneBuilder {
    fn default() -> SceneBuilder {
        SceneBuilder {
            dimension: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_dimension() {
        let scene: Scene = Scene { dimension: 3 };
        assert_eq!(scene.render(), 3);
    }
}
