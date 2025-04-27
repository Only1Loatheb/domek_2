use crate::look::CameraSensitivity;
use crate::DIRECTIONAL_LIGHT_MOVEMENT_SPEED;
use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct CameraMovement {
  speed: f32,
}

impl Default for CameraMovement {
  fn default() -> Self {
    Self { speed: 15.0 }
  }
}

pub fn movement(input: Res<ButtonInput<KeyCode>>, timer: Res<Time>, mut camera: Query<(&mut Transform, &CameraMovement)>) {
  let mut movement = Vec3::ZERO;
  let (mut transform, camara_movement) = camera.single_mut();
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

  // movement.y = 0.0;
  movement = camara_movement.speed * timer.delta_secs() * movement.normalize_or_zero();
  if input.pressed(KeyCode::ShiftLeft) {
    movement *= 2.;
  }
  transform.translation += movement;
}
