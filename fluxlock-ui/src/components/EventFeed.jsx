import { useEffect, useState } from "react";

function getColor(level) {
  switch (level) {
    case "success":
      return "text-[#00FF9C]";
    case "error":
      return "text-[#FF3B3B]";
    case "critical":
      return "text-[#FF0000] animate-pulse";
    default:
      return "text-[#8B9BB4]";
  }
}

export default function EventFeed() {
  const [events, setEvents] = useState([]);

  useEffect(() => {
    const mockEvents = [
      {
        type: "InvalidContinuity",
        message: "Invalid identity lineage detected",
        level: "error",
      },
      {
        type: "ValidatorSlashed",
        message: "Validator slashed: -15 stake",
        level: "critical",
      },
      {
        type: "RotationSuccess",
        message: "Identity rotated successfully",
        level: "success",
      },
    ];

    let index = 0;

    const interval = setInterval(() => {
      if (index >= mockEvents.length) return;

      setEvents((prev) => [mockEvents[index], ...prev]);
      index++;
    }, 1200);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-[#121821] border border-[#1F2A38] rounded-xl p-4 h-[80vh] overflow-y-auto">
      {events.length === 0 && (
        <p className="text-[#8B9BB4]">Waiting for events...</p>
      )}

      {events.map((event, i) => (
        <div
          key={i}
          className={`mb-3 p-3 rounded-lg border border-[#1F2A38] bg-[#0F141B] ${getColor(event.level)}`}
        >
          <p className="text-xs uppercase opacity-70">
            {event.type}
          </p>
          <p className="text-lg">{event.message}</p>
        </div>
      ))}
    </div>
  );
}