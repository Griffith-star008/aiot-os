/// API for workflow_designer
use aiot_core::api::AiotError;

pub trait WorkflowDesignerInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
