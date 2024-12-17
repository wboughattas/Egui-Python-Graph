use tokio::runtime::Runtime;

/// Initializes the Tokio runtime.
pub fn initialize_runtime() -> Runtime {
    Runtime::new().expect("Failed to create Tokio runtime")
}