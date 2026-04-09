import IdentityGraph from "./components/IdentityGraph";
import EventFeed from "./components/EventFeed";

export default function App() {
  return (
    <div className="h-screen w-full p-6 bg-[#0B0F14]">
      <h1 className="text-2xl mb-4 text-[#3B82F6]">
        ⚡ FLUXLOCK LIVE EVENT STREAM
      </h1>

      <EventFeed />
      <IdentityGraph />
    </div>
  );
}