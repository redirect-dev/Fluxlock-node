import EventFeed from "./components/EventFeed";
import ValidatorPanel from "./components/ValidatorPanel";
import IdentityGraph from "./components/IdentityGraph";
import ValidatorNetwork from "./components/ValidatorNetwork";

function App() {
  return (
    <div style={{ padding: "20px", backgroundColor: "#0d1117", color: "#fff" }}>
      <h1 style={{ textAlign: "center" }}>
        ⚡ FLUXLOCK LIVE EVENT STREAM
      </h1>

      <EventFeed />
      <ValidatorPanel />
      <IdentityGraph />
      <ValidatorNetwork />
    </div>
  );
}

export default App;