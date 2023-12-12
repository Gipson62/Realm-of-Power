extern crate blue_engine;

use blue_engine::{
    header::{Engine, ObjectSettings},
    primitive_shapes::triangle,
};

pub struct Game {
    pub engine: Engine,
}

impl Game {
    pub fn new() -> anyhow::Result<Self> {
        // initialize the engine
        let mut engine = Engine::new()?;

        // create an object
        blue_engine_utilities::model_load::load_gltf(
            Some("monke"),
            &std::path::Path::new("assets/models/monkey.glb"),
            &mut engine.renderer,
            &mut engine.objects,
        )?;

        Ok(Self { engine })
    }

    pub fn run(self) -> anyhow::Result<()> {
        // get the engine
        let Self { engine } = self;

        // run the engine
        engine.update_loop(move |_, _, obj, _, _, _| {
            if let Some(t) = obj.get_mut("monke") {
                t.set_rotatation(0.5, blue_engine::RotateAxis::Y)
            };
        })?;

        Ok(())
    }
}
