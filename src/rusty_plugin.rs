use bevy::prelude::*;
use std::f32::consts::PI;

struct Rusty;

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
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            timer.reset();
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut rusty_query: Query<(&Rusty, &mut Translation, &mut Rotation)>,
) {
    for (_rusty, mut translation, mut rotation) in &mut rusty_query.iter() {
        if keyboard_input.pressed(KeyCode::Up) {
            translation.0 += Vec3::new(0f32, 1f32, 0f32);
        } else if keyboard_input.pressed(KeyCode::Down) {
            translation.0 += Vec3::new(0f32, -1f32, 0f32);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            translation.0 += Vec3::new(1f32, 0f32, 0f32);
            *rotation = Rotation::from_rotation_z(-10f32 * (PI / 180f32));
        } else if keyboard_input.pressed(KeyCode::Left) {
            translation.0 += Vec3::new(-1f32, 0f32, 0f32);
            *rotation = Rotation::from_rotation_z(10f32 * (PI / 180f32));
        } else {
            *rotation = Rotation::from_rotation_z(0f32);
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/rusty.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(5.0),
            ..Default::default()
        })
        .with(Rusty)
        .with(Timer::from_seconds(0.1));
}