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
        setLog("🚨 Attacker neutralized — localized containment complete");
      } else if (attacker?.drift_score > 40) {
        setLog("⚠️ Suspicious validator behavior rising");
      } else {
        setLog("System stable — monitoring behavior");
      }
    } catch (err) {
      console.error(err);
      setLog("⚠️ Unable to connect to engine");
    }
  };

  useEffect(() => {
    fetchData();
    const interval = setInterval(fetchData, 1000);
    return () => clearInterval(interval);
  }, []);

  // 🧠 COLOR SYSTEM (tight + readable)
  const getColor = (n) => {
    if (n.status === "attacked") return "#ff3b3b";
    if (n.drift_score > 70) return "#ff6b00";
    if (n.drift_score > 30) return "#f9c74f";
    return "#4cc9f0";
  };

  // ✨ GLOW SYSTEM (less fog, more signal)
  const getGlow = (n) => {
    if (n.status === "attacked") return "0 0 30px rgba(255, 59, 59, 0.8)";
    if (n.shock > 1)
      return `0 0 ${12 + n.shock}px rgba(255,255,255,0.6)`;
    if (n.drift_score > 70)
      return "0 0 25px rgba(255,107,0,0.7)";
    if (n.drift_score > 30)
      return "0 0 18px rgba(249,199,79,0.7)";
    return "0 0 18px rgba(76,201,240,0.6)";
  };

  // 📏 FIXED SIZE (NO JITTER)
  const BASE_SIZE = 70;

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
      <h1
        style={{
          marginBottom: "30px",
          letterSpacing: "3px",
          fontWeight: "300",
        }}
      >
        FLUXLOCK LIVE NETWORK
      </h1>

      {/* GRID */}
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(5, 110px)",
          gap: "28px",
        }}
      >
        {nodes.map((n) => (
          <motion.div
            key={n.id}
            animate={{
              scale:
                n.status === "attacked"
                  ? 0.5
                  : n.shock > 1
                  ? [1, 1.25, 1] // localized pulse
                  : [1, 1.05, 1],

              opacity: n.status === "attacked" ? 0.35 : 1,
            }}
            transition={{
              duration: 0.8,
              repeat: n.status === "attacked" ? 0 : Infinity,
            }}
            style={{
              width: BASE_SIZE,
              height: BASE_SIZE,
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
        ))}
      </div>

      {/* STATUS PANEL */}
      <div
        style={{
          marginTop: "35px",
          padding: "12px 24px",
          border: "1px solid #222",
          background: "#0d1117",
          letterSpacing: "1px",
          fontSize: "14px",
          minWidth: "360px",
          textAlign: "center",
        }}
      >
        {log}
      </div>
    </div>
  );
}