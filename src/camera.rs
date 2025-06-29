use bevy::{
    app::App,
    input::mouse::MouseWheel,
    math::Vec3,
    prelude::*
};

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, zoom_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}

fn zoom_camera(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut scroll_evr: EventReader<MouseWheel>
) {
    let scroll = scroll_evr.read().map(|ev| ev.y).sum::<f32>();
    if scroll == 0.0 {
        return;
    }

    for mut transform in camera_query.iter_mut() {
        let mut translation = transform.translation;
        translation.z -= scroll * 0.5;
        translation.z = translation.z.clamp(3.0, 10.0);
        transform.translation = translation
    }
}