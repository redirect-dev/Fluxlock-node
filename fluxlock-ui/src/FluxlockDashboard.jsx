import React, { useEffect, useState } from "react";
import { motion } from "framer-motion";

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState([]);
  const [log, setLog] = useState("");

  // 🔄 Fetch simulation data
  const fetchData = async () => {
    try {
      const res = await fetch("http://localhost:8080/simulation");
      const data = await res.json();

      setNodes(data);

      const attacker = data.find((n) => n.id === 19);

      if (attacker?.status === "attacked") {
        setLog("🚨 Attacker isolated — network rejecting node");
      } else if (attacker?.drift_score > 40) {
        setLog("⚠️ Suspicious validator behavior rising");
      } else {
        setLog("System stable — trust network intact");
      }
    } catch {
      setLog("⚠️ Unable to connect to engine");
    }
  };

  useEffect(() => {
    fetchData();
    const interval = setInterval(fetchData, 1000);
    return () => clearInterval(interval);
  }, []);

  // 📍 Grid positioning
  const getPosition = (id) => {
    const col = id % 5;
    const row = Math.floor(id / 5);
    return {
      x: col * 120,
      y: row * 120,
    };
  };

  // 🎨 Color logic
  const getColor = (n) => {
    if (n.status === "attacked") return "#ff3b3b";
    if (n.drift_score > 70) return "#ff6b00";
    if (n.drift_score > 30) return "#f9c74f";
    return "#4cc9f0";
  };

  // ✨ Glow logic
  const getGlow = (n) => {
    if (n.status === "attacked") return "0 0 25px rgba(255,0,0,0.9)";
    if (n.drift_score > 30) return "0 0 18px rgba(249,199,79,0.7)";
    return "0 0 15px rgba(76,201,240,0.6)";
  };

  // 🔥 CRITICAL FIX — CONNECTION FILTER
  const shouldConnect = (a, b) => {
    // No self / duplicate
    if (a.id === b.id) return false;

    // NEVER connect attacked nodes
    if (a.status === "attacked" || b.status === "attacked") return false;

    // GRID LOCALITY (prevents spiderweb)
    const dx = Math.abs((a.id % 5) - (b.id % 5));
    const dy = Math.abs(Math.floor(a.id / 5) - Math.floor(b.id / 5));

    if (dx > 1 || dy > 1) return false;

    // TRUST SIMILARITY
    if (Math.abs(a.trust - b.trust) > 15) return false;

    return true;
  };

  return (
    <div
      style={{
        background: "#05070d",
        height: "100vh",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        color: "white",
      }}
    >
      <h1 style={{ marginBottom: "20px", fontWeight: "300" }}>
        FLUXLOCK NETWORK GRAPH
      </h1>

      <div style={{ position: "relative", width: 600, height: 600 }}>
        {/* 🔗 CONNECTION LINES */}
        <svg
          width="600"
          height="600"
          style={{ position: "absolute", top: 0, left: 0 }}
        >
          {nodes.map((a) =>
            nodes.map((b) => {
              if (a.id >= b.id) return null;
              if (!shouldConnect(a, b)) return null;

              const posA = getPosition(a.id);
              const posB = getPosition(b.id);

              return (
                <line
                  key={`${a.id}-${b.id}`}
                  x1={posA.x + 35}
                  y1={posA.y + 35}
                  x2={posB.x + 35}
                  y2={posB.y + 35}
                  stroke="rgba(100,200,255,0.6)"
                  strokeWidth="1.5"
                />
              );
            })
          )}
        </svg>

        {/* 🔵 NODES */}
        {nodes.map((n) => {
          const pos = getPosition(n.id);

          return (
            <motion.div
              key={n.id}
              animate={{
                scale:
                  n.status === "attacked"
                    ? 0.6
                    : n.drift_score > 30
                    ? [1, 1.15, 1]
                    : [1, 1.05, 1],
                opacity: n.status === "attacked" ? 0.5 : 1,
              }}
              transition={{
                duration: 0.8,
                repeat: n.status === "attacked" ? 0 : Infinity,
              }}
              style={{
                position: "absolute",
                left: pos.x,
                top: pos.y,
                width: 70,
                height: 70,
                borderRadius: "50%",
                background: getColor(n),
                boxShadow: getGlow(n),
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                fontSize: "12px",
                fontWeight: "bold",
              }}
            >
              {n.id}
            </motion.div>
          );
        })}
      </div>

      {/* 📊 STATUS PANEL */}
      <div
        style={{
          marginTop: "20px",
          padding: "12px 24px",
          border: "1px solid #222",
          background: "#0d1117",
          minWidth: "360px",
          textAlign: "center",
          letterSpacing: "1px",
        }}
      >
        {log}
      </div>
    </div>
  );
}