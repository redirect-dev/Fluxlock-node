import React, { useEffect, useState } from "react";
import { motion } from "framer-motion";

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState([]);
  const [log, setLog] = useState("Connecting to Fluxlock engine...");

  const fetchData = async () => {
    try {
      const res = await fetch("http://localhost:8080/simulation");
      const data = await res.json();

      setNodes(data);

      const attacker = data.find((n) => n.id === 19);

      if (attacker?.status === "attacked") {
        setLog("🚨 Attacker neutralized");
      } else if (attacker?.drift_score > 40) {
        setLog("⚠️ Suspicious validator behavior rising");
      } else {
        setLog("System stable — monitoring behavior");
      }
    } catch (err) {
      console.error(err);
      setLog("⚠️ Engine connection failed");
    }
  };

  useEffect(() => {
    fetchData();
    const interval = setInterval(fetchData, 1000);
    return () => clearInterval(interval);
  }, []);

  const getState = (node) => {
    if (node.status === "attacked") return "attacked";
    if (node.id === 19 && node.drift_score > 70) return "critical";
    if (node.drift_score > 30) return "drifting";
    return "normal";
  };

  const getColor = (node) => {
    const s = getState(node);
    if (s === "attacked") return "#ff3b3b";
    if (s === "critical") return "#ff6b00";
    if (s === "drifting") return "#f9c74f";
    return "#4cc9f0";
  };

  const getGlow = (node) => {
    const s = getState(node);
    if (s === "attacked") return "0 0 40px #ff3b3b";
    if (s === "critical") return "0 0 35px #ff6b00";
    if (s === "drifting") return "0 0 20px #f9c74f";
    return "0 0 20px #4cc9f0";
  };

  const getSize = (node) => 60 + node.influence * 0.3;

  return (
    <div
      style={{
        background: "#05070d",
        height: "100vh",
        color: "white",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <h1 style={{ marginBottom: "20px", letterSpacing: "2px" }}>
        FLUXLOCK LIVE NETWORK
      </h1>

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(5, 120px)",
          gap: "30px",
        }}
      >
        {nodes.map((node) => {
          const state = getState(node);

          return (
            <motion.div
              key={node.id}
              animate={{
                width: getSize(node),
                height: getSize(node),
                opacity: state === "attacked" ? 0.4 : 1,
                scale:
                  state === "attacked"
                    ? 0.5
                    : state === "critical"
                    ? [1, 1.2, 1]
                    : [1, 1.05, 1],
              }}
              transition={{
                duration: 1,
                repeat: state === "attacked" ? 0 : Infinity,
              }}
              style={{
                borderRadius: "50%",
                background: getColor(node),
                boxShadow: getGlow(node),
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                fontSize: "12px",
              }}
            >
              {node.id}
            </motion.div>
          );
        })}
      </div>

      <div
        style={{
          marginTop: "30px",
          padding: "10px 20px",
          background: "#0d1117",
          border: "1px solid #222",
          fontSize: "14px",
          minWidth: "320px",
          textAlign: "center",
        }}
      >
        {log}
      </div>
    </div>
  );
}