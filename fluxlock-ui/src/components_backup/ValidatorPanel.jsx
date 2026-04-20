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

      <p>🔑 <strong>Epoch:</strong> {node.epoch ?? "—"}</p>
      <p>⏱ <strong>Age:</strong> {node.epochAge ?? "—"}</p>

      <hr />

      <p>📊 <strong>Trust:</strong> {node.trust.toFixed(2)}</p>
      <p>🌪 <strong>Drift:</strong> {node.drift.toFixed(2)}</p>
      <p><strong>Status:</strong> {node.status}</p>

      {node.compromised && (
        <p style={{ color: "magenta", fontWeight: "bold" }}>
          ⚠️ COMPROMISED
        </p>
      )}

      <hr />

      {/* ✅ REAL CONSENSUS */}
      <h3>🧠 Consensus</h3>

      <p>Local: {node.localValid ? "✅" : "❌"}</p>
      <p>Network: {node.networkAccepted ? "✅" : "❌"}</p>
      <p>Global: {node.globalValid ? "✅" : "❌"}</p>

      <p>
        Votes → ✅ {node.peerVotes?.valid ?? 0} / ❌ {node.peerVotes?.invalid ?? 0}
      </p>

      <hr />

      {/* ✅ FINAL DECISION BASED ON CONSENSUS */}
      <h3>🧠 Decision</h3>

      <p
        style={{
          color: node.globalValid
            ? "lime"
            : node.networkAccepted
            ? "orange"
            : "red",
          fontWeight: "bold",
        }}
      >
        {node.globalValid
          ? "ACCEPTED"
          : node.networkAccepted
          ? "PARTIAL (NETWORK ONLY)"
          : "REJECTED"}
      </p>

      <p style={{ fontSize: "0.9rem", opacity: 0.8 }}>
        {!node.localValid
          ? "local identity invalid"
          : !node.networkAccepted
          ? "network rejected"
          : !node.globalValid
          ? "not globally finalized"
          : "fully validated"}
      </p>

      <hr />

      {/* ⚔️ ATTACK PANEL */}
      <h3>⚔️ Attack Panel</h3>

      <button onClick={() => onAttack("spike")}>⚡ Spike Attack</button>
      <br /><br />

      <button onClick={() => onAttack("critical")}>
        ☠️ Critical Breach
      </button>
      <br /><br />

      <button onClick={() => onAttack("network")}>
        🌐 Network Attack
      </button>

      <br /><br />

      <button onClick={onClose}>Close</button>
    </div>
  );
}