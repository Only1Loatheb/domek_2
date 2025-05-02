use bevy::prelude::*;

use crate::common::{
  repeat_texture, BATHROOM_WALL_THICKNESS, BATHROOM_X, BATHROOM_Z, BEIGE, DOOR_Y, EPSILON, FLAT_HEIGHT, LIVING_ROOM_TO_BATHROOM_Z,
  NOT_BEIGE, PLANK_THICKNESS, TILE_PLUS_GLUE,
};
use bevy::asset::AssetContainer;
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
  massa_tail: [Handle<StandardMaterial>; 4],
  honey_wood_tail: [Handle<StandardMaterial>; 3],
  beige: Handle<StandardMaterial>,
}

pub(crate) const BATHROOM_ORIGIN: Vec3 = vec3(-BATHROOM_X + BATHROOM_WALL_THICKNESS, 0., LIVING_ROOM_TO_BATHROOM_Z);
const RIGHT_WALL_X: f32 = BATHROOM_WIDTH + BATHROOM_WALL_THICKNESS - TILE_PLUS_GLUE;

fn setup_bathroom_common(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
  let parent = commands
    .spawn((
      Transform::from_translation(BATHROOM_ORIGIN).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();
  let massa_tail = [1, 2, 3, 4].map(|x| {
    repeat_texture(
      format!("massa/PP-Massa-1198x2398-{}.jpg", x),
      &mut materials,
      &asset_server,
      Vec2 { x: 10., y: 10. },
      Vec2 { x: 0.1, y: 0.1 },
    )
  });
  let honey_wood_tail = [1, 2, 3].map(|x| {
    repeat_texture(
      format!("honey_wood/PP-Honey-wood-beige-1198x2748-f{}.jpg", x),
      &mut materials,
      &asset_server,
      Vec2 { x: 10., y: 10. },
      Vec2 { x: 0.1, y: 0.1 },
    )
  });
  commands.insert_resource(BathroomCommon {
    parent,
    massa_tail,
    honey_wood_tail,
    beige: materials.add(BEIGE),
  });
}

const LEFT_DOOR_WALL_LENGTH: f32 = 9.3;
const RIGHT_DOOR_WALL_LENGTH: f32 = 6.5;
const DOOR_WIDTH: f32 = 9.2;
const BATHROOM_DEPTH: f32 = BATHROOM_X - BATHROOM_WALL_THICKNESS;
const BATHROOM_WIDTH: f32 = BATHROOM_Z - 2. * BATHROOM_WALL_THICKNESS;
const VENT_DEPTH: f32 = 4. + TILE_PLUS_GLUE;
const VENT_WIDTH: f32 = 5.5 + TILE_PLUS_GLUE;

fn spawn_walls(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, common: Res<BathroomCommon>) {
  {
    let left_door_wall_size = vec3(
      BATHROOM_WALL_THICKNESS + LEFT_DOOR_WALL_LENGTH,
      FLAT_HEIGHT,
      BATHROOM_WALL_THICKNESS,
    );
    let left_door_wall = Cuboid::from_size(left_door_wall_size.clone());
    let translation = left_door_wall.half_size;
    commands
      .spawn((
        Mesh3d(meshes.add(left_door_wall)),
        MeshMaterial3d(common.massa_tail[0].clone()),
        Transform::from_translation(translation + vec3(0., 0., -TILE_PLUS_GLUE)),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
    commands
      .spawn((
        Mesh3d(meshes.add(Cuboid::from_size(left_door_wall_size + vec3(4. * EPSILON, 0., 0.)))),
        MeshMaterial3d(common.beige.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    let right_door_wall_size = vec3(
      RIGHT_DOOR_WALL_LENGTH + BATHROOM_WALL_THICKNESS,
      FLAT_HEIGHT,
      BATHROOM_WALL_THICKNESS,
    );
    let right_door_wall = Cuboid::from_size(right_door_wall_size);
    let translation = right_door_wall.half_size + vec3(LEFT_DOOR_WALL_LENGTH + DOOR_WIDTH + BATHROOM_WALL_THICKNESS, 0., 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(right_door_wall)),
        MeshMaterial3d(common.massa_tail[1].clone()),
        Transform::from_translation(translation + vec3(0., 0., -TILE_PLUS_GLUE)),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
    commands
      .spawn((
        Mesh3d(meshes.add(Cuboid::from_size(right_door_wall_size + vec3(4. * EPSILON, 0., 0.)))),
        MeshMaterial3d(common.beige.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    {
      let over_door_wall = Cuboid::new(DOOR_WIDTH, FLAT_HEIGHT - DOOR_Y - EPSILON, BATHROOM_WALL_THICKNESS);
      let translation = over_door_wall.half_size + vec3(LEFT_DOOR_WALL_LENGTH + BATHROOM_WALL_THICKNESS, DOOR_Y, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(over_door_wall)),
          MeshMaterial3d(common.massa_tail[2].clone()),
          Transform::from_translation(translation + vec3(0., EPSILON, -TILE_PLUS_GLUE)),
          Bathroom,
          BathroomWall,
        ))
        .set_parent(common.parent);
    }
    {
      let over_door_wall = Cuboid::new(DOOR_WIDTH, FLAT_HEIGHT - DOOR_Y, BATHROOM_WALL_THICKNESS);
      let translation = over_door_wall.half_size + vec3(LEFT_DOOR_WALL_LENGTH + BATHROOM_WALL_THICKNESS, DOOR_Y, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(over_door_wall)),
          MeshMaterial3d(common.beige.clone()),
          Transform::from_translation(translation),
          Bathroom,
          BathroomWall,
        ))
        .set_parent(common.parent);
    }
  }
  {
    let left_wall = Cuboid::new(BATHROOM_WALL_THICKNESS, FLAT_HEIGHT, BATHROOM_X);
    let translation = left_wall.half_size + vec3(0., 0., -BATHROOM_X);
    commands
      .spawn((
        Mesh3d(meshes.add(left_wall)),
        MeshMaterial3d(common.honey_wood_tail[0].clone()),
        Transform::from_translation(translation + vec3(TILE_PLUS_GLUE, 0., 0.)),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
    commands
      .spawn((
        Mesh3d(meshes.add(left_wall)),
        MeshMaterial3d(common.beige.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    let right_wall = Cuboid::new(BATHROOM_WALL_THICKNESS, FLAT_HEIGHT, BATHROOM_X);
    let translation = right_wall.half_size + vec3(BATHROOM_WIDTH + BATHROOM_WALL_THICKNESS, 0., -BATHROOM_X);
    commands
      .spawn((
        Mesh3d(meshes.add(right_wall)),
        MeshMaterial3d(common.honey_wood_tail[2].clone()),
        Transform::from_translation(translation + vec3(-TILE_PLUS_GLUE, 0., 0.)),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
    commands
      .spawn((
        Mesh3d(meshes.add(right_wall)),
        MeshMaterial3d(common.beige.clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    let back_wall = Cuboid::new(RIGHT_WALL_X, FLAT_HEIGHT, BATHROOM_WALL_THICKNESS);
    let translation = back_wall.half_size + vec3(0., 0., -BATHROOM_DEPTH - BATHROOM_WALL_THICKNESS + TILE_PLUS_GLUE);
    commands
      .spawn((
        Mesh3d(meshes.add(back_wall)),
        MeshMaterial3d(common.massa_tail[1].clone()),
        Transform::from_translation(translation),
        Bathroom,
        BathroomWall,
      ))
      .set_parent(common.parent);
  }
  {
    // for amogus to go in
    let vent = Cuboid::new(VENT_WIDTH, FLAT_HEIGHT, VENT_DEPTH);
    let translation = vent.half_size + vec3(RIGHT_WALL_X - VENT_WIDTH, 0., -BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(vent)),
        MeshMaterial3d(common.massa_tail[2].clone()),
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
        MeshMaterial3d(common.massa_tail[3].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
  {
    let shelf_side_wall = Cuboid::new(SHELF_SPACE_DEPTH + EPSILON, FLAT_HEIGHT, SHELF_WALL_THICKNESS + EPSILON);
    let translation = shelf_side_wall.half_size + vec3(SHELF_X - EPSILON, 0., SHELF_SPACE_WIDTH - BATHROOM_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(shelf_side_wall)),
        MeshMaterial3d(common.massa_tail[0].clone()),
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
        MeshMaterial3d(common.massa_tail[1].clone()),
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
        MeshMaterial3d(common.massa_tail[2].clone()),
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
        MeshMaterial3d(common.massa_tail[3].clone()),
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
        MeshMaterial3d(common.massa_tail[0].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
}

// umywalka https://catalog.geberit.pl/pl-PL/product/PRO_1832860
// szafka https://catalog.geberit.pl/pl-PL/product/PRO_634571
const SINK_DEPTH: f32 = 4.8;
const SINK_MODEL_WIDTH: f32 = 74.5;
const SINK_WIDTH: f32 = 8.;

fn spawn_sink(mut commands: Commands, asset_server: Res<AssetServer>, common: Res<BathroomCommon>, mut meshes: ResMut<Assets<Mesh>>) {
  let sink_transform = Transform {
    translation: vec3(SHELF_X + SHELF_DEPTH + 0.5 * SINK_WIDTH + 0.15, 0., -BATHROOM_DEPTH),
    rotation: Quat::from_rotation_x(-PI / 2.0),
    scale: vec3(SINK_WIDTH / SINK_MODEL_WIDTH, 0.1, 0.1),
  };
  {
    let sink_model = asset_server.load("bathroom/umywalka_geberit_500.249.00.2.glb#Scene0");
    commands
      .spawn((SceneRoot(sink_model), sink_transform, Bathroom))
      .set_parent(common.parent);
  }
  {
    let sink_cabinet_model = asset_server.load("bathroom/500.353.00.1_P.glb#Scene0");
    commands
      .spawn((
        SceneRoot(sink_cabinet_model),
        sink_transform.with_translation(sink_transform.translation.with_y(1.5)),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
  {
    let tap = asset_server.load("bathroom/delta-pi-bateria-umywalkowa-podtynkowa-miedziany-szczotkowany.glb#Scene0");
    commands
      .spawn((
        SceneRoot(tap),
        sink_transform.with_translation(sink_transform.translation + vec3(1., 9.5, -0.4 + TILE_PLUS_GLUE)),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
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
  let translation = vec3(BATHROOM_WALL_THICKNESS + TILE_PLUS_GLUE, 0., -BATHROOM_DEPTH);
  commands
    .spawn((
      SceneRoot(model_handle),
      Transform {
        // translation: translation.with_z(translation.z + SHELF_WIDTH),
        translation: translation.with_z(translation.z + SHELF_WIDTH) + vec3(-0.4, -0.4, -0.4),
        rotation: Quat::from_rotation_y(PI / 2.0),
        // scale: Vec3::new(-10.0 * (SHOWER_WIDTH - SHELF_WIDTH) / SHOWER_WIDTH, 10., 10. * SHOWER_DEPTH / 8.),
        scale: Vec3::new(-9.25, 10., 9.25),
      },
    ))
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
  {
    let tap = asset_server
      .load("bathroom/pi-zestaw-prysznicowo-wannowy-podtynkowy-2-funkcyjny-z-deszczownica-25-cm-miedziany-szczotkowany.glb#Scene0");
    commands
      .spawn((
        SceneRoot(tap),
        Transform::from_translation(translation + vec3(4.5, 9.5, 0. + TILE_PLUS_GLUE)),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
}

const FLUSH_DEPTH: f32 = 2.5;
const FLUSH_HEIGHT: f32 = 12.;
const FLUSH_WIDTH: f32 = 7.;

fn spawn_toilet(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, common: Res<BathroomCommon>) {
  let model_handle = asset_server.load("bathroom/muszla.glb#Scene0");
  let toilet_transform = vec3(RIGHT_WALL_X - FLUSH_DEPTH, 0., VENT_DEPTH - BATHROOM_DEPTH);
  commands
    .spawn((
      SceneRoot(model_handle),
      Transform {
        translation: toilet_transform + vec3(0., 0., 0.5 * FLUSH_WIDTH),
        rotation: Quat::from_rotation_y(-PI / 2.0),
        scale: Vec3::new(10.0, 10., 10.),
      },
    ))
    .set_parent(common.parent);
  {
    let toilet_flush = Cuboid::new(FLUSH_DEPTH, FLUSH_HEIGHT, FLUSH_WIDTH);
    let translation = toilet_flush.half_size + toilet_transform;
    commands
      .spawn((
        Mesh3d(meshes.add(toilet_flush)),
        MeshMaterial3d(common.massa_tail[1].clone()),
        Transform::from_translation(translation),
        Bathroom,
      ))
      .set_parent(common.parent);
  }
}

const WASHING_MACHINE_HALF_WIDTH: f32 = 3.;
const WASHING_MACHINE_HEIGHT: f32 = 8.5;

fn spawn_washing_machine(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  common: Res<BathroomCommon>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  let model_handle = asset_server.load("bathroom/washing_machine.glb#Scene0");
  let transform = Transform::from_scale(Vec3::splat(10.0)).with_rotation(Quat::from_rotation_y(-FRAC_PI_2));
  let a = 3. * PLANK_THICKNESS;
  commands
    .spawn((
      SceneRoot(model_handle.clone()),
      transform.with_translation(vec3(
        RIGHT_WALL_X - WASHING_MACHINE_HALF_WIDTH,
        WASHING_MACHINE_HEIGHT / 2.0,
        -WASHING_MACHINE_HALF_WIDTH - a - TILE_PLUS_GLUE,
      )),
    ))
    .set_parent(common.parent);
  commands
    .spawn((
      SceneRoot(model_handle),
      transform.with_translation(vec3(
        RIGHT_WALL_X - WASHING_MACHINE_HALF_WIDTH,
        1.5 * WASHING_MACHINE_HEIGHT,
        -WASHING_MACHINE_HALF_WIDTH - a - TILE_PLUS_GLUE,
      )),
    ))
    .set_parent(common.parent);

  {
    let washing_machine_width = 2. * WASHING_MACHINE_HALF_WIDTH;
    let washing_machine_cabinet_depth = RIGHT_WALL_X - washing_machine_width;
    {
      let plank_cube = Cuboid::new(washing_machine_width, BATHROOM_Z, PLANK_THICKNESS);
      let plank = meshes.add(plank_cube);
      commands
        .spawn((
          Mesh3d(plank.clone()),
          MeshMaterial3d(common.beige.clone()),
          Transform::from_translation(
            plank_cube.half_size + vec3(washing_machine_cabinet_depth, 0., -washing_machine_width - 2. * a - TILE_PLUS_GLUE),
          ),
          Bathroom,
        ))
        .set_parent(common.parent);

      commands
        .spawn((
          Mesh3d(plank),
          MeshMaterial3d(common.beige.clone()),
          Transform::from_translation(plank_cube.half_size + vec3(washing_machine_cabinet_depth, 0., -PLANK_THICKNESS - TILE_PLUS_GLUE)),
          Bathroom,
        ))
        .set_parent(common.parent);
    }

    {
      let h = 2. * WASHING_MACHINE_HEIGHT + 2. * PLANK_THICKNESS;
      let plank_cube = Cuboid::new(PLANK_THICKNESS, BATHROOM_Z - h, washing_machine_width + 2. * a);
      let plank = meshes.add(plank_cube);
      commands
        .spawn((
          Mesh3d(plank),
          MeshMaterial3d(common.beige.clone()),
          Transform::from_translation(
            plank_cube.half_size + vec3(washing_machine_cabinet_depth, h, -washing_machine_width - 2. * a - TILE_PLUS_GLUE),
          ),
          Bathroom,
        ))
        .set_parent(common.parent);
    }
  }
}

const CLOSET_HEIGHT: f32 = FLAT_HEIGHT;
const CLOSED_DEPTH: f32 = 4.;
const CLOSET_WIDTH: f32 = 6. - TILE_PLUS_GLUE;
const MIDDLE_PLANK_DEPTH: f32 = CLOSED_DEPTH;
const MIDDLE_VERTICAL_PLANK_HEIGHT: f32 = CLOSET_HEIGHT - PLANK_THICKNESS;
const MIDDLE_HORIZONTAL_PLANK_Y: f32 = 20.0;
const TOP_HORIZONTAL_DIVIDER_PLANK_Y: f32 = MIDDLE_HORIZONTAL_PLANK_Y + 0.5 * (CLOSET_HEIGHT - MIDDLE_HORIZONTAL_PLANK_Y);
const LAUNDRY_BASKET_TOP_Y: f32 = 5.0;
const VERTICAL_MIDDLE_PLANK_X: f32 = 0.5 * (CLOSET_WIDTH);
const HORIZONTAL_MIDDLE_PLANK_X: f32 = 12.0;

fn spawn_closet(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
  let parent = commands
    .spawn((
      Transform::from_translation(BATHROOM_ORIGIN + vec3(TILE_PLUS_GLUE, 0., BATHROOM_WALL_THICKNESS)),
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
        let top_horizontal_plank = Cuboid::new(CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH);
        let over_the_hanger_rod = top_horizontal_plank.half_size + vec3(PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, 0.);
        commands
          .spawn((
            Mesh3d(meshes.add(top_horizontal_plank)),
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
      let laundry_basket_top_plank = Cuboid::new(CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH);
      let like_in_our_hall_drawer = laundry_basket_top_plank.half_size + vec3(PLANK_THICKNESS, LAUNDRY_BASKET_TOP_Y, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(laundry_basket_top_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(like_in_our_hall_drawer),
        ))
        .set_parent(parent);
    }
  }
  {
    let middle_horizontal_plank = Cuboid::new(CLOSET_WIDTH - 2. * PLANK_THICKNESS, PLANK_THICKNESS, MIDDLE_PLANK_DEPTH);
    let like_in_our_hall_drawer = middle_horizontal_plank.half_size + vec3(PLANK_THICKNESS, HORIZONTAL_MIDDLE_PLANK_X, 0.);
    commands
      .spawn((
        Mesh3d(meshes.add(middle_horizontal_plank)),
        MeshMaterial3d(closet_colour.clone()),
        Transform::from_translation(like_in_our_hall_drawer),
      ))
      .set_parent(parent);
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
      let shower_side_plank = side_plank.half_size + vec3(CLOSET_WIDTH - PLANK_THICKNESS, 0., 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(side_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(shower_side_plank),
        ))
        .set_parent(parent);
    }
  }
  {
    {
      let broom_compartment_and_drawers_divider_plank = Cuboid::new(PLANK_THICKNESS, MIDDLE_HORIZONTAL_PLANK_Y, MIDDLE_PLANK_DEPTH);
      let broom_compartment_and_drawers_divider =
        broom_compartment_and_drawers_divider_plank.half_size + vec3(VERTICAL_MIDDLE_PLANK_X, 0., 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(broom_compartment_and_drawers_divider_plank)),
          MeshMaterial3d(closet_colour.clone()),
          Transform::from_translation(broom_compartment_and_drawers_divider),
        ))
        .set_parent(parent);
    }
  }
}

pub(crate) struct BathroomPlugin;

impl Plugin for BathroomPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, (setup_bathroom_common, spawn_closet)).add_systems(
      Startup,
      (
        spawn_walls,
        spawn_washing_machine,
        spawn_shower_stall,
        spawn_toilet,
        spawn_sink,
        spawn_shower_shelf,
      )
        .after(setup_bathroom_common),
    );
  }
}
