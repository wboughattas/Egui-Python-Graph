use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
    pub id: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Edge {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GraphData {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub current_node: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Logs {
    pub logs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LogMessage {
    pub log: String,
}


/// Fetch initial graph and logs
pub async fn fetch_initial_data() -> (Arc<Mutex<GraphData>>, Arc<Mutex<Vec<String>>>) {
    loop {
        match connect_async("ws://127.0.0.1:8000/ws").await {
            Ok((mut ws_stream, _)) => {
                println!("Connected to WebSocket server.");

                // Request graph and logs
                ws_stream
                    .send(Message::Text(r#"{"action":"get_graph"}"#.to_string()))
                    .await
                    .unwrap();
                ws_stream
                    .send(Message::Text(r#"{"action":"get_logs"}"#.to_string()))
                    .await
                    .unwrap();

                let mut graph_data = None;
                let mut logs = None;

                while let Some(msg) = ws_stream.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        if text.contains("nodes") && text.contains("edges") {
                            graph_data = Some(serde_json::from_str::<GraphData>(&text).unwrap());
                        } else if text.contains("logs") {
                            logs = Some(serde_json::from_str::<Logs>(&text).unwrap().logs);
                        }
                    }

                    if graph_data.is_some() && logs.is_some() {
                        return (
                            Arc::new(Mutex::new(graph_data.unwrap())),
                            Arc::new(Mutex::new(logs.unwrap())),
                        );
                    }
                }
            }
            Err(e) => {
                println!("Connection failed: {:?}. Retrying...", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}

/// Dynamically update graph data and logs
pub async fn listen_for_updates(
    graph_data: Arc<Mutex<GraphData>>,
    logs: Arc<Mutex<Vec<String>>>,
) {
    if let Ok((mut ws_stream, _)) = connect_async("ws://127.0.0.1:8000/ws").await {
        println!("Listening for updates...");
        while let Some(msg) = ws_stream.next().await {
            if let Ok(Message::Text(text)) = msg {
                println!("Received from backend: {}", text);

                if text.contains("nodes") && text.contains("edges") {
                    let updated_graph = serde_json::from_str::<GraphData>(&text).unwrap();
                    println!("Updated graph data: {:?}", updated_graph);
                    *graph_data.lock().unwrap() = updated_graph;
                }

                if text.contains("log") {
                    let log_message = serde_json::from_str::<LogMessage>(&text).unwrap();
                    println!("New log: {:?}", log_message); // Log the message
                    logs.lock().unwrap().push(log_message.log);
                }
            }
        }
    }
}