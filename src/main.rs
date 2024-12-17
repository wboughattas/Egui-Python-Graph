mod app;

use app::{graph::GraphApp, websocket::{fetch_initial_data, listen_for_updates}, runtime::initialize_runtime};
use std::sync::{Arc};

fn main() -> Result<(), eframe::Error> {
    let runtime = initialize_runtime();

    let (graph_data, logs) = runtime.block_on(fetch_initial_data());

    let graph_data_clone = Arc::clone(&graph_data);
    let logs_clone = Arc::clone(&logs);
    runtime.spawn(async move {
        listen_for_updates(graph_data_clone, logs_clone).await;
    });

    let app = GraphApp::new(graph_data.clone(), logs.clone());
    let options = eframe::NativeOptions::default();
    eframe::run_native("Dynamic Graph with Logs", options, Box::new(|_| Ok(Box::new(app))))
}