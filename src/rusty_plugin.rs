use crate::gravity::Gravity;
use bevy::prelude::*;
use std::f32::consts::PI;

enum Direction {
    Left,
    Right,
}

struct Rusty {
    direction: Direction,
    speed: f32,
}

impl Rusty {
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

pub struct RustyPlugin;

impl Plugin for RustyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(animate_sprite_system.system())
            .add_system(keyboard_input_system.system());
    }
}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer,
        &Rusty,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, rusty, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            match rusty.direction() {
                Direction::Right => {
                    if sprite.index == 2 {
                        sprite.index = 0;
                    } else {
                        sprite.index =
                            ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
                    }
                    timer.reset();
                }
                Direction::Left => {
                    if sprite.index == 5 {
                        sprite.index = 3;
                    } else {
                        sprite.index =
                            ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
                    }
                    timer.reset();
                }
            }
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut rusty_query: Query<(&mut Rusty, &mut Translation, &mut Rotation)>,
) {
    for (mut rusty, mut translation, mut rotation) in &mut rusty_query.iter() {
        if keyboard_input.pressed(KeyCode::Up) {
            *translation.0.y_mut() = translation.0.y() + 1f32 * rusty.speed;
        } else if keyboard_input.pressed(KeyCode::Down) {
            *translation.0.y_mut() = translation.0.y() - 1f32 * rusty.speed;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            *translation.0.x_mut() = translation.0.x() + 1f32 * rusty.speed;
            *rotation = Rotation::from_rotation_z(-10f32 * (PI / 180f32));
            rusty.set_direction(Direction::Right);
        } else if keyboard_input.pressed(KeyCode::Left) {
            *translation.0.x_mut() = translation.0.x() - 1f32 * rusty.speed;
            *rotation = Rotation::from_rotation_z(10f32 * (PI / 180f32));
            rusty.set_direction(Direction::Left);
        } else {
            *rotation = Rotation::from_rotation_z(0f32);
        }

        *translation.0.x_mut() = f32::max(-400.0, f32::min(380.0, translation.0.x()));
        *translation.0.y_mut() = f32::max(-400.0, f32::min(380.0, translation.0.y()));
        // *translation.0.y_mut() = translation.0.y() - 2f32;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/rusty_left_right_cropped.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(5.0),
            ..Default::default()
        })
        .with(Rusty {
            direction: Direction::Right,
            speed: 5f32,
        })
        .with(Gravity)
        .with(Timer::from_seconds(0.1));
}
