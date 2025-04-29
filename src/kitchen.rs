use crate::common::*;
use bevy::math::vec3;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;
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
  let bottom_cabinets = [
    Cuboid::new(CABINET_WIDTH / 2.0, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH / 2.0, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
  ];

  {
    let mut x_acc: f32 = 0.0;
    for bottom_cabinet in bottom_cabinets.into_iter() {
      let translation = bottom_cabinet.half_size + vec3(x_acc, BOTTOM_CABINET_Y, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(bottom_cabinet)),
          MeshMaterial3d(common.cabinets_colour.clone()),
          Transform::from_translation(translation),
          KitchenCabinet,
        ))
        .set_parent(common.parent);
      x_acc += bottom_cabinet.size().x;
    }
  }

  let middle_cabinets = [
    Cuboid::new(CABINET_WIDTH / 2.0, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
    Cuboid::new(CABINET_WIDTH / 2.0, MIDDLE_CABINET_HEIGHT, MIDDLE_CABINET_DEPTH),
  ];

  {
    let middle_cabinet_material = repeat_texture(
      "kitchen/dab_vicenza.jpg",
      &mut materials,
      &asset_server,
      Vec2 { x: 1., y: 1. },
      Vec2 { x: 0.5, y: 0.77 },
    );
    let mut x_acc: f32 = 0.0;
    for middle_cabinet in middle_cabinets.into_iter() {
      let translation = middle_cabinet.half_size + vec3(x_acc, MIDDLE_CABINET_Y, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(middle_cabinet)),
          MeshMaterial3d(middle_cabinet_material.clone()),
          Transform::from_translation(translation),
          KitchenCabinet,
        ))
        .set_parent(common.parent);
      x_acc += middle_cabinet.size().x;
    }
  }

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
    commands
      .spawn((
        Mesh3d(meshes.add(top_cabinet)),
        MeshMaterial3d(common.cabinets_colour.clone()),
        Transform::from_translation(translation),
        KitchenCabinet,
      ))
      .set_parent(common.parent);
    x_acc += top_cabinet.size().x;
  }
  let owen_and_stuff = Cuboid::new(CABINET_WIDTH, FLAT_HEIGHT - BOTTOM_CABINET_Y, BOTTOM_CABINET_DEPTH);
  let translation = owen_and_stuff.half_size + vec3(x_acc, BOTTOM_CABINET_Y, 0.);
  commands
    .spawn((
      Mesh3d(meshes.add(owen_and_stuff)),
      MeshMaterial3d(common.cabinets_colour.clone()),
      Transform::from_translation(translation),
      KitchenCabinet,
    ))
    .set_parent(common.parent);

  {
    let counter_top_width = 36.;
    let counter_top_depth = 6.;
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
    commands
      .spawn((
        Mesh3d(meshes.add(countertop)),
        MeshMaterial3d(material_handle),
        Transform::from_translation(translation),
        KitchenCabinet,
      ))
      .set_parent(common.parent);
  }
}

const VENT_DEPTH: f32 = 8.;
const VENT_WIDTH: f32 = 7.;
const KITCHEN_WALL_LENGTH: f32 = 9.; //6.; // if Emilka doesn't want extra kitchen storage space :(

fn spawn_walls(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, common: Res<KitchenCommon>) {
  {
    let kitchen_vent = Cuboid::new(VENT_WIDTH, FLAT_HEIGHT, VENT_DEPTH);
    let translation = kitchen_vent.half_size + vec3(-VENT_WIDTH, 0., 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(kitchen_vent)),
        MeshMaterial3d(common.wall_colour.clone()),
        Transform::from_translation(translation),
      ))
      .set_parent(common.parent);
  }
  {
    let kitchen_hall_division_wall = Cuboid::new(KITCHEN_WALL_THICKNESS, FLAT_HEIGHT, KITCHEN_WALL_LENGTH);
    let translation = kitchen_hall_division_wall.half_size + vec3(-VENT_WIDTH, 0., VENT_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(kitchen_hall_division_wall)),
        MeshMaterial3d(common.wall_colour.clone()),
        Transform::from_translation(translation),
      ))
      .set_parent(common.parent);
  }
}

const CLOSET_HEIGHT: f32 = FLAT_HEIGHT;
const CLOSED_DEPTH: f32 = 6.8;
const CLOSET_WIDTH: f32 = VENT_DEPTH + KITCHEN_WALL_LENGTH;
const BROOM_COMPARTMENT_WIDTH: f32 = 1.5;
const DRAWERS_WIDTH: f32 = 5.;
const SLIDING_DOORS_DEPTH: f32 = 1.;
const MIDDLE_PLANK_DEPTH: f32 = CLOSED_DEPTH - SLIDING_DOORS_DEPTH;
const MIDDLE_VERTICAL_PLANK_HEIGHT: f32 = CLOSET_HEIGHT - PLANK_THICKNESS;
const MIDDLE_HORIZONTAL_PLANK_Y: f32 = 20.0;
const TOP_HORIZONTAL_DIVIDER_PLANK_Y: f32 = MIDDLE_HORIZONTAL_PLANK_Y + 0.5 * (CLOSET_HEIGHT - MIDDLE_HORIZONTAL_PLANK_Y);
const HANGER_ROD_Y: f32 = 19.0;
const SHOES_DRAWER_TOP_Y: f32 = 5.0;
const SHOES_DRAWER_MIDDLE_Y: f32 = 2.5;
const HANGER_SPACE_W: f32 = CLOSET_WIDTH - 3. * PLANK_THICKNESS - BROOM_COMPARTMENT_WIDTH - DRAWERS_WIDTH;
const BROOM_X: f32 = CLOSET_WIDTH - 2. * PLANK_THICKNESS - BROOM_COMPARTMENT_WIDTH;
const MAX_DRAWER_Y: f32 = 16.;

fn spawn_hall_closet(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
  let parent = commands
    .spawn((
      Transform::from_translation(KITCHEN_ORIGIN + vec3(EPSILON, 0., VENT_WIDTH + EPSILON)),
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();

  let closet_colour = materials.add(NOT_BEIGE);
  {
    {
      let top_plank = Cuboid::new(CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, CLOSED_DEPTH);
      let just_under_the_ceiling = top_plank.half_size + vec3(PLANK_THICKNESS, MIDDLE_VERTICAL_PLANK_HEIGHT, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(top_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(just_under_the_ceiling),
        ))
        .set_parent(parent);
    }
    {
      {
        let middle_horizontal_plank = Cuboid::new(CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH);
        let over_the_hanger_rod = middle_horizontal_plank.half_size + vec3(PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, 0.);
        commands
          .spawn((
            Mesh3d(meshes.add(middle_horizontal_plank)),
            MeshMaterial3d(closet_colour.clone()),
            Transform::from_translation(over_the_hanger_rod),
          ))
          .set_parent(parent);
      }
      {
        let top_horizontal_divider_plank = Cuboid::new(CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH);
        let half_way_there = top_horizontal_divider_plank.half_size + vec3(PLANK_THICKNESS, TOP_HORIZONTAL_DIVIDER_PLANK_Y, 0.);
        commands
          .spawn((
            Mesh3d(meshes.add(top_horizontal_divider_plank)),
            MeshMaterial3d(closet_colour.clone()),
            Transform::from_translation(half_way_there),
          ))
          .set_parent(parent);
      }
    }
    {
      let hanger_rod = Cuboid::new(HANGER_SPACE_W, PLANK_THICKNESS, PLANK_THICKNESS);
      let like_in_our_closet = hanger_rod.half_size + vec3(PLANK_THICKNESS, HANGER_ROD_Y, 0.5 * CLOSED_DEPTH);
      commands
        .spawn((
          Mesh3d(meshes.add(hanger_rod)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(like_in_our_closet),
        ))
        .set_parent(parent);
    }
    {
      let shoes_drawer_top_plank = Cuboid::new(BROOM_X, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH);
      let like_in_our_hall_drawer = shoes_drawer_top_plank.half_size + vec3(PLANK_THICKNESS, SHOES_DRAWER_TOP_Y, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(shoes_drawer_top_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(like_in_our_hall_drawer),
        ))
        .set_parent(parent);
    }
    {
      let shoes_drawer_top_plank = Cuboid::new(BROOM_X, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH);
      let like_in_our_hall_drawer = shoes_drawer_top_plank.half_size + vec3(PLANK_THICKNESS, SHOES_DRAWER_MIDDLE_Y, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(shoes_drawer_top_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(like_in_our_hall_drawer),
        ))
        .set_parent(parent);
    }
  }
  {
    let side_plank = Cuboid::new(PLANK_THICKNESS, CLOSET_HEIGHT, CLOSED_DEPTH);
    {
      let entrance_side_plank = side_plank.half_size + vec3(0., 0., 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(side_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(entrance_side_plank),
        ))
        .set_parent(parent);
    }
    {
      let bathroom_side_plank = side_plank.half_size + vec3(CLOSET_WIDTH - PLANK_THICKNESS, 0., 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(side_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(bathroom_side_plank),
        ))
        .set_parent(parent);
    }
  }
  {
    {
      let broom_compartment_and_drawers_divider_plank = Cuboid::new(PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, MIDDLE_PLANK_DEPTH);
      let broom_compartment_and_drawers_divider = broom_compartment_and_drawers_divider_plank.half_size + vec3(BROOM_X, 0., 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(broom_compartment_and_drawers_divider_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(broom_compartment_and_drawers_divider),
        ))
        .set_parent(parent);
    }
    {
      let drawers_and_hanging_space_divider_plank = Cuboid::new(PLANK_THICKNESS, MIDDLE_VERTICAL_PLANK_HEIGHT, MIDDLE_PLANK_DEPTH);
      let drawers_and_hanging_space_divider = drawers_and_hanging_space_divider_plank.half_size + vec3(HANGER_SPACE_W, 0., 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(drawers_and_hanging_space_divider_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(drawers_and_hanging_space_divider),
        ))
        .set_parent(parent);
    }
  }
  {
    let drawer_spacing_y = 0.05;
    let drawer_spacing_x = 0.05;
    let drawer_x = HANGER_SPACE_W + PLANK_THICKNESS + drawer_spacing_x;
    let start_y = SHOES_DRAWER_TOP_Y + PLANK_THICKNESS + drawer_spacing_y;
    let num_drawers = 5;
    let drawer_height = (MAX_DRAWER_Y - start_y) / num_drawers as f32 - drawer_spacing_y;
    let drawer_depth = MIDDLE_PLANK_DEPTH;
    
    let drawer = Cuboid::new(DRAWERS_WIDTH - 2. * drawer_spacing_x, drawer_height, drawer_depth);
    for i in 0..num_drawers {
      let y = start_y + i as f32 * (drawer_height + drawer_spacing_y);
      commands
        .spawn((
          Mesh3d(meshes.add(drawer)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(drawer.half_size + Vec3::new(drawer_x, y, 0.0)),
        ))
        .set_parent(parent);
    }
    if MAX_DRAWER_Y + PLANK_THICKNESS < MIDDLE_HORIZONTAL_PLANK_Y {
      let shoes_drawer_top_plank = Cuboid::new(DRAWERS_WIDTH, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH);
      let like_in_our_hall_drawer = shoes_drawer_top_plank.half_size + vec3(HANGER_SPACE_W + PLANK_THICKNESS, MAX_DRAWER_Y, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(shoes_drawer_top_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(like_in_our_hall_drawer),
        ))
        .set_parent(parent);
    }
  }
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
