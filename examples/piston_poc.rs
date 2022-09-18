extern crate find_folder;
extern crate piston_window;

use blackboard_lib::blackboard;
// use piston_window::{PistonWindow, WindowSettings};
// Dunno why its good to implicitly import in Rust good,
// but let leave it like that for now.
use piston_window::*;

fn main() {
    let mut database = LocalDatabase::new();

    let mut window: PistonWindow = WindowSettings::new("piston: hello_world", [1400, 700])
        .exit_on_esc(true)
        // .opengl(OpenGL::V4_5) // Set a different OpenGl version
        .build()
        .unwrap();

    // NOTE: Keeping this code for reference
    // let assets = find_folder::Search::ParentsThenKids(3, 3)
    //     .for_folder("resources")
    //     .unwrap();
    // println!("{:?}", assets);
    // let mut glyphs = window
    //     .load_font(assets.join("FiraSans-Regular.ttf"))
    //     .unwrap();
    // NOTE END

    window.set_lazy(true);

    let mut cursor_placement = blackboard::Point { x: 0, y: 0 };
    while let Some(piston_event) = window.next() {
        match &piston_event {
            Event::Input(input_evt, _) => {
                use Input::{Button, Move, Text};
                match input_evt {
                    Button(button_args) => {
                        // println!("Button");
                        match button_args.state {
                            ButtonState::Press => {
                                database.put_new_item(cursor_placement.clone());
                            }
                            ButtonState::Release => {}
                        }
                    }
                    Move(move_args) => {
                        // println!("Move")
                        match move_args {
                            Motion::MouseCursor(coord) => {
                                cursor_placement.x = coord[0] as isize;
                                cursor_placement.y = coord[1] as isize;
                            }
                            _ => {}
                        }
                    }
                    Text(_) => {
                        println!("Text")
                    }
                    _ => {}
                }
            }
            Event::Loop(loop_event) => {
                use Loop::{Render, Update};
                match loop_event {
                    Render(render_args) => {
                        // println!("Render")
                    }
                    Update(_) => {
                        // println!("Update")
                    }
                    _ => {}
                }
            }
            Event::Custom(_, _, _) => {}
        }
        render_blackboard(&piston_event, &mut window, database.get_all_items());

        // NOTE: Keeping this code as reference.
        // window.draw_2d(&piston_event, |c, g, device| {
        //     let transform = c.transform.trans(10.0, 100.0);

        //     clear([0.0, 0.0, 0.0, 1.0], g);
        //     text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
        //         "Hello world!",
        //         &mut glyphs,
        //         &c.draw_state,
        //         transform, g
        //     ).unwrap();

        //     // Update glyphs before rendering.
        //     glyphs.factory.encoder.flush(device);
        // });
    }
}

const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn render_blackboard(
    event: &Event,
    window: &mut PistonWindow,
    content: &Vec<blackboard::StickyNote>,
) {
    // NOTE: Render text resources
    // TODO: the resources shouldn-t be load every render event. Should be moved someware.
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("resources")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    // END NOTE

    window.draw_2d(event, |ctx, gl, device| {
        clear(BACKGROUND, gl);
        // NOTE: Render background text
        text::Text::new_color(BLACK, 50)
            .draw(
                "Click on Mouse to spawn new note",
                &mut glyphs,
                &ctx.draw_state,
                ctx.transform.trans(300.0, 350.0),
                gl,
            )
            .unwrap();
        glyphs.factory.encoder.flush(device); // I have no idea what this is for. Need to read
        // END NOTE

        for item in content {
            rectangle(
                FOREGROUND,
                rectangle::square(0.0, 0.0, item.representation.dx as f64),
                ctx.transform
                    .trans(item.placement.x as f64, item.placement.y as f64),
                gl,
            );
        }
    });
}

#[derive(Debug, Clone)]
struct RClientError;
trait RucollaClient {
    fn get_all_items(&self) -> &Vec<blackboard::StickyNote>;
    fn put_new_item(&mut self, placement: blackboard::Point) -> Result<(), RClientError>;
}

struct LocalDatabase {
    board: Vec<blackboard::StickyNote>,
}

impl LocalDatabase {
    fn new() -> LocalDatabase {
        let mut some_blackboard = vec![
            blackboard::new_note(),
            blackboard::new_note(),
            blackboard::new_note(),
        ];
        some_blackboard[1].change_pos(15, 15);
        some_blackboard[2].change_pos(35, 35);
        LocalDatabase {
            board: some_blackboard,
        }
    }
}

impl RucollaClient for LocalDatabase {
    fn get_all_items(&self) -> &Vec<blackboard::StickyNote> {
        &self.board
    }
    fn put_new_item(&mut self, placement: blackboard::Point) -> Result<(), RClientError> {
        let mut sticky = blackboard::new_note();
        sticky.placement = placement;
        self.board.push(sticky);
        Ok(())
    }
}
