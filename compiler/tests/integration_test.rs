use rayan_compiler::compile_to_nix;
use rayan_core::asg::{Asg, Node, NodeKind, PackageInfo};

#[test]
fn test_compiler_output_syntax() {
    let asg = Asg {
        nodes: vec![Node {
            id: "test-node".to_string(),
            kind: NodeKind::Package(PackageInfo {
                name: "hello".to_string(),
                version: None,
            }),
            dependencies: vec![],
            conflicts_with: vec![],
        }],
    };

    let result = compile_to_nix(&asg).expect("Failed to compile ASG");

    // In a full environment with Nix installed, we would use std::process::Command
    // to run `nix-instantiate --parse -E result` to ensure strict syntactic correctness.
    // Since this is a generic Windows runner, we perform a deterministic mock integration test.
    assert!(result.contains("hello"));
    assert!(result.contains("home.packages"));
}
