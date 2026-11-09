//! Fog in the level editor's environment mixer: its global fog and local
//! fog volumes, which the plugin's root registers through the level
//! editor's extension points (`ui_level_editor::extend`). Each add is one
//! undoable command; the components are this plugin's.

use ui_level_editor::LevelEditorState;
use ui_level_editor::commands::{CommandResult, TypedComponent};
use ui_level_editor::extend::add_object;
use ui_level_editor::scene_edit::{ObjectType, Transform};

/// The mixer's global fog: a default medium filling the world.
pub fn add_global_fog(state: &mut LevelEditorState) -> CommandResult {
    add_object(
        state,
        "Global Fog",
        ObjectType::Empty,
        Transform::default(),
        vec![TypedComponent::new(
            fog_component::GlobalFogComponent::default(),
        )],
    )
}

/// The mixer's local fog volume: a default medium in its owner's box.
pub fn add_local_fog_volume(state: &mut LevelEditorState) -> CommandResult {
    add_object(
        state,
        "Local Fog Volume",
        ObjectType::Empty,
        Transform::default(),
        vec![TypedComponent::new(
            fog_component::LocalFogVolumeComponent::default(),
        )],
    )
}
