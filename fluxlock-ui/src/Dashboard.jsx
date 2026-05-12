import React, {
  useEffect,
  useState,
} from "react";

export default function Dashboard({
  node,
  onSpike,
  onBreach,
  onNetwork,
}) {

  const [decision, setDecision] =
    useState(null);

  const [identity, setIdentity] =
    useState(null);

  // =========================
  // 🧠 DECISION API
  // =========================
  useEffect(() => {

    if (!node) return;

    fetch(
      "http://127.0.0.1:3001/evaluate",
      {
        method: "POST",

        headers: {
          "Content-Type":
            "application/json",
        },

        body: JSON.stringify({
          id: node.id,
        }),
      }
    )
      .then((res) => res.json())
      .then(setDecision)
      .catch(() =>
        setDecision(null)
      );

  }, [node]);

  // =========================
  // 🔗 ACTIVE IDENTITY
  // =========================
  useEffect(() => {

    const handler = (e) => {

      const data =
        e.detail || {};

      setIdentity(data);
    };

    window.addEventListener(
      "fluxlock-auth",
      handler
    );

    return () =>
      window.removeEventListener(
        "fluxlock-auth",
        handler
      );

  }, []);

  if (!node) {

    return (
      <div style={styles.panel}>
        <h2>
          No node selected
        </h2>
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

    rehabilitation_score,
    rehabilitation_epochs,

    peer_votes_valid,
    peer_votes_invalid,

    local_valid,
    network_accepted,
    global_valid,

    identity_chain,
    chain_valid,

    attack_history,
    successful_recoveries,

    resilience_score,
    scar_level,
    immune_response,

    consensus_pressure,
    instability_radius,

  } = node;

  // =========================
  // 🔗 LINEAGE
  // =========================
  const lineage =
    identity?.lineage || [];

  const latestProof =
    lineage.length > 0
      ? lineage[
          lineage.length - 1
        ]
      : null;

  // =========================
  // 🎨 STAGE COLOR
  // =========================
  const stageColor =
    getStageColor(
      identity?.status
    );

  // =========================
  // 🧠 NODE COLOR
  // =========================
  const nodeColor =
    getNodeColor(status);

  return (
    <div style={styles.panel}>

      {/* ========================= */}
      {/* 🌐 HEADER */}
      {/* ========================= */}
      <div
        style={{
          ...styles.header,
          borderColor: nodeColor,
        }}
      >

        <h2>
          Validator {id}
        </h2>

        <div
          style={{
            color: nodeColor,
            fontWeight: "bold",
            textTransform:
              "uppercase",
          }}
        >
          {status}
        </div>

      </div>

      {/* ========================= */}
      {/* 🧠 DECISION */}
      {/* ========================= */}
      <div style={styles.section}>

        <h3>
          🧠 Consensus Decision
        </h3>

        {decision && (
          <>

            <p
              style={
                decisionStyle(
                  decision.decision
                )
              }
            >
              {decision.decision}
            </p>

            <MetricBar
              label="Weight"
              value={
                (
                  decision.weight || 0
                ) * 100
              }
              color="#00ffee"
            />

            <p style={styles.sub}>
              {decision.reason}
            </p>

          </>
        )}

      </div>

      <hr style={styles.hr} />

      {/* ========================= */}
      {/* 📊 CORE METRICS */}
      {/* ========================= */}
      <div style={styles.section}>

        <h3>
          📊 Core Metrics
        </h3>

        <MetricBar
          label="Trust"
          value={trust}
          color="#00ff88"
        />

        <MetricBar
          label="Drift"
          value={drift}
          color="#ffaa00"
        />

        <MetricBar
          label="Resilience"
          value={resilience_score}
          color="#00ccff"
        />

        <MetricBar
          label="Immune Response"
          value={immune_response * 10}
          color="#00ffff"
        />

        <MetricBar
          label="Scar Level"
          value={scar_level * 10}
          color="#ff4444"
        />

      </div>

      <hr style={styles.hr} />

      {/* ========================= */}
      {/* 🌐 NETWORK PRESSURE */}
      {/* ========================= */}
      <div style={styles.section}>

        <h3>
          🌐 Consensus Physics
        </h3>

        <MetricBar
          label="Pressure"
          value={
            consensus_pressure * 10
          }
          color="#ff66ff"
        />

        <MetricBar
          label="Instability Radius"
          value={
            instability_radius * 10
          }
          color="#ff9933"
        />

        <p>
          Epoch Age:
          {" "}
          {epoch_age}
        </p>

        <p>
          Recovery Timer:
          {" "}
          {recovery_timer}
        </p>

      </div>

      <hr style={styles.hr} />

      {/* ========================= */}
      {/* 🛡 RECOVERY */}
      {/* ========================= */}
      <div style={styles.section}>

        <h3>
          🛡 Rehabilitation
        </h3>

        <MetricBar
          label="Rehabilitation"
          value={
            rehabilitation_score
          }
          color="#ffaa00"
        />

        <p>
          Recovery Epochs:
          {" "}
          {
            rehabilitation_epochs
          }
        </p>

        <p>
          Successful Recoveries:
          {" "}
          {
            successful_recoveries
          }
        </p>

        <p>
          Attack History:
          {" "}
          {attack_history}
        </p>

      </div>

      <hr style={styles.hr} />

      {/* ========================= */}
      {/* 🗳 CONSENSUS */}
      {/* ========================= */}
      <div style={styles.section}>

        <h3>
          🗳 Consensus
        </h3>

        <ConsensusState
          label="Local"
          value={local_valid}
        />

        <ConsensusState
          label="Network"
          value={
            network_accepted
          }
        />

        <ConsensusState
          label="Global"
          value={global_valid}
        />

        <ConsensusState
          label="Chain"
          value={chain_valid}
        />

        <p>
          Votes →
          {" "}
          ✅ {peer_votes_valid}
          {" / "}
          ❌ {peer_votes_invalid}
        </p>

      </div>

      <hr style={styles.hr} />

      {/* ========================= */}
      {/* 🔗 IDENTITY */}
      {/* ========================= */}
      {identity && (

        <>

          <div style={styles.section}>

            <h3>
              🔗 Identity Evolution
            </h3>

            <p>

              Stage:
              {" "}

              <span
                style={{
                  color:
                    stageColor,
                  fontWeight:
                    "bold",
                }}
              >
                {
                  identity.status
                }
              </span>

            </p>

            <MetricBar
              label="Continuity"
              value={
                identity
                  .continuity_score
              }
              color="#00ffee"
            />

            <MetricBar
              label="Identity Trust"
              value={
                identity
                  .trust_score
              }
              color="#00ff88"
            />

            <MetricBar
              label="Identity Drift"
              value={
                identity
                  .drift_score
              }
              color="#ffaa00"
            />

            <p>
              Credential Depth:
              {" "}
              {
                identity
                  .credential_depth
              }
            </p>

            <p>
              Sessions:
              {" "}
              {
                identity
                  .session_count
              }
            </p>

          </div>

          <hr style={styles.hr} />

          {/* ========================= */}
          {/* 🔐 PROOFS */}
          {/* ========================= */}
          <div style={styles.section}>

            <h3>
              🔐 Proof Lineage
            </h3>

            <p>
              Total Proofs:
              {" "}
              {lineage.length}
            </p>

            {latestProof && (

              <>

                <HashBlock
                  title="Current Proof"
                  value={
                    latestProof
                      .proof_hash
                  }
                />

                <HashBlock
                  title="Previous Proof"
                  value={
                    latestProof
                      .previous_hash
                  }
                />

              </>
            )}

          </div>

        </>
      )}

      <hr style={styles.hr} />

      {/* ========================= */}
      {/* 🔑 CHAIN */}
      {/* ========================= */}
      <div style={styles.section}>

        <h3>
          🔑 Validator Chain
        </h3>

        <p>
          Depth:
          {" "}
          {
            identity_chain
              ?.length
          }
        </p>

        {identity_chain
          ?.slice(-5)
          .reverse()
          .map((entry, i) => (

            <div
              key={i}
              style={
                styles.identityBlock
              }
            >

              <div>
                🔑
                {" "}
                {
                  shortKey(
                    entry.public_key
                  )
                }
              </div>

              <div
                style={styles.sub}
              >
                sig:
                {" "}
                {
                  entry.signature
                    ? "✔ linked"
                    : "GENESIS"
                }
              </div>

            </div>
        ))}

      </div>

      <hr style={styles.hr} />

      {/* ========================= */}
      {/* ⚔ ATTACKS */}
      {/* ========================= */}
      <div style={styles.section}>

        <h3>
          ⚔ Attack Controls
        </h3>

        <button
          style={styles.btn}
          onClick={() =>
            onSpike?.(id)
          }
        >
          ⚡ Spike Attack
        </button>

        <button
          style={{
            ...styles.btn,
            border:
              "1px solid rgba(255,80,80,0.4)",
          }}
          onClick={() =>
            onBreach?.(id)
          }
        >
          ☠ Critical Breach
        </button>

        <button
          style={styles.btn}
          onClick={() =>
            onNetwork?.()
          }
        >
          🌊 Network Attack
        </button>

      </div>

    </div>
  );
}

// =========================
// 📊 METRIC BAR
// =========================
function MetricBar({
  label,
  value,
  color,
}) {

  const safe =
    Math.max(
      0,
      Math.min(value || 0, 100)
    );

  return (

    <div
      style={{
        marginBottom: 12,
      }}
    >

      <div style={styles.metricRow}>

        <span>
          {label}
        </span>

        <span>
          {safe.toFixed(1)}
        </span>

      </div>

      <div style={styles.metricTrack}>

        <div
          style={{
            ...styles.metricFill,
            width: `${safe}%`,
            background: color,
          }}
        />

      </div>

    </div>
  );
}

// =========================
// 🗳 CONSENSUS
// =========================
function ConsensusState({
  label,
  value,
}) {

  return (

    <div
      style={{
        marginBottom: 6,
      }}
    >

      {label}
      {" → "}

      <span
        style={{
          color:
            value
              ? "#00ff88"
              : "#ff4444",
        }}
      >

        {value
          ? "VALID"
          : "INVALID"}

      </span>

    </div>
  );
}

// =========================
// 🔐 HASH BLOCK
// =========================
function HashBlock({
  title,
  value,
}) {

  return (

    <div style={styles.hashBlock}>

      <div style={styles.hashLabel}>
        {title}
      </div>

      <div style={styles.hash}>
        {value}
      </div>

    </div>
  );
}

// =========================
// 🔑 HELPERS
// =========================
const shortKey = (key) => {

  if (!key)
    return "unknown";

  return (
    key
      .slice(0, 6)
      .map(
        (b) =>
          b.toString(16)
      )
      .join("")
    + "..."
  );
};

const decisionStyle = (d) => {

  if (d === "ACCEPT")
    return {
      color: "#00ff88",
      fontWeight: "bold",
      fontSize: 18,
    };

  if (d === "REJECT")
    return {
      color: "#ff4444",
      fontWeight: "bold",
      fontSize: 18,
    };

  return {
    color: "#ffaa00",
    fontWeight: "bold",
    fontSize: 18,
  };
};

const getStageColor =
  (status) => {

  switch (status) {

    case "genesis":
      return "#888";

    case "emerging":
      return "#00ccff";

    case "stabilizing":
      return "#00ffaa";

    case "established":
      return "#00ff88";

    case "sovereign":
      return "#00ffff";

    case "immune":
      return "#66ffff";

    case "recovering":
      return "#ffaa00";

    case "quarantined":
      return "#ff3333";

    default:
      return "#ffffff";
  }
};

const getNodeColor =
  (status) => {

  switch (status) {

    case "healthy":
      return "#00ffee";

    case "recovering":
      return "#ffaa00";

    case "quarantined":
      return "#ff4444";

    case "immune":
      return "#66ffff";

    case "fractured":
      return "#ff00aa";

    default:
      return "#ffffff";
  }
};

// =========================
// 🎨 STYLES
// =========================
const styles = {

  panel: {

    width: 360,

    minHeight: "100vh",

    background:
      "#050f1f",

    color: "white",

    padding: 20,

    borderLeft:
      "1px solid #1e2a3a",

    fontFamily:
      "monospace",

    overflowY: "auto",
  },

  header: {

    marginBottom: 20,

    paddingBottom: 12,

    borderBottom:
      "1px solid rgba(0,255,255,0.2)",
  },

  section: {
    marginBottom: 20,
  },

  metricRow: {

    display: "flex",

    justifyContent:
      "space-between",

    marginBottom: 4,

    fontSize: 12,
  },

  metricTrack: {

    width: "100%",

    height: 8,

    background:
      "rgba(255,255,255,0.06)",

    borderRadius: 999,
  },

  metricFill: {

    height: "100%",

    borderRadius: 999,

    transition:
      "all 0.3s ease",
  },

  identityBlock: {

    marginBottom: 8,

    padding: 6,

    background:
      "#0b1a2a",

    borderRadius: 4,
  },

  hashBlock: {

    marginBottom: 12,

    padding: 10,

    background:
      "#08131f",

    border:
      "1px solid rgba(0,255,255,0.08)",

    borderRadius: 6,
  },

  hashLabel: {

    fontSize: 11,

    opacity: 0.7,

    marginBottom: 6,
  },

  hash: {

    fontSize: 10,

    color: "#00ffee",

    wordBreak:
      "break-all",
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

    background:
      "#112233",

    color: "white",

    border:
      "1px solid rgba(0,255,255,0.08)",

    padding: 10,

    cursor: "pointer",

    borderRadius: 6,
  },
};