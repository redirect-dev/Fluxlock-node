import { useEffect, useState } from "react";
import axios from "axios";

import FluxlockDashboard from "./FluxlockDashboard";

export default function App() {

  // =========================
  // 🧠 PERSISTENT IDENTITY
  // =========================
  const [identityId, setIdentityId] = useState(null);

  // auth state
  const [status, setStatus] = useState("idle");
  const [result, setResult] = useState(null);

  // =========================
  // 🔁 LOAD / CREATE IDENTITY
  // =========================
  useEffect(() => {

    let stored =
      localStorage.getItem("fluxlock_identity");

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
  // 🔐 AUTH FLOW
  // =========================
  const login = async () => {

    if (!identityId) return;

    try {

      setStatus("authenticating");

      // =========================
      // 🔐 STEP 1 — SIGN
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
      // 🌊 STEP 2 — AUTH
      // =========================
      const authRes =
        await axios.post(
          "http://127.0.0.1:3001/auth/flow",
          {
            message: "login",

            signature,

            validator_id: 0,

            // 🔥 NEW
            identity_id: identityId,

            nonce:
              Math.random()
                .toString(36)
                .substring(2),

            timestamp:
              Math.floor(Date.now() / 1000),
          }
        );

      const data = authRes.data;

      setResult(data);

      // =========================
      // 🧠 STATUS RESOLUTION
      // =========================
      let resolved = "denied";

      if (data.authenticated) {

        if (data.status === "healthy") {

          resolved = "healthy";

        } else if (
          data.status === "recovering"
        ) {

          resolved = "recovering";

        } else {

          resolved = "granted";
        }

      } else {

        resolved = "denied";
      }

      setStatus(resolved);

      // =========================
      // 🌊 EMIT NETWORK EVENT
      // =========================
      window.dispatchEvent(
        new CustomEvent(
          "fluxlock-auth",
          {
            detail: {
              ...data,
              resolvedStatus: resolved,
            },
          }
        )
      );

    } catch (err) {

      console.error(err);

      setStatus("error");

      // emit failure event
      window.dispatchEvent(
        new CustomEvent(
          "fluxlock-auth",
          {
            detail: {
              authenticated: false,
              status: "error",
              confidence: 0,
              resolvedStatus: "error",
            },
          }
        )
      );
    }
  };

  // =========================
  // 🎨 STATUS COLORS
  // =========================
  const getColor = () => {

    if (status === "healthy")
      return "#00ffcc";

    if (status === "recovering")
      return "#ffaa00";

    if (status === "denied")
      return "#ff4444";

    if (status === "authenticating")
      return "#8888ff";

    if (status === "error")
      return "#ff00ff";

    return "#ffffff";
  };

  // =========================
  // 🚀 UI
  // =========================
  return (
    <div>

      {/* 🌐 CORE VISUAL SYSTEM */}
      <FluxlockDashboard />

      {/* 🔐 AUTH PANEL */}
      <div
        style={{
          position: "fixed",
          top: 20,
          left: 20,

          width: 320,

          background:
            "rgba(5,15,31,0.92)",

          border:
            "1px solid rgba(0,255,200,0.2)",

          borderRadius: 14,

          padding: 18,

          color: "white",

          fontFamily: "monospace",

          backdropFilter: "blur(10px)",

          zIndex: 1000,

          boxShadow:
            "0 0 40px rgba(0,255,200,0.15)",
        }}
      >

        <h2
          style={{
            marginTop: 0,
            color: "#00ffcc",
          }}
        >
          Fluxlock Identity
        </h2>

        {/* ========================= */}
        {/* 🧠 IDENTITY */}
        {/* ========================= */}
        <div
          style={{
            marginBottom: 16,
          }}
        >

          <div
            style={{
              opacity: 0.7,
              fontSize: 12,
              marginBottom: 4,
            }}
          >
            Persistent Identity
          </div>

          <div
            style={{
              color: "#00ffcc",
              fontSize: 13,
              wordBreak: "break-word",
            }}
          >
            {identityId}
          </div>
        </div>

        {/* ========================= */}
        {/* 🔐 LOGIN */}
        {/* ========================= */}
        <button
          onClick={login}
          style={{
            width: "100%",

            padding: 12,

            background:
              "linear-gradient(90deg,#00ffcc,#0088ff)",

            color: "#001018",

            border: "none",

            borderRadius: 10,

            fontWeight: "bold",

            cursor: "pointer",

            marginBottom: 16,
          }}
        >
          Authenticate Identity
        </button>

        {/* ========================= */}
        {/* 📊 STATUS */}
        {/* ========================= */}
        <div
          style={{
            marginBottom: 16,
          }}
        >

          <div
            style={{
              opacity: 0.7,
              fontSize: 12,
            }}
          >
            Identity Status
          </div>

          <div
            style={{
              color: getColor(),
              fontWeight: "bold",
              fontSize: 18,
            }}
          >
            {status.toUpperCase()}
          </div>
        </div>

        {/* ========================= */}
        {/* 🌊 CONTINUITY */}
        {/* ========================= */}
        {result && (
          <div
            style={{
              background:
                "rgba(0,255,200,0.05)",

              border:
                "1px solid rgba(0,255,200,0.15)",

              borderRadius: 10,

              padding: 12,

              marginBottom: 12,
            }}
          >

            <div
              style={{
                marginBottom: 8,
                color: "#00ffcc",
              }}
            >
              Identity Continuity
            </div>

            <div>
              Sessions:
              {" "}
              {result.session_count}
            </div>

            <div>
              Continuity:
              {" "}
              {result.continuity_score?.toFixed(2)}
            </div>

            <div>
              Credential Depth:
              {" "}
              {result.credential_depth}
            </div>

            <div>
              Confidence:
              {" "}
              {(
                result.confidence * 100
              ).toFixed(1)}%
            </div>

          </div>
        )}

        {/* ========================= */}
        {/* 🔍 RAW RESULT */}
        {/* ========================= */}
        {result && (
          <pre
            style={{
              fontSize: 10,

              background: "#020812",

              padding: 10,

              borderRadius: 8,

              overflow: "auto",

              maxHeight: 220,

              color: "#00ff88",
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
    </div>
  );
}