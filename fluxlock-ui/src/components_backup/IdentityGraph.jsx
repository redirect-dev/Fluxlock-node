import { useEffect, useState } from "react";

function decodeIdentity(arr) {
  if (typeof arr === "string") return arr;
  if (!Array.isArray(arr)) return "";
  return new TextDecoder().decode(new Uint8Array(arr));
}

export default function IdentityGraph() {
  const [chains, setChains] = useState([]);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const res = await fetch("/events.json?ts=" + Date.now());
        const data = await res.json();

        const newChains = [];

        data.forEach((event) => {
          if (event.RotationSuccess) {
            newChains.push({
              from: decodeIdentity(event.RotationSuccess.identity),
              to: decodeIdentity(event.RotationSuccess.new_identity),
              valid: true,
            });
          }

          if (event.InvalidContinuity) {
            newChains.push({
              from: decodeIdentity(event.InvalidContinuity.identity),
              to: "❌",
              valid: false,
            });
          }
        });

        setChains(newChains);
      } catch (err) {
        console.error(err);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{ marginTop: "40px", textAlign: "center" }}>
      <h2>🔗 Identity Chain</h2>

      {chains.map((chain, index) => (
        <div
          key={index}
          style={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            marginBottom: "15px",
            gap: "10px",
            animation: "fadeIn 0.5s ease",
          }}
        >
          <div
            style={{
              padding: "10px 15px",
              borderRadius: "8px",
              border: "1px solid #4dff88",
              transition: "all 0.3s ease",
            }}
          >
            {chain.from}
          </div>

          <div style={{ fontSize: "20px" }}>→</div>

          <div
            style={{
              padding: "10px 15px",
              borderRadius: "8px",
              border: `1px solid ${
                chain.valid ? "#4dff88" : "#ff4d4d"
              }`,
              color: chain.valid ? "#4dff88" : "#ff4d4d",
              transform: chain.valid ? "scale(1)" : "scale(1.1)",
              transition: "all 0.3s ease",
            }}
          >
            {chain.to}
          </div>
        </div>
      ))}

      <style>
        {`
          @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
          }
        `}
      </style>
    </div>
  );
}