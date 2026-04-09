import { useEffect, useState } from "react";

// 🎨 Color styling based on event type
function getColor(type) {
  switch (type) {
    case "RotationSuccess":
      return "text-[#00FF9C]";
    case "InvalidContinuity":
      return "text-[#FF3B3B]";
    case "ValidatorSlashed":
      return "text-[#FF0000] animate-pulse";
    case "InvalidNonce":
    case "ForkDetected":
    case "IdentityExpired":
    case "CommitmentMismatch":
      return "text-[#FFA500]";
    default:
      return "text-[#8B9BB4]";
  }
}

// 🔥 Convert byte array → readable string
function formatPayload(payload) {
  if (payload.identity) {
    try {
      const decoded = new TextDecoder().decode(
        new Uint8Array(payload.identity)
      );
      return { ...payload, identity: decoded };
    } catch {
      return payload;
    }
  }
  return payload;
}

export default function EventFeed() {
  const [events, setEvents] = useState([]);

  useEffect(() => {
    const loadEvents = () => {
      fetch("/events.json?t=" + new Date().getTime()) // 🔥 prevents caching
        .then((res) => res.json())
        .then((data) => {
          setEvents(data.reverse()); // newest first
        })
        .catch((err) => {
          console.error("Failed to load events:", err);
        });
    };

    loadEvents(); // initial load

    const interval = setInterval(loadEvents, 1000); // 🔄 refresh every second

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-[#121821] border border-[#1F2A38] rounded-xl p-4 h-[80vh] overflow-y-auto">
      <h2 className="text-lg font-bold mb-4 text-[#8B9BB4]">
        FLUXLOCK LIVE EVENT STREAM
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