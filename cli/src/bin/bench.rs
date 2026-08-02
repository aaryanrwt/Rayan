use std::time::Instant;

fn main() {
    let mock_doc = r#"
# Rayan Mock Config

```rayan
[
  {
    "id": "git_pkg",
    "kind": { "Package": { "name": "git", "version": null } }
  },
  {
    "id": "neovim_pkg",
    "kind": { "Package": { "name": "neovim", "version": null } }
  },
  {
    "id": "docker_svc",
    "kind": { "Service": { "name": "docker", "enable": true } }
  }
]
```
"#;

    let iterations = 10_000;

    println!(
        "Starting Rayan Core Benchmarks ({} iterations)...\n",
        iterations
    );

    // 1. Benchmark Parser
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = rayan_parser::parse_literate_document(mock_doc).unwrap();
    }
    let parse_time = start.elapsed();
    println!("Parser: {:?}", parse_time);

    let asg = rayan_parser::parse_literate_document(mock_doc).unwrap();

    // 2. Benchmark Compiler to Nix
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = rayan_compiler::compile_to_nix(&asg).unwrap();
    }
    let nix_time = start.elapsed();
    println!("Compiler (Nix): {:?}", nix_time);

    // 3. Benchmark Compiler to Darwin
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = rayan_compiler::compile_to_darwin(&asg).unwrap();
    }
    let darwin_time = start.elapsed();
    println!("Compiler (Darwin): {:?}", darwin_time);

    // 4. Benchmark Compiler to OCI
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = rayan_compiler::compile_to_oci(&asg).unwrap();
    }
    let oci_time = start.elapsed();
    println!("Compiler (OCI): {:?}", oci_time);
}
