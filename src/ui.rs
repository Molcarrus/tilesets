use bevy::prelude::*;

use bevy_egui::{ egui::{self, Slider}, EguiContextPass, EguiContexts };

use crate::builder::{
    Environment,
    generate_tileset
};

use crate::draw::draw;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Environment>()
            .add_systems(EguiContextPass, ui_system);
    }
}

fn ui_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut values: ResMut<Environment>,
    mut contexts: EguiContexts
) {
    egui::Window::new("Environment values")
        .show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Size(rows and columns): ");
            ui.add(Slider::new(
                    &mut values.size, 
                    2..=50
            ))
        });

        ui.horizontal(|ui| {
            ui.label("Tileset: ");
            ui.label(format!("{:?}", values.tileset));
        });

        ui.horizontal(|ui| {
            if ui
                .add(egui::Button::new("Generate"))
                .clicked()
            {
                values.tileset = generate_tileset(values.size);
                draw(commands, meshes, materials, values.size, values.tileset.clone());
            }
        })
    });
}