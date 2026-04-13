import { useEffect, useState } from "react";

export default function EventFeed() {
  const [events, setEvents] = useState([]);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const res = await fetch("/events.json?ts=" + Date.now());
        const data = await res.json();

        setEvents(data.events || []);
      } catch (err) {
        console.error(err);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{ marginTop: "20px" }}>
      <h2>⚡ FLUXLOCK LIVE EVENT STREAM</h2>

      {events.map((event, index) => {
        const [type, value] = Object.entries(event)[0];

        return (
          <div key={index} style={{ marginBottom: "10px" }}>
            <strong>{type}</strong>
            <pre style={{ color: "#ccc" }}>
              {JSON.stringify(value, null, 2)}
            </pre>
          </div>
        );
      })}
    </div>
  );
}