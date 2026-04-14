import React, { useEffect, useState } from "react";
import { runEpoch, rebalanceTrust } from "./epochs";

const NODE_COUNT = 20;
const RADIUS = 260;
const CENTER = 350;
const MAX_EDGES_PER_NODE = 4;

// 🔗 Smart edge builder (CONTROLLED)
const buildEdges = (nodes) => {
  const edges = [];
  const connectionCount = {};

  nodes.forEach((n) => (connectionCount[n.id] = 0));

  const tryAddEdge = (a, b) => {
    if (a === b) return;

    if (
      connectionCount[a] >= MAX_EDGES_PER_NODE ||
      connectionCount[b] >= MAX_EDGES_PER_NODE
    )
      return;

    if (nodes[a].status === "attacked" || nodes[b].status === "attacked")
      return;

    const exists = edges.some(
      ([x, y]) => (x === a && y === b) || (x === b && y === a)
    );

    if (!exists) {
      edges.push([a, b]);
      connectionCount[a]++;
      connectionCount[b]++;
    }
  };

  nodes.forEach((node) => {
    const next = (node.id + 1) % nodes.length;
    const prev = (node.id - 1 + nodes.length) % nodes.length;

    // 🔵 Ring stability (baseline)
    tryAddEdge(node.id, next);
    tryAddEdge(node.id, prev);

    // 🧠 Smart skip connection (like real networks)
    const skip = (node.id + 3) % nodes.length;
    tryAddEdge(node.id, skip);

    // 🟢 Immune nodes get ONE extra strategic link
    if (node.immune) {
      const target = (node.id + 10) % nodes.length;
      tryAddEdge(node.id, target);
    }

    // 🟠 Medium trust nodes get occasional link
    if (node.trust > 60 && Math.random() < 0.3) {
      const random =
        nodes[Math.floor(Math.random() * nodes.length)].id;
      tryAddEdge(node.id, random);
    }
  });

  return edges;
};

const createInitialNodes = () =>
  Array.from({ length: NODE_COUNT }, (_, i) => ({
    id: i,
    angle: (i / NODE_COUNT) * 2 * Math.PI,
    trust: 100,
    drift: 0,
    status: "normal",
    immune: false,
  }));

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState(createInitialNodes());
  const [edges, setEdges] = useState([]);
  const [selected, setSelected] = useState(null);

  // 🔴 Initial attack
  useEffect(() => {
    setNodes((prev) =>
      prev.map((n) =>
        n.id === 19 ? { ...n, status: "attacked", trust: 0 } : n
      )
    );
  }, []);

  // 🔴 Attack spread
  useEffect(() => {
    const interval = setInterval(() => {
      setNodes((prev) =>
        prev.map((node) => {
          if (node.status === "attacked") {
            return {
              ...node,
              drift: node.drift + 5,
              trust: Math.max(0, node.trust - 5),
            };
          }

          if (!node.immune && node.trust < 30 && Math.random() < 0.15) {
            return { ...node, status: "attacked" };
          }

          return node;
        })
      );
    }, 1500);

    return () => clearInterval(interval);
  }, []);

  // 🟢 Healing
  useEffect(() => {
    const interval = setInterval(() => {
      setNodes((prev) =>
        prev.map((node) => {
          if (node.status === "attacked") return node;

          const trust = Math.min(100, node.trust + 2);
          const drift = Math.max(0, node.drift - 2);
          const immune = trust > 90 && drift < 5;

          return { ...node, trust, drift, immune };
        })
      );
    }, 1200);

    return () => clearInterval(interval);
  }, []);

  // 🔁 Epoch engine
  useEffect(() => {
    const interval = setInterval(() => {
      setNodes((prev) => {
        let updated = runEpoch(prev);
        updated = rebalanceTrust(updated);
        return updated;
      });
    }, 10000);

    return () => clearInterval(interval);
  }, []);

  // 🔗 Controlled edge rebuild
  useEffect(() => {
    const interval = setInterval(() => {
      setEdges(buildEdges(nodes));
    }, 800);

    return () => clearInterval(interval);
  }, [nodes]);

  const getColor = (node) => {
    if (node.immune) return "#22c55e";
    if (node.status === "attacked") return "#ff3b3b";
    if (node.trust < 40) return "#f59e0b";
    return "#4cc9f0";
  };

  const getPosition = (node) => ({
    x: CENTER + RADIUS * Math.cos(node.angle),
    y: CENTER + RADIUS * Math.sin(node.angle),
  });

  return (
    <div style={{ background: "#020617", height: "100vh", color: "white" }}>
      <h1 style={{ textAlign: "center", letterSpacing: "6px" }}>
        FLUXLOCK NETWORK GRAPH
      </h1>

      <svg width="100%" height="100%">
        {/* 🔗 EDGES */}
        {edges.map(([a, b], i) => {
          const p1 = getPosition(nodes[a]);
          const p2 = getPosition(nodes[b]);

          return (
            <line
              key={i}
              x1={p1.x}
              y1={p1.y}
              x2={p2.x}
              y2={p2.y}
              stroke="#4cc9f0"
              strokeOpacity="0.35"
            />
          );
        })}

        {/* 🔵 NODES */}
        {nodes.map((node) => {
          const { x, y } = getPosition(node);

          return (
            <g key={node.id}>
              <circle
                cx={x}
                cy={y}
                r={18}
                fill={getColor(node)}
                onClick={() => setSelected(node)}
                style={{ cursor: "pointer" }}
              />
              <text x={x} y={y + 4} textAnchor="middle" fontSize="10">
                {node.id}
              </text>
            </g>
          );
        })}
      </svg>

      {/* 📊 PANEL */}
      {selected && (
        <div style={{ position: "absolute", right: 40, top: 120 }}>
          <h3>Validator {selected.id}</h3>
          <p>Trust: {selected.trust.toFixed(2)}</p>
          <p>Drift: {selected.drift.toFixed(2)}</p>
          <p>Status: {selected.status}</p>
          <p>Immune: {selected.immune ? "Yes" : "No"}</p>
          <button onClick={() => setSelected(null)}>Close</button>
        </div>
      )}
    </div>
  );
}