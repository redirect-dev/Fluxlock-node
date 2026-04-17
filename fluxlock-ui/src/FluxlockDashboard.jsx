import { useEffect, useState, useRef } from "react";
import * as d3 from "d3";
import {
  createNetwork,
  simulateStep,
  requestSignature,
  verifySignature,
} from "./trustEngine";

function getColor(node) {
  if (node.compromised) return "#ff00ff";
  if (node.status === "attacked") return "#ff4d4d";
  if (node.status === "drifting") return "#ffaa00";
  if (node.status === "warning") return "#ffd633";
  return "#00ffaa";
}

function shortKey(key) {
  if (!key) return "—";
  return key.slice(0, 8);
}

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState([]);
  const [selectedId, setSelectedId] = useState(null);
  const svgRef = useRef();

  const selected = nodes.find(n => n.id === selectedId);

  // INIT
  useEffect(() => {
    setNodes(createNetwork(20));
  }, []);

  // SIM LOOP
  useEffect(() => {
    const interval = setInterval(() => {
      setNodes(prev => simulateStep(prev));
    }, 300);
    return () => clearInterval(interval);
  }, []);

  // =========================
  // 🔥 SIGNATURE + VERIFY ENGINE
  // =========================
  useEffect(() => {
    nodes.forEach(node => {
      node.identityChain.forEach((entry, idx) => {
        // SIGN
        if (!entry.signature && entry.needsSignature) {
          requestSignature(entry.needsSignature.message, node.id).then(sig => {
            setNodes(prev =>
              prev.map(n => {
                if (n.id !== node.id) return n;

                const updatedChain = [...n.identityChain];
                updatedChain[idx] = {
                  ...updatedChain[idx],
                  signature: sig,
                  needsSignature: null,
                };

                return { ...n, identityChain: updatedChain };
              })
            );
          });
        }

        // VERIFY
        if (
          entry.signature &&
          entry.signature !== "genesis" &&
          !entry.verified
        ) {
          verifySignature(entry.publicKey, entry.signature, node.id).then(valid => {
            setNodes(prev =>
              prev.map(n => {
                if (n.id !== node.id) return n;

                const updatedChain = [...n.identityChain];
                updatedChain[idx] = {
                  ...updatedChain[idx],
                  verified: true,
                  invalidSignature: !valid,
                };

                return { ...n, identityChain: updatedChain };
              })
            );
          });
        }
      });
    });
  }, [nodes]);

  // =========================
  // ATTACKS
  // =========================
  const spikeAttack = () => {
    if (!selected) return;
    setNodes(prev =>
      prev.map(n =>
        n.id === selected.id
          ? { ...n, drift: n.drift + 60, trust: n.trust - 25 }
          : n
      )
    );
  };

  const criticalBreach = () => {
    if (!selected) return;
    setNodes(prev =>
      prev.map(n =>
        n.id === selected.id
          ? {
              ...n,
              drift: n.drift + 120,
              trust: n.trust - 50,
              compromised: true,
              recoveryTimer: 0,
            }
          : n
      )
    );
  };

  const networkAttack = () => {
    if (!selected) return;
    setNodes(prev =>
      prev.map(n => {
        if (n.id === selected.id || selected.connections.includes(n.id)) {
          return {
            ...n,
            drift: n.drift + 40,
            trust: n.trust - 15,
          };
        }
        return n;
      })
    );
  };

  // =========================
  // GRAPH
  // =========================
  useEffect(() => {
    if (!nodes.length) return;

    const width = 700;
    const height = 700;
    const radius = 260;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    svg.attr("width", width).attr("height", height);

    const cx = width / 2;
    const cy = height / 2;

    nodes.forEach((node, i) => {
      const angle = (i / nodes.length) * 2 * Math.PI;
      node.x = cx + radius * Math.cos(angle);
      node.y = cy + radius * Math.sin(angle);
    });

    nodes.forEach(node => {
      node.connections.forEach(i => {
        const t = nodes[i];
        if (!t) return;

        svg.append("line")
          .attr("x1", node.x)
          .attr("y1", node.y)
          .attr("x2", t.x)
          .attr("y2", t.y)
          .attr("stroke", "#00ffff22");
      });
    });

    svg.selectAll("circle")
      .data(nodes)
      .enter()
      .append("circle")
      .attr("cx", d => d.x)
      .attr("cy", d => d.y)
      .attr("r", 10)
      .attr("fill", d => getColor(d))
      .style("cursor", "pointer")
      .on("click", (_, d) => setSelectedId(d.id));

    svg.selectAll("text")
      .data(nodes)
      .enter()
      .append("text")
      .attr("x", d => d.x)
      .attr("y", d => d.y + 3)
      .attr("text-anchor", "middle")
      .attr("fill", "#000")
      .style("font-size", "9px")
      .text(d => d.id);

  }, [nodes]);

  // =========================
  // UI
  // =========================
  return (
    <div style={{ display: "flex", padding: "20px", color: "white" }}>
      <div>
        <h1>FLUXLOCK NETWORK GRAPH</h1>
        <h3>Node Count: {nodes.length}</h3>
        <svg ref={svgRef} style={{ background: "#020c1b" }} />
      </div>

      {selected && (
        <div style={{
          marginLeft: "40px",
          padding: "20px",
          width: "340px",
          background: "#111827",
          borderRadius: "10px"
        }}>
          <h2>Validator {selected.id}</h2>

          <p>🔑 Epoch: {selected.epoch}</p>
          <p>⏱ Age: {selected.epochAge}</p>

          <p>📊 Trust: {selected.trust.toFixed(2)}</p>
          <p>🌪 Drift: {selected.drift.toFixed(2)}</p>
          <p>Status: {selected.status}</p>

          <hr />

          <p>🔑 Current Key: {shortKey(selected.publicKey)}</p>

          <h4>🔗 Identity Chain</h4>
          {selected.identityChain?.map((k, i) => (
            <div key={i}>
              🔑 {shortKey(k.publicKey)} → {k.trust}
              <div style={{ fontSize: "0.7rem", opacity: 0.6 }}>
                sig: {k.signature === "genesis"
                  ? "GENESIS"
                  : k.signature
                  ? shortKey(k.signature)
                  : "pending"}
              </div>

              {k.invalidSignature && (
                <div style={{ color: "red", fontSize: "0.7rem" }}>
                  INVALID SIGNATURE
                </div>
              )}
            </div>
          ))}

          <hr />

          <h4>🧪 Chain Validation</h4>
          <p style={{
            color:
              selected.chainReason === "awaiting cryptographic signature"
                ? "#ffaa00"
                : selected.chainValid
                ? "#00ff99"
                : "#ff4d4d",
            fontWeight: "bold"
          }}>
            {
              selected.chainReason === "awaiting cryptographic signature"
                ? "PENDING"
                : selected.chainValid
                ? "VALID"
                : "INVALID"
            }
          </p>

          <p>{selected.chainReason}</p>

          <hr />

          <h3>⚔️ Attack Panel</h3>

          <button onClick={spikeAttack}>⚡ Spike Attack</button>
          <br /><br />

          <button onClick={criticalBreach}>☠️ Critical Breach</button>
          <br /><br />

          <button onClick={networkAttack}>🌊 Network Attack</button>

          <br /><br />
          <button onClick={() => setSelectedId(null)}>Close</button>
        </div>
      )}
    </div>
  );
}