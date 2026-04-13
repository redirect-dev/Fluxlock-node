import React, { useState } from "react";
import { motion } from "framer-motion";

const initialValidators = [
  { name: "A", stake: 1000, rep: 130, influence: 169, x: 250, y: 200 },
  { name: "C1", stake: 800, rep: 110, influence: 120, x: 600, y: 300 },
  { name: "C2", stake: 800, rep: 110, influence: 120, x: 700, y: 350 },
  { name: "C3", stake: 800, rep: 110, influence: 120, x: 650, y: 250 },
];

export default function FluxlockDashboard() {
  const [validators, setValidators] = useState(initialValidators);
  const [logs, setLogs] = useState([]);

  const runSybilAttack = () => {
    const updated = validators.map((v) => {
      if (v.name.startsWith("C")) {
        return {
          ...v,
          rep: v.rep - 20,
          influence: Math.max(v.influence - 50, 50),
          x: 650, // cluster together
          y: 300,
        };
      }

      return {
        ...v,
        rep: v.rep + 5,
        influence: v.influence + 15,
      };
    });

    setValidators(updated);

    setLogs([
      "🚨 Sybil cluster detected",
      "🧠 Coordinated behavior identified",
      "⚖️ Influence dampening applied",
      "✔ Network stabilized",
    ]);
  };

  return (
    <div style={{ minHeight: "100vh", background: "black", color: "white", padding: "20px" }}>
      
      <h1 style={{ textAlign: "center", fontSize: "42px", color: "#00D4FF" }}>
        Fluxlock Sybil Defense Visualization
      </h1>

      <div style={{ textAlign: "center", margin: "20px" }}>
        <button
          onClick={runSybilAttack}
          style={{
            padding: "12px 24px",
            background: "#dc2626",
            borderRadius: "8px",
            color: "white",
            fontSize: "16px",
            cursor: "pointer",
          }}
        >
          Run Sybil Attack
        </button>
      </div>

      <div
        style={{
          position: "relative",
          width: "100%",
          height: "500px",
          border: "1px solid #00D4FF",
          borderRadius: "10px",
        }}
      >
        {/* CONNECTION LINES */}
        <svg style={{ position: "absolute", width: "100%", height: "100%" }}>
          {validators.slice(1).map((v, i) => (
            <line
              key={i}
              x1={validators[0].x}
              y1={validators[0].y}
              x2={v.x}
              y2={v.y}
              stroke="#00D4FF"
              strokeWidth="1"
              opacity="0.3"
            />
          ))}
        </svg>

        {/* NODES */}
        {validators.map((v, i) => {
          const size = Math.min(70 + v.influence / 4, 120);

          const color =
            v.rep > 110
              ? "#00FF9C"
              : v.rep > 80
              ? "#FFD700"
              : "#FF4D4D";

          const glow = `${color}88`;

          return (
            <motion.div
              key={i}
              animate={{ x: v.x, y: v.y, scale: 1 + v.influence / 400 }}
              transition={{ duration: 0.8 }}
              style={{
                position: "absolute",
                width: size,
                height: size,
                backgroundColor: color,
                borderRadius: "50%",
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                justifyContent: "center",
                fontWeight: "bold",
                color: "black",
                boxShadow: `0 0 20px ${glow}, 0 0 40px ${glow}`,
              }}
            >
              <div>{v.name}</div>
              <div style={{ fontSize: "12px" }}>{v.influence}</div>
            </motion.div>
          );
        })}
      </div>

      {/* LOG PANEL */}
      <div
        style={{
          marginTop: "20px",
          background: "#18181b",
          padding: "15px",
          borderRadius: "10px",
          border: "1px solid #9333ea",
          maxWidth: "500px",
          marginInline: "auto",
        }}
      >
        {logs.map((log, i) => (
          <p key={i}>{log}</p>
        ))}
      </div>
    </div>
  );
}