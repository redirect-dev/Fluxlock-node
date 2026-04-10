import { useEffect, useState } from "react";

function decodeIdentity(arr) {
  if (typeof arr === "string") return arr;
  if (!Array.isArray(arr)) return "";
  return new TextDecoder().decode(new Uint8Array(arr));
}

export default function EventFeed() {
  const [events, setEvents] = useState([]);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const res = await fetch("/events.json?ts=" + Date.now());
        const data = await res.json();
        setEvents(data.reverse());
      } catch (err) {
        console.error("Error fetching events:", err);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  const getColor = (event) => {
    if (event.InvalidContinuity || event.ValidatorSlashed) return "#ff4d4d";
    if (event.RotationSuccess) return "#4dff88";
    return "#ccc";
  };

  return (
    <div>
      <h2 style={{ textAlign: "center" }}>
        ⚡ FLUXLOCK LIVE EVENT STREAM
      </h2>

      {events.map((event, index) => {
        const type = Object.keys(event)[0];
        const data = event[type];
        const color = getColor(event);

        const formattedData = { ...data };

        if (data.identity) {
          formattedData.identity = decodeIdentity(data.identity);
        }

        if (data.new_identity) {
          formattedData.new_identity = decodeIdentity(data.new_identity);
        }

        return (
          <div
            key={index}
            style={{
              marginBottom: "30px",
              padding: "15px",
              border: `1px solid ${color}`,
              borderRadius: "10px",
              backgroundColor: "#111",
            }}
          >
            <h3 style={{ color }}>{type}</h3>

            <pre style={{ whiteSpace: "pre-wrap", color: "#ccc" }}>
              {JSON.stringify(formattedData, null, 2)}
            </pre>
          </div>
        );
      })}
    </div>
  );
}