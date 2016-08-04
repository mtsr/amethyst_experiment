extern crate time;
extern crate amethyst;
extern crate cgmath;

use amethyst::engine::{Application, ApplicationBuilder};
use amethyst::config::Element;

mod components;
mod systems;
mod main_state;

fn main() {
    use amethyst::context::Config;
    use systems::{Update,Render};

    let config = Config::from_file("resources/config.yml").expect("Expect ../resources/config.yml to exist");
    let mut game = ApplicationBuilder::new(main_state::HelloWorld, config)
    .with(Update, "update", 50)
    .with(Render::new(), "render", 100)
    .done();
    game.run();
}