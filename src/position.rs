use super::constants;
use super::direction;

use ggez::{graphics, Context};
use rand::distributions::{IndependentSample, Range};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: constants::HALF_WINDOW_WIDTH,
            y: constants::HALF_WINDOW_HEIGHT,
        }
    }

    pub fn shunt(&mut self, direction: direction::Direction) {
        match direction {
            direction::Direction::Up => {
                if self.y > 0 {
                    self.y -= constants::CELL_LENGTH;
                } else {
                    self.y = constants::WINDOW_HEIGHT;
                }
            }
            direction::Direction::Down => {
                if self.y < constants::WINDOW_HEIGHT {
                    self.y += constants::CELL_LENGTH;
                } else {
                    self.y = 0;
                }
            }
            direction::Direction::Left => {
                if self.x > 0 {
                    self.x -= constants::CELL_LENGTH;
                } else {
                    self.x = constants::WINDOW_WIDTH;
                }
            }
            direction::Direction::Right => {
                if self.x < constants::WINDOW_WIDTH {
                    self.x += constants::CELL_LENGTH;
                } else {
                    self.x = 0;
                }
            }
        }
    }

    pub fn shunt_towards(&mut self, other: &Position) {
        let direction = self.random_direction_towards(other);
        self.shunt(direction);
    }

    fn random_direction_towards(&self, other: &Position) -> direction::Direction {
        let impossible_directions = match (self.x - other.x, self.y - other.y) {
            // 0 self.......  12 other -> x is negative..so x+=1
            // 0 other ........ 12 self -> x i positive.. so x-= 1
            (x, _y) if x > 0 => direction::Direction::Right,
            (x, _y) if x <= 0 => direction::Direction::Left,
            (_x, y) if y > 0 => direction::Direction::Down,
            (_x, y) if y <= 0 => direction::Direction::Up,
            _ => direction::Direction::Up,
        };
        let possible_positions = impossible_directions.all_except();

        let mut rng = rand::thread_rng();

        let rand_direction_range = Range::new(0, 3);
        let rand_direction_idx = rand_direction_range.ind_sample(&mut rng);
        possible_positions[rand_direction_idx]
    }

    pub fn draw(&self, ctx: &mut Context, colour: graphics::Color) {
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.x as f32,
                self.y as f32,
                constants::CELL_LENGTH as f32,
                constants::CELL_LENGTH as f32,
            ),
            colour,
        )
        .unwrap();

        graphics::draw(ctx, &square, (ggez::mint::Point2 { x: 0.0, y: 0.0 },)).unwrap();
    }

    pub fn random_edge_position() -> Position {
        let mut rng = rand::thread_rng();

        let which_edge = Range::new(0, 4);
        let height = Range::new(0, constants::WINDOW_HEIGHT);
        let width = Range::new(0, constants::WINDOW_WIDTH);
        let zero_to_three = which_edge.ind_sample(&mut rng);

        match zero_to_three {
            0 => Position {
                x: 0,
                y: height.ind_sample(&mut rng),
            },
            1 => Position {
                x: width.ind_sample(&mut rng),
                y: 0,
            },
            2 => Position {
                x: constants::WINDOW_WIDTH - constants::CELL_LENGTH,
                y: height.ind_sample(&mut rng),
            },
            3 => Position {
                x: width.ind_sample(&mut rng),
                y: constants::WINDOW_HEIGHT - constants::CELL_LENGTH,
            },
            _ => panic!("uh-oh"),
        }
    }
}
