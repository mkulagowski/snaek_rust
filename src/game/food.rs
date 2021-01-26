use crate::game::consts;
use crate::game::coords::Coords;
use ggez::graphics::Rect;

pub struct Food {
    pub bbox: Rect,
}

impl Food {
    pub fn random() -> Self {
        let pos = Coords::random(consts::FOOD_SIZE, consts::SCREEN_SIZE.x - consts::FOOD_SIZE);
        Self {
            bbox: Rect::new(
                pos.x - consts::FOOD_HALF_SIZE,
                pos.y - consts::FOOD_HALF_SIZE,
                consts::FOOD_SIZE,
                consts::FOOD_SIZE,
            ),
        }
    }
}
