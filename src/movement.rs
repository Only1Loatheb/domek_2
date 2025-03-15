use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;

const SPEED: f32 = 5.0;

pub fn movement(input: Res<ButtonInput<KeyCode>>, timer: Res<Time>, mut camera_transform: Query<&mut Transform, With<Camera3d>>) {
  let mut movement = Vec3::ZERO;
  let mut transform = camera_transform.single_mut();
  let forward = transform.forward().as_vec3();
  let right = transform.right().as_vec3();
  if input.pressed(KeyCode::KeyW) {
    movement += forward;
  }
  if input.pressed(KeyCode::KeyS) {
    movement -= forward;
  }
  if input.pressed(KeyCode::KeyA) {
    movement -= right;
  }
  if input.pressed(KeyCode::KeyD) {
    movement += right;
  }
  
  movement.y = 0.0;
  movement = SPEED * timer.delta_secs() * movement.normalize_or_zero();
  transform.translation += movement;
}
