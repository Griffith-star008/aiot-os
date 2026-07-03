//! Public API Interfaces


#![deny(unsafe_code)]

/// World Model Layer
/// Xây dựng biểu diễn nội tại của môi trường và các thực thể xung quanh.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for EntityType.
pub enum EntityType {
    Obstacle,
    Human,
    Vehicle,
    Unknown,
}

/// Documentation for SpatialObject.
pub struct SpatialObject {
    /// Documentation for field `id`.
    pub id: u32,
    /// Documentation for field `entity_type`.
    pub entity_type: EntityType,
    /// Documentation for field `x_pos`.
    pub x_pos: i32,
    /// Documentation for field `y_pos`.
    pub y_pos: i32,
    /// Documentation for field `z_pos`.
    pub z_pos: i32,
    /// Documentation for field `velocity_x`.
    pub velocity_x: i32,
}

/// Documentation for EnvironmentModel.
pub trait EnvironmentModel {
    /// Cập nhật vị trí của các thực thể trong môi trường (SLAM / Perception)
    fn update_objects(&mut self, objects: &[SpatialObject]) -> Result<(), WorldError>;

    /// Trả về đối tượng gần nhất để cảnh báo va chạm
    fn get_nearest_obstacle(&self) -> Option<SpatialObject>;

    /// Kích hoạt mô phỏng tương lai 1 giây (Predictive Modeling)
    fn simulate_future_state(&self) -> Result<(), WorldError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for WorldError.
pub enum WorldError {
    MapFull,
    SimulationTimeout,
}
