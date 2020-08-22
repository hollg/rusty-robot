use crate::gravity::Gravity;
use bevy::prelude::*;
use rand::Rng;

struct AsteroidTimer(Timer);

struct Asteroid;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(AsteroidTimer(Timer::from_seconds(1.0, true)))
            .add_system(asteroid_creator_system.system())
            .add_system(asteroid_destroyer_system.system());
    }
}

fn asteroid_creator_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidTimer>,
) {
    let mut rng = rand::thread_rng();
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                translation: Translation(Vec3::new(rng.gen_range(-400.0, 400.0), 400.0, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(30.0, 30.0),
                },
                ..Default::default()
            })
            .with(Asteroid)
            .with(Gravity);
        timer.0.reset();
    }
}

fn asteroid_destroyer_system(
    mut commands: Commands,
    mut query: Query<(&Asteroid, &mut Translation, Entity)>,
) {
    for (_asteroid, translation, entity) in &mut query.iter() {
        if translation.0.y() < -400.0 {
            println!("despawn");
            commands.despawn(entity);
        }
    }
}
