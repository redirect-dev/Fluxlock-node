import EventFeed from "./components/EventFeed";
import ValidatorPanel from "./components/ValidatorPanel";
import IdentityGraph from "./components/IdentityGraph";

function App() {
  return (
    <div style={{ padding: "20px", backgroundColor: "#0d1117", color: "#fff" }}>
      <h1 style={{ textAlign: "center" }}>
        ⚡ FLUXLOCK LIVE EVENT STREAM
      </h1>

      <EventFeed />
      <ValidatorPanel />
      <IdentityGraph />
    </div>
  );
}

export default App;