use bevy::prelude::*;
mod asteroids;
mod gravity;
mod rusty;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Rusty Robot".to_string(),
            width: 800,
            height: 800,
            vsync: true,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(rusty::RustyPlugin)
        .add_plugin(asteroids::AsteroidsPlugin)
        .add_plugin(gravity::GravityPlugin)
        .run();
}
