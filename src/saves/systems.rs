use std::fs::File;

use bevy::{
    prelude::{
        default, AppTypeRegistry, AssetServer, Commands, EventReader, EventWriter, GlobalTransform,
        Input, KeyCode, Query, Res, With, World,
    },
    scene::{DynamicScene, DynamicSceneBundle},
    tasks::IoTaskPool,
    utils::info,
};

use crate::{player::components::Player, saves::components::PositionSave};

use super::components::SaveEvent;

// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

// The new, updated scene data will be saved here so that you can see the changes
const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

pub fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Spawning" a scene bundle creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    commands.spawn(DynamicSceneBundle {
        // Scenes are loaded just like any other asset.
        scene: asset_server.load("assets/scene.scn.ron"),
        ..default()
    });
}

pub fn check_for_save(input: Res<Input<KeyCode>>, mut event: EventWriter<SaveEvent>) {
    if input.pressed(KeyCode::P) {
        event.send_default();
    }
}

pub fn save_scene_system(
    world: &mut World,
    // player: Query<&GlobalTransform, With<Player>>,
    // mut event: EventReader<SaveEvent>,
) {
    // Scenes can be created from any ECS World.
    // You can either create a new one for the scene or use the current World.
    // For demonstration purposes, we'll create a new one.
    let mut scene_world = World::new();

    // The `TypeRegistry` resource contains information about all registered types (including components).
    // This is used to construct scenes, so we'll want to ensure that our previous type registrations
    // exist in this new scene world as well.
    // To do this, we can simply clone the `AppTypeRegistry` resource.
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);
    scene_world.spawn(PositionSave { x: 3.0, y: 4.0 });

    // With our sample world ready to go, we can now create our scene:
    let scene = DynamicScene::from_world(&scene_world);

    // Scenes can be serialized like this:
    let type_registry_ref = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(type_registry_ref).unwrap();

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in WASM as there is no filesystem access
    std::fs::write("assets/scene.scn.ron", serialized_scene).unwrap();
}
