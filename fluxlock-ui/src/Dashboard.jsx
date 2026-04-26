import React, { useEffect, useState } from "react";

export default function Dashboard({
  node,
  onSpike,
  onBreach,
  onNetwork,
}) {
  const [decision, setDecision] = useState(null);

  // ================= DECISION API =================
  useEffect(() => {
    if (!node) return;

    fetch("http://127.0.0.1:3001/evaluate", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ id: node.id }),
    })
      .then((res) => res.json())
      .then(setDecision)
      .catch(() => setDecision(null));
  }, [node]);

  if (!node) {
    return (
      <div style={styles.panel}>
        <h2>No node selected</h2>
      </div>
    );
  }

  const {
    id,
    trust,
    drift,
    status,
    epoch_age,
    recovery_timer,
    peer_votes_valid,
    peer_votes_invalid,
    local_valid,
    network_accepted,
    global_valid,
    identity_chain,
    chain_valid,
  } = node;

  return (
    <div style={styles.panel}>
      <h2>Validator {id}</h2>

      {/* ================= DECISION ================= */}
      <div style={styles.section}>
        <h3>🧠 Decision</h3>

        {decision && (
          <>
            <p style={decisionStyle(decision.decision)}>
              {decision.decision}
            </p>
            <p>Weight: {decision.weight?.toFixed(2)}</p>
            <p>Status: {decision.status}</p>
            <p style={styles.sub}>{decision.reason}</p>
          </>
        )}
      </div>

      <hr style={styles.hr} />

      {/* ================= METRICS ================= */}
      <div style={styles.section}>
        <h3>📊 Metrics</h3>
        <p>Trust: {trust?.toFixed(2)}</p>
        <p>Drift: {drift?.toFixed(2)}</p>
        <p>Status: {status}</p>
        <p>Epoch Age: {epoch_age}</p>
        <p>Recovery: {recovery_timer}</p>
      </div>

      <hr style={styles.hr} />

      {/* ================= CONSENSUS ================= */}
      <div style={styles.section}>
        <h3>🗳️ Consensus</h3>
        <p>Local: {local_valid ? "✅" : "❌"}</p>
        <p>Network: {network_accepted ? "✅" : "❌"}</p>
        <p>Global: {global_valid ? "✅" : "❌"}</p>

        <p>
          Votes → ✅ {peer_votes_valid} / ❌ {peer_votes_invalid}
        </p>
      </div>

      <hr style={styles.hr} />

      {/* ================= IDENTITY ================= */}
      <div style={styles.section}>
        <h3>🔑 Identity Chain</h3>

        <p>
          Chain Valid:{" "}
          <span style={{ color: chain_valid ? "#00ff88" : "#ff4d4d" }}>
            {chain_valid ? "YES" : "BROKEN"}
          </span>
        </p>

        <p>Depth: {identity_chain?.length}</p>

        {identity_chain?.slice(-5).reverse().map((entry, i) => (
          <div key={i} style={styles.identityBlock}>
            <div>🔑 {shortKey(entry.public_key)}</div>

            <div style={styles.sub}>
              sig: {entry.signature ? "✔ linked" : "GENESIS"}
            </div>
          </div>
        ))}
      </div>

      <hr style={styles.hr} />

      {/* ================= ATTACK PANEL ================= */}
      <div style={styles.section}>
        <h3>⚔️ Attack Panel</h3>

        <button style={styles.btn} onClick={() => onSpike?.(id)}>
          ⚡ Spike Attack
        </button>

        <button style={styles.btn} onClick={() => onBreach?.(id)}>
          ☠️ Critical Breach
        </button>

        <button style={styles.btn} onClick={() => onNetwork?.()}>
          🌊 Network Attack
        </button>
      </div>
    </div>
  );
}

// ================= HELPERS =================

const shortKey = (key) => {
  if (!key) return "unknown";
  return key.slice(0, 6).map((b) => b.toString(16)).join("") + "...";
};

const decisionStyle = (d) => {
  if (d === "ACCEPT") return { color: "#00ff88", fontWeight: "bold" };
  if (d === "REJECT") return { color: "#ff4d4d", fontWeight: "bold" };
  return { color: "#ffaa00", fontWeight: "bold" };
};

// ================= STYLES =================

const styles = {
  panel: {
    width: 320,
    minHeight: "100vh",
    background: "#050f1f",
    color: "white",
    padding: 20,
    borderLeft: "1px solid #1e2a3a",
    fontFamily: "monospace",
  },
  section: {
    marginBottom: 20,
  },
  identityBlock: {
    marginBottom: 8,
    padding: 6,
    background: "#0b1a2a",
    borderRadius: 4,
  },
  sub: {
    fontSize: 11,
    opacity: 0.7,
  },
  hr: {
    margin: "15px 0",
    opacity: 0.2,
  },
  btn: {
    width: "100%",
    marginBottom: 8,
    background: "#112233",
    color: "white",
    border: "none",
    padding: 8,
    cursor: "pointer",
  },
};