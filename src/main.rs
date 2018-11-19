extern crate quicksilver;

/**
 * Import external modules
 */
mod simon;
mod rieke;
mod jan;

/**
 * Namespaces of different modules
 */
use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector, Shape},
    graphics::{Background::Col, Color, View},
    lifecycle::{Settings, State, Window, run},
    input::{Key}
};

/**
 * Struct for game state
 */
struct Game {
    simon_update_struct: simon::SimonUpdateStruct,
    rieke_update_struct: rieke::RiekeUpdateStruct,
    jan_update_struct: jan::JanUpdateStruct
}

/**
 * Game state for running, updating and event handling the main window
 */
impl State for Game {

    /**
     * Is called right after instantiating the new game state
     * Sets default parameters
     */
    fn new() -> Result<Game> {
        Ok(Game{
            simon_update_struct: simon::SimonUpdateStruct {x_pos: 0, y_pos: 0},
            rieke_update_struct: rieke::RiekeUpdateStruct {},
            jan_update_struct: jan::JanUpdateStruct {}
        })
    }

    /**
     * Updates the main window frequently
     */
    fn update(&mut self, _window: &mut Window) -> Result<()> {

	simon::update(_window);
	rieke::update(_window);
	jan::update(_window);

	Ok(())
   }

    /**
     * Draws components inside of the main window
     */
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        Ok(())
    }
}

/**
 * Main function which is called when starting the game
 */
fn main() {
    run::<Game>("Draw Geometry", Vector::new(800, 600), Settings::default());
}


