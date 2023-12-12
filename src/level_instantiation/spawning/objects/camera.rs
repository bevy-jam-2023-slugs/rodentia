use std::f32::consts::PI;

use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::Model;
use crate::player_control::actions::create_camera_action_input_manager_bundle;
use crate::player_control::camera::IngameCamera;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
#[cfg(feature = "dev")]
use bevy_editor_pls::default_windows::cameras::EditorCamera;

#[derive(Bundle, Default)]
struct ModifiedPbrBundle {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

pub(crate) fn spawn(
    In(transform): In<Transform>,
    mut commands: Commands,
    _animations: Res<AnimationAssets>,
    scene_handles: Res<SceneAssets>,
) {
    let camera_entity = commands
        .spawn((
            IngameCamera::default(),
            Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
                    ..default()
                },
                transform,
                ..default()
            },
            ModifiedPbrBundle::default(),
            // PbrBundle
            Rig::builder()
                .with(Position::new(default()))
                .with(YawPitch::new())
                .with(Smooth::new_position_rotation(default(), default()))
                .with(Arm::new(default()))
                .with(LookAt::new(default()).tracking_predictive(true))
                .build(),
            create_camera_action_input_manager_bundle(),
            Name::new("Main Camera"),
            GameObject::Camera,
            Visibility::Visible,
            #[cfg(feature = "dev")]
            EditorCamera,
        ))
        .id();

    commands
        .spawn((
            Model {
                target: camera_entity,
            },
            SpatialBundle::default(),
            Name::new("Camera Model Parent"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_handles.sword.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.3, 0.3, -1.),
                        rotation: Quat::from_axis_angle(Vec3::Z, PI),
                        scale: Vec3::splat(0.1),
                    },
                    ..default()
                },
                Name::new("Camera Model"),
            ));
        });
}
