//! What the fog plugin registers for a game (and the editor): every runtime
//! contribution, grouped by the engine seam it plugs into.

use engine_backend::scene::SceneRowsRegistration;
use engine_backend::subsystems::render::RenderFeatureRegistration;
use fog_component::{GlobalFogComponent, LocalFogVolumeComponent};

// ── Component classes ───────────────────────────────────────────────────────

pulsar_world_registry::submit_world_component!(GlobalFogComponent, LocalFogVolumeComponent);

// ── Rendering ───────────────────────────────────────────────────────────────

// The media's source rows and the row join that places a local volume at
// its owner.
inventory::submit! {
    SceneRowsRegistration {
        origin: pulsar_plugins::origin!(),
        name: "fog.rows",
        register_columns: fog_component::register_columns,
        derivation: Some(fog_component::derivation),
    }
}

// The volumetric medium and its composite; they read no project settings
// through the renderer config.
inventory::submit! {
    RenderFeatureRegistration {
        origin: pulsar_plugins::origin!(),
        features: fog_component::render_features,
        configure: |config| config,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn registers_its_feature() {
        let features =
            engine_backend::subsystems::render::helio_renderer::render_features::plugin_features();
        assert!(
            features.ids().contains(&"helio.fog"),
            "features: {:?}",
            features.ids()
        );
    }

    /// Every registration of the plugin names this root, so it resolves to
    /// the plugin's id.
    #[test]
    fn every_registration_resolves_to_the_plugin() {
        let ours: Vec<_> = pulsar_plugins::contributions()
            .into_iter()
            .filter(|summary| summary.origin == pulsar_plugins::origin!())
            .collect();
        assert!(
            ours.iter()
                .any(|summary| summary.category == "component.class")
        );
        for summary in &ours {
            assert_eq!(
                pulsar_plugins::plugin_of(summary.origin).map(|plugin| plugin.id.as_str()),
                Some("pulsar.fog"),
                "{summary:?}"
            );
        }
    }
}
