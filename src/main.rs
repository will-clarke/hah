#![feature(clamp)]

use ggez::event::{self, EventHandler, KeyMods};
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, timer};
use ggez::{Context, ContextBuilder, GameResult};
use std::collections::HashMap;

const CELL_LENGTH: usize = 10;
const WINDOW_HEIGHT: usize = 600;
const WINDOW_WIDTH: usize = 800;

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!(
            "Exited cleanly. Update count = {}\nlast position = {:?}",
            my_game.update_count, my_game.square
        ),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Wall {
    colour: graphics::Color,
}

impl Wall {
    pub fn new() -> Wall {
        Wall {
            colour: graphics::Color {
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 0.1,
            },
        }
    }
}

struct MyGame {
    update_count: usize,
    quit: bool,
    square: Position,
    walls: HashMap<Position, Wall>,
    direction: Direction,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: (WINDOW_WIDTH / 2) as usize / CELL_LENGTH,
            y: (WINDOW_HEIGHT / 2) as usize / CELL_LENGTH,
        }
    }

    pub fn shunt(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.y > 0 {
                    self.y -= CELL_LENGTH;
                } else {
                    self.y = WINDOW_HEIGHT;
                }
            }
            Direction::Down => {
                if self.y < WINDOW_HEIGHT {
                    self.y += CELL_LENGTH;
                } else {
                    self.y = 0;
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    self.x -= CELL_LENGTH;
                } else {
                    self.x = WINDOW_WIDTH;
                }
            }
            Direction::Right => {
                if self.x < WINDOW_WIDTH {
                    self.x += CELL_LENGTH;
                } else {
                    self.x = 0;
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, colour: graphics::Color) {
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.x as f32,
                self.y as f32,
                CELL_LENGTH as f32,
                CELL_LENGTH as f32,
            ),
            colour,
        )
        .unwrap();

        graphics::draw(ctx, &square, (ggez::mint::Point2 { x: 0.0, y: 0.0 },)).unwrap();
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
        // Load/create resources here: images, fonts, sounds, etc.
        MyGame {
            update_count: 0,
            quit: false,
            square: Position::new(),
            walls: HashMap::new(),
            direction: Direction::Right,
        }
    }

    pub fn should_move(&self, ctx: &Context, speed: Speed) -> bool {
        let speed_value = match speed {
            Speed::MegaSlow => 8,
            Speed::Slow => 4,
            Speed::Fast => 2,
            Speed::MegaFast => 1,
        };
        let ticks = timer::ticks(ctx);
        ticks % speed_value == 0
    }

    pub fn move_square(&mut self, direction: Direction) {
        self.square.shunt(direction);

        // if self.walls.contains_key(&self.square) {
        match self.walls.get_mut(&self.square) {
            Some(wall) => {
                wall.colour = graphics::Color {
                    r: wall.colour.r,
                    g: wall.colour.g,
                    b: wall.colour.b,
                    a: f32::clamp(wall.colour.a + 0.10, wall.colour.a, 1.0),
                };
            }
            None => {
                self.walls.insert(self.square, Wall::new());
            }
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let speed = match timer::ticks(ctx) {
            0..=500 => Speed::MegaSlow,
            500..=1500 => Speed::Slow,
            1500..=3000 => Speed::Fast,
            _ => Speed::MegaFast,
        };
        if self.should_move(ctx, speed) {
            self.move_square(self.direction);
        }

        if self.quit == true {
            event::quit(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let c = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
        graphics::clear(ctx, c);

        self.square.draw(ctx, graphics::BLACK);

        for (position, wall) in self.walls.iter() {
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
            KeyCode::W => self.direction = Direction::Up,
            KeyCode::Up => self.direction = Direction::Up,
            KeyCode::S => self.direction = Direction::Down,
            KeyCode::Down => self.direction = Direction::Down,
            KeyCode::A => self.direction = Direction::Left,
            KeyCode::Left => self.direction = Direction::Left,
            KeyCode::D => self.direction = Direction::Right,
            KeyCode::Right => self.direction = Direction::Right,
            _ => {}
        }
    }
}
