use bevy::{ 
    prelude::*,
    input::mouse::MouseMotion
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_player)
            .add_startup_system(grab_cursor_system)
            .add_system(move_player);
    }
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Position(Transform);

fn init_player(mut commands: Commands) {
    let player_transform = Transform::from_xyz(-2.0, 7.0, 5.0).looking_at(Vec3::ZERO,Vec3::Y);
    commands.spawn_bundle(Camera3dBundle {
        transform: player_transform,
        ..default()
    }).insert(Person);
}

fn move_player(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_pos: Query<(&mut Transform, &mut Person)>) {
    let (mut cam, _player) = player_pos.single_mut();
    for event in mouse_motion_events.iter() {
        //camera rotation by mouse movement
        let angle_x = event.delta.x*time.delta_seconds()/4.0;
        let angle_y = event.delta.y*time.delta_seconds()/7.0;
        let yaw = Quat::from_rotation_y((angle_x.atan()/2.0).sin()).inverse();
        let pitch = Quat::from_rotation_x((angle_y.atan()/2.0).sin()).inverse();
        cam.rotation = cam.rotation * pitch;
        cam.rotation = yaw * cam.rotation; 
    }

    //keyboard inputs
    let player_speed = 10.0;
    let mut normal = cam.forward().normalize()*1.2*player_speed;
    if keyboard_input.pressed(KeyCode::W) {
        cam.translation.x += time.delta_seconds()*normal.x;
        cam.translation.z += time.delta_seconds()*normal.z;
    }
    if keyboard_input.pressed(KeyCode::S) {
        cam.translation.x -= time.delta_seconds()*normal.x;
        cam.translation.z -= time.delta_seconds()*normal.z;
    }

    normal = cam.left().normalize()*player_speed;
    if keyboard_input.pressed(KeyCode::A) {
        cam.translation.x += time.delta_seconds()*normal.x;
        cam.translation.z += time.delta_seconds()*normal.z;
    }
    if keyboard_input.pressed(KeyCode::D) {
        cam.translation.x -= time.delta_seconds()*normal.x;
        cam.translation.z -= time.delta_seconds()*normal.z;
    }

    //TODO: PLAYER GRAVITY
}

fn grab_cursor_system (
    mut windows: ResMut<Windows>,
    ){
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}
