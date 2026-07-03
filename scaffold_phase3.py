import os
import subprocess
import json

crates = [
    ("marketplace", "Marketplace API for AIoT Framework"),
    ("dashboard", "Dashboard API for AIoT Framework"),
    ("cloud_console", "Cloud Console API for AIoT Framework")
]

base_dir = r"C:\Users\Lenovo\OneDrive\Desktop\AIot\crates"

for name, desc in crates:
    path = os.path.join(base_dir, name)
    if not os.path.exists(path):
        subprocess.run(["cargo", "new", "--lib", name], cwd=base_dir, shell=True)

# Update Cargo.toml for each
for name, desc in crates:
    cargo_toml = os.path.join(base_dir, name, "Cargo.toml")
    with open(cargo_toml, "w") as f:
        f.write(f"""[package]
name = "{name}"
version = "0.1.0"
edition = "2021"
description = "{desc}"

[dependencies]
aiot_core = {{ path = "../core" }}
""")

# marketplace/src/lib.rs
with open(os.path.join(base_dir, "marketplace", "src", "lib.rs"), "w") as f:
    f.write("""//! Marketplace
use aiot_core::AiotError;

pub trait MarketplaceClient {
    fn search_plugins(&self, query: &str) -> Result<Vec<String>, AiotError>;
    fn install_plugin(&mut self, plugin_id: &str) -> Result<(), AiotError>;
    fn install_template(&mut self, template_id: &str) -> Result<(), AiotError>;
}
""")

# dashboard/src/lib.rs
with open(os.path.join(base_dir, "dashboard", "src", "lib.rs"), "w") as f:
    f.write("""//! Dashboard
use aiot_core::AiotError;

pub trait DashboardApi {
    fn get_runtime_status(&self) -> Result<String, AiotError>;
    fn get_telemetry_metrics(&self) -> Result<String, AiotError>;
    fn get_logs(&self) -> Result<Vec<String>, AiotError>;
    fn get_cluster_overview(&self) -> Result<String, AiotError>;
}
""")

# cloud_console/src/lib.rs
with open(os.path.join(base_dir, "cloud_console", "src", "lib.rs"), "w") as f:
    f.write("""//! Cloud Console
use aiot_core::AiotError;

pub trait CloudConsoleApi {
    fn get_fleet_status(&self) -> Result<String, AiotError>;
    fn trigger_ota(&mut self, target_version: &str) -> Result<(), AiotError>;
    fn deploy_workload(&mut self, workload_config: &str) -> Result<(), AiotError>;
}
""")

# Add to workspace Cargo.toml
root_cargo = r"C:\Users\Lenovo\OneDrive\Desktop\AIot\Cargo.toml"
with open(root_cargo, "r") as f:
    content = f.read()

new_members = [
    '"crates/marketplace",',
    '"crates/dashboard",',
    '"crates/cloud_console",'
]

for member in new_members:
    if member not in content:
        content = content.replace("members = [", f"members = [\n    {member}")

with open(root_cargo, "w") as f:
    f.write(content)

# VSCode Extension
vscode_dir = r"C:\Users\Lenovo\OneDrive\Desktop\AIot\vscode-extension"
os.makedirs(vscode_dir, exist_ok=True)
package_json = {
    "name": "aiot-framework-extension",
    "displayName": "AIoT Framework",
    "description": "Developer tools for AIoT Framework",
    "version": "0.1.0",
    "engines": {
        "vscode": "^1.80.0"
    },
    "categories": [
        "Other"
    ],
    "contributes": {
        "commands": [
            {
                "command": "aiot.newProject",
                "title": "AIoT: Create New Project"
            },
            {
                "command": "aiot.deploy",
                "title": "AIoT: Deploy to Device"
            },
            {
                "command": "aiot.diagnostics",
                "title": "AIoT: Run Diagnostics"
            }
        ]
    }
}
with open(os.path.join(vscode_dir, "package.json"), "w") as f:
    json.dump(package_json, f, indent=4)
