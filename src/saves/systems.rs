use std::{fs::File, io::Write};

use bevy::{prelude::*, tasks::IoTaskPool};

use crate::player::components::Player;

use super::components::{PositionSaveComponent, PositionSaveInformation, SaveEvent};

// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/scene.scn.ron";

pub fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Spawning" a scene bundle creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    commands.spawn(DynamicSceneBundle {
        // Scenes are loaded just like any other asset.
        scene: asset_server.load(SCENE_FILE_PATH),
        ..default()
    });
}

pub fn move_player(
    mut commands: Commands,
    mut query_player: Query<&mut Transform, With<Player>>,
    position_save_query: Query<(Entity, &PositionSaveComponent)>,
) {
    let mut player = query_player.single_mut();

    if let Ok((entity, position)) = position_save_query.get_single() {
        player.translation.x = position.x;
        player.translation.y = position.y;
        commands.entity(entity).despawn();
    }
}

pub fn check_for_save(input: Res<Input<KeyCode>>, mut event: EventWriter<SaveEvent>) {
    if input.pressed(KeyCode::P) {
        event.send_default();
    }
}

pub fn save_scene_system(world: &mut World) {
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
    let pos_save = world.resource::<PositionSaveInformation>();
    scene_world.spawn(PositionSaveComponent {
        x: pos_save.x,
        y: pos_save.y,
    });

    // With our sample world ready to go, we can now create our scene:
    let scene = DynamicScene::from_world(&scene_world);

    // Scenes can be serialized like this:
    let type_registry_ref = scene_world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(type_registry_ref).unwrap();

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in WASM as there is no filesystem access
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}
