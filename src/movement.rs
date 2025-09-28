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

#[derive(Event)]
pub struct MoveEvent {
  pub translation: Vec3,
}

fn get_movement(
  input: Res<ButtonInput<KeyCode>>,
  timer: Res<Time>,
  camera: Query<(&Transform, &CameraMovement)>,
  mut writer: EventWriter<MoveEvent>,
) {
  let mut movement = Vec3::ZERO;
  let (transform, camara_movement) = camera.single().unwrap();
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
  writer.write(MoveEvent { translation: movement });
}

fn move_camera(mut camera: Query<&mut Transform, With<CameraMovement>>, mut reader: EventReader<MoveEvent>) {
  let mut transform = camera.single_mut().unwrap();
  for e in reader.read() {
    transform.translation += e.translation;
  }
}

pub(crate) struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<MoveEvent>().add_systems(Update, (get_movement, move_camera));
  }
}
