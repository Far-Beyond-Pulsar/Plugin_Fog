//! Fog (`pulsar.fog`): the editor crate -- the environment mixer's fog
//! entries ([`level_editor`]). It registers nothing itself: the plugin's
//! root crate (`fog_plugin`, with its `editor` feature) submits them
//! through the level editor's extension points. The inspector edits the
//! components through their reflected properties.

pub mod level_editor;
