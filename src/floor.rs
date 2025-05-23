use bevy::prelude::*;

use crate::common::*;
use bevy::math::vec3;
// https://bevyengine.org/examples/3d-rendering/3d-shapes/

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct LoadBearingWall;

#[derive(Resource)]
struct FloorCommon {
  parent: Entity,
}

const LOAD_BEARING_WALL_THICKNESS: f32 = 1.;

fn setup_floor_common(mut commands: Commands) {
  let parent = commands
    .spawn((
      Transform::from_scale(vec3(-1., 1., 1.)),
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();
  commands.insert_resource(FloorCommon { parent })
}

const FLOOR_DEPTH: f32 = 1.;
fn spawn_floors(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  common: Res<FloorCommon>,
) {
  let floor_material: Handle<StandardMaterial> = materials.add(Color::hsl(0., 0., 1.));
  {
    let living_room_floor = Cuboid::new(LIVING_ROOM_X, FLOOR_DEPTH, LIVING_ROOM_TO_BATHROOM_Z);
    let translation = living_room_floor.half_size + Vec3::ZERO.with_y(-FLOOR_DEPTH);
    commands
      .spawn((
        Mesh3d(meshes.add(living_room_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(common.parent);
  }
  {
    let hall_floor = Cuboid::new(HALL_X, FLOOR_DEPTH, HALL_Z);
    let translation = hall_floor.half_size + vec3(LIVING_ROOM_X_HALL_OFFSET, -FLOOR_DEPTH, LIVING_ROOM_TO_HALL_Z);
    commands
      .spawn((
        Mesh3d(meshes.add(hall_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(common.parent);
  }
  {
    let bathroom_floor = Cuboid::new(BATHROOM_X, FLOOR_DEPTH, BATHROOM_Z);
    let translation = bathroom_floor.half_size + vec3(0., -FLOOR_DEPTH, LIVING_ROOM_TO_BATHROOM_Z);
    commands
      .spawn((
        Mesh3d(meshes.add(bathroom_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(common.parent);
  }
  {
    let office_floor = Cuboid::new(OFFICE_X, FLOOR_DEPTH, OFFICE_Z);
    let translation = office_floor.half_size + vec3(BATHROOM_X + SMALL_HALL_X, -FLOOR_DEPTH, OFFICE_Z_POS);
    commands
      .spawn((
        Mesh3d(meshes.add(office_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(common.parent);
  }
  {
    let small_hall_floor = Cuboid::new(SMALL_HALL_X, FLOOR_DEPTH, SMALL_HALL_Z);
    let translation = small_hall_floor.half_size + vec3(BATHROOM_X, -FLOOR_DEPTH, OFFICE_Z_POS);
    commands
      .spawn((
        Mesh3d(meshes.add(small_hall_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(common.parent);
  }
  {
    let bedroom_floor = Cuboid::new(BEDROOM_X, FLOOR_DEPTH, BEDROOM_Z);
    let translation = bedroom_floor.half_size + vec3(0., -FLOOR_DEPTH, LIVING_ROOM_TO_BATHROOM_Z + BATHROOM_Z);
    commands
      .spawn((
        Mesh3d(meshes.add(bedroom_floor)),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_translation(translation),
        Floor,
      ))
      .set_parent(common.parent);
  }
}

fn spawn_walls(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  common: Res<FloorCommon>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  {
    let kithen_wall_colour = materials.add(BEIGE);
    {
      let living_room_wall = Cuboid::new(LOAD_BEARING_WALL_THICKNESS, FLAT_HEIGHT, LIVING_ROOM_TO_BATHROOM_Z - EPSILON);
      let translation = living_room_wall.half_size + vec3(-LOAD_BEARING_WALL_THICKNESS, 0., 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(living_room_wall)),
          MeshMaterial3d(kithen_wall_colour.clone()),
          Transform::from_translation(translation),
          LoadBearingWall,
        ))
        .set_parent(common.parent);
    }
    {
      let massa = repeat_texture(
        "massa/PP-Massa-1198x2398-1.jpg",
        &mut materials,
        &asset_server,
        Vec2 { x: 20., y: 20. },
        Vec2 { x: 0.1, y: 0.1 },
      );
      let kitchen_wall = Cuboid::new(LOAD_BEARING_WALL_THICKNESS, FLAT_HEIGHT, LIVING_ROOM_TO_HALL_Z - LOAD_BEARING_WALL_THICKNESS);
      let translation = kitchen_wall.half_size + vec3(LIVING_ROOM_X, 0., 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(kitchen_wall)),
          MeshMaterial3d(massa),
          Transform::from_translation(translation),
          LoadBearingWall,
        ))
        .set_parent(common.parent);
    }
    {
      let hall_wall = Cuboid::new(TM_WALL_X, FLAT_HEIGHT, LOAD_BEARING_WALL_THICKNESS);
      let translation = hall_wall.half_size + vec3(LIVING_ROOM_X, 0., LIVING_ROOM_TO_HALL_Z - LOAD_BEARING_WALL_THICKNESS);
      commands
        .spawn((
          Mesh3d(meshes.add(hall_wall)),
          MeshMaterial3d(kithen_wall_colour.clone()),
          Transform::from_translation(translation),
          LoadBearingWall,
        ))
        .set_parent(common.parent);
    }
    {
      {
        let office_wall_small = Cuboid::new(SMALL_WALL_W, FLAT_HEIGHT, OFFICE_WALL_THICKNESS);
        let translation = office_wall_small.half_size + vec3(OFFICE_X_POS, 0., OFFICE_Z_POS);
        commands
          .spawn((
            Mesh3d(meshes.add(office_wall_small)),
            MeshMaterial3d(kithen_wall_colour.clone()),
            Transform::from_translation(translation),
            LoadBearingWall,
          ))
          .set_parent(common.parent);
      }
      { // office_wall_entrance
        // Good old `comparing floats with eq`. Nothing beats that. S04E19
        let office_wall_entrance = Cuboid::new(OFFICE_WALL_LENGTH, FLAT_HEIGHT, OFFICE_WALL_THICKNESS);
        let translation = office_wall_entrance.half_size + vec3(OFFICE_X_POS + OFFICE_DOOR_PLUS_SMALL_WALL, 0., OFFICE_Z_POS);
        commands
          .spawn((
            Mesh3d(meshes.add(office_wall_entrance)),
            MeshMaterial3d(kithen_wall_colour.clone()),
            Transform::from_translation(translation),
            LoadBearingWall,
          ))
          .set_parent(common.parent);
      }
      {
        let office_wall_over_the_door = Cuboid::new(DOOR_WIDTH, FLAT_HEIGHT - DOOR_Y, OFFICE_WALL_THICKNESS);
        let translation = office_wall_over_the_door.half_size + vec3(OFFICE_X_POS + SMALL_WALL_W, DOOR_Y, OFFICE_Z_POS);
        commands
          .spawn((
            Mesh3d(meshes.add(office_wall_over_the_door)),
            MeshMaterial3d(kithen_wall_colour.clone()),
            Transform::from_translation(translation),
            LoadBearingWall,
          ))
          .set_parent(common.parent);
      }
      {
        let bedroom_office_wall = Cuboid::new(OFFICE_WALL_THICKNESS, FLAT_HEIGHT, OFFICE_Z);
        let translation = bedroom_office_wall.half_size + vec3(OFFICE_X_POS, 0., OFFICE_Z_POS);
        commands
          .spawn((
            Mesh3d(meshes.add(bedroom_office_wall)),
            MeshMaterial3d(kithen_wall_colour.clone()),
            Transform::from_translation(translation),
            LoadBearingWall,
          ))
          .set_parent(common.parent);
      }
    }
  }
  {
    let bedroom_wall_colour = materials.add(Color::hsl(0., 0., 1.));
    let bedroom_wall = Cuboid::new(LOAD_BEARING_WALL_THICKNESS, FLAT_HEIGHT, BEDROOM_Z);
    let translation = bedroom_wall.half_size + vec3(-LOAD_BEARING_WALL_THICKNESS, 0., LIVING_ROOM_TO_BATHROOM_Z + BATHROOM_Z + EPSILON);
    commands
      .spawn((
        Mesh3d(meshes.add(bedroom_wall)),
        MeshMaterial3d(bedroom_wall_colour.clone()),
        Transform::from_translation(translation),
        LoadBearingWall,
      ))
      .set_parent(common.parent);
  }
}

pub(crate) struct FloorPlugin;

impl Plugin for FloorPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_floor_common)
      .add_systems(Startup, (spawn_floors, spawn_walls).after(setup_floor_common));
  }
}
