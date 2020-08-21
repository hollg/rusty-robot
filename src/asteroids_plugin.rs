use bevy::prelude::*;
use rand::Rng;
struct Gravity;

struct AsteroidTimer(Timer);

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(AsteroidTimer(Timer::from_seconds(0.5)))
            .add_system(asteroid_system.system())
            .add_system(gravity_system.system());
    }
}

fn gravity_system(mut query: Query<(&Gravity, &mut Translation)>) {
    for (_gravity, mut translation) in &mut query.iter() {
        *translation.0.y_mut() = translation.y() + -2.5;
    }
}

fn asteroid_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidTimer>,
) {
    let mut rng = rand::thread_rng();
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        println!("spawn asteroid");
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                translation: Translation(Vec3::new(rng.gen_range(-400.0, 400.0), 400.0, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(30.0, 30.0),
                },
                ..Default::default()
            })
            .with(Gravity);
        timer.0.reset();
    }
}
