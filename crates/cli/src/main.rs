use clap::{Parser, Subcommand};
use runtime::{RuntimeBuilder, LifecycleState};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "aiot", about = "AIoT Global Framework CLI", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new component (e.g. plugin)
    New {
        /// Type of component to create (e.g., plugin)
        component_type: String,
        /// Name of the component
        name: String,
    },
    /// Run the AIoT framework
    Run,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { component_type, name } => {
            if component_type == "plugin" {
                println!("Scaffolding new plugin: {}", name);
                
                let dir_path = Path::new(name);
                fs::create_dir_all(dir_path).expect("Failed to create plugin directory");
                fs::create_dir_all(dir_path.join("src")).expect("Failed to create src directory");

                let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition.workspace = true

[dependencies]
plugins = {{ path = "../../crates/plugins" }}
aiot_core = {{ path = "../../crates/core" }}
"#, name);
                fs::write(dir_path.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");

                let plugin_toml = format!(r#"name = "{}"
version = "0.1.0"
api_version = "v1"
capabilities = ["base"]
"#, name);
                fs::write(dir_path.join("plugin.toml"), plugin_toml).expect("Failed to write plugin.toml");

                let lib_rs = format!(r#"use plugins::{{Plugin, PluginId, PluginType}};
use aiot_core::api::AiotError;

pub struct {}Plugin {{
    id: PluginId,
}}

impl Plugin for {}Plugin {{
    fn id(&self) -> PluginId {{
        PluginId(self.id.0)
    }}
    
    fn plugin_type(&self) -> PluginType {{
        PluginType::ControlAlgorithm
    }}

    fn version(&self) -> u16 {{
        1
    }}

    fn start(&mut self) -> Result<(), AiotError> {{
        println!("{} plugin started");
        Ok(())
    }}

    fn stop(&mut self) -> Result<(), AiotError> {{
        println!("{} plugin stopped");
        Ok(())
    }}
}}
"#, name, name, name, name);
                fs::write(dir_path.join("src").join("lib.rs"), lib_rs).expect("Failed to write lib.rs");

                println!("- Created {}/src/lib.rs (Plugin trait implementation)", name);
                println!("- Created {}/Cargo.toml", name);
                println!("- Created {}/plugin.toml", name);
                println!("Plugin `{}` generated successfully!", name);
            } else {
                println!("Unknown component type: {}", component_type);
            }
        }
        Commands::Run => {
            println!("Starting AIoT Framework...");
            let mut _runtime = RuntimeBuilder::new()
                .with_initial_state(LifecycleState::Created)
                .build();
            println!("Framework started successfully in Created state.");
        }
    }
}
