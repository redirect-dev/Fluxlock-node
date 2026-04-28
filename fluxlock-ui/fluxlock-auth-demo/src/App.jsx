import { useState } from "react";
import axios from "axios";

export default function App() {
  const [status, setStatus] = useState("idle");
  const [result, setResult] = useState(null);

  const login = async () => {
    try {
      setStatus("authenticating");

      // 1️⃣ sign
      const signRes = await axios.post("http://127.0.0.1:3001/sign", {
        message: "login",
        validator_id: 0,
      });

      const signature = signRes.data.signature;

      // 2️⃣ auth
      const authRes = await axios.post("http://127.0.0.1:3001/auth/flow", {
        message: "login",
        signature,
        validator_id: 0,
        nonce: Math.random().toString(36).substring(2),
        timestamp: Math.floor(Date.now() / 1000),
      });

      setResult(authRes.data);

      if (authRes.data.authenticated) {
        setStatus("granted");

        // 🔥 trigger your visual system later
        window.dispatchEvent(new Event("fluxlock-auth-success"));
      } else {
        setStatus("denied");
      }
    } catch (err) {
      console.error(err);
      setStatus("error");
    }
  };

  return (
    <div style={{ textAlign: "center", marginTop: 100 }}>
      <h1>Fluxlock Auth Demo</h1>

      <button onClick={login} style={{ padding: 12, fontSize: 16 }}>
        Login with Fluxlock
      </button>

      <p>Status: {status}</p>

      {result && (
        <pre
          style={{
            textAlign: "left",
            margin: "20px auto",
            width: 400,
            background: "#111",
            color: "#0f0",
            padding: 10,
          }}
        >
          {JSON.stringify(result, null, 2)}
        </pre>
      )}
    </div>
  );
}