use anyhow::Result;
use rayan_core::asg::{Asg, NodeKind};
use serde_json::json;

/// Generates a CycloneDX v1.4 compatible SBOM from a Rayan ASG.
pub fn generate_cyclonedx_sbom(asg: &Asg) -> Result<String> {
    let mut components = Vec::new();

    for node in &asg.nodes {
        if let NodeKind::Package(pkg) = &node.kind {
            components.push(json!({
                "type": "application",
                "name": pkg.name,
                "version": pkg.version.clone().unwrap_or_else(|| "latest".to_string()),
                "purl": format!("pkg:generic/{}@{}", pkg.name, pkg.version.clone().unwrap_or_else(|| "latest".to_string()))
            }));
        }
    }

    let sbom = json!({
        "bomFormat": "CycloneDX",
        "specVersion": "1.4",
        "version": 1,
        "metadata": {
            "component": {
                "type": "operating-system",
                "name": "Rayan Environment"
            }
        },
        "components": components
    });

    Ok(serde_json::to_string_pretty(&sbom)?)
}
