extern crate blue_engine;

use blue_engine::{
    header::{ Engine, ObjectSettings },
    primitive_shapes::triangle
};

fn main() {
    // initialize the engine
    let mut engine = Engine::new().expect("engine couldn't be initialized");

    // create a triangle
    triangle("my triangle", ObjectSettings::default(), &mut engine.renderer, &mut engine.objects).unwrap();

    // run the engine
    engine
        .update_loop(move |_, _, obj, _, _, _| {
            if let Some (t) = obj.get_mut("my triangle") {
                t.set_rotatation(0.5, blue_engine::RotateAxis::Y)
            };
        })
        .expect("Error during update loop");
}