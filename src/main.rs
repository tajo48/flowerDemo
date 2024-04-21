use bevy::prelude::*;
use bevy_psx::{camera::PsxCamera, material::PsxMaterial, PsxPlugin};
use serde_json;
use std::fs;
#[path = "camera_controller.rs"]
mod camera_controller;
use camera_controller::{CameraController, CameraControllerPlugin};
// TA ZMIENNA JEST MAGICZNA DEFINIUJE ONA JAK CZESTO MA BYC WYKONYWANA AKCJA ROŚNIECIA IM WIĘKSZY NUMEREK TYM ŻADZIEJ ROSNIE
// Przy ustawieniu jej na 10 rośnienie pierwszego kwiatka trwa 50 sekund
// dla szybkiej demonstracji ustawiamy na 1
// dla realistycznego rośnięcia ustawiamy na parę tysięcy
pub static MAGIC_MULTIPLIER: u64 = 3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxPlugin)
        .add_plugins(CameraControllerPlugin)
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .add_systems(Update, despawn_all_old_flowers)
        .run();
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PsxMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // set age to current unix time
    if !std::path::Path::new("state.json").exists() {
        let denage = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let state = serde_json::json!({
            "plant": {
                "age": denage
            }
        });
        fs::write("state.json", serde_json::to_string_pretty(&state).unwrap())
            .expect("Unable to write file");
    }

    let contents = fs::read_to_string("state.json").expect("Something went wrong reading the file");
    println!("Contents: {}", contents);
    let v: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let denage = v["plant"]["age"].as_u64().unwrap();
    println!("Age: {}", denage);
    /*
    commands.spawn((
       Camera3dBundle {
           transform: Transform::from_xyz(-1.0, 1.0, 1.0)
               .looking_at(Vec3::new(0.0, 0.0, -0.5), Vec3::Y),
           ..default()
       },
       CameraController::default(),
    ));
    */
    commands.spawn((PsxCamera::default(), CameraController::default()));
    //commands.spawn(PsxCamera::default());
    let transform =
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 0.0, -0.5));

    commands.spawn((
        MaterialMeshBundle {
            mesh: asset_server.load("foliage/psxFlower.glb#Mesh4/Primitive0"),
            material: materials.add(PsxMaterial {
                color_texture: Some(asset_server.load("foliage/psxFlower.glb#Texture4")),
                // snap_amount: 40000000000.0,
                snap_amount: 150.0,
                fog_distance: Vec2::new(250.0, 750.0),

                ..Default::default()
            }),
            transform,
            ..default()
        },
        Rotates,
    ));

    commands.spawn((
        MaterialMeshBundle {
            mesh: asset_server.load("foliage/psxFlower.glb#Mesh3/Primitive0"),
            material: materials.add(PsxMaterial {
                color_texture: Some(asset_server.load("foliage/psxFlower.glb#Texture3")),
                fog_distance: Vec2::new(250.0, 750.0),
                snap_amount: 150.0,
                ..Default::default()
            }),
            transform,
            ..default()
        },
        Rotates,
        Flower {
            age: denage,
            stage: 1,
        },
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}

#[derive(Component)]
struct Flower {
    age: u64,
    stage: u8,
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.92 * time.delta_seconds());
        // transform.scale = Vec3::splat(0.25);
        transform.rotate_x(0.3 * time.delta_seconds());
        transform.rotate_z(0.3 * time.delta_seconds());
    }
}

fn despawn_all_old_flowers(
    mut commands: Commands,
    query: Query<(Entity, &Flower, &Transform)>,
    _meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PsxMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, flower, transform) in query.iter() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let age = current_time - flower.age;
        println!("Age: {}", age);

        if age < 10 * MAGIC_MULTIPLIER && age >= 5 * MAGIC_MULTIPLIER && !(flower.stage == 2) {
            println!("Stage 2");
            commands.entity(entity).despawn();
            commands.spawn((
                MaterialMeshBundle {
                    mesh: asset_server.load("foliage/psxFlower.glb#Mesh0/Primitive0"),
                    material: materials.add(PsxMaterial {
                        color_texture: Some(asset_server.load("foliage/psxFlower.glb#Texture0")),
                        fog_distance: Vec2::new(250.0, 750.0),
                        snap_amount: 150.0,
                        ..Default::default()
                    }),
                    transform: *transform,
                    ..default()
                },
                Rotates,
                Flower {
                    age: flower.age,
                    stage: 2,
                },
            ));
        }
        if age < 15 * MAGIC_MULTIPLIER && age >= 10 * MAGIC_MULTIPLIER && !(flower.stage == 3) {
            println!("Stage 3");
            commands.entity(entity).despawn();
            commands.spawn((
                MaterialMeshBundle {
                    mesh: asset_server.load("foliage/psxFlower.glb#Mesh2/Primitive0"),
                    material: materials.add(PsxMaterial {
                        color_texture: Some(asset_server.load("foliage/psxFlower.glb#Texture2")),
                        fog_distance: Vec2::new(250.0, 750.0),
                        snap_amount: 150.0,
                        ..Default::default()
                    }),
                    transform: *transform,
                    ..default()
                },
                Rotates,
                Flower {
                    age: flower.age,
                    stage: 3,
                },
            ));
        }
        if age >= 15 * MAGIC_MULTIPLIER && !(flower.stage == 4) {
            println!("Stage 4");
            commands.entity(entity).despawn();
            commands.spawn((
                MaterialMeshBundle {
                    mesh: asset_server.load("foliage/psxFlower.glb#Mesh1/Primitive0"),
                    material: materials.add(PsxMaterial {
                        color_texture: Some(asset_server.load("foliage/psxFlower.glb#Texture1")),
                        fog_distance: Vec2::new(250.0, 750.0),
                        snap_amount: 150.0,
                        ..Default::default()
                    }),
                    transform: *transform,
                    ..default()
                },
                Rotates,
                Flower {
                    age: flower.age,
                    stage: 4,
                },
            ));
        }
    }
}
