use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct CamPlugin;

#[derive(Component)]
pub struct Camera {
    pub speed: f32,
}

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_movement);
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    let width = 256.0;
    let height = 144.0;

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: width,
        min_height: height,
    };

    camera.camera.hdr = true;

    commands.spawn((camera, Camera { speed: 200.0 }, Name::new("Camera")));
}

fn camera_movement(
    mut characters: Query<(&mut Transform, &Camera)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {

    for (mut transform, camera) in &mut characters {
        let movement_amount = camera.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }

        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }

        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }

        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
    }
}