/// Plugin Marketplace API
pub enum Category {
    Vision, Speech, Cloud, Robot, IoT, Database, AI, Analytics
}

pub struct MarketplaceManifest {
    pub name: String,
    pub downloads: u64,
    pub rating: f32,
    pub compatible_framework_version: String,
}

pub trait MarketplaceClient {
    fn search(&self, keyword: &str, category: Option<Category>) -> std::vec::Vec<MarketplaceManifest>;
}
