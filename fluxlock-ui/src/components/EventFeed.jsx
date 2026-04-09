import { useEffect, useState } from "react";

function getColor(type) {
  switch (type) {
    case "RotationSuccess":
      return "text-[#00FF9C]";
    case "InvalidContinuity":
      return "text-[#FF3B3B]";
    case "ValidatorSlashed":
      return "text-[#FF0000] animate-pulse";
    default:
      return "text-[#8B9BB4]";
  }
}

// 🔥 Decode helper
function decode(bytes) {
  try {
    return new TextDecoder().decode(new Uint8Array(bytes));
  } catch {
    return bytes;
  }
}

// 🔥 Clean payload formatting
function formatPayload(payload) {
  const updated = { ...payload };

  if (updated.identity) {
    updated.identity = decode(updated.identity);
  }

  if (updated.new_identity) {
    updated.new_identity = decode(updated.new_identity);
  }

  return updated;
}

export default function EventFeed() {
  const [events, setEvents] = useState([]);

  useEffect(() => {
    let lastData = "";

    const loadEvents = async () => {
      try {
        const res = await fetch("/events.json?t=" + Date.now());
        const text = await res.text();

        // 🔥 Only update if changed
        if (text !== lastData) {
          lastData = text;
          const data = JSON.parse(text);
          setEvents(data.reverse());
        }
      } catch (err) {
        console.error("Failed to load events:", err);
      }
    };

    loadEvents();
    const interval = setInterval(loadEvents, 500);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-[#121821] border border-[#1F2A38] rounded-xl p-4 h-[80vh] overflow-y-auto">
      <h2 className="text-lg font-bold mb-4 text-[#8B9BB4]">
        ⚡ FLUXLOCK LIVE EVENT STREAM
      </h2>

      {events.length === 0 && (
        <p className="text-[#8B9BB4]">Waiting for events...</p>
      )}

      {events.map((event, i) => {
        const type = Object.keys(event)[0];
        const payload = formatPayload(event[type]);

        return (
          <div
            key={i}
            className={`mb-3 p-3 rounded-lg border border-[#1F2A38] bg-[#0F141B] ${getColor(
              type
            )}`}
          >
            <p className="text-xs uppercase opacity-70 mb-1">{type}</p>
            <pre className="text-sm">
              {JSON.stringify(payload, null, 2)}
            </pre>
          </div>
        );
      })}
    </div>
  );
}