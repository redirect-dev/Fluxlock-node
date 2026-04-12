import { useEffect, useState } from "react";

export default function ConsensusPanel() {
  const [validators, setValidators] = useState([]);

  useEffect(() => {
    const interval = setInterval(async () => {
      const res = await fetch("/events.json?ts=" + Date.now());
      const data = await res.json();
      setValidators(data.validators || []);
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  const TRUST_FLOOR = 30;

  const activeValidators = validators.filter(
    v => v.reputation >= 20 && v.stake >= 200
  );

  const maxInfluence =
    activeValidators.length > 0
      ? Math.max(...activeValidators.map(v => v.influence || 0))
      : 0;

  const consensusValid = maxInfluence >= TRUST_FLOOR;

  return (
    <div style={{ marginTop: "40px", textAlign: "center" }}>
      <h2>⚖️ Network Consensus (Economic Trust Anchored)</h2>

      {!consensusValid ? (
        <>
          <p style={{ color: "#ffaa00", fontWeight: "bold" }}>
            ⚠️ CONSENSUS FAILURE — INSUFFICIENT TRUST + STAKE
          </p>
          <p>
            Max Influence: {maxInfluence.toFixed(2)} (Threshold: {TRUST_FLOOR})
          </p>
        </>
      ) : (
        <>
          <p style={{ color: "#4dff88", fontWeight: "bold" }}>
            VALID ROTATION ✅
          </p>
          <p>Max Influence: {maxInfluence.toFixed(2)}</p>
        </>
      )}

      <h3>Dominant (Weighted Influence)</h3>
      {activeValidators
        .sort((a, b) => (b.influence || 0) - (a.influence || 0))
        .map((v, i) => (
          <p key={i}>
            ✔ {v.name} — Rep: {v.reputation} | Stake: {v.stake} | Influence:{" "}
            {v.influence.toFixed(2)} | Status: {v.status}
          </p>
        ))}

      <h3>Subordinate</h3>
      {validators
        .filter(v => v.reputation < 20 || v.stake < 200)
        .map((v, i) => (
          <p key={i}>
            ❌ {v.name} — Rep: {v.reputation} | Stake: {v.stake} | Status:{" "}
            {v.status}
          </p>
        ))}
    </div>
  );
}