import React, { useEffect, useState } from "react";
import IdentityGraph from "./components_backup/IdentityGraph";
import Dashboard from "./Dashboard";

import { createNetwork } from "./trustEngine";
import { evaluateNetwork } from "./evaluateNetwork";

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState([]);
  const [selectedId, setSelectedId] = useState(null);

  // ---------------- INIT ----------------
  useEffect(() => {
    const net = createNetwork(20);
    setNodes(net);
    setSelectedId(0);
  }, []);

  // ---------------- SIM LOOP ----------------
  useEffect(() => {
    const interval = setInterval(() => {
      setNodes(prev => evaluateNetwork(prev));
    }, 300);

    return () => clearInterval(interval);
  }, []);

  // ---------------- SAFE SELECT (FIX) ----------------
  const handleSelect = (id) => {
    setSelectedId(Number(id)); // ✅ FORCE NUMBER
  };

  // ---------------- ATTACKS ----------------
  const spikeAttack = (id) => {
    setNodes(prev =>
      prev.map(n =>
        n.id === id
          ? { ...n, drift: n.drift + 80, trust: n.trust - 40 }
          : n
      )
    );
  };

  const criticalBreach = (id) => {
    setNodes(prev =>
      prev.map(n =>
        n.id === id
          ? {
              ...n,
              drift: 150,
              trust: n.trust * 0.3,
              compromised: true,
            }
          : n
      )
    );
  };

  const networkAttack = () => {
    setNodes(prev =>
      prev.map(n => ({
        ...n,
        drift: n.drift + 20,
        trust: n.trust - 10,
      }))
    );
  };

  // ---------------- SAFE FIND (FIX) ----------------
  const selected =
    nodes.find(n => n.id === selectedId) || nodes[0] || null;

  return (
    <div style={{ display: "flex" }}>
      <IdentityGraph
        validators={nodes}
        onSelectNode={handleSelect} // ✅ FIXED
      />

      <Dashboard
        node={selected}
        onSpike={spikeAttack}
        onBreach={criticalBreach}
        onNetwork={networkAttack}
      />
    </div>
  );
}