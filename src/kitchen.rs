use crate::common::*;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::transform;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use std::ops::{Add, Not};
// https://bevyengine.org/examples/3d-rendering/3d-shapes/

#[derive(Component)]
struct KitchenCabinet;

const BOTTOM_CABINET_Y: f32 = 1.00; // legs
const BOTTOM_CABINET_HEIGHT: f32 = 7.20;
const BOTTOM_CABINET_DEPTH: f32 = 5.60;
const CABINET_WIDTH: f32 = 6.0;
const COUNTERTOP_Y: f32 = BOTTOM_CABINET_HEIGHT + BOTTOM_CABINET_Y;

const COUNTERTOP_HEIGHT: f32 = 0.38;
const GAP_BETWEEN_CABINETS_HEIGHT: f32 = 5.10;

const MIDDLE_CABINET_Y: f32 = COUNTERTOP_Y + COUNTERTOP_HEIGHT + GAP_BETWEEN_CABINETS_HEIGHT;
const MIDDLE_CABINET_DEPTH: f32 = 3.20; // guesstimate
const MIDDLE_CABINET_HEIGHT: f32 = BOTTOM_CABINET_HEIGHT;

const TOP_CABINET_Y: f32 = MIDDLE_CABINET_Y + MIDDLE_CABINET_HEIGHT;
const TOP_CABINET_DEPTH: f32 = BOTTOM_CABINET_DEPTH;
const TOP_CABINET_HEIGHT: f32 = 5.74 + 0.18;
const KITCHEN_WIDTH: f32 = 42.35;
const KITCHEN_ORIGIN: Vec3 = vec3(-LIVING_ROOM_X, 0., KITCHEN_WIDTH);

#[derive(Resource)]
struct KitchenCommon {
  parent: Entity,
  cabinets_colour: Handle<StandardMaterial>,
  wall_colour: Handle<StandardMaterial>,
}

fn setup_kitchen_common(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
  let parent = commands
    .spawn((
      Transform::from_translation(KITCHEN_ORIGIN)
        .with_scale(vec3(-1., 1., -1.))
        .with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();
  let beige = materials.add(BEIGE);
  commands.insert_resource(KitchenCommon {
    parent,
    cabinets_colour: beige.clone(),
    wall_colour: beige,
  });
}

fn setup_kitchen(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  asset_server: Res<AssetServer>,
  common: Res<KitchenCommon>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  {
    let cabinet_handle: Handle<Mesh> = asset_server.load("stl/kitchen_handle.stl");
    let cabinet_handle_material = MeshMaterial3d(repeat_texture(
      "brushed_copper/Radaway_BrushedCopper_COL_4K_METALNESS.jpg",
      &mut materials,
      &asset_server,
      Vec2 { x: 10., y: 10. },
      Vec2 { x: 0.1, y: 0.1 },
    ));
    let bottom_cabinets = [
      Cuboid::new(CABINET_WIDTH / 2.0, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH / 2.0, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    ];
    let mut x_acc: f32 = 0.0;
    for (idx, bottom_cabinet) in bottom_cabinets.into_iter().enumerate() {
      let translation = bottom_cabinet.half_size + vec3(x_acc, BOTTOM_CABINET_Y, 0.);
      let id = commands
        .spawn((
          Mesh3d(meshes.add(bottom_cabinet)),
          MeshMaterial3d(common.cabinets_colour.clone()),
          Transform::from_translation(translation),
          KitchenCabinet,
          ChildOf(common.parent),
        ))
        .id();
      fn cabinet_handle_transform(bottom_cabinet: Cuboid) -> Transform {
        Transform::from_translation(bottom_cabinet.half_size.with_x(0.))
          .with_scale(vec3(0.02, 0.02, 0.01))
          .with_rotation(Quat::from_euler(EulerRot::XYZ, 0., -FRAC_PI_2, 0.))
      }
      // commands.spawn((
      //   Mesh3d(cabinet_handle.clone()),
      //   cabinet_handle_material.clone(),
      //   cabinet_handle_transform(bottom_cabinet),
      //   ChildOf(id),
      // ));
      // if [0, 6].contains(&idx).not() {
      //   commands.spawn((
      //     Mesh3d(cabinet_handle.clone()),
      //     cabinet_handle_material.clone(),
      //     cabinet_handle_transform(bottom_cabinet).with_translation(Vec3::ZERO.with_z(bottom_cabinet.half_size.z)),
      //     ChildOf(id),
      //   ));
      // }
      x_acc += bottom_cabinet.size().x;
    }
  }
  {
    let middle_cabinets = [
      Cuboid::new(CABINET_WIDTH / 2.0, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH / 2.0, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
    ];
    let middle_cabinet_material = repeat_texture(
      "kitchen/dab_vicenza.jpg",
      &mut materials,
      &asset_server,
      Vec2 { x: 1., y: 1. },
      Vec2 { x: 0.5, y: 0.5 },
    );
    let mut x_acc: f32 = 0.0;
    for middle_cabinet in middle_cabinets.into_iter() {
      let translation = middle_cabinet.half_size + vec3(x_acc, MIDDLE_CABINET_Y, 0.);
      commands.spawn((
        Mesh3d(meshes.add(middle_cabinet)),
        MeshMaterial3d(middle_cabinet_material.clone()),
        Transform::from_translation(translation),
        KitchenCabinet,
        ChildOf(common.parent),
      ));
      x_acc += middle_cabinet.size().x;
    }
  }
  {
    let top_cabinets = [
      Cuboid::new(CABINET_WIDTH / 2.0, TOP_CABINET_HEIGHT, TOP_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, TOP_CABINET_HEIGHT, TOP_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, TOP_CABINET_HEIGHT, TOP_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, TOP_CABINET_HEIGHT, TOP_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, TOP_CABINET_HEIGHT, TOP_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH, TOP_CABINET_HEIGHT, TOP_CABINET_DEPTH),
      Cuboid::new(CABINET_WIDTH / 2.0, TOP_CABINET_HEIGHT, TOP_CABINET_DEPTH),
    ];

    let mut x_acc: f32 = 0.0;
    for top_cabinet in top_cabinets.into_iter() {
      let translation = top_cabinet.half_size + vec3(x_acc, TOP_CABINET_Y, 0.);
      commands.spawn((
        Mesh3d(meshes.add(top_cabinet)),
        MeshMaterial3d(common.cabinets_colour.clone()),
        Transform::from_translation(translation),
        KitchenCabinet,
        ChildOf(common.parent),
      ));
      x_acc += top_cabinet.size().x;
    }
    let owen_and_stuff = Cuboid::new(CABINET_WIDTH, FLAT_HEIGHT - BOTTOM_CABINET_Y, BOTTOM_CABINET_DEPTH);
    let translation = owen_and_stuff.half_size + vec3(x_acc, BOTTOM_CABINET_Y, 0.);
    commands.spawn((
      Mesh3d(meshes.add(owen_and_stuff)),
      MeshMaterial3d(common.cabinets_colour.clone()),
      Transform::from_translation(translation),
      KitchenCabinet,
      ChildOf(common.parent),
    ));
  }
  {
    let counter_top_width = 36.;
    let counter_top_depth = 5.6;
    let countertop = Cuboid::new(counter_top_width, 0.2, counter_top_depth);
    let translation = countertop.half_size + vec3(0.0, COUNTERTOP_Y, 0.);
    let material_handle = repeat_texture(
      "kitchen/ambient_light.jpg",
      &mut materials,
      &asset_server,
      Vec2 {
        x: counter_top_width,
        y: counter_top_depth,
      },
      Vec2 { x: 0.05, y: 0.1 },
    );
    commands.spawn((
      Mesh3d(meshes.add(countertop)),
      MeshMaterial3d(material_handle),
      Transform::from_translation(translation),
      KitchenCabinet,
      ChildOf(common.parent),
    ));
  }

  {
    let tap = asset_server.load("kitchen/table.glb#Scene0");
    commands.spawn((
      SceneRoot(tap),
      Transform::from_translation(vec3(30., 0., 25.)),
      KitchenCabinet,
      ChildOf(common.parent),
    ));
  }
}

const VENT_DEPTH: f32 = 8.;
const VENT_WIDTH: f32 = 7.;
const KITCHEN_WALL_LENGTH: f32 = 9.; //6.; // if Emilka doesn't want extra kitchen storage space :(

fn spawn_walls(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, common: Res<KitchenCommon>) {
  {
    let kitchen_vent = Cuboid::new(VENT_WIDTH, FLAT_HEIGHT, VENT_DEPTH);
    let translation = kitchen_vent.half_size + vec3(-VENT_WIDTH, 0., 0.);
    commands.spawn((
      Mesh3d(meshes.add(kitchen_vent)),
      MeshMaterial3d(common.wall_colour.clone()),
      Transform::from_translation(translation),
      ChildOf(common.parent),
    ));
  }
  {
    let kitchen_hall_division_wall = Cuboid::new(KITCHEN_WALL_THICKNESS, FLAT_HEIGHT, KITCHEN_WALL_LENGTH);
    let translation = kitchen_hall_division_wall.half_size + vec3(-VENT_WIDTH, 0., VENT_DEPTH);
    commands.spawn((
      Mesh3d(meshes.add(kitchen_hall_division_wall)),
      MeshMaterial3d(common.wall_colour.clone()),
      Transform::from_translation(translation),
      ChildOf(common.parent),
    ));
  }
}

fn spawn_hall_closet(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
  let transform = Transform::from_translation(KITCHEN_ORIGIN + vec3(EPSILON, 0., VENT_WIDTH + EPSILON));
  commands.spawn((
    Mesh3d(asset_server.load("stl/hall_cabinet.stl")),
    MeshMaterial3d(materials.add(CLOSET_COLOUR)),
    transform,
  ));
}

pub(crate) struct KitchenPlugin;

impl Plugin for KitchenPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_kitchen_common)
      .add_systems(Startup, (setup_kitchen, spawn_walls).after(setup_kitchen_common))
      .add_systems(Startup, spawn_hall_closet);
  }
}
