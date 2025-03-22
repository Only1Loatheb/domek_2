use bevy::prelude::*;

use crate::common::FLAT_HEIGHT;
use bevy::math::vec3;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
  color::palettes::basic::SILVER,
  prelude::*,
  render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
  },
};
use std::f32::consts::PI;
// https://bevyengine.org/examples/3d-rendering/3d-shapes/

#[derive(Component)]
struct Bathroom;

#[derive(Component)]
struct BathroomWall;

const WALL_THICKNESS: f32 = 1.;
const LEFT_DOOR_WALL_LENGTH: f32 = 9.3;
const RIGHT_DOOR_WALL_LENGTH: f32 = 6.5;
const BATHROOM_DEPTH: f32 = 18.1;
const BATHROOM_WIDTH: f32 = 25.;
const VENT_DEPTH: f32 = 4.;
const VENT_WIDTH: f32 = 5.5;

fn setup_bathroom(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  let bathroom_origin: Vec3 = vec3(30., 0., 0.);
  let parent = commands
    .spawn((
      Transform::from_translation(bathroom_origin).with_rotation(Quat::from_rotation_y(PI)), // 180-degree
      // rotation
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();

  // NCS S 1505-y40R
  // https://www.w3schools.com/colors/colors_converter.asp?color=ncs(1505-y40R)
  let color = Color::hsl(30.0, 0.29, 0.85);
  let material = materials.add(color);

  {
    let left_door_wall = Cuboid::new(LEFT_DOOR_WALL_LENGTH + WALL_THICKNESS, FLAT_HEIGHT, WALL_THICKNESS);
    let translation = left_door_wall.half_size + vec3(0., 0., 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(left_door_wall)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(parent);
  }
  {
    let right_door_wall = Cuboid::new(RIGHT_DOOR_WALL_LENGTH + WALL_THICKNESS, FLAT_HEIGHT, WALL_THICKNESS);
    let translation = right_door_wall.half_size + vec3(BATHROOM_WIDTH - RIGHT_DOOR_WALL_LENGTH - WALL_THICKNESS, 0., 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(right_door_wall)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(parent);
  }
  {
    let left_wall = Cuboid::new(WALL_THICKNESS, FLAT_HEIGHT, BATHROOM_DEPTH + WALL_THICKNESS);
    let translation = left_wall.half_size + vec3(0., 0., -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(left_wall)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(parent);
  }
  {
    let right_wall = Cuboid::new(WALL_THICKNESS, FLAT_HEIGHT, BATHROOM_DEPTH + WALL_THICKNESS);
    let translation = right_wall.half_size + vec3(BATHROOM_WIDTH, 0., -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(right_wall)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(parent);
  }
  {
    let back_wall = Cuboid::new(BATHROOM_WIDTH, FLAT_HEIGHT, WALL_THICKNESS);
    let translation = back_wall.half_size + vec3(0., 0., -BATHROOM_DEPTH - WALL_THICKNESS);
    commands
      .spawn((
        Mesh3d(meshes.add(back_wall)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(parent);
  }
  {
    // for amogus to go in
    let vent = Cuboid::new(VENT_WIDTH, FLAT_HEIGHT, VENT_DEPTH);
    let translation = vent.half_size + vec3(BATHROOM_WIDTH - VENT_WIDTH, 0., -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(vent)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(parent);
  }
  spawn_washing_machine(commands, asset_server, parent);
}

fn spawn_washing_machine(mut commands: Commands, asset_server: Res<AssetServer>, parent: Entity) {
  let model_handle = asset_server.load("bathroom/washing_machine.glb#Scene0"); // Load the first scene
  commands.spawn(SceneBundle {
    scene: bevy::prelude::SceneRoot(model_handle),
    transform: Transform::from_translation(vec3(0., 4.25, 0.)).with_scale(Vec3::splat(10.0)),
    ..default()
  })
  .set_parent(parent);
}

pub(crate) struct BathroomPlugin;

impl Plugin for BathroomPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_bathroom);
  }
}
