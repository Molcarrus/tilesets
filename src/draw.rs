use bevy::prelude::*;

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

pub fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    size: u8,
    tileset: Vec<Vec<u8>>
) {
    let coord_system = get_coords(size);

    commands.spawn((
        Mesh3d(
            meshes.add(Cuboid::new(3.0 * (size-1) as f32 / 4.0, 0.0, 3.0 * (size-1) as f32 / 4.0)
            )
        ),
        MeshMaterial3d(
            materials.add(
                Color::srgb(0.128, 0.128, 0.128)
            )
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));

    for i in 0..size as usize {
        for j in 0..size as usize {
            commands.spawn((
                Mesh3d(
                    meshes.add(
                        Cuboid::new(1.0, tileset[i][j] as f32, 0.0)
                    )
                ),
                MeshMaterial3d(
                    materials.add(
                        Color::srgb(0.165, 0.42, 0.42)
                    )
                ),
                Transform::from_translation(coord_system[i][j])
            ));
        }
    }
}