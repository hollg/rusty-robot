use bevy::prelude::*;
mod rusty_plugin;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(rusty_plugin::RustyPlugin)
        .run();
}
