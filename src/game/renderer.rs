use crate::game::{consts, food::Food, turn::TurnType};
use crate::game::{coords::Coords, snake::Snake};
use ggez::{
    graphics::{self, Color, FillOptions, Mesh, MeshBuilder, Text},
    Context, GameError,
};
use graphics::Image;
use itertools as it;

/// Helper struct for various drawing functions.
/// It helps to draw each type of object in a proper manner.
pub struct Renderer {}

impl Renderer {
    /// Draws a tiled background. Given image is scaled down to 50%
    /// and tiled as needed, depending on the screen size.
    ///
    pub fn draw_bg(ctx: &mut Context, img: &Image) {
        let scale = 0.5;
        let (x_step, y_step) = {
            let dims = img.dimensions();
            ((dims.w * scale) as usize, (dims.h * scale) as usize)
        };
        (0..consts::SCREEN_SIZE.y as i32)
            .step_by(x_step)
            .for_each(|yy| {
                (0..consts::SCREEN_SIZE.x as i32)
                    .step_by(y_step)
                    .map(|xx| (xx as f32, yy as f32))
                    .for_each(|(x, y)| {
                        graphics::draw(
                            ctx,
                            img,
                            graphics::DrawParam::new()
                                .dest(Coords { x, y })
                                .scale([scale, scale]),
                        )
                        .expect("Error while drawing background");
                    })
            });
    }

    /// Draws a sprite on the position from the `Food` instance
    ///
    pub fn draw_food(ctx: &mut Context, food: &Food, img: &Image) {
        let scalex = consts::FOOD_SIZE / img.dimensions().w;
        let scaley = consts::FOOD_SIZE / img.dimensions().h;
        graphics::draw(
            ctx,
            img,
            graphics::DrawParam::new()
                .dest(Coords::new(food.bbox.x, food.bbox.y))
                .scale([scalex, scaley]),
        )
        .expect("Error while drawing Food");

        #[cfg(feature = "debug")]
        {
            let mesh = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(1.),
                food.bbox,
                Color::from_rgb(255, 0, 0),
            )
            .unwrap();
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())
                .expect("Error while drawing Food border");
        }
    }

    /// Draws whole `LineSnake` structure
    ///
    pub fn draw_snake(ctx: &mut Context, snake: &Snake) {
        for segment in &snake.body {
            segment.draw(ctx);
        }
    }

    /// Draws given text in a white color with a black outline
    ///
    /// # Parameters
    ///
    /// - `ctx`: game context
    /// - `txt`: the text itself
    /// - `pos`: position of the top left corner of the text
    ///
    pub fn draw_text_with_outline(ctx: &mut Context, txt: &Text, pos: Coords) {
        const WIDTH: f32 = 2.;
        [-WIDTH, 0., WIDTH].iter().for_each(|x| {
            [-WIDTH, 0., WIDTH].iter().for_each(|y| {
                let params = graphics::DrawParam::default()
                    .dest(Coords {
                        x: pos.x + x,
                        y: pos.y + y,
                    })
                    .color(graphics::BLACK);
                graphics::draw(ctx, txt, params).expect("Error while drawing score");
            })
        });

        let params = graphics::DrawParam::default().dest(pos);
        graphics::draw(ctx, txt, params.color(graphics::WHITE)).expect("Error while drawing score");
    }

    /// Draws given text in a black color
    ///
    /// # Parameters
    ///
    /// - `ctx`: game context
    /// - `txt`: the text itself
    /// - `pos`: position of the top left corner of the text
    ///
    pub fn _draw_text(ctx: &mut Context, txt: &Text, pos: Coords) {
        let params = graphics::DrawParam::default().dest(pos);
        graphics::draw(ctx, txt, params.color(graphics::BLACK)).expect("Error while drawing score");
    }

    fn get_arc(mid: Coords, r: f32, from_angle: f32, to_angle: f32, step: f32) -> Vec<Coords> {
        let from_angle = from_angle.to_radians();
        let to_angle = to_angle.to_radians();
        let step = step.to_radians();

        let arc_points = it::iterate(from_angle, |v| v + step)
            .take_while(|&v| v <= to_angle)
            .map(|v| {
                let (sin, cos) = v.sin_cos();
                Coords::new(mid.x + cos * r, mid.y + sin * r)
            });

        let mut points = vec![mid];
        points.extend(arc_points);

        points
    }

    /// Creates mesh of a quater of the ring.
    /// How much part is drawn and the starting edge can be chosen via params.
    ///
    /// # Parameters
    ///
    /// - `ctx`: game context
    /// - `pos`: position where middle of the mesh should be
    /// - `r1`: radius of the outer edge of the ring, must be > 0
    /// - `r2`: radius of the inner edge of the ring, must be >= 0
    /// - `turn`: determines which quater is created
    /// - `progress`: how much of a quater should be created, between 0 and 1
    /// - `reversed`: determines from which end the progress is
    /// - `is_head`: whether the segment drawn is a head and we want to draw eyes
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    ///
    /// - `Ok`: A `Mesh` that is ready to be drawn
    /// - `Err`: Something went wrong I guess...or You gave a negative radius
    ///
    pub fn create_qt_ring(
        ctx: &mut Context,
        pos: Coords,
        r1: f32,
        r2: f32,
        turn: TurnType,
        progress: f32,
        reversed: bool,
        is_head: bool,
    ) -> Result<Mesh, GameError> {
        if r1 <= 0. || r2 < 0. {
            return Result::Err(GameError::ConfigError(
                "Radiuses cannot be negative!".to_string(),
            ));
        }
        let (from, to) = turn.get_arc_bounds();
        let pos = pos + r1 * Self::get_arc_translation(turn);

        let (from, to) = if reversed {
            (to - 90. * progress, to)
        } else {
            (from, from + 90. * progress)
        };

        let outers = Self::get_arc(pos, r1, from, to, 1.);
        let inners = Self::get_arc(pos, r2, from, to, 1.);
        let polys = it::chain(outers, it::rev(inners)).collect();

        if is_head {
            create_head(polys, ctx)
        } else {
            create_body(polys, ctx)
        }
    }

    fn get_arc_translation(quarter: TurnType) -> Coords {
        match quarter {
            TurnType::DownRight => Coords::new(0.5, -0.5),
            TurnType::DownLeft => Coords::new(-0.5, -0.5),
            TurnType::UpLeft => Coords::new(-0.5, 0.5),
            TurnType::UpRight => Coords::new(0.5, 0.5),
        }
    }
}

fn create_body(polys: Vec<Coords>, ctx: &mut Context) -> Result<Mesh, ggez::GameError> {
    MeshBuilder::new()
        .polygon(
            graphics::DrawMode::Fill(FillOptions::default()),
            &polys,
            Color::from_rgb(255, 255, 0),
        )
        .unwrap()
        .build(ctx)
}

fn create_head(polys: Vec<Coords>, ctx: &mut Context) -> Result<Mesh, ggez::GameError> {
    let pt1 = polys.first().unwrap();
    let pt2 = polys.last().unwrap();

    let vect = *pt2 - *pt1;
    let len = (vect.x.powi(2) + vect.y.powi(2)).sqrt();
    let norm = Coords::new(vect.x / len, vect.y / len);
    let ppd = Coords::new(norm.y, -norm.x);
    let eye1 = *pt1 + (vect * 0.25 * len) + (ppd * 0.25 * len);
    let eye2 = *pt1 + (vect * 0.75 * len) + (ppd * 0.25 * len);

    MeshBuilder::new()
        .polygon(
            graphics::DrawMode::Fill(FillOptions::default()),
            &polys,
            Color::from_rgb(255, 255, 0),
        )
        .unwrap()
        .circle(
            graphics::DrawMode::Fill(FillOptions::default()),
            eye1,
            10.,
            1.,
            graphics::BLACK,
        )
        .circle(
            graphics::DrawMode::Fill(FillOptions::default()),
            eye2,
            10.,
            1.,
            graphics::BLACK,
        )
        .build(ctx)
}
