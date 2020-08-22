use bevy::prelude::*;

const GRAVITY: f32 = -2.5;

pub struct Gravity;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(gravity_system.system());
    }
}

fn gravity_system(mut query: Query<(&Gravity, &mut Translation)>) {
    for (_gravity, mut translation) in &mut query.iter() {
        *translation.0.y_mut() = translation.y() + GRAVITY;
    }
}
