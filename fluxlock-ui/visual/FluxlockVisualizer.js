import React, { useEffect, useState } from "react";
import IdentityGraph from "./components_backup/IdentityGraph";
import Dashboard from "./Dashboard";

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState([]);
  const [selectedId, setSelectedId] = useState(null);

  // ================= BACKEND STATE LOOP =================
  useEffect(() => {
    const fetchState = () => {
      fetch("http://127.0.0.1:3001/state")
        .then(res => res.json())
        .then(data => {
          setNodes(data.validators);

          if (selectedId === null && data.validators.length > 0) {
            setSelectedId(data.validators[0].id);
          }
        })
        .catch(err => console.error("State fetch error:", err));
    };

    fetchState();

    const interval = setInterval(fetchState, 300);

    return () => clearInterval(interval);
  }, [selectedId]);

  // ================= SELECT =================
  const handleSelect = (id) => {
    setSelectedId(Number(id));
  };

  // ================= ATTACKS (REAL API) =================
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
    nodes.find(n => n.id === selectedId) || nodes[0] || null;

  return (
    <div style={{ display: "flex" }}>
      <IdentityGraph
        validators={nodes}
        onSelectNode={handleSelect}
      />

      <Dashboard
        node={selected}
        onSpike={spikeAttack}
        onBreach={breachAttack}
        onNetwork={networkAttack}
      />
    </div>
  );
}