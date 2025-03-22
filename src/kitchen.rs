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
 
fn setup_kitchen(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
  let kitchen_origin: Vec3 = vec3(0., BOTTOM_CABINET_Y, -50.);
  let parent = commands
    .spawn((
      Transform::from_translation(kitchen_origin).with_rotation(Quat::IDENTITY),
      // rotation
      GlobalTransform::default(),
      InheritedVisibility::default(),
    ))
    .id();

  // NCS S 1505-y40R
  // https://www.w3schools.com/colors/colors_converter.asp?color=ncs(1505-y40R)
  let color = Color::hsl(30.0, 0.29, 0.85);
  let material = materials.add(color);

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
          MeshMaterial3d(material.clone()),
          Transform::from_translation(translation),
          KitchenCabinet,
        ))
        .set_parent(parent);
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
    let mut x_acc: f32 = 0.0;
    for middle_cabinet in middle_cabinets.into_iter() {
      let translation = middle_cabinet.half_size + vec3(x_acc, MIDDLE_CABINET_Y, 0.);
      commands
        .spawn((
          Mesh3d(meshes.add(middle_cabinet)),
          MeshMaterial3d(material.clone()),
          Transform::from_translation(translation),
          KitchenCabinet,
        ))
        .set_parent(parent);
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
        MeshMaterial3d(material.clone()),
        Transform::from_translation(translation),
        KitchenCabinet,
      ))
      .set_parent(parent);
    x_acc += top_cabinet.size().x;
  }
  let owen_and_stuff = Cuboid::new(CABINET_WIDTH, FLAT_HEIGHT - BOTTOM_CABINET_Y, BOTTOM_CABINET_DEPTH);
  let translation = owen_and_stuff.half_size + vec3(x_acc, BOTTOM_CABINET_Y, 0.);
  commands
    .spawn((
      Mesh3d(meshes.add(owen_and_stuff)),
      MeshMaterial3d(material.clone()),
      Transform::from_translation(translation),
      KitchenCabinet,
    ))
    .set_parent(parent);
}

pub(crate) struct KitchenPlugin;

impl Plugin for KitchenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_kitchen); 
    }
}