use bevy::prelude::*;
mod asteroids_plugin;
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
        .run();
}
