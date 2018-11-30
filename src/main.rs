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
    geom::{/*Circle, Line, Rectangle, Transform, Triangle,*/ Vector/*, Shape*/},
    graphics::{/*Background::Col,*/ Color/*, View*/},
    lifecycle::{Settings, State, Window, run},
    //input::{Key}
};

/**
 * Struct for game state
 */
pub struct Game {
    _simon_update_struct: simon::SimonUpdateStruct,
    _rieke_update_struct: rieke::RiekeUpdateStruct,
    _jan_update_struct: jan::JanUpdateStruct,

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
            _simon_update_struct: simon::init(),
            _rieke_update_struct: rieke::init(),
            _jan_update_struct: jan::init(),

        })
    }

    /**
     * Updates the game logic
     */
    fn update(&mut self, window: &mut Window) -> Result<()> {

	simon::update(window, self);
	rieke::update(window, self);
	jan::update(window, self);

	Ok(())
   }

    /**
     * Draws components inside of the main window
     */
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        simon::draw(window, self);
        rieke::draw(window, self);
        jan::draw(window, self);
        Ok(())
    }
}

/**
 * Main function which is called when starting the game
 */
fn main() {
    run::<Game>("Draw Geometry", Vector::new(800, 600), Settings::default());
}


