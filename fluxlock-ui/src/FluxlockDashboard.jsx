import React, { useEffect, useState } from "react";

const NODE_COUNT = 20;

// -----------------------------
// CREATE NODES (SPREAD START)
// -----------------------------
const createNodes = () => {
  const centerX = window.innerWidth / 2;
  const centerY = window.innerHeight / 2;

  return Array.from({ length: NODE_COUNT }, (_, i) => {
    const angle = (i / NODE_COUNT) * Math.PI * 2;
    const radius = 300 + Math.random() * 100;

    return {
      id: i,
      x: centerX + Math.cos(angle) * radius,
      y: centerY + Math.sin(angle) * radius,
      vx: 0,
      vy: 0,
      trust: 85 + Math.random() * 15,
      drift: 0,
      attacked: false,
      connections: [],
    };
  });
};

// -----------------------------
// CONNECT NODES (REAL NETWORK)
// -----------------------------
const connectNodes = (nodes) => {
  let updated = nodes.map(n => ({ ...n, connections: [] }));

  // STEP 1: LOCAL NEIGHBORS
  updated.forEach((node) => {
    const neighbors = updated
      .filter(n => n.id !== node.id)
      .map(n => ({
        id: n.id,
        dist: Math.hypot(n.x - node.x, n.y - node.y)
      }))
      .sort((a, b) => a.dist - b.dist);

    const connectionCount = 2 + Math.floor(Math.random() * 2);

    neighbors.slice(0, connectionCount).forEach(({ id }) => {
      if (!node.connections.includes(id)) {
        node.connections.push(id);
      }
    });
  });

  // STEP 2: BIDIRECTIONAL FIX
  updated.forEach((node) => {
    node.connections.forEach((cid) => {
      if (!updated[cid].connections.includes(node.id)) {
        updated[cid].connections.push(node.id);
      }
    });
  });

  // STEP 3: NO ISOLATED NODES
  updated.forEach((node) => {
    if (node.connections.length === 0) {
      const closest = updated
        .filter(n => n.id !== node.id)
        .map(n => ({
          id: n.id,
          dist: Math.hypot(n.x - node.x, n.y - node.y)
        }))
        .sort((a, b) => a.dist - b.dist)[0];

      node.connections.push(closest.id);
      updated[closest.id].connections.push(node.id);
    }
  });

  // STEP 4: LIGHT RANDOM LINKS
  updated.forEach((node) => {
    if (Math.random() < 0.2) {
      const target = Math.floor(Math.random() * updated.length);
      if (
        target !== node.id &&
        !node.connections.includes(target)
      ) {
        node.connections.push(target);
        updated[target].connections.push(node.id);
      }
    }
  });

  return updated;
};

// -----------------------------
// PHYSICS (BALANCED)
// -----------------------------
const physicsStep = (nodes) => {
  const updated = nodes.map(n => ({ ...n }));

  const centerX = window.innerWidth / 2;
  const centerY = window.innerHeight / 2;

  updated.forEach((node) => {
    let fx = 0;
    let fy = 0;

    // REPULSION
    updated.forEach((other) => {
      if (node.id === other.id) return;

      const dx = node.x - other.x;
      const dy = node.y - other.y;
      const dist = Math.sqrt(dx * dx + dy * dy) + 0.01;

      const repulsion = 20000 / (dist * dist);
      fx += (dx / dist) * repulsion;
      fy += (dy / dist) * repulsion;
    });

    // SPRINGS
    node.connections.forEach((cid) => {
      const other = updated[cid];
      if (!other) return;

      const dx = other.x - node.x;
      const dy = other.y - node.y;
      const dist = Math.sqrt(dx * dx + dy * dy) + 0.01;

      const ideal = 180; // 🔥 tighter network
      const spring = (dist - ideal) * 0.08;

      fx += (dx / dist) * spring;
      fy += (dy / dist) * spring;
    });

    // CENTERING
    fx += (centerX - node.x) * 0.01;
    fy += (centerY - node.y) * 0.01;

    node.vx = (node.vx + fx) * 0.90;
    node.vy = (node.vy + fy) * 0.90;

// soft clamp instead of full stop
if (Math.abs(node.vx) < 0.01) node.vx = 0;
if (Math.abs(node.vy) < 0.01) node.vy = 0;

    node.x += node.vx;
    node.y += node.vy;
  });

  return updated;
};

// -----------------------------
// TRUST PROPAGATION
// -----------------------------
const propagateTrust = (nodes) => {
  return nodes.map((node) => {
    let influence = 0;
    let drift = node.drift * 0.9;

    node.connections.forEach((cid) => {
      const other = nodes[cid];
      if (!other) return;

      influence += (other.trust - 50) * 0.01;

      if (other.attacked) drift += 2;
    });

    let trust = node.trust + influence - drift * 0.08;

    if (node.attacked) trust -= 4;

    trust = Math.max(0, Math.min(100, trust));

    return { ...node, trust, drift };
  });
};

// -----------------------------
// EDGE STYLE
// -----------------------------
const getEdgeStyle = (a, b) => {
  if (a.attacked || b.attacked) {
    return { color: "rgba(255,80,80,0.9)", width: 2 };
  }

  const avgTrust = (a.trust + b.trust) / 2;

  return {
    color: `rgba(80,200,255,${0.3 + avgTrust / 140})`,
    width: 1 + avgTrust / 50,
  };
};

// -----------------------------
// COMPONENT
// -----------------------------
export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState([]);
  const [selected, setSelected] = useState(null);

  useEffect(() => {
    let base = createNodes();
    base = connectNodes(base);

    base[19].attacked = true;

    setNodes(base);

    const interval = setInterval(() => {
      setNodes(prev => {
        let next = physicsStep(prev);
        next = propagateTrust(next);
        return [...next];
      });
    }, 80);

    return () => clearInterval(interval);
  }, []);

  const getNodeColor = (node) => {
    if (node.attacked) return "#ff3b3b";
    if (node.drift > 30) return "#ffcc00";
    if (node.drift > 15) return "#ffaa00";
    return "#4cc9f0";
  };

  return (
    <div style={{
      width: "100vw",
      height: "100vh",
      background: "radial-gradient(circle at center, #020617, #000)",
      overflow: "hidden",
      position: "relative"
    }}>
      <h1 style={{
        textAlign: "center",
        color: "#e2e8f0",
        fontWeight: 200,
        letterSpacing: 4,
        marginTop: 20
      }}>
        FLUXLOCK NETWORK GRAPH
      </h1>

      <svg style={{ position: "absolute", width: "100%", height: "100%" }}>
        {nodes.map(node =>
          node.connections.map(cid => {
            if (node.id > cid) return null;
            const target = nodes[cid];
            if (!target) return null;

            const style = getEdgeStyle(node, target);

            return (
              <line
                key={`${node.id}-${cid}`}
                x1={node.x}
                y1={node.y}
                x2={target.x}
                y2={target.y}
                stroke={style.color}
                strokeWidth={style.width}
              />
            );
          })
        )}
      </svg>

      {nodes.map(node => (
        <div
          key={node.id}
          onClick={() => setSelected(node)}
          style={{
            position: "absolute",
            left: node.x,
            top: node.y,
            transform: "translate(-50%, -50%)",
            width: 42,
            height: 42,
            borderRadius: "50%",
            background: getNodeColor(node),
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            color: "#fff",
            fontSize: 12,
            boxShadow:
              node.attacked
                ? "0 0 30px red"
                : "0 0 18px rgba(0,200,255,0.7)",
          }}
        >
          {node.id}
        </div>
      ))}

      {selected && (
        <div style={{
          position: "absolute",
          right: 40,
          top: 120,
          width: 260,
          padding: 20,
          border: "1px solid rgba(255,255,255,0.1)",
          borderRadius: 8,
          color: "#e2e8f0",
          background: "rgba(0,0,0,0.4)"
        }}>
          <h3>Validator {selected.id}</h3>
          <p>Trust: {selected.trust.toFixed(2)}</p>
          <p>Drift: {selected.drift.toFixed(2)}</p>
          <p>Status: {selected.attacked ? "attacked" : "normal"}</p>
          <button onClick={() => setSelected(null)}>Close</button>
        </div>
      )}

      <div style={{
        position: "absolute",
        bottom: 40,
        left: "50%",
        transform: "translateX(-50%)",
        padding: "10px 20px",
        borderRadius: 8,
        background: "rgba(0,0,0,0.5)",
        color: "#e2e8f0"
      }}>
        Adaptive trust network — stabilized topology
      </div>
    </div>
  );
}