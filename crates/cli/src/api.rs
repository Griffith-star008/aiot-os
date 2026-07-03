/// CLI Package Manager API
use aiot_core::api::AiotError;

pub enum PackageManagerCommand {
    Add { plugin_name: String },
    Remove { plugin_name: String },
    Update,
    Publish,
}

pub trait PackageManagerInterface {
    fn execute(&mut self, cmd: PackageManagerCommand) -> Result<(), AiotError>;
}

/// Other CLI commands
pub enum GeneralCommand {
    New { project_type: String },
    Doctor,
    Benchmark,
    Lint,
    Upgrade,
    Migrate,
}
