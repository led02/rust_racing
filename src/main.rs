extern crate quicksilver;

/**
 * Import external modules
 */
mod simon;
mod rieke;
mod jan;
mod input_manager;
mod racing_car;
mod mathhelper;
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
    _input_update_struct: input_manager::InputUpdateStruct,
    _racingcar_update_struct: racing_car::RacingCarUpdateStruct,
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
            _input_update_struct: input_manager::init(),
            _racingcar_update_struct: racing_car::init(),
        })
    }

    /**
     * Updates the game logic
     */
    fn update(&mut self, window: &mut Window) -> Result<()> {
	self._simon_update_struct.update(window);
	rieke::update(window, self);
	jan::update(window, self);
    self._racingcar_update_struct.update(window);
	Ok(())
   }

    /**
     * Draws components inside of the main window
     */
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        self._simon_update_struct.draw(window);
        rieke::draw(window, self);
        jan::draw(window, self);
        self._racingcar_update_struct.draw(window);
        Ok(())
    }
}

/**
 * Main function which is called when starting the game
 */
fn main() {
    let mut settings = Settings::default();
    settings.update_rate = 40.0;
    run::<Game>("Draw Geometry", Vector::new(800, 600), settings);
}


