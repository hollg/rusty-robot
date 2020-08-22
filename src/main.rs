use bevy::prelude::*;
mod asteroids_plugin;
mod gravity;
mod rusty_plugin;

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
        .add_plugin(rusty_plugin::RustyPlugin)
        .add_plugin(asteroids_plugin::AsteroidsPlugin)
        .add_plugin(gravity::GravityPlugin)
        .run();
}
