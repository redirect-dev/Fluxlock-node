import React from "react";

export default function Dashboard({
  node,
  onSpike,
  onBreach,
  onNetwork,
}) {
  if (!node) {
    return (
      <div
        style={{
          width: 320,
          minHeight: "100vh",
          background: "#0b1a2a",
          color: "white",
          padding: 20,
        }}
      >
        <h2>No node selected</h2>
      </div>
    );
  }

  const {
    id,
    trust,
    drift,
    status,
    epochAge,
    epoch,
    stabilityCounter,
    peerVotes,
    localValid,          // ✅ FIXED (was missing)
    networkAccepted,
    globalValid,
    chainReason,
    identityChain,
  } = node;

  return (
    <div
      style={{
        padding: 20,
        color: "white",
        width: 320,
        minHeight: "100vh",
        background: "#0b1a2a",
        borderLeft: "1px solid #1e2a3a",
        fontFamily: "Arial",
      }}
    >
      <h2 style={{ marginBottom: 10 }}>Validator {id}</h2>

      <p>📊 Trust: {trust?.toFixed(2)}</p>
      <p>🌪 Drift: {drift?.toFixed(2)}</p>
      <p>Status: {status}</p>

      <p style={{ color: "#00ffcc", marginTop: 10 }}>
        Epoch Age: {epochAge ?? 0}
      </p>

      <p style={{ opacity: 0.6 }}>
        Stability: {stabilityCounter ?? 0}
      </p>

      <hr style={{ margin: "15px 0", opacity: 0.2 }} />

      {/* ================= CONSENSUS ================= */}
      <h3>🧠 Consensus</h3>

      {/* ✅ FIXED: now uses real consensus output */}
      <p>Local: {localValid ? "✅" : "❌"}</p>
      <p>Network: {networkAccepted ? "✅" : "❌"}</p>
      <p>Global: {globalValid ? "✅" : "❌"}</p>

      <p>
        Votes → ✅ {peerVotes?.valid ?? 0} / ❌ {peerVotes?.invalid ?? 0}
      </p>

      <p style={{ color: "orange", fontSize: 12 }}>
        Reason: {chainReason}
      </p>

      <hr style={{ margin: "15px 0", opacity: 0.2 }} />

      {/* ================= IDENTITY ================= */}
      <h3>🔑 Identity Chain</h3>

      {identityChain?.map((entry, i) => (
        <div key={i} style={{ marginBottom: 10 }}>
          <div>
            🔑 {entry.fingerprint || "unknown"} → {entry.trust}
          </div>

          <div style={{ fontSize: 11, opacity: 0.6 }}>
            sig: {entry.sig || "none"}
          </div>

          <div style={{ fontSize: 10, opacity: 0.4 }}>
            {(entry.publicKey || "").slice(0, 32)}...
          </div>
        </div>
      ))}

      <hr style={{ margin: "15px 0", opacity: 0.2 }} />

      {/* ================= ATTACK PANEL ================= */}
      <h3>⚔️ Attack Panel</h3>

      <button
        style={{ width: "100%", marginBottom: 8 }}
        onClick={() => onSpike?.(id)}
      >
        ⚡ Spike Attack
      </button>

      <button
        style={{ width: "100%", marginBottom: 8 }}
        onClick={() => onBreach?.(id)}
      >
        ☠️ Critical Breach
      </button>

      <button
        style={{ width: "100%", marginBottom: 8 }}
        onClick={() => onNetwork?.()}
      >
        🌊 Network Attack
      </button>

      <button
        style={{ width: "100%" }}
        onClick={() => console.log("Close clicked")}
      >
        Close
      </button>
    </div>
  );
}