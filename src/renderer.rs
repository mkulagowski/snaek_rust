use crate::{consts, direction::Direction, food::Food, line::Line, turn::TurnType};
use crate::{coords::Coords, LineSnake};
use ggez::{
    graphics::{self, Color, FillOptions, Mesh, MeshBuilder, Text},
    Context,
};
use graphics::Image;

pub struct Renderer {}

impl Renderer {
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
    }

    pub fn _draw_snake_no_turns(ctx: &mut Context, snake: &LineSnake) {
        for seg in &snake.body {
            let mesh = seg.create_mesh(ctx, Color::from_rgb(255, 255, 0));
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())
                .expect("Error while drawing Line");
        }
    }

    fn draw_line(ctx: &mut Context, line: &Line) {
        let mesh = line.create_mesh(ctx, Color::from_rgb(255, 255, 0));
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
            .expect("Error while drawing Line");
    }

    pub fn draw_snake(ctx: &mut Context, snake: &LineSnake) {
        for seg in snake.body.iter() {
            let mut line = seg.clone();
            if let Some(turn) = seg.turn {
                let turn_progress = turn.percentage;
                if turn_progress > 0. {
                    line.shrink(turn_progress * consts::SNAKE_WIDTH);
                    Self::draw_turn(ctx, seg);
                }
            }
            Self::draw_line(ctx, &line);
        }
    }

    fn draw_turn(ctx: &mut Context, line: &Line) {
        let d1 = line.dir;
        let turn = line.turn.unwrap();
        let (margin, is_reversed) = match turn.turn_type {
            TurnType::DownRight => (Coords { x: 1., y: -1. }, d1 == Direction::UP),
            TurnType::DownLeft => (Coords { x: -1., y: -1. }, d1 == Direction::LEFT),
            TurnType::UpLeft => (Coords { x: -1., y: 1. }, d1 == Direction::DOWN),
            TurnType::UpRight => (Coords { x: 1., y: 1. }, d1 == Direction::RIGHT),
        };

        let pos = turn.pos
            + line.dir.as_coords() * consts::SNAKE_HALF_WIDTH
            + margin * consts::HALF_TURN_MARGIN;
        if let Some(mesh) = create_qt_ring(
            ctx,
            &pos,
            consts::SNAKE_WIDTH + consts::TURN_MARGIN,
            consts::TURN_MARGIN,
            turn.turn_type,
            turn.percentage,
            is_reversed ^ turn.is_growing,
            true,
        ) {
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())
                .expect("Error while drawing Turn");
        }
    }

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

    pub fn _draw_text(ctx: &mut Context, txt: &Text, pos: Coords) {
        let params = graphics::DrawParam::default().dest(pos);
        graphics::draw(ctx, txt, params.color(graphics::BLACK)).expect("Error while drawing score");
    }
}

fn get_arc(mid: Coords, r: f32, from_angle: f32, to_angle: f32, step: f32) -> Vec<Coords> {
    let from_angle = from_angle.to_radians();
    let to_angle = to_angle.to_radians();
    let step = step.to_radians();
    let mut points = vec![mid];
    let mut i = from_angle;
    while i <= to_angle {
        let (sini, cosi) = i.sin_cos();
        points.push(Coords::new(mid.x + cosi * r, mid.y + sini * r));
        i += step;
    }

    points
}

fn create_qt_ring(
    ctx: &mut Context,
    pos: &Coords,
    r1: f32,
    r2: f32,
    turn: TurnType,
    progress: f32,
    reversed: bool,
    is_head: bool,
) -> Option<Mesh> {
    let (from, to) = turn.get_arc_bounds();
    let pos = pos.to_owned() + r1 * get_arc_translation(turn);

    let (from, to) = if reversed {
        (to - 90. * progress, to)
    } else {
        (from, from + 90. * progress)
    };

    let outers = get_arc(pos, r1, from, to, 1.);
    let inners = get_arc(pos, r2, from, to, 1.);
    let polys: Vec<Coords> = outers.into_iter().chain(inners.into_iter().rev()).collect();
    let result_mesh = if is_head {
        let pt1 = polys.first().unwrap();
        let pt2 = polys.last().unwrap();

        let vect = *pt2 - *pt1;
        let len = (vect.x.powi(2) + vect.y.powi(2)).sqrt();
        let norm = { Coords::new(vect.x / len, vect.y / len) };
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
    } else {
        MeshBuilder::new()
            .polygon(
                graphics::DrawMode::Fill(FillOptions::default()),
                &polys,
                Color::from_rgb(255, 255, 0),
            )
            .unwrap()
            .build(ctx)
    };

    // let result_mesh = result_mesh
    //     //MeshBuilder::new()
    //     // .polygon(
    //     //     graphics::DrawMode::Fill(FillOptions::default()),
    //     //     &polys,
    //     //     Color::from_rgb(255, 255, 0),
    //     // )
    //     // .unwrap()
    //     .build(ctx);

    if let Ok(mesh) = result_mesh {
        return Some(mesh);
    }
    None
}

fn get_arc_translation(quarter: TurnType) -> Coords {
    match quarter {
        TurnType::DownRight => Coords::new(0.5, -0.5),
        TurnType::DownLeft => Coords::new(-0.5, -0.5),
        TurnType::UpLeft => Coords::new(-0.5, 0.5),
        TurnType::UpRight => Coords::new(0.5, 0.5),
    }
}
