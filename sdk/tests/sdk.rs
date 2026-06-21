use sdk::ZtnaPluginManifest;

#[test]
fn manifest_defaults() {
    let manifest = ZtnaPluginManifest::new("ztna-plugin", "0.1.0", "custom");
    assert_eq!(manifest.name, "ztna-plugin");
    assert_eq!(manifest.min_trust_score, 50);
}
