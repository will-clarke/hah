use ggez::event::{self, EventHandler, KeyMods};
use ggez::graphics;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use std::collections::HashSet;

const CELL_LENGTH: i16 = 10;
const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;

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

struct MyGame {
    update_count: i32,
    quit: bool,
    square: Position,
    walls: HashSet<Position>,
    direction: Direction,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: (WINDOW_WIDTH / 2) as i16 / CELL_LENGTH,
            y: (WINDOW_HEIGHT / 2) as i16 / CELL_LENGTH,
        }
    }

    pub fn shunt(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= CELL_LENGTH,
            Direction::Down => self.y += CELL_LENGTH,
            Direction::Left => self.x -= CELL_LENGTH,
            Direction::Right => self.x += CELL_LENGTH,
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

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        MyGame {
            update_count: 0,
            quit: false,
            square: Position::new(),
            walls: HashSet::new(),
            direction: Direction::Left,
        }
    }

    pub fn handle_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.direction = Direction::Up;
                self.square.shunt(Direction::Up);
            }
            Direction::Down => {
                self.direction = Direction::Down;
                self.square.shunt(Direction::Down);
            }
            Direction::Left => {
                self.direction = Direction::Left;
                self.square.shunt(Direction::Left);
            }
            Direction::Right => {
                self.direction = Direction::Right;
                self.square.shunt(Direction::Right);
            }
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.update_count += 1;
        if self.quit == true {
            event::quit(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let c = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
        graphics::clear(ctx, c);

        self.square.draw(ctx, graphics::BLACK);

        let wall_colour = graphics::Color {
            r: 0.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };

        for wall in self.walls.iter() {
            wall.draw(ctx, wall_colour);
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
        match keycode {
            KeyCode::Q => self.quit = true,
            KeyCode::W => self.handle_direction(Direction::Up),
            KeyCode::Up => self.handle_direction(Direction::Up),
            KeyCode::S => self.handle_direction(Direction::Down),
            KeyCode::Down => self.handle_direction(Direction::Down),
            KeyCode::A => self.handle_direction(Direction::Left),
            KeyCode::Left => self.handle_direction(Direction::Left),
            KeyCode::D => self.handle_direction(Direction::Right),
            KeyCode::Right => self.handle_direction(Direction::Right),
            KeyCode::Space => {
                // why not make a wall when we mash the spacebar?
                self.walls.insert(self.square.clone());
                self.square.shunt(self.direction);
                () // is this legit? Seems a bit dubious to me...
            }
            _ => (),
        }
    }
}
