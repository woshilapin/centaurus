#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[derive(Debug, Default, Copy, Clone)]
pub struct Scene {
    width: u32,
    height: u32,
    dimension: u8,
}

/// Some nice structure
/// ```
/// # use centaurus_core::SceneBuilder;
/// let scene = SceneBuilder::new().with_dimension(3).build();
/// assert_eq!(scene.render(), 3);
/// ```
#[derive(Default)]
pub struct SceneBuilder {
    scene: Scene,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            scene: Scene {
                width: 600,
                height: 400,
                dimension: 3,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn with_width(&mut self, width: u32) -> &mut Self {
        self.scene.width = width;
        self
    }

    pub fn with_height(&mut self, height: u32) -> &mut Self {
        self.scene.height = height;
        self
    }

    pub fn with_dimension(&mut self, dimension: u8) -> &mut Self {
        self.scene.dimension = dimension;
        self
    }

    pub fn build(&self) -> Scene {
        self.scene.clone()
    }
}

impl Scene {
    pub fn render(&self) -> u8 {
        self.dimension
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_dimension() {
        let scene: Scene = SceneBuilder::new().build();
        assert_eq!(scene.render(), 3);
    }
}
