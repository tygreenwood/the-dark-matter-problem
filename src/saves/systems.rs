use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

use crate::{player::components::ControlledPlayer, wheel::components::Wheel};

use super::{
    components::{PositionSaveComponent, WheelSaveComponent},
    configs::SAVES_PATH,
    resources::{PositionSaveInformation, SaveGame, WheelSaveInformation},
};

pub fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Spawning" a scene bundle creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    commands.spawn(DynamicSceneBundle {
        // Scenes are loaded just like any other asset.
        scene: asset_server.load(SAVES_PATH),
        ..default()
    });
}

pub fn load_save(
    mut commands: Commands,
    mut query_player: Query<&mut Transform, (With<ControlledPlayer>, Without<Wheel>)>,
    mut query_wheel: Query<&mut Transform, (With<Wheel>, Without<ControlledPlayer>)>,
    position_save_query: Query<(Entity, &PositionSaveComponent)>,
    wheel_save_query: Query<(Entity, &WheelSaveComponent)>,
) {
    let mut player = query_player.single_mut();
    let mut wheel = query_wheel.single_mut();

    if let Ok((entity, position)) = position_save_query.get_single() {
        player.translation.x = position.x;
        player.translation.y = position.y;
        commands.entity(entity).despawn();
    }

    if let Ok((entity, rotation)) = wheel_save_query.get_single() {
        wheel.rotation = rotation.rot;
        commands.entity(entity).despawn();
    }
}

pub fn check_for_save(mut save_game: ResMut<SaveGame>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::P) {
        save_game.save = true;
    }
}

pub fn save_scene_system(world: &mut World) {
    if world.resource::<SaveGame>().save {
        if let Some(mut save_game) = world.get_resource_mut::<SaveGame>() {
            save_game.save = false;
        }

        let mut scene_world = World::new();

        let type_registry = world.resource::<AppTypeRegistry>().clone();
        scene_world.insert_resource(type_registry);
        let pos_save = world.resource::<PositionSaveInformation>();
        scene_world.spawn(PositionSaveComponent {
            x: pos_save.x,
            y: pos_save.y,
        });

        let wheel_rot_save = world.resource::<WheelSaveInformation>();
        scene_world.spawn(WheelSaveComponent {
            rot: wheel_rot_save.rot,
        });

        let scene = DynamicScene::from_world(&scene_world);

        let type_registry_ref = scene_world.resource::<AppTypeRegistry>();
        let serialized_scene = scene.serialize_ron(type_registry_ref).unwrap();

        // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
        // as they are blocking
        // This can't work in WASM as there is no filesystem access
        #[cfg(not(target_arch = "wasm32"))]
        IoTaskPool::get()
            .spawn(async move {
                // Write the scene RON data to file
                File::create(format!("assets/{SAVES_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing scene to file");
            })
            .detach();
    }
}
