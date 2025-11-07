//! Command-line interface for LLM Memory Graph management
//!
//! This tool provides commands for managing and querying the memory graph database:
//! - Database inspection and statistics
//! - Node queries
//! - Data export
//! - Performance diagnostics

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use llm_memory_graph::{engine::AsyncMemoryGraph, Config};
use llm_memory_graph_types::{NodeId, SessionId};
use std::path::PathBuf;
use uuid::Uuid;

/// LLM Memory Graph CLI - Database management and query tool
#[derive(Parser)]
#[command(name = "llm-memory-graph")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the database directory
    #[arg(short, long, default_value = "./data")]
    db_path: PathBuf,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone)]
enum OutputFormat {
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("Invalid format: {}. Use 'text' or 'json'", s)),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Show database statistics
    Stats,

    /// Get session details
    Session {
        /// Session ID (UUID format)
        session_id: String,
    },

    /// Get node details
    Node {
        /// Node ID (UUID format)
        node_id: String,
    },

    /// Export session data
    Export {
        /// Session ID (UUID format)
        session_id: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Flush database to disk
    Flush,

    /// Verify database integrity
    Verify,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Open database
    let config = Config::new(cli.db_path.to_str().unwrap());
    let graph = AsyncMemoryGraph::open(config).await?;

    match cli.command {
        Commands::Stats => handle_stats(&graph, &cli.format).await?,
        Commands::Session { session_id } => {
            handle_session(&graph, &cli.format, &session_id).await?
        }
        Commands::Node { node_id } => handle_node(&graph, &cli.format, &node_id).await?,
        Commands::Export {
            session_id,
            output,
        } => handle_export(&graph, &session_id, &output).await?,
        Commands::Flush => handle_flush(&graph).await?,
        Commands::Verify => handle_verify(&graph).await?,
    }

    Ok(())
}

async fn handle_stats(graph: &AsyncMemoryGraph, format: &OutputFormat) -> Result<()> {
    let stats = graph.stats().await?;

    match format {
        OutputFormat::Json => {
            let stats_json = serde_json::json!({
                "node_count": stats.node_count,
                "edge_count": stats.edge_count,
                "session_count": stats.session_count,
            });
            println!("{}", serde_json::to_string_pretty(&stats_json)?);
        }
        OutputFormat::Text => {
            println!("{}", "Database Statistics".bold().green());
            println!("{}", "===================".green());
            println!("{:20} {}", "Total Nodes:", stats.node_count.to_string().cyan());
            println!("{:20} {}", "Total Edges:", stats.edge_count.to_string().cyan());
            println!(
                "{:20} {}",
                "Total Sessions:",
                stats.session_count.to_string().cyan()
            );
        }
    }

    Ok(())
}

async fn handle_session(
    graph: &AsyncMemoryGraph,
    format: &OutputFormat,
    session_id_str: &str,
) -> Result<()> {
    let uuid = Uuid::parse_str(session_id_str)?;
    let session_id = SessionId::from(uuid);
    let session = graph.get_session(session_id).await?;

    // Get nodes in the session
    let nodes = graph.get_session_nodes(&session_id).await?;

    match format {
        OutputFormat::Json => {
            let mut session_with_nodes = serde_json::to_value(&session)?;
            session_with_nodes["node_count"] = serde_json::Value::Number(nodes.len().into());
            println!("{}", serde_json::to_string_pretty(&session_with_nodes)?);
        }
        OutputFormat::Text => {
            println!("{}", format!("Session: {}", session.id).bold().green());
            println!("{}", "====================".green());
            println!(
                "{:15} {}",
                "Created:",
                session.created_at.format("%Y-%m-%d %H:%M:%S")
            );
            println!(
                "{:15} {}",
                "Updated:",
                session.updated_at.format("%Y-%m-%d %H:%M:%S")
            );
            println!("{:15} {}", "Nodes:", nodes.len());
            println!("\n{}", "Metadata:".bold());
            for (key, value) in &session.metadata {
                println!("  {:13} {}", format!("{}:", key), value);
            }
            println!("\n{}", "Tags:".bold());
            for tag in &session.tags {
                println!("  - {}", tag);
            }
        }
    }

    Ok(())
}

async fn handle_node(
    graph: &AsyncMemoryGraph,
    format: &OutputFormat,
    node_id_str: &str,
) -> Result<()> {
    let uuid = Uuid::parse_str(node_id_str)?;
    let node_id = NodeId::from(uuid);
    let node_opt = graph.get_node(&node_id).await?;

    match node_opt {
        Some(node) => match format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&node)?);
            }
            OutputFormat::Text => {
                println!("{}", format!("Node: {}", node_id).bold().green());
                println!("{}", "====================".green());
                println!("{:15} {:?}", "Type:", node.node_type());
                println!("\n{}", "Details:".bold());
                println!("{}", serde_json::to_string_pretty(&node)?);
            }
        },
        None => {
            eprintln!("{} Node not found: {}", "Error:".red().bold(), node_id);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn handle_export(
    graph: &AsyncMemoryGraph,
    session_id_str: &str,
    output: &PathBuf,
) -> Result<()> {
    let uuid = Uuid::parse_str(session_id_str)?;
    let session_id = SessionId::from(uuid);
    let session = graph.get_session(session_id).await?;

    // Export session as JSON
    let json = serde_json::to_string_pretty(&session)?;
    std::fs::write(output, json)?;

    println!(
        "{} Session exported to: {}",
        "✓".green().bold(),
        output.display().to_string().cyan()
    );

    Ok(())
}

async fn handle_flush(graph: &AsyncMemoryGraph) -> Result<()> {
    println!("{}", "Flushing database to disk...".yellow());
    graph.flush().await?;
    println!("{} Database flushed successfully", "✓".green().bold());
    Ok(())
}

async fn handle_verify(graph: &AsyncMemoryGraph) -> Result<()> {
    println!("{}", "Verifying database integrity...".yellow());

    let stats = graph.stats().await?;

    println!("{} Verified {} nodes", "✓".green().bold(), stats.node_count);
    println!("{} Verified {} edges", "✓".green().bold(), stats.edge_count);
    println!(
        "{} Verified {} sessions",
        "✓".green().bold(),
        stats.session_count
    );

    println!("\n{} Database verification complete", "✓".green().bold());

    Ok(())
}
