use bevy::prelude::*;

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

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

const BOTTOM_CABINET_HEIGHT: f32 = 7.20;
const BOTTOM_CABINET_DEPTH: f32 = 5.60;
const BOTTOM_CABINET_FEET_HEIGHT: f32 = 0.10;
const CABBINET_WIDTH: f32 = 6.0;
const COUNTERTOP_Y: f32 = BOTTOM_CABINET_HEIGHT + BOTTOM_CABINET_FEET_HEIGHT;

pub(crate) fn setup_kitchen(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut images: ResMut<Assets<Image>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // NCS S 1505-y40R
  // https://www.w3schools.com/colors/colors_converter.asp?color=ncs(1505-y40R)
  let color = Color::hsl(30.0, 0.29, 0.85);
  let material = materials.add(color);

  let bottom_cabinets = [
    Cuboid::new(CABBINET_WIDTH / 2.0, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABBINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABBINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABBINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABBINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABBINET_WIDTH, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
    Cuboid::new(CABBINET_WIDTH / 2.0, BOTTOM_CABINET_HEIGHT, BOTTOM_CABINET_DEPTH),
  ];

  let mut acc: f32 = 0.0;
  for bottom_cabinet in bottom_cabinets.into_iter() {
    commands.spawn((
      Mesh3d(meshes.add(bottom_cabinet)),
      MeshMaterial3d(material.clone()),
      Transform::from_translation(vec3(acc + bottom_cabinet.half_size.x, BOTTOM_CABINET_FEET_HEIGHT, -10.)),
      KitchenCabinet,
    ));
    acc += bottom_cabinet.size().x;
  }
}
