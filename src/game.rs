use ggez::event::{self, EventHandler, KeyMods};
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, timer};
use ggez::{Context, GameResult};
use std::collections::hash_map::HashMap;

use super::constants;
use super::direction;
use super::position::Position;

pub struct MyGame {
    quit: bool,
    direction: direction::Direction,
    player: Player,
    baddies: Vec<Baddie>,
    background: Background,
}

#[cfg(test)]
mod test {
    use super::Background;
    use super::Entity;
    use super::Position;
    use ggez::graphics;
    use std::collections::HashMap;
    #[test]
    fn background_add() {
        let e: Entity = Entity {
            position: Position::new(),
            colour: graphics::Color::new(0.0, 1.0, 1.0, 1.0),
        };
        let mut b = Background(HashMap::new());
        b.add(e);
        b.add(e);
        assert_eq!(
            b.0.get(&Position::new()).unwrap(),
            &graphics::Color::new(0.0, 2.0, 2.0, 2.0)
        )
    }
}
struct Background(HashMap<Position, graphics::Color>);
impl Background {
    fn add(&mut self, e: Entity) {
        let colour = self
            .0
            .entry(e.position)
            .or_insert(graphics::Color::new(0.0, 0.0, 0.0, 0.0));
        *colour = graphics::Color::new(
            colour.r + e.colour.r,
            colour.g + e.colour.g,
            colour.b + e.colour.b,
            colour.a + e.colour.a,
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Entity {
    position: Position,
    colour: graphics::Color,
}

struct Player(Entity);
struct Baddie(Entity);

impl Player {
    pub fn new() -> Player {
        Player(Entity {
            position: Position::new(),
            colour: graphics::Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 0.2,
            },
        })
    }
}

impl Baddie {
    pub fn new() -> Baddie {
        Baddie(Entity {
            position: Position::random_edge_position(),
            colour: graphics::Color {
                r: 0.2,
                g: 0.0,
                b: 0.0,
                a: 0.2,
            },
        })
    }
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
        MyGame {
            quit: false,
            player: Player::new(),
            direction: direction::Direction::Right,
            baddies: Vec::new(),
            background: Background(HashMap::new()),
        }
    }

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
        self.player.0.position.shunt(direction);

        // if self.walls.contains_key(&self.player) {
        // match self.player.trail.get_mut(&self.player.position) {
        //     Some(wall) => {
        //         wall.0.colour = graphics::Color {
        //             r: wall.0.colour.r,
        //             g: wall.0.colour.g,
        //             b: wall.0.colour.b,
        //             a: f32::clamp(wall.0.colour.a + 0.20, wall.0.colour.a, 1.0),
        //         };
        //     }
        //     None => {
        //         self.player.trail.insert(self.player.position, Trail::new());
        //     }
        // }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let ticks = timer::ticks(ctx);

        if ticks % 100 == 0 {
            self.baddies.push(Baddie::new());
        }
        // if ticks % 200 == 0 {
        //     // self.spawn_goodies();
        // }

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
            baddie.0.position.shunt_towards(&self.player.0.position);
            if (self.player.0.position.x % constants::WINDOW_WIDTH
                == baddie.0.position.x % constants::WINDOW_WIDTH)
                && (self.player.0.position.y % constants::WINDOW_HEIGHT
                    == baddie.0.position.y % constants::WINDOW_HEIGHT)
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

        self.player.0.position.draw(ctx, graphics::BLACK);

        for baddie in self.baddies.iter() {
            baddie.0.position.draw(ctx, baddie.0.colour);
        }

        // for (0.position, wall) in self.player.trail.iter() {
        //     0.position.draw(ctx, wall.0.colour);
        // }

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
