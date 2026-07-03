use std::collections::BTreeMap;
use alloc::format;
use std::string::String;

#[derive(Clone, Debug, PartialEq, Eq)]
/// Documentation for ComponentInfo.
pub struct ComponentInfo {
    /// Documentation for field `name`.
    pub name: String,
    /// Documentation for field `version`.
    pub version: String,
    /// Documentation for field `license`.
    pub license: String,
    /// Documentation for field `risk_level`.
    pub risk_level: u8,
}

/// Documentation for SbomGenerator.
pub struct SbomGenerator {
    components: BTreeMap<String, ComponentInfo>,
}

impl SbomGenerator {
    /// Documentation for new.
    pub fn new() -> Self {
        Self {
            components: BTreeMap::new(),
        }
    }

    /// Documentation for register_component.
    pub fn register_component(&mut self, name: &str, version: &str, license: &str, risk: u8) {
        self.components.insert(
            String::from(name),
            ComponentInfo {
                name: String::from(name),
                version: String::from(version),
                license: String::from(license),
                risk_level: risk,
            },
        );
    }

    /// Documentation for generate_report.
    pub fn generate_report(&self) -> String {
        // Build a JSON-like string manually since we don't have serde_json in no_std
        let mut report = String::from("{\n  \"sbom\": [\n");
        let mut count = 0;
        let total = self.components.len();

        for (name, info) in &self.components {
            report.push_str("    {\n");
            report.push_str(&format!("      \"name\": \"{}\",\n", name));
            report.push_str(&format!("      \"version\": \"{}\",\n", info.version));
            report.push_str(&format!("      \"license\": \"{}\",\n", info.license));
            report.push_str(&format!("      \"risk_level\": {}\n", info.risk_level));
            report.push_str("    }");

            count += 1;
            if count < total {
                report.push_str(",\n");
            } else {
                report.push('\n');
            }
        }
        report.push_str("  ]\n}");
        report
    }
}

impl Default for SbomGenerator {
    fn default() -> Self {
        Self::new()
    }
}
