import { useEffect, useState } from "react";

export default function ValidatorNetwork() {
  const [validators, setValidators] = useState([]);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const res = await fetch("/events.json?ts=" + Date.now());
        const data = await res.json();

        let state = {
          "Validator A": { stake: 1000, status: "Healthy" },
          "Validator B": { stake: 1000, status: "Healthy" },
          "Validator C": { stake: 1000, status: "Healthy" },
        };

        data.forEach((event) => {
          if (event.ValidatorSlashed) {
            const { validator, amount } = event.ValidatorSlashed;

            if (state[validator]) {
              state[validator].stake -= amount;
              state[validator].status = "Slashed";
            }
          }
        });

        setValidators(
          Object.entries(state).map(([name, val]) => ({
            name,
            ...val,
          }))
        );
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
              width: "200px",
              borderRadius: "10px",
              background: "#111",
              border: `1px solid ${
                v.status === "Slashed" ? "#ff4d4d" : "#4dff88"
              }`,
              textAlign: "center",
              transition: "all 0.3s ease",
              boxShadow:
                v.status === "Slashed"
                  ? "0 0 15px rgba(255,0,0,0.5)"
                  : "0 0 10px rgba(0,255,100,0.3)",
            }}
          >
            <h3>{v.name}</h3>

            <p>
              <strong>Stake:</strong> {v.stake}
            </p>

            <p
              style={{
                color: v.status === "Slashed" ? "#ff4d4d" : "#4dff88",
                fontWeight: "bold",
              }}
            >
              {v.status}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
}