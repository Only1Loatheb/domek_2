use bevy::prelude::*;

use crate::common::{repeat_texture, BATHROOM_WALL_THICKNESS, BATHROOM_X, BATHROOM_Z, DOOR_Y, FLAT_HEIGHT, LIVING_ROOM_Z};
use bevy::math::vec3;
use std::f32::consts::{FRAC_PI_2, PI};
// https://bevyengine.org/examples/3d-rendering/3d-shapes/

#[derive(Component)]
struct Bathroom;

#[derive(Component)]
struct BathroomWall;

#[derive(Resource)]
struct BathroomCommon {
  parent: Entity,
  wall_material: [Handle<StandardMaterial>; 4],
}

pub(crate) const BATHROOM_ORIGIN: Vec3 = vec3(-BATHROOM_X + BATHROOM_WALL_THICKNESS, 0., LIVING_ROOM_Z + 
BATHROOM_WALL_THICKNESS);

fn setup_bathroom_common(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
  let parent = commands
    .spawn((
      Transform::from_translation(BATHROOM_ORIGIN).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();
  let wall_material = [1, 2, 3, 4].map(|x| {
    repeat_texture(
      format!("massa/PP-Massa-1198x2398-{}.jpg", x),
      &mut materials,
      &asset_server,
      Vec2 { x: 10., y: 10. },
      Vec2 { x: 0.1, y: 0.1 },
    )
  });
  commands.insert_resource(BathroomCommon { parent, wall_material });
}

const LEFT_DOOR_WALL_LENGTH: f32 = 9.3;
const RIGHT_DOOR_WALL_LENGTH: f32 = 6.5;
const DOOR_WIDTH: f32 = 9.2;
const BATHROOM_DEPTH: f32 = BATHROOM_X - BATHROOM_WALL_THICKNESS;
const BATHROOM_WIDTH: f32 = BATHROOM_Z - 2. * BATHROOM_WALL_THICKNESS;
const VENT_DEPTH: f32 = 4.;
const VENT_WIDTH: f32 = 5.5;

fn spawn_walls(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, common: Res<BathroomCommon>) {
  let wall_material = common.wall_material.clone();
  {
    let left_door_wall = Cuboid::new(LEFT_DOOR_WALL_LENGTH, FLAT_HEIGHT, BATHROOM_WALL_THICKNESS);
    let translation = left_door_wall.half_size + vec3(0., 0., 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(left_door_wall)),
        MeshMaterial3d(wall_material[0].clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    let right_door_wall = Cuboid::new(
      RIGHT_DOOR_WALL_LENGTH + BATHROOM_WALL_THICKNESS,
      FLAT_HEIGHT,
      BATHROOM_WALL_THICKNESS,
    );
    let translation = right_door_wall.half_size + vec3(LEFT_DOOR_WALL_LENGTH + DOOR_WIDTH, 0., 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(right_door_wall)),
        MeshMaterial3d(wall_material[1].clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    let over_door_wall = Cuboid::new(DOOR_WIDTH, FLAT_HEIGHT - DOOR_Y, BATHROOM_WALL_THICKNESS);
    let translation = over_door_wall.half_size + vec3(LEFT_DOOR_WALL_LENGTH, DOOR_Y, 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(over_door_wall)),
        MeshMaterial3d(wall_material[2].clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    let left_wall = Cuboid::new(BATHROOM_WALL_THICKNESS, FLAT_HEIGHT, BATHROOM_X);
    let translation = left_wall.half_size + vec3(0., 0., -BATHROOM_X);
    commands
      .spawn((
        Mesh3d(meshes.add(left_wall)),
        MeshMaterial3d(wall_material[3].clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    let right_wall = Cuboid::new(BATHROOM_WALL_THICKNESS, FLAT_HEIGHT, BATHROOM_X);
    let translation = right_wall.half_size + vec3(BATHROOM_WIDTH, 0., -BATHROOM_X);
    commands
      .spawn((
        Mesh3d(meshes.add(right_wall)),
        MeshMaterial3d(wall_material[0].clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    let back_wall = Cuboid::new(BATHROOM_WIDTH + BATHROOM_WALL_THICKNESS, FLAT_HEIGHT, BATHROOM_WALL_THICKNESS);
    let translation = back_wall.half_size + vec3(0., 0., -BATHROOM_DEPTH - BATHROOM_WALL_THICKNESS);
    commands
      .spawn((
        Mesh3d(meshes.add(back_wall)),
        MeshMaterial3d(wall_material[1].clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    // for amogus to go in
    let vent = Cuboid::new(VENT_WIDTH, FLAT_HEIGHT, VENT_DEPTH);
    let translation = vent.half_size + vec3(BATHROOM_WIDTH - VENT_WIDTH, 0., -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(vent)),
        MeshMaterial3d(wall_material[2].clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
}

const SHELF_WALL_THICKNESS: f32 = 0.8;
const SHELF_SPACE_DEPTH: f32 = 1.5;
const SHELF_SPACE_WIDTH: f32 = 2.2;
const SHELF_SPACE_HEIGHT: f32 = 3.0;
const SHELF_EXTENDS_INTO_SHOWER_TO_SUPPORT_SHOWER_STALL: f32 = 0.4;
const EPSILON: f32 = 0.0002;
const SHELF_WIDTH: f32 = SHELF_SPACE_WIDTH + SHELF_WALL_THICKNESS;

const SHELF_DEPTH: f32 = SHELF_SPACE_DEPTH + SHELF_WALL_THICKNESS;
const SHELF_X: f32 = BATHROOM_WALL_THICKNESS + SHOWER_DEPTH - SHELF_EXTENDS_INTO_SHOWER_TO_SUPPORT_SHOWER_STALL;
const BOTTOM_SHELF_Y: f32 = 6.;
const MIDDLE_SHELF_Y: f32 = BOTTOM_SHELF_Y + SHELF_SPACE_HEIGHT + SHELF_WALL_THICKNESS;
const TOP_SHELF_Y: f32 = MIDDLE_SHELF_Y + SHELF_SPACE_HEIGHT + SHELF_WALL_THICKNESS;

fn spawn_shower_shelf(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, common: Res<BathroomCommon>) {
  {
    let shelf_back_wall = Cuboid::new(SHELF_WALL_THICKNESS, FLAT_HEIGHT, SHELF_WIDTH);
    let translation = shelf_back_wall.half_size + vec3(SHELF_X + SHELF_SPACE_DEPTH, 0., -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(shelf_back_wall)),
        MeshMaterial3d(common.wall_material[3].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
  {
    let shelf_side_wall = Cuboid::new(SHELF_SPACE_DEPTH + EPSILON, FLAT_HEIGHT, SHELF_WALL_THICKNESS +  EPSILON);
    let translation = shelf_side_wall.half_size + vec3(SHELF_X - EPSILON, 0., SHELF_SPACE_WIDTH - BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(shelf_side_wall)),
        MeshMaterial3d(common.wall_material[0].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
  {
    let bottom_shelf = Cuboid::new(SHELF_SPACE_DEPTH, BOTTOM_SHELF_Y, SHELF_WIDTH);
    let translation = bottom_shelf.half_size + vec3(SHELF_X, 0., -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(bottom_shelf)),
        MeshMaterial3d(common.wall_material[1].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
  {
    let middle_shelf = Cuboid::new(SHELF_SPACE_DEPTH, SHELF_WALL_THICKNESS, SHELF_WIDTH);
    let translation = middle_shelf.half_size + vec3(SHELF_X, MIDDLE_SHELF_Y - SHELF_WALL_THICKNESS, -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(middle_shelf)),
        MeshMaterial3d(common.wall_material[2].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
  {
    let top_shelf = Cuboid::new(SHELF_SPACE_DEPTH, SHELF_WALL_THICKNESS, SHELF_WIDTH);
    let translation = top_shelf.half_size + vec3(SHELF_X, TOP_SHELF_Y - SHELF_WALL_THICKNESS, -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(top_shelf)),
        MeshMaterial3d(common.wall_material[3].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
  {
    let shelves_overhang = Cuboid::new(SHELF_SPACE_DEPTH, FLAT_HEIGHT - TOP_SHELF_Y - SHELF_SPACE_HEIGHT, SHELF_WIDTH);
    let translation = shelves_overhang.half_size + vec3(SHELF_X, TOP_SHELF_Y + SHELF_SPACE_HEIGHT, -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(shelves_overhang)),
        MeshMaterial3d(common.wall_material[0].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
}

// umywalka https://catalog.geberit.pl/pl-PL/product/PRO_1832860
// szafka https://catalog.geberit.pl/pl-PL/product/PRO_634571
const SINK_DEPTH: f32 = 4.8;
const SINK_WIDTH: f32 = 7.45;

fn spawn_sink(mut commands: Commands, asset_server: Res<AssetServer>, common: Res<BathroomCommon>) {
  let model_handle = asset_server.load("bathroom/umywalka_geberit_500.249.00.2.glb#Scene0");
  commands
    .spawn(SceneBundle {
      scene: SceneRoot(model_handle.clone()),
      transform: Transform {
        translation: vec3(SHELF_X + SHELF_DEPTH + 0.5 * SINK_WIDTH + 0.1, 0., -BATHROOM_DEPTH),
        rotation: Quat::from_rotation_x(-PI / 2.0),
        scale: Vec3::splat(0.1),
      },
      ..default()
    })
    .set_parent(common.parent);
  // {
  //   let material = materials.add(Color::WHITE);
  //   let sink_cabinet = Cuboid::new(FLUSH_DEPTH, FLUSH_HEIGHT, FLUSH_WIDTH);
  //   let translation = sink_cabinet.half_size + vec3(BATHROOM_WIDTH - FLUSH_DEPTH, 0., VENT_DEPTH - BATHROOM_DEPTH);
  //   commands
  //     .spawn((
  //       Mesh3d(meshes.add(sink_cabinet)),
  //       MeshMaterial3d(material),
  //       Transform::from_translation(translation),
  //       Bathroom,
  //     ))
  //     .set_parent(parent.parent);
  // }
}

const SHOWER_DEPTH: f32 = 9.;
const SHOWER_WIDTH: f32 = 12.;

// kabina https://www.radaway.pl/kategoria/furo-brushed-copper-kdj/#
// albo https://www.radaway.pl/kategoria/furo-sl-brushed-copper-kdd/
fn spawn_shower_stall(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
  common: Res<BathroomCommon>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // let model_handle = asset_server.load("bathroom/Furo-KDJ-Stabilizacja-krzyzowa.glb#Scene0");
  let model_handle = asset_server.load("bathroom/furo-sl-kdd__blend.glb#Scene0");
  let translation = vec3(BATHROOM_WALL_THICKNESS, 0., -BATHROOM_DEPTH);
  commands
    .spawn(SceneBundle {
      scene: SceneRoot(model_handle.clone()),
      transform: Transform {
        // translation: translation.with_z(translation.z + SHELF_WIDTH),
        translation: translation.with_z(translation.z + SHELF_WIDTH) + vec3(-0.4, -0.4, -0.4),
        rotation: Quat::from_rotation_y(PI / 2.0),
        // scale: Vec3::new(-10.0 * (SHOWER_WIDTH - SHELF_WIDTH) / SHOWER_WIDTH, 10., 10. * SHOWER_DEPTH / 8.),
        scale: Vec3::new(-9.25, 10., 9.25),
      },
      ..default()
    })
    .set_parent(common.parent);
  {
    // just to be sure that measurements are right
    let material = materials.add(Color::WHITE);
    let shower_tray = Cuboid::new(SHOWER_DEPTH, 0.1, SHOWER_WIDTH);
    commands
      .spawn((
        Mesh3d(meshes.add(shower_tray)),
        MeshMaterial3d(material),
        Transform::from_translation(shower_tray.half_size + translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
}
const CABINET_WIDTH: f32 = 6.;
const CABINET_DEPTH: f32 = 4.;

fn spawn_cabinet(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  common: Res<BathroomCommon>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let material = materials.add(Color::WHITE);
  let cabinet = Cuboid::new(CABINET_DEPTH, FLAT_HEIGHT, CABINET_WIDTH);
  let translation = cabinet.half_size + vec3(BATHROOM_WALL_THICKNESS, 0., -CABINET_WIDTH);
  commands
    .spawn((
      Mesh3d(meshes.add(cabinet)),
      MeshMaterial3d(material),
      Transform::from_translation(translation),
      Bathroom,
    ))
    .set_parent(common.parent);
}

const FLUSH_DEPTH: f32 = 2.;
const FLUSH_HEIGHT: f32 = 12.;
const FLUSH_WIDTH: f32 = 7.;

fn spawn_toilet(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, common: Res<BathroomCommon>) {
  let model_handle = asset_server.load("bathroom/muszla.glb#Scene0");
  commands
    .spawn(SceneBundle {
      scene: SceneRoot(model_handle.clone()),
      transform: Transform {
        translation: vec3(BATHROOM_WIDTH - FLUSH_DEPTH, 0., 0.5 * FLUSH_WIDTH + VENT_DEPTH - BATHROOM_DEPTH),
        rotation: Quat::from_rotation_y(-PI / 2.0),
        scale: Vec3::new(10.0, 10., 10.),
      },
      ..default()
    })
    .set_parent(common.parent);
  {
    let toilet_flush = Cuboid::new(FLUSH_DEPTH, FLUSH_HEIGHT, FLUSH_WIDTH);
    let translation = toilet_flush.half_size + vec3(BATHROOM_WIDTH - FLUSH_DEPTH, 0., VENT_DEPTH - BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(toilet_flush)),
        MeshMaterial3d(common.wall_material[1].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
}

const WASHING_MACHINE_HALF_WIDTH: f32 = 3.;
const WASHING_MACHINE_HEIGHT: f32 = 8.5;

fn spawn_washing_machine(mut commands: Commands, asset_server: Res<AssetServer>, common: Res<BathroomCommon>) {
  let model_handle = asset_server.load("bathroom/washing_machine.glb#Scene0");
  let transform = Transform::from_scale(Vec3::splat(10.0)).with_rotation(Quat::from_rotation_y(-FRAC_PI_2));
  commands
    .spawn(SceneBundle {
      scene: SceneRoot(model_handle.clone()),
      transform: transform.with_translation(vec3(
        BATHROOM_WIDTH - WASHING_MACHINE_HALF_WIDTH,
        WASHING_MACHINE_HEIGHT / 2.0,
        -WASHING_MACHINE_HALF_WIDTH,
      )),
      ..default()
    })
    .set_parent(common.parent);
  commands
    .spawn(SceneBundle {
      scene: SceneRoot(model_handle),
      transform: transform.with_translation(vec3(
        BATHROOM_WIDTH - WASHING_MACHINE_HALF_WIDTH,
        1.5 * WASHING_MACHINE_HEIGHT,
        -WASHING_MACHINE_HALF_WIDTH,
      )),
      ..default()
    })
    .set_parent(common.parent);
}

pub(crate) struct BathroomPlugin;

impl Plugin for BathroomPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_bathroom_common).add_systems(
      Startup,
      (
        spawn_walls,
        spawn_washing_machine,
        spawn_shower_stall,
        spawn_cabinet,
        spawn_toilet,
        spawn_sink,
        spawn_shower_shelf,
      )
        .after(setup_bathroom_common),
    );
  }
}
