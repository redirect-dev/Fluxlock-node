import React, { useEffect, useState } from "react";
import IdentityGraph from "./components/IdentityGraph";
import Dashboard from "./components/Dashboard";
import FluxlockVisualizer from "./components/FluxlockVisualizer";

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState([]);
  const [selectedId, setSelectedId] = useState(null);

  // 🔥 AUTH SYSTEM STATE
  const [authPulse, setAuthPulse] = useState(0);
  const [authData, setAuthData] = useState(null);
  const [authStatus, setAuthStatus] = useState("idle");

  // ================= BACKEND STATE LOOP =================
  useEffect(() => {
    const fetchState = () => {
      fetch("http://127.0.0.1:3001/state")
        .then((res) => res.json())
        .then((data) => {
          if (!data || !data.validators) return;

          setNodes(data.validators);

          if (selectedId === null && data.validators.length > 0) {
            setSelectedId(data.validators[0].id);
          }
        })
        .catch((err) => console.error("State fetch error:", err));
    };

    fetchState();
    const interval = setInterval(fetchState, 300);

    return () => clearInterval(interval);
  }, [selectedId]);

  // ================= 🔥 AUTH EVENT LISTENER =================
  useEffect(() => {
    const handler = (e) => {
      const data = e.detail || {};

      // normalize status
      let resolved = "denied";

      if (data.authenticated) {
        if (data.status === "healthy") resolved = "healthy";
        else if (data.status === "recovering") resolved = "recovering";
        else resolved = "granted";
      }

      setAuthStatus(resolved);
      setAuthData({
        ...data,
        resolvedStatus: resolved,
      });

      // 🔥 trigger pulse
      setAuthPulse(Date.now());

      // 🎯 smart node targeting
      if (resolved === "healthy" || resolved === "recovering") {
        setSelectedId(0);
      }
    };

    window.addEventListener("fluxlock-auth", handler);

    return () => window.removeEventListener("fluxlock-auth", handler);
  }, []);

  // ================= SELECT =================
  const handleSelect = (id) => {
    setSelectedId(Number(id));
  };

  // ================= ATTACKS =================
  const spikeAttack = (id) => {
    fetch("http://127.0.0.1:3001/attack/spike", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ id }),
    });
  };

  const breachAttack = (id) => {
    fetch("http://127.0.0.1:3001/attack/breach", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ id }),
    });
  };

  const networkAttack = () => {
    fetch("http://127.0.0.1:3001/attack/network", {
      method: "POST",
    });
  };

  // ================= SELECTED =================
  const selected =
    nodes.find((n) => n.id === selectedId) || nodes[0] || null;

  return (
    <div style={{ display: "flex", position: "relative" }}>
      
      {/* 🌊 FLUXLOCK FIELD */}
      <FluxlockVisualizer
        node={selected}
        nodes={nodes}
        authTrigger={authPulse}
        authData={authData}
        authStatus={authStatus}
      />

      {/* 🌐 NETWORK GRAPH */}
      <IdentityGraph
        validators={nodes}
        onSelectNode={handleSelect}
      />

      {/* 📊 SIDE PANEL */}
      <Dashboard
        node={selected}
        onSpike={spikeAttack}
        onBreach={breachAttack}
        onNetwork={networkAttack}
      />

      {/* 🧪 DEBUG BUTTON */}
      <button
        onClick={() => setAuthPulse(Date.now())}
        style={{
          position: "fixed",
          bottom: 20,
          left: 20,
          padding: "10px 16px",
          background: "#00ffcc",
          color: "#000",
          border: "none",
          borderRadius: "8px",
          cursor: "pointer",
          zIndex: 10,
        }}
      >
        Trigger Pulse
      </button>
    </div>
  );
}