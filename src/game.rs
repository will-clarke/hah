use ggez::event::{self, EventHandler, KeyMods};
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, timer};
use ggez::{Context, GameResult};
use rand::distributions::{IndependentSample, Range};
use std::collections::hash_map::HashMap;

use super::constants;
use super::direction;
use super::position;

#[derive(Debug)]
struct Trail {
    colour: graphics::Color,
}

impl Trail {
    pub fn new() -> Trail {
        Trail {
            colour: graphics::Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 0.2,
            },
        }
    }
}

pub struct MyGame {
    quit: bool,
    player: Player,
    direction: direction::Direction,
    baddies: Vec<Baddie>,
}

#[derive(Debug)]
struct Player {
    position: position::Position,
    trail: HashMap<position::Position, Trail>,
}

struct Baddie {
    colour: graphics::Color,
    position: position::Position,
}

// screen size = 800 width x 600 height
// maybe I could have a grid 80 width x 60 height?

enum Speed {
    MegaSlow,
    Slow,
    Fast,
    MegaFast,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        MyGame {
            quit: false,
            player: Player {
                position: position::Position::new(),
                trail: HashMap::new(),
            },
            direction: direction::Direction::Right,
            baddies: Vec::new(),
        }
    }

    fn random_edge_position(&self) -> position::Position {
        let mut rng = rand::thread_rng();

        let which_edge = Range::new(0, 4);
        let height = Range::new(0, constants::WINDOW_HEIGHT);
        let width = Range::new(0, constants::WINDOW_WIDTH);
        let zero_to_three = which_edge.ind_sample(&mut rng);

        match zero_to_three {
            0 => position::Position {
                x: 0,
                y: height.ind_sample(&mut rng),
            },
            1 => position::Position {
                x: width.ind_sample(&mut rng),
                y: 0,
            },
            2 => position::Position {
                x: constants::WINDOW_WIDTH - constants::CELL_LENGTH,
                y: height.ind_sample(&mut rng),
            },
            3 => position::Position {
                x: width.ind_sample(&mut rng),
                y: constants::WINDOW_HEIGHT - constants::CELL_LENGTH,
            },
            _ => panic!("uh-oh"),
        }
    }

    pub fn spawn_baddies(&mut self) {
        let new_baddie = Baddie {
            position: self.random_edge_position(),
            colour: graphics::Color {
                r: 1.,
                g: 0.,
                b: 0.,
                a: 1.,
            },
        };
        self.baddies.push(new_baddie);
    }

    pub fn spawn_goodies(&mut self) {}

    fn should_move(&self, ctx: &Context, speed: Speed) -> bool {
        let speed_value = match speed {
            Speed::MegaSlow => 8,
            Speed::Slow => 4,
            Speed::Fast => 2,
            Speed::MegaFast => 1,
        };
        let ticks = timer::ticks(ctx);
        ticks % speed_value == 0
    }

    pub fn move_square(&mut self, direction: direction::Direction) {
        self.player.position.shunt(direction);

        // if self.walls.contains_key(&self.player) {
        match self.player.trail.get_mut(&self.player.position) {
            Some(wall) => {
                wall.colour = graphics::Color {
                    r: wall.colour.r,
                    g: wall.colour.g,
                    b: wall.colour.b,
                    a: f32::clamp(wall.colour.a + 0.20, wall.colour.a, 1.0),
                };
            }
            None => {
                self.player.trail.insert(self.player.position, Trail::new());
            }
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let ticks = timer::ticks(ctx);

        if ticks % 100 == 0 {
            self.spawn_baddies();
        }
        if ticks % 200 == 0 {
            self.spawn_goodies();
        }

        let speed = match ticks {
            0..=50 => Speed::MegaSlow,
            51..=1500 => Speed::Slow,
            1501..=3000 => Speed::Fast,
            _ => Speed::MegaFast,
        };
        if self.should_move(ctx, speed) {
            self.move_square(self.direction);
        }

        for baddie in self.baddies.iter_mut() {
            baddie.position.shunt_towards(&self.player.position);
            if (self.player.position.x % constants::WINDOW_WIDTH
                == baddie.position.x % constants::WINDOW_WIDTH)
                && (self.player.position.y % constants::WINDOW_HEIGHT
                    == baddie.position.y % constants::WINDOW_HEIGHT)
            {
                event::quit(ctx);
            }
        }

        if self.quit {
            event::quit(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let c = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
        graphics::clear(ctx, c);

        self.player.position.draw(ctx, graphics::BLACK);

        for baddie in self.baddies.iter() {
            baddie.position.draw(ctx, baddie.colour);
        }

        for (position, wall) in self.player.trail.iter() {
            position.draw(ctx, wall.colour);
        }

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        // self.direction = direction;
        match keycode {
            KeyCode::Q => self.quit = true,
            KeyCode::W => self.direction = direction::Direction::Up,
            KeyCode::Up => self.direction = direction::Direction::Up,
            KeyCode::S => self.direction = direction::Direction::Down,
            KeyCode::Down => self.direction = direction::Direction::Down,
            KeyCode::A => self.direction = direction::Direction::Left,
            KeyCode::Left => self.direction = direction::Direction::Left,
            KeyCode::D => self.direction = direction::Direction::Right,
            KeyCode::Right => self.direction = direction::Direction::Right,
            _ => {}
        }
    }
}
