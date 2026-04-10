import { useEffect, useState } from "react";

export default function ConsensusPanel() {
  const [consensus, setConsensus] = useState({
    decision: "Unknown",
    majority: [],
    minority: [],
  });

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const res = await fetch("/events.json?ts=" + Date.now());
        const data = await res.json();

        let valid = [];
        let allValidators = new Set();

        data.forEach((event) => {
          if (event.RotationSuccess) {
            const v = event.RotationSuccess.validator;
            valid.push(v);
            allValidators.add(v);
          }

          if (event.ValidatorSlashed) {
            allValidators.add(event.ValidatorSlashed.validator);
          }
        });

        // Everyone not in valid list is wrong
        let invalid = [...allValidators].filter(
          (v) => !valid.includes(v)
        );

        let decision = "No Consensus";
        let majority = [];
        let minority = [];

        if (valid.length > 0) {
          decision = "VALID ROTATION ✅";
          majority = valid;
          minority = invalid;
        }

        setConsensus({
          decision,
          majority,
          minority,
        });
      } catch (err) {
        console.error(err);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{ marginTop: "40px", textAlign: "center" }}>
      <h2>⚖️ Network Consensus</h2>

      <h3 style={{ color: "#4dff88" }}>{consensus.decision}</h3>

      <div style={{ marginTop: "20px" }}>
        <h4>Majority (Correct)</h4>
        {consensus.majority.map((v, i) => (
          <p key={i} style={{ color: "#4dff88" }}>
            ✔ {v}
          </p>
        ))}
      </div>

      <div style={{ marginTop: "20px" }}>
        <h4>Minority (Incorrect)</h4>
        {consensus.minority.map((v, i) => (
          <p key={i} style={{ color: "#ff4d4d" }}>
            ❌ {v}
          </p>
        ))}
      </div>
    </div>
  );
}