use crate::movement::{CameraMovement, MoveEvent};
use bevy::app::{App, Plugin, Update};
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;


// pub fn rotate_item_with_mouse(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
//   if drag.button == PointerButton::Secondary {
//     if let Ok(mut transform) = transforms.get_mut(drag.target()) {
//       transform.rotate_y(drag.delta.x * 0.02);
//       transform.rotate_x(drag.delta.y * 0.02);
//     }
//   }
// }

pub fn on_pressed_save(click: Trigger<Pointer<Pressed>>, mut clicked_entity: ResMut<ClickedEntity>) {
  if click.button == PointerButton::Primary {
    clicked_entity.clicked = Some(click.target);
  }
}

pub fn on_released_clear(_: Trigger<Pointer<Released>>, mut clicked_entity: ResMut<ClickedEntity>) {
  clicked_entity.clicked = None;
}

fn move_clicked(clicked_entity: Res<ClickedEntity>, mut reader: EventReader<MoveEvent>, mut transforms: Query<&mut Transform>) {
  match clicked_entity.clicked {
    None => {
      if !reader.is_empty() {
        reader.clear();
      }
    }
    Some(entity) => {
      for e in reader.read() {
        transforms.get_mut(entity).iter_mut().for_each(|transform| {
          transform.translation += (e.translation * 0.2);
        });
      }
    }
  }
}

#[derive(Resource)]
pub struct ClickedEntity {
  clicked: Option<Entity>,
}

pub(crate) struct ControlPlugin;

impl Plugin for ControlPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<MoveEvent>()
      .insert_resource(ClickedEntity { clicked: None })
      .add_systems(Update, move_clicked);
  }
}
