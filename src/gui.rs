use piston_window::*;
use find_folder;
use textwrap;
use std::collections::HashMap;

use crate::galaxy_generator;
use crate::star_generator;
use crate::star_generator::StarColor;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

pub fn render_galaxy(galaxy: &galaxy_generator::Galaxy){

    let color_map: HashMap<star_generator::StarColor, [f32; 4]> = HashMap::from([
        (
            StarColor::Red,
            [1.0, 0.2, 0.2, 1.0]
        ),
        (
            StarColor::Yellow,
            [1.0, 1.0, 0.4, 1.0]
        ),
        (
            StarColor::Orange,
            [1.0, 0.6, 0.2, 1.0]
        ),
        (
            StarColor::Blue,
            [0.4, 0.4, 1.0, 1.0]
        ),
        (
            StarColor::White,
            [1.0, 1.0, 1.0, 1.0]
        ),
    ]);

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear([0.0; 4], g);

            // rectangle([1.0, 0.0, 0.0, 1.0], // red color
            //           [0.0, 0.0, 100.0, 100.0], // rectangle position and size
            //           c.transform, g);

        });

        let mut n = 0;
        for i in 1..=10 {
            for j in 1..=10 {
                if n == galaxy.stars.len() { break; }
                window.draw_2d(&event, |c, g, _| {
                    ellipse(color_map[&galaxy.stars[n].color], // red color
                            [(j * 40) as f64, (i * 30) as f64, 15.0, 15.0], // ellipse position and size
                            c.transform, g);
                });
                n += 1;
            }
        }


    }
}

pub fn render_text(text_to_draw: &str) {
    let mut window: PistonWindow = WindowSettings::new(
        "piston: hello_world",
        [200, 200]
    )
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            let transform = c.transform.trans(10.0, 100.0); // position in window

            let wrapped_text = textwrap::wrap(text_to_draw, 18);
            clear([0.0, 0.0, 0.0, 1.0], g);
            // color and font size
            for (i, line) in wrapped_text.iter().enumerate() {
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16).draw(
                    line,
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 10.0 + i as f64 * 20.0), g
                ).unwrap();

                rectangle([1.0, 0.0, 0.0, 1.0], // red color
                          [10.0, 0.0, 10.0, 10.0], // rectangle position and size
                          c.transform, g);
            }

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}