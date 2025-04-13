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

const FLOOR_SCALA: Vec3 = vec3(-1., 1., 1.);

fn spawn_floors(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
  let floor_depth: f32 = 1.;
  let floor_origin: Vec3 = vec3(0., -floor_depth, 0.);
  let parent = commands
    .spawn((
      Transform::from_translation(floor_origin).with_scale(FLOOR_SCALA),
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();
  let color = Color::hsl(0., 0., 1.);
  let floor_material: Handle<StandardMaterial> = materials.add(color);

  {
    let living_room_floor = Cuboid::new(LIVING_ROOM_X, floor_depth, LIVING_ROOM_Z);
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
    let hall_floor = Cuboid::new(HALL_X, floor_depth, HALL_Z);
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
    let bathroom_floor = Cuboid::new(BATHROOM_X, floor_depth, BATHROOM_Z);
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
  {
    let office_floor = Cuboid::new(OFFICE_X, floor_depth, OFFICE_Z);
    let translation = office_floor.half_size + vec3(BATHROOM_X + SMALL_HALL_X, 0., LIVING_ROOM_Z + HALL_Z);
    commands
      .spawn((
        Mesh3d(meshes.add(office_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(parent);
  }
  {
    let small_hall_floor = Cuboid::new(SMALL_HALL_X, floor_depth, SMALL_HALL_Z);
    let translation = small_hall_floor.half_size + vec3(BATHROOM_X, 0., LIVING_ROOM_Z + HALL_Z);
    commands
      .spawn((
        Mesh3d(meshes.add(small_hall_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(parent);
  }
  {
    let bedroom_floor = Cuboid::new(BEDROOM_X, floor_depth, BEDROOM_Z);
    let translation = bedroom_floor.half_size + vec3(0., 0., LIVING_ROOM_Z + BATHROOM_Z);
    commands
      .spawn((
        Mesh3d(meshes.add(bedroom_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(parent);
  }
}

const LOAD_BEARING_WALL_THICKNESS: f32 = 1.;

fn spawn_walls(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
  let parent = commands
    .spawn((
      Transform::from_scale(FLOOR_SCALA),
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();
  let color = Color::hsl(0., 0., 1.);
  let wall_material: Handle<StandardMaterial> = materials.add(color);
  {
    let living_room_wall = Cuboid::new(LOAD_BEARING_WALL_THICKNESS, FLAT_HEIGHT, LIVING_ROOM_Z);
    let translation = living_room_wall.half_size + vec3(-LOAD_BEARING_WALL_THICKNESS, 0., 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(living_room_wall)),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_translation(translation),
        LoadBearingWall,
      ))
      .set_parent(parent);
  }
  {
    let bedroom_wall = Cuboid::new(LOAD_BEARING_WALL_THICKNESS, FLAT_HEIGHT, BEDROOM_Z);
    let translation = bedroom_wall.half_size + vec3(-LOAD_BEARING_WALL_THICKNESS, 0., LIVING_ROOM_Z + BATHROOM_Z);
    commands
      .spawn((
        Mesh3d(meshes.add(bedroom_wall)),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_translation(translation),
        LoadBearingWall,
      ))
      .set_parent(parent);
  }
  {
    let kitchen_wall = Cuboid::new(LOAD_BEARING_WALL_THICKNESS, FLAT_HEIGHT, LIVING_ROOM_Z);
    let translation = kitchen_wall.half_size + vec3(LIVING_ROOM_X, 0., 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(kitchen_wall)),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_translation(translation),
        LoadBearingWall,
      ))
      .set_parent(parent);
  }
  {
    let hall_wall = Cuboid::new(TM_WALL_X, FLAT_HEIGHT, LOAD_BEARING_WALL_THICKNESS);
    let translation = hall_wall.half_size + vec3(LIVING_ROOM_X, 0., LIVING_ROOM_Z - LOAD_BEARING_WALL_THICKNESS);
    commands
      .spawn((
        Mesh3d(meshes.add(hall_wall)),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_translation(translation),
        LoadBearingWall,
      ))
      .set_parent(parent);
  }
}

pub(crate) struct FloorPlugin;

impl Plugin for FloorPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, (spawn_floors, spawn_walls));
  }
}
