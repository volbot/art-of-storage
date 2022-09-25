use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_world);
    }
}

fn init_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    let roads = gen_roads();
    for road in roads {
        let mut i = 0.;
        while i < road.len as f32 {
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
                material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                transform: Transform::from_xyz(
                    match road.dir {
                        Cardinal::East => {25.}
                        Cardinal::West => {-25.}
                        _ => {0.}
                    } * (road.pos.x + i),
                    match road.dir {
                        Cardinal::North => {25.}
                        Cardinal::South => {-25.}
                        _ => {0.}
                    } * (road.pos.z + i),
                    0.),
                ..default()
            });
            i += 1.;
        }
    }
}


pub struct Road {
    len: usize,
    pos: Vec3,
    dir: Cardinal
}

impl Road {
    pub fn new(len: usize, pos: Vec3, dir: Cardinal) -> Road {
        Road{len,pos,dir}
    }
}

#[allow(dead_code)]
pub enum Cardinal {
    North,
    South,
    East,
    West
}

fn gen_roads() -> Vec<Road> {
    vec![Road::new(5,Vec3::new(0.,0.,0.),Cardinal::East)]
}
