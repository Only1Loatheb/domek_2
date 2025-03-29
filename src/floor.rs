use bevy::prelude::*;

use crate::common::*;
use bevy::math::vec3;
use bevy::transform;
use std::f32::consts::PI;
// https://bevyengine.org/examples/3d-rendering/3d-shapes/

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct LoadBearingWall;

const FLOOR_DEPTH: f32 = 1.;

fn spawn_floors(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
  let floor_origin: Vec3 = vec3(0., -FLOOR_DEPTH, 0.);
  let parent = commands
    .spawn((
      Transform::from_translation(floor_origin).with_scale(vec3(-1., 1., 1.)),
      // rotation
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();
  let color = Color::hsl(0., 0., 1.);
  let floor_material: Handle<StandardMaterial> = materials.add(color);

  {
    let living_room_floor = Cuboid::new(LIVING_ROOM_X, FLOOR_DEPTH, LIVING_ROOM_Z);
    let translation = living_room_floor.half_size;
    commands
      .spawn((
        Mesh3d(meshes.add(living_room_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(parent);
  }
  {
    let hall_floor = Cuboid::new(HALL_X, FLOOR_DEPTH, HALL_Z);
    let translation = hall_floor.half_size + vec3(LIVING_ROOM_X_HALL_OFFSET, 0., LIVING_ROOM_Z);
    commands
      .spawn((
        Mesh3d(meshes.add(hall_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(parent);
  }
  {
    let bathroom_floor = Cuboid::new(BATHROOM_X, FLOOR_DEPTH, BATHROOM_Z);
    let translation = bathroom_floor.half_size + vec3(0., 0., LIVING_ROOM_Z);
    commands
      .spawn((
        Mesh3d(meshes.add(bathroom_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(parent);
  }
}

pub(crate) struct FloorPlugin;

impl Plugin for FloorPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_floors);
  }
}
