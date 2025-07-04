use bevy::prelude::*;

use bevy_egui::{ egui::{self, Slider}, EguiContextPass, EguiContexts };

use crate::builder::{
    Environment,
    generate_tileset
};

#[derive(Event)]
pub struct ValueChangeEvent;

#[derive(Component)]
pub struct Building;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ValueChangeEvent>()
            .init_resource::<Environment>()
            .add_systems(EguiContextPass, ui_system);
    }
}

fn ui_system(
    mut values: ResMut<Environment>,
    mut contexts: EguiContexts,
    mut value_changed_event: EventWriter<ValueChangeEvent>
) {
    egui::Window::new("Environment values")
        .show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Size(rows and columns): ");
            if ui.add(Slider::new(
                    &mut values.size, 
                    2..=50
            )).changed() {
                value_changed_event.write(ValueChangeEvent);
            }
            
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
                value_changed_event.write(ValueChangeEvent);
            }
        })
    });
}

fn render_buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<Building>>,
    mut values: ResMut<Environment>,
    mut value_events: EventReader<ValueChangeEvent>
) {
    for _ in value_events.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        let coords = get_coords(values.size);
        values.tileset = generate_tileset(values.size);

        for i in 0..values.size {
            for j in 0..values.size {
                let entity_id = commands.spawn((
                    Mesh3d(meshes.add(
                        Cuboid::new(
                            0.0, values.tileset[i as usize][j as usize] as f32, 1.0
                        )
                    )),
                    MeshMaterial3d(materials.add(
                        Color::srgb(0.128, 0.128, 0.128)
                    )),
                    Transform::from_translation(coords[i as usize][j as usize])
                )).id();
            }
        }
    }
}

pub fn get_coords(size: u8) -> Vec<Vec<Vec3>> {
    let mut coords: Vec<Vec<Vec3>> = vec![vec![Vec3::ZERO; size as usize]; size as usize];

    let x_start = - 1.0 * ((3.0 * (size - 1) as f32) / 4.0);

    let mut x_coord = x_start;

    for i in 0..size as usize {
        let mut z_coord = x_start.abs();
        for j in 0..size as usize {
            coords[i][j].x = x_coord;
            coords[i][j].z = z_coord;

            x_coord += 1.5;
        }
        x_coord = x_start;
        z_coord = z_coord - 1.5;
    }

    coords
}