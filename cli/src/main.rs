use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser)]
#[command(author, version, about = "Rayan: The Neuro-Symbolic Environment Engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Applies the literate document to the current machine
    Up {
        #[arg(short, long, default_value = "rayan.md")]
        file: String,
    },
    /// Shows the history of applied machine states
    History,
    /// Reverts the machine state to a previous generation
    Revert { generation: String },
    /// Starts the Language Server Protocol (LSP) daemon
    Lsp,
    /// Starts the background drift detection daemon
    Daemon,
    /// Pulls a semantic ASG node from the Rayan Global Knowledge Commons
    Pull { node_id: String },
    /// Starts the Rayan Terminal User Interface (TUI) Dashboard
    Tui,
    /// Asks the Neuro-Symbolic Agent to generate an ASG mutation from natural language
    Reason { prompt: String },
    /// Pushes the local ASG history to the Rayan Global Commons
    Push,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Up { file } => {
            info!("Applying literate document: {}", file);
            // 1. Read document
            let doc = std::fs::read_to_string(file).unwrap_or_else(|_| String::new());

            // 2. Parse into ASG
            let asg = rayan_parser::parse_literate_document(&doc)?;
            info!("Parsed ASG with {} nodes", asg.nodes.len());

            // 3. Validate ASG using the Neuro-Symbolic StateEngine
            let state_engine = rayan_core::state::StateEngine::new();
            if let Err(e) = state_engine.validate_transition(&asg) {
                tracing::error!("Validation Failed:\n{}", e);
                std::process::exit(1);
            }
            info!("ASG Validation Passed (0 conflicts)");

            // 4. Compile to Nix (or trigger solver)
            let nix_expr = rayan_compiler::compile_to_nix(&asg)?;
            println!(
                "\n--- Generated Nix ---\n{}\n---------------------",
                nix_expr
            );
            info!("Generated Nix expression length: {}", nix_expr.len());

            // 5. (Future) Trigger evaluation
        }
        Commands::History => {
            info!("Showing machine generation history...");
        }
        Commands::Revert { generation } => {
            info!("Reverting to generation {}", generation);
        }
        Commands::Lsp => {
            // We do not want tracing to write to stdout because LSP uses stdout for JSON-RPC
            // In a real implementation we would route tracing to a file or stderr.
            rayan_lsp::run_server().await;
        }
        Commands::Daemon => {
            info!("Initializing Drift Daemon...");
            // Watch a mock config directory for testing Phase 1
            let detector = rayan_daemon::DriftDetector::new("./mock_config");
            if let Err(e) = detector.start() {
                tracing::error!("Daemon crashed: {}", e);
            }
        }
        Commands::Pull { node_id } => {
            info!("Connecting to Global Knowledge Commons...");
            let client = rayan_commons_client::RayanHubClient::default();
            match client.pull_node(node_id).await {
                Ok(node) => {
                    println!("\n--- Pulled Node: {} ---", node_id);
                    println!("{}", serde_json::to_string_pretty(&node).unwrap());
                    println!("----------------------------------");
                    info!("Successfully merged into local ASG (Mocked)");
                }
                Err(e) => {
                    tracing::error!("Pull failed: {}", e);
                }
            }
        }
        Commands::Tui => {
            if let Err(e) = rayan_tui::run_tui().await {
                tracing::error!("TUI crashed: {}", e);
            }
        }
        Commands::Reason { prompt } => {
            info!("Initializing Neuro-Symbolic Agent...");
            let agent = rayan_agent::NeuroAgent::default();
            match agent.reason(prompt).await {
                Ok(asg) => {
                    info!("Agent generated ASG. Passing to StateEngine for validation...");
                    let state_engine = rayan_core::state::StateEngine::new();
                    if let Err(e) = state_engine.validate_transition(&asg) {
                        tracing::error!("Agent hallucinated an invalid configuration!\n{}", e);
                        std::process::exit(1);
                    }

                    println!("\n--- Agent Generated Valid ASG ---");
                    println!("{}", serde_json::to_string_pretty(&asg).unwrap());
                    println!("---------------------------------");
                }
                Err(e) => {
                    tracing::error!("Agent reasoning failed: {}", e);
                }
            }
        }
        Commands::Push => {
            info!("Initializing Cloud Sync Engine...");
            let syncer = rayan_daemon::CloudSyncer::new("http://localhost:3000");
            let tracker = rayan_daemon::RollbackTracker::new("history.db").unwrap_or_else(|_| {
                tracing::error!("Could not open local history.db");
                std::process::exit(1);
            });
            let records = tracker.list_generations().unwrap_or_default();
            info!(
                "Found {} local generations. Pushing to Global Commons...",
                records.len()
            );
            for record in records {
                if let Err(e) = syncer.push_generation(&record).await {
                    tracing::error!("Failed to sync generation {}: {}", record.id, e);
                }
            }
            info!("Cloud sync complete.");
        }
    }

    Ok(())
}
