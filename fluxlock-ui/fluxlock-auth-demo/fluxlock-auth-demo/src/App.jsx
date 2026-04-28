import { useState } from "react";
import axios from "axios";
import FluxlockDashboard from "./FluxlockDashboard";

export default function App() {
  const [status, setStatus] = useState("idle");
  const [result, setResult] = useState(null);

  const login = async () => {
    try {
      setStatus("authenticating");

      const signRes = await axios.post("http://127.0.0.1:3001/sign", {
        message: "login",
        validator_id: 0,
      });

      const signature = signRes.data.signature;

      const authRes = await axios.post("http://127.0.0.1:3001/auth/flow", {
        message: "login",
        signature,
        validator_id: 0,
        nonce: Math.random().toString(36).substring(2),
        timestamp: Math.floor(Date.now() / 1000),
      });

      const data = authRes.data;
      setResult(data);

      let resolvedStatus = "denied";

      if (data.authenticated) {
        if (data.status === "healthy") resolvedStatus = "healthy";
        else if (data.status === "recovering") resolvedStatus = "recovering";
        else resolvedStatus = "granted";
      }

      setStatus(resolvedStatus);

      // 🔥 SEND TO VISUAL SYSTEM
      window.dispatchEvent(
        new CustomEvent("fluxlock-auth", {
          detail: {
            ...data,
            resolvedStatus,
          },
        })
      );
    } catch (err) {
      console.error(err);
      setStatus("error");
    }
  };

  const getColor = () => {
    if (status === "healthy") return "#00ffcc";
    if (status === "recovering") return "#ffaa00";
    if (status === "denied") return "#ff4444";
    if (status === "authenticating") return "#8888ff";
    return "#ffffff";
  };

  return (
    <div
      style={{
        display: "flex",
        height: "100vh",
        background: "#0a0f1a",
        color: "#fff",
        fontFamily: "Arial",
      }}
    >
      {/* ================= LEFT PANEL ================= */}
      <div
        style={{
          width: "35%",
          minWidth: 400,
          padding: 40,
          borderRight: "1px solid rgba(255,255,255,0.05)",
        }}
      >
        <h1 style={{ fontSize: 42, marginBottom: 20 }}>
          Fluxlock Auth Demo
        </h1>

        <button
          onClick={login}
          style={{
            padding: "12px 20px",
            fontSize: 16,
            cursor: "pointer",
            background: "#222",
            color: "#fff",
            border: "1px solid #444",
            borderRadius: 6,
          }}
        >
          Login with Fluxlock
        </button>

        <p style={{ marginTop: 20 }}>
          Status:{" "}
          <span style={{ color: getColor(), fontWeight: "bold" }}>
            {status.toUpperCase()}
          </span>
        </p>

        {result && (
          <pre
            style={{
              marginTop: 20,
              background: "#111",
              color: "#0f0",
              padding: 15,
              borderRadius: 8,
              fontSize: 12,
              overflow: "auto",
              maxHeight: 400,
            }}
          >
            {JSON.stringify(result, null, 2)}
          </pre>
        )}
      </div>

      {/* ================= RIGHT PANEL ================= */}
      <div style={{ flex: 1, position: "relative" }}>
        <FluxlockDashboard />
      </div>
    </div>
  );
}