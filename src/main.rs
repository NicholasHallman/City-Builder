mod terrain_gen;
use terrain_gen::{gen_terrain};
use std::f32::consts::PI;
use bevy::{prelude::*, input::mouse::MouseMotion};

fn main() {
    App::new()
        .insert_resource(Msaa {samples: 4})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(key_controller)
        .add_system(mouse_move)
        .run();
}

#[derive(Component)]
struct Camera {}

#[derive(Component)]
struct Cursor {}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {size: 5.0})),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
 
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.25, 0.0),
        ..default()
    })
    .insert(Cursor {})
    .with_children(|parent| {
        // camera
        parent.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Camera {});
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // noise test
    let map = gen_terrain(200);
    for x in 0..32 {
        print!("{:?}\n", map[x])
    }
}

fn key_controller(mut keys: Res<Input<KeyCode>>, 
    mut q_child: Query<(&Parent, &Transform), With<Camera>>,
    mut q_parent: Query<(&mut Transform), Without<Camera>>)
{
    let pressed = [keys.pressed(KeyCode::W), keys.pressed(KeyCode::D), keys.pressed(KeyCode::S), keys.pressed(KeyCode::A)];
    let count = pressed.into_iter().fold(0.0, |acc, cur| if cur == false { acc } else { acc + 1.0 });
    
    for (parent, mut transform) in q_child.iter_mut() {
        let mut parent_transform = q_parent.get_mut(parent.0)
            .expect("No parent");

        let axis = transform.rotation.to_euler(EulerRot::YXZ);
        let speed = 0.1 / count;

        if pressed[0] {
            parent_transform.translation.z -= speed * axis.0.cos();
            parent_transform.translation.x -= speed * axis.0.sin();
        }
        if pressed[1] {
            parent_transform.translation.z -= speed * axis.0.sin();
            parent_transform.translation.x += speed * axis.0.cos();
        }
        if pressed[2] {
            parent_transform.translation.z += speed * axis.0.cos();
            parent_transform.translation.x += speed * axis.0.sin();
        }
        if pressed[3] {
            parent_transform.translation.z += speed * axis.0.sin();
            parent_transform.translation.x -= speed * axis.0.cos();
        }
    }
}

fn mouse_move(mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_buttons: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Camera>>)
{

    for mut transform in query.iter_mut() {
        if !mouse_buttons.pressed(MouseButton::Middle) { return }
        let some_event = mouse_motion_events.iter().last();
        match some_event{
            Some(event) => {
                transform.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::XYZ, 0.0, event.delta.x / 100.0, 0.0));
                transform.look_at(Vec3::ZERO, Vec3::Y);
            },
            None => return
        }
    }
}