import { useEffect, useState } from "react";

export default function ConsensusPanel() {
  const [consensus, setConsensus] = useState({
    decision: "Unknown",
    majority: [],
    minority: [],
    weights: {},
  });

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const res = await fetch("/events.json?ts=" + Date.now());
        const data = await res.json();

        const events = data.events || [];
        const validators = data.validators || [];

        let repMap = {};
        validators.forEach(v => {
          repMap[v.name] = v.reputation;
        });

        let valid = [];
        let invalid = [];

        events.forEach((event) => {
          if (event.RotationSuccess) {
            valid.push(event.RotationSuccess.validator);
          }

          if (event.ValidatorSlashed) {
            invalid.push(event.ValidatorSlashed.validator);
          }
        });

        valid = [...new Set(valid)];
        invalid = [...new Set(invalid)];

        // -----------------------------
        // 🔥 MAX-TRUST MODEL
        // -----------------------------
        let maxValid = Math.max(...valid.map(v => repMap[v] || 0), 0);
        let maxInvalid = Math.max(...invalid.map(v => repMap[v] || 0), 0);

        let decision = "Unknown";

        if (maxValid > maxInvalid) {
          decision = "VALID ROTATION ✅";
        } else if (maxInvalid > maxValid) {
          decision = "INVALID ROTATION ❌";
        }

        setConsensus({
          decision,
          majority: maxValid >= maxInvalid ? valid : invalid,
          minority: maxValid >= maxInvalid ? invalid : valid,
          weights: {
            maxValid,
            maxInvalid,
          },
        });
      } catch (err) {
        console.error(err);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{ marginTop: "40px", textAlign: "center" }}>
      <h2>⚖️ Network Consensus (Trust Anchored)</h2>

      <h3 style={{ color: "#4dff88" }}>{consensus.decision}</h3>

      <p>
        Max Trust (Valid): {consensus.weights.maxValid} | Max Trust (Invalid): {consensus.weights.maxInvalid}
      </p>

      <div style={{ marginTop: "20px" }}>
        <h4>Dominant (Highest Trust)</h4>
        {consensus.majority.map((v, i) => (
          <p key={i} style={{ color: "#4dff88" }}>
            ✔ {v}
          </p>
        ))}
      </div>

      <div style={{ marginTop: "20px" }}>
        <h4>Subordinate</h4>
        {consensus.minority.map((v, i) => (
          <p key={i} style={{ color: "#ff4d4d" }}>
            ❌ {v}
          </p>
        ))}
      </div>
    </div>
  );
}