use ggez::event;
use ggez::timer;
use ggez::ContextBuilder;

mod constants;
mod direction;
mod game;
mod position;

pub fn run() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = game::MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Nice one m8 - your score = {}", timer::ticks(&ctx)),
        Err(e) => println!("Error occured: {}", e),
    }
}
