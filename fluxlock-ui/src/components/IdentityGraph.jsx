import { useEffect, useState } from "react";

function decode(bytes) {
  try {
    return new TextDecoder().decode(new Uint8Array(bytes));
  } catch {
    return null;
  }
}

export default function IdentityGraph() {
  const [chain, setChain] = useState([]);

  useEffect(() => {
    let lastData = "";

    const loadEvents = async () => {
      try {
        const res = await fetch("/events.json?t=" + Date.now());
        const text = await res.text();

        // 🔥 Only update when file changes
        if (text !== lastData) {
          lastData = text;
          const data = JSON.parse(text);

          const parsed = [];

          data.forEach((event) => {
            const type = Object.keys(event)[0];
            const payload = event[type];

            if (type === "RotationSuccess") {
              parsed.push({
                type,
                from: decode(payload.identity),
                to: decode(payload.new_identity),
              });
            }

            if (type === "InvalidContinuity") {
              parsed.push({
                type,
                from: decode(payload.identity),
                to: "❌",
              });
            }
          });

          setChain(parsed);
        }
      } catch (err) {
        console.error("Failed to load chain:", err);
      }
    };

    loadEvents();
    const interval = setInterval(loadEvents, 500);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="mt-6 p-4 border border-[#1F2A38] rounded-xl bg-[#0F141B]">
      <h2 className="text-[#8B9BB4] mb-4 font-bold">
        IDENTITY CHAIN
      </h2>

      <div className="flex items-center flex-wrap gap-4">
        {chain.map((link, index) => {
          const isError = link.type === "InvalidContinuity";

          return (
            <div key={index} className="flex items-center">
              <div className="px-3 py-2 rounded-lg border border-[#1F2A38] text-white">
                {link.from}
              </div>

              <div
                className={`mx-2 ${
                  isError ? "text-red-500" : "text-green-500"
                }`}
              >
                →
              </div>

              <div
                className={`px-3 py-2 rounded-lg border ${
                  isError
                    ? "border-red-500 text-red-400"
                    : "border-green-500 text-green-400"
                }`}
              >
                {link.to}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}