import React, { useEffect, useState } from "react";

const GRID_SIZE = 5;
const NODE_COUNT = 20;

function getNeighbors(id) {
  const row = Math.floor(id / GRID_SIZE);
  const col = id % GRID_SIZE;

  const neighbors = [];

  for (let r = row - 1; r <= row + 1; r++) {
    for (let c = col - 1; c <= col + 1; c++) {
      if (
        r >= 0 &&
        r < GRID_SIZE &&
        c >= 0 &&
        c < GRID_SIZE &&
        !(r === row && c === col)
      ) {
        neighbors.push(r * GRID_SIZE + c);
      }
    }
  }

  return neighbors;
}

function createNode(id) {
  return {
    id,
    trust: 100,
    drift: 0,
    behavior: 100,
    influence: 100,
    status: "normal",
  };
}

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState(
    Array.from({ length: NODE_COUNT }, (_, i) => createNode(i))
  );

  const [selectedNode, setSelectedNode] = useState(null);

  useEffect(() => {
    const interval = setInterval(() => {
      setNodes((prev) => {
        if (!prev || prev.length !== NODE_COUNT) return prev;

        const updated = prev.map((n) => ({ ...n }));

        const attacker = updated[19];
        if (!attacker) return prev;

        // =========================
        // ⚠️ ATTACK PROGRESSION
        // =========================
        attacker.trust = Math.max(attacker.trust - 2, 0);
        attacker.drift += 5;
        attacker.behavior = Math.max(attacker.behavior - 2, 0);
        attacker.influence = Math.max(attacker.influence - 3, 0);

        if (attacker.trust < 40) attacker.status = "drifting";
        if (attacker.trust < 20) attacker.status = "attacked";

        // =========================
        // 🌊 PROPAGATION (TUNED)
        // =========================
        const neighbors = getNeighbors(19);

        neighbors.forEach((id) => {
          const n = updated[id];
          if (!n) return;

          const resistance =
            attacker.status === "attacked"
              ? 0.3
              : attacker.status === "drifting"
              ? 0.6
              : 1;

          // 🔧 Slightly stronger propagation
          n.trust = Math.max(n.trust - 1 * resistance, 0);
          n.drift += 1 * resistance;
          n.behavior = Math.max(n.behavior - 0.4 * resistance, 0);

          // 🔧 Early awareness trigger (NEW)
          if ((n.trust < 75 || n.drift > 5) && n.status === "normal") {
            n.status = "suspicious";
          }

          // escalate if worsening
          if (n.trust < 50 && n.status === "suspicious") {
            n.status = "drifting";
          }
        });

        // =========================
        // 🔒 CONTAINMENT
        // =========================
        if (attacker.status === "attacked") {
          attacker.influence = Math.max(attacker.influence - 10, 0);
        }

        // =========================
        // 🌱 PASSIVE RECOVERY
        // =========================
        updated.forEach((n) => {
          if (n.status === "normal") {
            n.trust = Math.min(n.trust + 0.2, 100);
            n.behavior = Math.min(n.behavior + 0.1, 100);
          }
        });

        return updated;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  function getNodeColor(node) {
    if (node.status === "attacked") return "#ff3b3b";
    if (node.status === "drifting") return "#f59e0b";
    if (node.status === "suspicious") return "#facc15";
    return "#4cc9f0";
  }

  function getGlow(node) {
    if (node.status === "attacked")
      return "0 0 35px rgba(255, 0, 0, 0.9)";
    if (node.status === "drifting")
      return "0 0 25px rgba(245, 158, 11, 0.8)";
    if (node.status === "suspicious")
      return "0 0 18px rgba(250, 204, 21, 0.7)";
    return "0 0 18px rgba(76, 201, 240, 0.7)";
  }

  const attackedNode = nodes.find((n) => n.status === "attacked");

  // 🔧 UPDATED COUNT (accurate adaptation)
  const adaptingCount = nodes.filter(
    (n) => n.status === "suspicious" || n.status === "drifting"
  ).length;

  return (
    <div
      style={{
        background: "radial-gradient(circle at center, #020617, #01030a)",
        minHeight: "100vh",
        color: "white",
        padding: "20px",
        fontFamily: "sans-serif",
      }}
    >
      <h1
        style={{
          textAlign: "center",
          fontSize: "48px",
          letterSpacing: "4px",
          marginBottom: "20px",
          opacity: 0.9,
        }}
      >
        FLUXLOCK NETWORK GRAPH
      </h1>

      <div style={{ display: "flex", justifyContent: "center" }}>
        {/* GRID */}
        <div
          style={{
            display: "grid",
            gridTemplateColumns: `repeat(${GRID_SIZE}, 110px)`,
            gap: "30px",
          }}
        >
          {nodes.map((node) => (
            <div
              key={node.id}
              onClick={() => setSelectedNode(node)}
              style={{
                width: 90,
                height: 90,
                borderRadius: "50%",
                background: getNodeColor(node),
                boxShadow: getGlow(node),
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                cursor: "pointer",
                fontWeight: "bold",
                transition: "all 0.3s ease",
              }}
            >
              {node.id}
            </div>
          ))}
        </div>

        {/* SIDE PANEL */}
        {selectedNode && (
          <div
            style={{
              marginLeft: "50px",
              minWidth: "260px",
              padding: "25px",
              border: "1px solid rgba(255,255,255,0.08)",
              background: "rgba(255,255,255,0.03)",
              backdropFilter: "blur(6px)",
            }}
          >
            <h2>Validator {selectedNode.id}</h2>
            <p>Trust: {selectedNode.trust.toFixed(2)}</p>
            <p>Drift: {selectedNode.drift.toFixed(2)}</p>
            <p>Behavior: {selectedNode.behavior.toFixed(2)}</p>
            <p>Influence: {selectedNode.influence.toFixed(2)}</p>
            <p>Status: {selectedNode.status}</p>

            <button
              onClick={() => setSelectedNode(null)}
              style={{
                marginTop: "20px",
                padding: "10px",
                width: "100%",
                background: "transparent",
                border: "1px solid rgba(255,255,255,0.2)",
                color: "white",
                cursor: "pointer",
              }}
            >
              Close
            </button>
          </div>
        )}
      </div>

      {/* STATUS BAR */}
      <div
        style={{
          position: "fixed",
          bottom: "20px",
          left: "50%",
          transform: "translateX(-50%)",
          padding: "14px 35px",
          borderRadius: "8px",
          border: "1px solid rgba(255,255,255,0.1)",
          background: "rgba(0,0,0,0.65)",
          backdropFilter: "blur(10px)",
          fontSize: "16px",
          letterSpacing: "1px",
        }}
      >
        {attackedNode
          ? `🚨 Instability detected — ${adaptingCount} nodes adapting`
          : "System stable — trust network intact"}
      </div>
    </div>
  );
}