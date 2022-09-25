use bevy::prelude::*;

pub struct ShedPlugin;

impl Plugin for ShedPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_sheds);
    }
}

fn init_sheds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    let plot = Plot::new(Vec3::new(75.,0.,75.));
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 25. })),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        transform: Transform::from_translation(plot.pos),
        ..default()
    });
}

pub struct Plot {
    pos: Vec3,
}

impl Plot {
    pub fn new(pos: Vec3) -> Plot {
        Plot{pos}
    }
}
