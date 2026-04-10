import { useEffect, useState } from "react";

export default function ValidatorNetwork() {
  const [validators, setValidators] = useState([]);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const res = await fetch("/events.json?ts=" + Date.now());
        const data = await res.json();

        setValidators(data.validators || []);
      } catch (err) {
        console.error(err);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{ marginTop: "40px" }}>
      <h2 style={{ textAlign: "center" }}>🌐 Validator Network</h2>

      <div
        style={{
          display: "flex",
          justifyContent: "center",
          gap: "20px",
          marginTop: "20px",
        }}
      >
        {validators.map((v, i) => (
          <div
            key={i}
            style={{
              padding: "20px",
              width: "220px",
              borderRadius: "10px",
              background: "#111",
              border: `1px solid ${
                v.reputation < 80 ? "#ff4d4d" : "#4dff88"
              }`,
              textAlign: "center",
            }}
          >
            <h3>{v.name}</h3>

            <p>Stake: {v.stake}</p>

            <p>
              Reputation:{" "}
              <span
                style={{
                  color: v.reputation < 80 ? "#ff4d4d" : "#4dff88",
                  fontWeight: "bold",
                }}
              >
                {v.reputation}
              </span>
            </p>
          </div>
        ))}
      </div>
    </div>
  );
}