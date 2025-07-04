use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};

mod builder;
mod ui;
mod camera;

use bevy_egui::EguiPlugin;
use ui::UiPlugin;

use crate::camera::MainCameraPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tileset Generator".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins(UiPlugin)
        .add_plugins(MainCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Mesh3d(meshes.add(
            PlaneMeshBuilder::from_length(20.0)
        )),
        MeshMaterial3d( materials.add(
            Color::srgb(0.0, 1.0, 0.3)
        )),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}