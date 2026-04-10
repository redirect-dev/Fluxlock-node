import { useEffect, useState } from "react";

export default function ValidatorPanel() {
  const [validator, setValidator] = useState({
    stake: 1000,
    lastEvent: "None",
    status: "Healthy",
  });

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const res = await fetch("/events.json?ts=" + Date.now());
        const data = await res.json();

        let stake = 1000;
        let lastEvent = "None";
        let status = "Healthy";

        data.forEach((event) => {
          if (event.ValidatorSlashed) {
            stake -= event.ValidatorSlashed.amount;
            lastEvent = "ValidatorSlashed";
            status = "Slashed";
          }

          if (event.InvalidContinuity) {
            lastEvent = "InvalidContinuity";
            status = "Slashed";
          }

          if (event.RotationSuccess) {
            lastEvent = "RotationSuccess";
          }
        });

        setValidator({
          stake,
          lastEvent,
          status,
        });
      } catch (err) {
        console.error("Validator panel error:", err);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div
      style={{
        marginTop: "40px",
        padding: "20px",
        border: "1px solid #333",
        borderRadius: "10px",
        background: "#0d1117",
        color: "#fff",
      }}
    >
      <h2>⚖️ Validator Status</h2>

      <p>
        <strong>Stake:</strong> {validator.stake}
      </p>

      <p>
        <strong>Last Event:</strong> {validator.lastEvent}
      </p>

      <p>
        <strong>Status:</strong>{" "}
        <span
          style={{
            color: validator.status === "Slashed" ? "red" : "lime",
          }}
        >
          {validator.status}
        </span>
      </p>
    </div>
  );
}