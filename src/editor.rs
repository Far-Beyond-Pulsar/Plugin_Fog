//! What the fog plugin registers for the editor (the `editor` feature): its
//! environment mixer entries.

use fog_editor::level_editor;
use ui_level_editor::extend::{CreateEntry, EnvironmentListing};

// ── Environment mixer ───────────────────────────────────────────────────────

inventory::submit! {
    CreateEntry {
        origin: pulsar_plugins::origin!(),
        id: "environment_global_fog",
        menu_label: None,
        environment_label: Some("Global Fog"),
        quick_add: None,
        offered: None,
        order: 410,
        create: level_editor::add_global_fog,
    }
}

inventory::submit! {
    EnvironmentListing {
        origin: pulsar_plugins::origin!(),
        class_name: "GlobalFogComponent",
        label: "Global Fog",
        order: 10,
        create_entry: Some("environment_global_fog"),
    }
}

inventory::submit! {
    CreateEntry {
        origin: pulsar_plugins::origin!(),
        id: "environment_local_fog_volume",
        menu_label: None,
        environment_label: Some("Local Fog Volume"),
        quick_add: None,
        offered: None,
        order: 420,
        create: level_editor::add_local_fog_volume,
    }
}

inventory::submit! {
    EnvironmentListing {
        origin: pulsar_plugins::origin!(),
        class_name: "LocalFogVolumeComponent",
        label: "Local Fog Volume",
        order: 20,
        create_entry: Some("environment_local_fog_volume"),
    }
}
