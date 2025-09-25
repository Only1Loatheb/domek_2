use crate::common::*;
use crate::control::*;
use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
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

const COUNTERTOP_HEIGHT: f32 = 0.2;
const GAP_BETWEEN_CABINETS_HEIGHT: f32 = 5.10;

const MIDDLE_CABINET_Y: f32 = COUNTERTOP_Y + COUNTERTOP_HEIGHT + GAP_BETWEEN_CABINETS_HEIGHT;
const MIDDLE_CABINET_DEPTH: f32 = 3.20; // guesstimate
const MIDDLE_CABINET_HEIGHT: f32 = BOTTOM_CABINET_HEIGHT + TOP_CABINET_HEIGHT;

const TOP_CABINET_Y: f32 = 0.; // MIDDLE_CABINET_Y + MIDDLE_CABINET_HEIGHT;
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

const CABINET_WIDTHS: [f32; 7] = [
  0.5 * CABINET_WIDTH,
  CABINET_WIDTH,
  CABINET_WIDTH,
  CABINET_WIDTH,
  CABINET_WIDTH,
  CABINET_WIDTH,
  0.5 * CABINET_WIDTH,
];

fn setup_kitchen(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  asset_server: Res<AssetServer>,
  common: Res<KitchenCommon>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  {
    let cabinet_handle_material = materials.add(Color::linear_rgb(255.0 / 512.0, 242. / 512.0, 207. / 512.0));
    let cabinet_handle: Handle<Mesh> = asset_server.load("stl/kitchen_handle.stl");
    let mut x_acc: f32 = 0.0;
    for bottom_cabinet_width in CABINET_WIDTHS.into_iter() {
      let bottom_cabinet = Cuboid::new(bottom_cabinet_width - 0.05, BOTTOM_CABINET_HEIGHT, EPSILON);
      let translation = bottom_cabinet.half_size + vec3(x_acc, BOTTOM_CABINET_Y, BOTTOM_CABINET_DEPTH - EPSILON);
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
          .with_scale(vec3(0.02, 0.02, 0.01 * (bottom_cabinet.size().x - 0.1) / CABINET_WIDTH))
          .with_rotation(Quat::from_euler(EulerRot::XYZ, 0., -FRAC_PI_2, 0.))
      }
      {
        commands.spawn((
          Mesh3d(cabinet_handle.clone()),
          MeshMaterial3d(cabinet_handle_material.clone()),
          cabinet_handle_transform(bottom_cabinet),
          ChildOf(id),
        ));

        commands.spawn((
          Mesh3d(cabinet_handle.clone()),
          MeshMaterial3d(cabinet_handle_material.clone()),
          cabinet_handle_transform(bottom_cabinet).with_translation(Vec3::ZERO.with_z(bottom_cabinet.half_size.z)),
          ChildOf(id),
        ));
      }
      x_acc += bottom_cabinet_width;
    }
  }
  {
    let middle_cabinet_material = repeat_texture(
      "kitchen/dab_vicenza.jpg",
      &mut materials,
      &asset_server,
      Vec2 { x: 1., y: 1. },
      Vec2 { x: 0.5, y: 0.5 },
    );
    let mut x_acc: f32 = 0.0;
    for (i, middle_cabinet_width) in CABINET_WIDTHS.into_iter().enumerate() {
      let middle_cabinet = Cuboid::new(middle_cabinet_width - 0.05, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH);
      let translation = middle_cabinet.half_size + vec3(x_acc, MIDDLE_CABINET_Y, 0.);
      commands.spawn((
        Mesh3d(meshes.add(middle_cabinet)),
        MeshMaterial3d(middle_cabinet_material.clone()),
        Transform::from_translation(translation),
        KitchenCabinet,
        ChildOf(common.parent),
      ));
      let shadows_enabled = (i == 0) || (i == (CABINET_WIDTHS.len() - 1));
      commands.spawn((
        Transform::from_translation(vec3(
          x_acc + middle_cabinet.half_size.x,
          MIDDLE_CABINET_Y - 0.3,
          middle_cabinet.half_size.z,
        ))
        .looking_at(translation.with_y(0.), Vec3::Y),
        PointLight {
          intensity: 100_000.0,
          range: MIDDLE_CABINET_HEIGHT,
          color: Color::WHITE,
          shadows_enabled: shadows_enabled,
          ..default()
        },
        ChildOf(common.parent),
      ));
      x_acc += middle_cabinet_width;
    }
  }
  {
    let mut x_acc: f32 = 0.0;
    for top_cabinet_width in CABINET_WIDTHS.into_iter() {
      let top_cabinet = Cuboid::new(top_cabinet_width, TOP_CABINET_HEIGHT, TOP_CABINET_DEPTH);
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
    let coffee_machine = asset_server.load("kitchen/coffee_machine.glb#Scene0");
    commands
      .spawn((
        SceneRoot(coffee_machine),
        Transform::from_translation(vec3(1.5, COUNTERTOP_Y + COUNTERTOP_HEIGHT, 1.8)).with_scale(vec3(10.0, 10.0, 10.0)),
        KitchenCabinet,
        ChildOf(common.parent),
      ))
      .observe(on_pressed_save)
      .observe(on_released_clear);
  }
  let countertop_width = CABINET_WIDTHS.iter().sum::<f32>();
  {
    let sink_cabinet_index = 1;
    let sink_width = 5.0;
    let sink_side_margin: f32 = 0.5 * (CABINET_WIDTH - sink_width);
    let counter_top_left_width = CABINET_WIDTHS.iter().take(sink_cabinet_index).sum::<f32>() + sink_side_margin;
    let counter_top_right_width = countertop_width - sink_width + 0.5 * sink_side_margin - counter_top_left_width;
    let counter_top_depth = 5.6;
    {
      let left_countertop = Cuboid::new(counter_top_left_width, COUNTERTOP_HEIGHT, counter_top_depth);
      let left_material = repeat_texture(
        "kitchen/ambient_light.jpg",
        &mut materials,
        &asset_server,
        Vec2 {
          x: counter_top_left_width,
          y: counter_top_depth,
        },
        Vec2 { x: 0.05, y: 0.1 },
      );
      commands.spawn((
        Mesh3d(meshes.add(left_countertop)),
        MeshMaterial3d(left_material),
        Transform::from_translation(left_countertop.half_size + vec3(0.0, COUNTERTOP_Y, 0.)),
        KitchenCabinet,
        ChildOf(common.parent),
      ));
    }
    {
      let depth = 0.5 * (counter_top_depth - 4.0);
      let front_countertop = Cuboid::new(sink_width, COUNTERTOP_HEIGHT, depth);
      let front_material = repeat_texture(
        "kitchen/ambient_light.jpg",
        &mut materials,
        &asset_server,
        Vec2 { x: sink_width, y: depth },
        Vec2 { x: 0.05, y: 0.1 },
      );
      commands.spawn((
        Mesh3d(meshes.add(front_countertop)),
        MeshMaterial3d(front_material.clone()),
        Transform::from_translation(front_countertop.half_size + vec3(counter_top_left_width, COUNTERTOP_Y, 0.)),
        KitchenCabinet,
        ChildOf(common.parent),
      ));
      commands.spawn((
        Mesh3d(meshes.add(front_countertop)),
        MeshMaterial3d(front_material),
        Transform::from_translation(front_countertop.half_size + vec3(counter_top_left_width, COUNTERTOP_Y, counter_top_depth - depth)),
        KitchenCabinet,
        ChildOf(common.parent),
      ));
      let kitchen_sink = asset_server.load("kitchen/kitchen_sink.glb#Scene0");
      commands.spawn((
        SceneRoot(kitchen_sink),
        Transform::from_translation(vec3(counter_top_left_width + 0.5 * sink_width + 0.5, COUNTERTOP_Y + 0.1, 5.2))
          .with_scale(vec3(7.0, 10.0, 10.0)),
        KitchenCabinet,
        ChildOf(common.parent),
      ));
    }
    {
      let right_countertop = Cuboid::new(counter_top_right_width, COUNTERTOP_HEIGHT, counter_top_depth);
      let right_material = repeat_texture(
        "kitchen/ambient_light.jpg",
        &mut materials,
        &asset_server,
        Vec2 {
          x: counter_top_right_width,
          y: counter_top_depth,
        },
        Vec2 { x: 0.05, y: 0.1 },
      );
      commands.spawn((
        Mesh3d(meshes.add(right_countertop)),
        MeshMaterial3d(right_material),
        Transform::from_translation(right_countertop.half_size + vec3(counter_top_left_width + sink_width, COUNTERTOP_Y, 0.)),
        KitchenCabinet,
        ChildOf(common.parent),
      ));
    }
  }

  {
    let table_pos = vec3(26., 0., 26.);
    {
      let table = asset_server.load("kitchen/table.glb#Scene0");
      commands.spawn((
        SceneRoot(table),
        Transform::from_translation(table_pos),
        KitchenCabinet,
        ChildOf(common.parent),
      ));
    }
    for (x, y, size) in [
      (25., 21., 30),
      (17., 21., 40),
      (0., 40., 30),
      (table_pos.x, table_pos.z, 60),
      (35., 40., 50),
    ] {
      {
        let lamp = asset_server.load(format!("kitchen/Marcus_{}_lamp.glb#Scene0", size));
        commands.spawn((
          SceneRoot(lamp),
          Transform::from_translation(vec3(x, FLAT_HEIGHT - 5., y)).with_scale(Vec3::splat(0.1)),
          KitchenCabinet,
          ChildOf(common.parent),
        ));
      }
      {
        commands.spawn((
          Transform::from_translation(vec3(x, 15., y)).looking_at(table_pos, Vec3::Y),
          PointLight {
            intensity: 4_000_000.0,
            range: 4. * FLAT_HEIGHT,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
          },
          ChildOf(common.parent),
        ));
      }
    }
  }
  {
    let transform = Transform {
      translation: vec3(-15., 0., 32.),
      rotation: Quat::from_rotation_x(-FRAC_PI_2) * Quat::from_rotation_z(PI),
      scale: Vec3::ONE,
    };
    commands.spawn((
      Mesh3d(asset_server.load("stl/sofa.stl")),
      MeshMaterial3d(materials.add(Color::hsl(32., 17. / 255., 223. / 255.))),
      transform,
      ChildOf(common.parent),
    ));
  }
  {
    let induction_texture = materials.add(StandardMaterial {
      base_color_texture: Some(asset_server.load("kitchen/induction-ELECTROLUX-EIV63440BW-SLIM-FIT.jpg")),
      ..default()
    });
    let induction_cube = Cuboid::new(5.2, 0.2, 5.3);
    let induction_cabinet_index = 4;
    commands.spawn((
      Mesh3d(meshes.add(induction_cube)),
      MeshMaterial3d(induction_texture),
      Transform::from_translation(vec3(
        CABINET_WIDTHS.iter().take(induction_cabinet_index).sum::<f32>() + 0.5 * CABINET_WIDTHS[induction_cabinet_index],
        COUNTERTOP_Y + COUNTERTOP_HEIGHT,
        BOTTOM_CABINET_DEPTH - induction_cube.half_size.z - EPSILON,
      )),
      KitchenCabinet,
      ChildOf(common.parent),
    ));
  }
  let oven_y = COUNTERTOP_Y - 2.0;
  {
    let oven_texture = materials.add(StandardMaterial {
      base_color_texture: Some(asset_server.load("kitchen/oven-WHIRLPOOL-WOI5S8PM2SEA-front.jpg")),
      ..default()
    });
    let oven_cube = Cuboid::new(5.95, 5.97, 5.64);
    commands.spawn((
      Mesh3d(meshes.add(oven_cube)),
      MeshMaterial3d(oven_texture),
      Transform::from_translation(oven_cube.half_size + vec3(countertop_width + EPSILON, oven_y, 0.)),
      KitchenCabinet,
      ChildOf(common.parent),
    ));
  }
  {
    let microwave_oven_texture = materials.add(StandardMaterial {
      base_color_texture: Some(asset_server.load("kitchen/microwave-oven-WHIRLPOOL-WMD54MBG-front.jpg")),
      ..default()
    });
    let microwave_oven_cube = Cuboid::new(5.95, 3.83, 5.64);
    commands.spawn((
      Mesh3d(meshes.add(microwave_oven_cube)),
      MeshMaterial3d(microwave_oven_texture),
      Transform::from_translation(microwave_oven_cube.half_size + vec3(countertop_width + EPSILON, oven_y + 6.0, 0.)),
      KitchenCabinet,
      ChildOf(common.parent),
    ));
  }
}

const VENT_DEPTH: f32 = 8.;
const VENT_WIDTH: f32 = 7.;
const KITCHEN_WALL_LENGTH: f32 = 9.; //6.; // if Emilka doesn't want extra kitchen storage space :(
const FRIDGE_DIM: f32 = 6.;

fn spawn_walls_and_fridge(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, common: Res<KitchenCommon>) {
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
  {
    let cargo_width = KITCHEN_WALL_LENGTH - FRIDGE_DIM;
    let cargo = Cuboid::new(6.0, FLAT_HEIGHT, cargo_width - 0.05);
    let translation = cargo.half_size + vec3(-VENT_WIDTH + KITCHEN_WALL_THICKNESS, 0., VENT_DEPTH + FRIDGE_DIM + 0.05);
    commands.spawn((
      Mesh3d(meshes.add(cargo)),
      MeshMaterial3d(common.wall_colour.clone()),
      Transform::from_translation(translation),
      ChildOf(common.parent),
    ));
  }
  {
    let fridge = Cuboid::new(6.0, FLAT_HEIGHT, FRIDGE_DIM);
    let translation = fridge.half_size + vec3(-VENT_WIDTH + KITCHEN_WALL_THICKNESS, 0., VENT_DEPTH + 0.02);
    commands.spawn((
      Mesh3d(meshes.add(fridge)),
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
      .add_systems(Startup, (setup_kitchen, spawn_walls_and_fridge).after(setup_kitchen_common))
      .add_systems(Startup, spawn_hall_closet);
  }
}
