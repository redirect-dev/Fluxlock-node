import { useEffect, useState } from "react";
import axios from "axios";
import FluxlockDashboard from "./FluxlockDashboard";

export default function App() {

  // =========================
  // 🧠 PERSISTENT IDENTITY
  // =========================
  const [identityId, setIdentityId] =
    useState(null);

  // =========================
  // 🔐 AUTH STATE
  // =========================
  const [status, setStatus] =
    useState("idle");

  const [result, setResult] =
    useState(null);

  // =========================
  // 🧬 LOAD / CREATE IDENTITY
  // =========================
  useEffect(() => {

    let stored =
      localStorage.getItem(
        "fluxlock_identity"
      );

    if (!stored) {

      stored =
        "flux-" +
        crypto.randomUUID();

      localStorage.setItem(
        "fluxlock_identity",
        stored
      );
    }

    setIdentityId(stored);

  }, []);

  // =========================
  // 🔐 LOGIN FLOW
  // =========================
  const login = async () => {

    try {

      setStatus("authenticating");

      // =========================
      // 🔏 SIGN REQUEST
      // =========================
      const signRes =
        await axios.post(
          "http://127.0.0.1:3001/sign",
          {
            message: "login",
            validator_id: 0,
          }
        );

      const signature =
        signRes.data.signature;

      // =========================
      // 🌊 AUTH FLOW
      // =========================
      const authRes =
        await axios.post(
          "http://127.0.0.1:3001/auth/flow",
          {
            message: "login",

            signature,

            validator_id: 0,

            // 🔥 REQUIRED
            identity_id:
              identityId,

            nonce:
              crypto.randomUUID(),

            timestamp:
              Math.floor(
                Date.now() / 1000
              ),
          }
        );

      const data =
        authRes.data;

      setResult(data);

      let resolvedStatus =
        "denied";

      if (data.authenticated) {

        if (
          data.status ===
          "healthy"
        ) {

          resolvedStatus =
            "healthy";

        } else if (
          data.status ===
          "recovering"
        ) {

          resolvedStatus =
            "recovering";

        } else {

          resolvedStatus =
            "granted";
        }
      }

      setStatus(
        resolvedStatus
      );

      // =========================
      // 🌊 VISUAL EVENT
      // =========================
      window.dispatchEvent(
        new CustomEvent(
          "fluxlock-auth",
          {
            detail: {
              ...data,
              resolvedStatus,
            },
          }
        )
      );

    } catch (err) {

      console.error(err);

      setStatus("error");
    }
  };

  // =========================
  // 🎨 STATUS COLOR
  // =========================
  const getColor = () => {

    if (status === "healthy")
      return "#00ffcc";

    if (
      status === "recovering"
    )
      return "#ffaa00";

    if (status === "denied")
      return "#ff4444";

    if (
      status ===
      "authenticating"
    )
      return "#8888ff";

    if (status === "error")
      return "#ff00ff";

    return "#ffffff";
  };

  // =========================
  // 🚀 UI
  // =========================
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
          borderRight:
            "1px solid rgba(255,255,255,0.05)",
        }}
      >

        <h1
          style={{
            fontSize: 42,
            marginBottom: 20,
          }}
        >
          Fluxlock Identity
        </h1>

        {/* 🧠 IDENTITY */}
        <div
          style={{
            marginBottom: 20,
            fontSize: 12,
            opacity: 0.7,
            wordBreak: "break-word",
          }}
        >
          Identity:
          <br />
          {identityId}
        </div>

        {/* 🔐 LOGIN */}
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
          Authenticate Identity
        </button>

        {/* 📊 STATUS */}
        <p style={{ marginTop: 20 }}>
          Status:{" "}
          <span
            style={{
              color: getColor(),
              fontWeight: "bold",
            }}
          >
            {status.toUpperCase()}
          </span>
        </p>

        {/* 📈 RESULT */}
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
            {JSON.stringify(
              result,
              null,
              2
            )}
          </pre>
        )}

      </div>

      {/* ================= RIGHT PANEL ================= */}
      <div
        style={{
          flex: 1,
          position: "relative",
        }}
      >
        <FluxlockDashboard />
      </div>

    </div>
  );
}