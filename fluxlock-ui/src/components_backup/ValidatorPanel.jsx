import React from "react";

export default function ValidatorPanel({ node, onClose, onAttack }) {
  if (!node) return null;

  return (
    <div
      style={{
        width: "300px",
        background: "#0d1b2a",
        padding: "20px",
        borderRadius: "10px",
        color: "white",
      }}
    >
      <h2>Validator {node.id}</h2>

      {/* ✅ EPOCH */}
      <p>
        🔑 <strong>Epoch:</strong> {node.epoch ?? "—"}
      </p>

      {/* ✅ AGE (THIS WAS MISSING) */}
      <p>
        ⏱ <strong>Age:</strong> {node.epochAge ?? "—"}
      </p>

      <hr />

      <p>
        📊 <strong>Trust:</strong> {node.trust.toFixed(2)}
      </p>

      <p>
        🌪 <strong>Drift:</strong> {node.drift.toFixed(2)}
      </p>

      <p>
        <strong>Status:</strong> {node.status}
      </p>

      {/* 🔥 COMPROMISED FLAG */}
      {node.compromised && (
        <p style={{ color: "magenta", fontWeight: "bold" }}>
          ⚠️ COMPROMISED
        </p>
      )}

      <hr />

      {/* 🧠 DECISION */}
      <h3>🧠 Decision</h3>
      <p
        style={{
          color: node.compromised
            ? "red"
            : node.drift > 80
            ? "orange"
            : "lime",
          fontWeight: "bold",
        }}
      >
        {node.compromised
          ? "REJECTED"
          : node.drift > 80
          ? "REJECTED"
          : "ACCEPTED"}
      </p>

      <p style={{ fontSize: "0.9rem", opacity: 0.8 }}>
        {node.compromised
          ? "identity compromised (key breach detected)"
          : node.drift > 80
          ? "identity unstable (high drift)"
          : "identity valid (stable + continuous)"}
      </p>

      <hr />

      {/* ⚔️ ATTACK PANEL */}
      <h3>⚔️ Attack Panel</h3>

      <button onClick={() => onAttack("spike")}>⚡ Spike Attack</button>
      <br />
      <br />

      <button onClick={() => onAttack("critical")}>
        ☠️ Critical Breach
      </button>
      <br />
      <br />

      <button onClick={() => onAttack("network")}>
        🌐 Network Attack
      </button>

      <br />
      <br />

      <button onClick={onClose}>Close</button>
    </div>
  );
}