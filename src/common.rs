use bevy::asset::{AssetPath, AssetServer, Assets, Handle};
use bevy::color::Color;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::math::{Affine2, Vec2};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{default, Res, ResMut};

pub const EPSILON: f32 = 0.0002;

pub const FLAT_HEIGHT: f32 = 26.8;
pub const OFFICE_WALL_THICKNESS: f32 = 0.8;
pub const BATHROOM_WALL_THICKNESS: f32 = 1.;
pub const KITCHEN_WALL_THICKNESS: f32 = 1.;
pub const DOOR_Y: f32 = 20.;
pub const DOOR_WIDTH: f32 = 9.2;
pub const TILE_PLUS_GLUE: f32 = 0.08 + 0.07;

pub const BATHROOM_X: f32 = 18.1 + BATHROOM_WALL_THICKNESS;
pub const BATHROOM_Z: f32 = 25. + 2. * BATHROOM_WALL_THICKNESS;

pub const LIVING_ROOM_X: f32 = 52.55;
pub const LIVING_ROOM_TO_BATHROOM_Z: f32 = 57.3;
pub const LIVING_ROOM_TO_HALL_Z: f32 =  56.163;
pub const LIVING_ROOM_X_HALL_OFFSET: f32 = BATHROOM_X;

pub const BEDROOM_X: f32 = 30.;
pub const BEDROOM_Z: f32 = 32.4;

pub const OFFICE_X: f32 = 30.3 + OFFICE_WALL_THICKNESS;
pub const OFFICE_Z: f32 = 33.8 + OFFICE_WALL_THICKNESS;
pub const OFFICE_Z_POS: f32 = LIVING_ROOM_TO_HALL_Z + HALL_Z;
pub const OFFICE_X_POS: f32 = BATHROOM_X + HALL_X - OFFICE_X;

pub const HALL_X: f32 = 41.9;
pub const HALL_Z: f32 = 14.3;

pub const TM_WALL_X: f32 = BATHROOM_X + HALL_X - LIVING_ROOM_X;

pub const SMALL_HALL_X: f32 = 10.8;
pub const SMALL_HALL_Z: f32 = 13.78;

pub const PLANK_THICKNESS: f32 = 0.18;

// NCS S 1505-y40R
// https://www.w3schools.com/colors/colors_converter.asp?color=ncs(1505-y40R)
pub const BEIGE: Color = Color::hsl(30.0, 0.29, 0.85);
pub const NOT_BEIGE: Color = Color::hsl(30.0, 0.29, 0.085);

pub fn repeat_texture<'a>(
  path: impl Into<AssetPath<'a>>,
  materials: &mut ResMut<Assets<StandardMaterial>>,
  asset_server: &Res<AssetServer>,
  object_size: Vec2,
  image_scale: Vec2,
) -> Handle<StandardMaterial> {
  // https://bevyengine.org/examples/assets/repeated-texture/
  let texture = asset_server.load_with_settings(path, |s: &mut _| {
    *s = ImageLoaderSettings {
      sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
        // rewriting mode to repeat image,
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
      }),
      ..default()
    }
  });
  let material_handle = materials.add(StandardMaterial {
    base_color_texture: Some(texture),
    metallic: 0.9,
    uv_transform: Affine2::from_scale(Vec2::new(image_scale.x * object_size.x, image_scale.y * object_size.y)),
    ..default()
  });
  material_handle
}
