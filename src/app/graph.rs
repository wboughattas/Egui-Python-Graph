use eframe::egui;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::app::websocket::GraphData;

pub struct GraphApp {
    graph_data: Arc<Mutex<GraphData>>,
    logs: Arc<Mutex<Vec<String>>>,
}

impl GraphApp {
    pub fn new(graph_data: Arc<Mutex<GraphData>>, logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self { graph_data, logs }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let graph_data = self.graph_data.lock().unwrap();
        let logs = self.logs.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            // Create two columns
            ui.columns(2, |columns| {
                // Column 1: Graph
                columns[0].heading("Graph");

                // Draw nodes
                for node in &graph_data.nodes {
                    let color = if Some(node.id.clone()) == graph_data.current_node {
                        egui::Color32::RED
                    } else {
                        egui::Color32::BLUE
                    };

                    columns[0].painter().circle_filled(
                        egui::pos2(node.x, node.y),
                        10.0,
                        color,
                    );
                    columns[0].label(&node.id);
                }

                // Draw edges with multiple-edge handling
                let mut offset_map = HashMap::new();

                for edge in &graph_data.edges {
                    // Find source and target nodes
                    let source = graph_data.nodes.iter().find(|n| n.id == edge.source);
                    let target = graph_data.nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(source), Some(target)) = (source, target) {
                        // Normalize the key for bidirectional edges
                        let key = if source.id < target.id {
                            (source.id.clone(), target.id.clone())
                        } else {
                            (target.id.clone(), source.id.clone())
                        };

                        // Use the offset for this edge and increment for subsequent edges
                        let offset = offset_map.entry(key.clone()).or_insert(0.0);
                        *offset += 5.0; // Increment the offset for each additional edge

                        let source_pos = egui::pos2(source.x, source.y);
                        let target_pos = egui::pos2(target.x, target.y);

                        // Calculate the midpoint with an offset for the arc
                        let midpoint = egui::pos2(
                            (source_pos.x + target_pos.x) / 2.0,
                            (source_pos.y + target_pos.y) / 2.0 + *offset,
                        );

                        // Draw the edge with an arc-like offset
                        columns[0].painter().line_segment(
                            [source_pos, midpoint],
                            (1.0, egui::Color32::GRAY),
                        );
                        columns[0].painter().line_segment(
                            [midpoint, target_pos],
                            (1.0, egui::Color32::GRAY),
                        );
                    }
                }

                // Column 2: Logs
                columns[1].heading("Logs");
                for log in logs.iter() {
                    columns[1].label(log);
                }
            });
        });

        ctx.request_repaint();
    }
}