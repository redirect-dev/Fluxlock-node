import EventFeed from "./components/EventFeed";
import ValidatorPanel from "./components/ValidatorPanel";

function App() {
  return (
    <div
      style={{
        backgroundColor: "#0b0f14",
        minHeight: "100vh",
        padding: "20px",
        fontFamily: "Arial, sans-serif",
        color: "#fff",
      }}
    >
      <h1
        style={{
          textAlign: "center",
          color: "#4da3ff",
          marginBottom: "30px",
        }}
      >
        ⚡ FLUXLOCK LIVE EVENT STREAM
      </h1>

      <EventFeed />

      <ValidatorPanel />
    </div>
  );
}

export default App;