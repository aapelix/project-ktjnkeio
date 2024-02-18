use bevy::app::{App, Plugin};
use bevy::input::Input;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprite.png"),
            ..default()
        }, Player { speed: 200.0 }, Name::new("Player")));
}

fn player_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {

    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }

        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
            transform.scale.x = -1.0;
        }

        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }

        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
            transform.scale.x = 1.0;
        }
    }
}