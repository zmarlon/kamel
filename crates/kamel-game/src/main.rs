use kamel::{
    bevy::{app::App, window::WindowDescriptor},
    DefaultPlugins
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "Kamel".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}

/*
struct Enabled {
    task_shader: bool,
    mesh_shader: bool,
    raytracing: bool,
    debug_utils: bool
}
 */
