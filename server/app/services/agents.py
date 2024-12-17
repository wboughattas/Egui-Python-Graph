graph_data = {
    "nodes": [
        {"id": "A", "x": 100, "y": 200},
        {"id": "B", "x": 300, "y": 200},
        {"id": "C", "x": 200, "y": 400},
    ],
    "edges": [
        {"source": "A", "target": "B"},
        {"source": "A", "target": "B"},
        {"source": "B", "target": "A"},
        {"source": "B", "target": "C"},
        {"source": "C", "target": "A"},
    ],
    "current_node": "A",
}
