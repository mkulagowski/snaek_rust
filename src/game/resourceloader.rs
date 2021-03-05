use ggez::{
    graphics::{Font, Image},
    Context,
};

/// Structure for loading and storing all resources need for the game.
///
pub struct ResourceLoader {
    pub bg_image: Image,
    pub food_image: Image,
    pub font: Font,
}

impl ResourceLoader {
    /// Load all resources and create new instance with them.
    ///
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            bg_image: Image::new(ctx, "/grass.png").unwrap(),
            food_image: Image::new(ctx, "/ball.png").unwrap(),
            font: Font::new(ctx, "/Roboto-Black.ttf").unwrap(),
        }
    }
}
