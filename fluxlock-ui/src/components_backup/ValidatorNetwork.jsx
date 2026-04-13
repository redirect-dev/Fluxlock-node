import { useEffect, useState } from "react";

export default function ValidatorNetwork() {
  const [validators, setValidators] = useState([]);

  useEffect(() => {
    const interval = setInterval(async () => {
      const res = await fetch("/events.json?ts=" + Date.now());
      const data = await res.json();
      setValidators(data.validators || []);
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  const getStatus = (rep) => {
    if (rep < 20) return { label: "EXILED", color: "#666" };
    if (rep < 60) return { label: "DEGRADED", color: "#ff4d4d" };
    return { label: "HEALTHY", color: "#4dff88" };
  };

  return (
    <div style={{ marginTop: "40px", textAlign: "center" }}>
      <h2>🌐 Validator Network</h2>

      {validators.map((v, i) => {
        const status = getStatus(v.reputation);

        return (
          <div key={i} style={{ marginBottom: "15px" }}>
            <h3>{v.name}</h3>
            <p>Stake: {v.stake}</p>
            <p>Reputation: {v.reputation}</p>

            <p style={{ color: status.color, fontWeight: "bold" }}>
              {status.label}
            </p>
          </div>
        );
      })}
    </div>
  );
}