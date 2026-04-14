import { useEffect, useState } from "react";

const NODE_COUNT = 20;
const WIDTH = 900;
const HEIGHT = 600;
const RADIUS = 220;
const CENTER_X = WIDTH / 2;
const CENTER_Y = HEIGHT / 2;

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState([]);
  const [selectedNodeId, setSelectedNodeId] = useState(null);

  // 👉 Live selected node
  const selectedNode = nodes.find(n => n.id === selectedNodeId);

  // 🧠 Initialize network
  useEffect(() => {
    const initial = Array.from({ length: NODE_COUNT }).map((_, i) => {
      const angle = (i / NODE_COUNT) * Math.PI * 2;

      return {
        id: i,
        x: CENTER_X + Math.cos(angle) * RADIUS,
        y: CENTER_Y + Math.sin(angle) * RADIUS,
        trust: 100,
        drift: 0,
        status: "normal",
        neighbors: [],
      };
    });

    // 🔥 HYBRID TOPOLOGY (fixes ring problem)
    initial.forEach((node) => {
      const others = initial.filter(n => n.id !== node.id);

      // 🔹 2 nearest neighbors (local structure)
      const nearest = others
        .map(n => ({
          id: n.id,
          dist: Math.hypot(node.x - n.x, node.y - n.y)
        }))
        .sort((a, b) => a.dist - b.dist)
        .slice(0, 2)
        .map(n => n.id);

      // 🔹 1 random long-range connection (break symmetry)
      const random =
        others[Math.floor(Math.random() * others.length)].id;

      node.neighbors = [...new Set([...nearest, random])];
    });

    // 🔴 Simulate attack
    initial[19].status = "attacked";
    initial[19].trust = 0;

    setNodes(initial);
  }, []);

  return (
    <div style={{ background: "#020617", height: "100vh", color: "white" }}>
      
      {/* 🧠 TITLE */}
      <h1
        style={{
          textAlign: "center",
          letterSpacing: "4px",
          fontWeight: "300",
          marginBottom: "10px"
        }}
      >
        FLUXLOCK NETWORK GRAPH
      </h1>

      {/* 🔷 GRAPH */}
      <svg
        width={WIDTH}
        height={HEIGHT}
        style={{ display: "block", margin: "0 auto" }}
      >
        {/* 🔗 EDGES */}
        {nodes.map(node =>
          node.neighbors.map(nId => {
            const target = nodes.find(n => n.id === nId);
            if (!target) return null;

            const isAttackEdge =
              node.status === "attacked" ||
              target.status === "attacked";

            return (
              <line
                key={`${node.id}-${nId}`}
                x1={node.x}
                y1={node.y}
                x2={target.x}
                y2={target.y}
                stroke={isAttackEdge ? "#ff3b3b" : "#38bdf8"}
                strokeWidth={isAttackEdge ? 2.5 : 1.5}
                opacity={0.85}
              />
            );
          })
        )}

        {/* 🔵 NODES */}
        {nodes.map(node => (
          <g
            key={node.id}
            onClick={() => setSelectedNodeId(node.id)}
            style={{ cursor: "pointer" }}
          >
            <circle
              cx={node.x}
              cy={node.y}
              r={18}
              fill={
                node.status === "attacked"
                  ? "#ff3b3b"
                  : node.trust < 50
                  ? "#f59e0b"
                  : "#38bdf8"
              }
              style={{
                filter: "drop-shadow(0 0 10px rgba(56,189,248,0.6))"
              }}
            />
            <text
              x={node.x}
              y={node.y + 4}
              textAnchor="middle"
              fontSize="10"
              fill="#fff"
            >
              {node.id}
            </text>
          </g>
        ))}
      </svg>

      {/* 📊 SIDE PANEL */}
      {selectedNode && (
        <div
          style={{
            position: "absolute",
            right: "40px",
            top: "120px",
            background: "#0f172a",
            padding: "20px",
            borderRadius: "10px",
            width: "220px",
            boxShadow: "0 0 20px rgba(0,0,0,0.5)"
          }}
        >
          <h3>Validator {selectedNode.id}</h3>
          <p>Trust: {selectedNode.trust.toFixed(2)}</p>
          <p>Drift: {selectedNode.drift.toFixed(2)}</p>
          <p>Status: {selectedNode.status}</p>

          <button
            onClick={() => setSelectedNodeId(null)}
            style={{ marginTop: "10px" }}
          >
            Close
          </button>
        </div>
      )}

      {/* 🧠 FOOTER */}
      <div
        style={{
          textAlign: "center",
          marginTop: "10px",
          opacity: 0.7
        }}
      >
        Adaptive trust network — stabilized topology
      </div>
    </div>
  );
}