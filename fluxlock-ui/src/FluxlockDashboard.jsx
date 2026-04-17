import { useEffect, useState, useRef } from "react";
import * as d3 from "d3";
import { createNetwork, simulateStep } from "./trustEngine";

function getColor(node) {
  if (node.compromised) return "#ff00ff";
  if (node.status === "attacked") return "#ff4d4d";
  if (node.status === "drifting") return "#ffaa00";
  if (node.status === "warning") return "#ffd633";
  return "#00ffaa";
}

export default function FluxlockDashboard() {
  const [nodes, setNodes] = useState([]);
  const [selected, setSelected] = useState(null);
  const svgRef = useRef();

  // INIT
  useEffect(() => {
    const initial = createNetwork(20);
    setNodes(initial);
  }, []);

  // SIM LOOP
  useEffect(() => {
    const interval = setInterval(() => {
      setNodes(prev => simulateStep(prev));
    }, 300);

    return () => clearInterval(interval);
  }, []);

  // ATTACKS
  const spikeAttack = () => {
    setNodes(prev =>
      prev.map(n =>
        n.id === selected.id
          ? { ...n, drift: n.drift + 60, trust: n.trust - 25 }
          : n
      )
    );
  };

  const criticalBreach = () => {
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

  // GRAPH
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

    // edges
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

    // nodes
    svg.selectAll("circle")
      .data(nodes)
      .enter()
      .append("circle")
      .attr("cx", d => d.x)
      .attr("cy", d => d.y)
      .attr("r", 14)
      .attr("fill", d => getColor(d))
      .style("cursor", "pointer")
      .on("click", (_, d) => {
        setSelected(d);
      });

    // labels
    svg.selectAll("text")
      .data(nodes)
      .enter()
      .append("text")
      .attr("x", d => d.x)
      .attr("y", d => d.y + 4)
      .attr("text-anchor", "middle")
      .attr("fill", "#000")
      .style("font-size", "10px")
      .text(d => d.id);

  }, [nodes]);

  return (
    <div style={{ display: "flex", padding: "20px", color: "white" }}>
      <div>
        <h1>FLUXLOCK NETWORK GRAPH</h1>
        <h3>Node Count: {nodes.length}</h3>
        <svg ref={svgRef}></svg>
      </div>

      {selected && (
        <div style={{
          marginLeft: "40px",
          padding: "20px",
          width: "320px",
          background: "#111827",
          borderRadius: "10px"
        }}>
          <h2>Validator {selected.id}</h2>

          {/* CORE STATE */}
          <p>🔑 Epoch: {selected.epoch}</p>
          <p>⏱ Age: {selected.epochAge}</p>

          <p>📊 Trust: {selected.trust.toFixed(2)}</p>
          <p>🌪 Drift: {selected.drift.toFixed(2)}</p>
          <p>Status: {selected.status}</p>

          {selected.compromised && (
            <p style={{ color: "#ff00ff" }}>⚠️ COMPROMISED</p>
          )}

          {/* 🔥 KEY + CHAIN (CORRECT PLACEMENT) */}
          <hr />

          <p>🔑 Current Key: {selected.key}</p>

          <h4>🔗 Identity Chain</h4>
          {selected.identityChain?.map((k, i) => (
            <div key={i}>
              🔑 {k.key} → {k.trust}
            </div>
          ))}

          <hr />

          {/* ATTACK PANEL */}
          <h3>⚔️ Attack Panel</h3>

          <button onClick={spikeAttack}>⚡ Spike Attack</button>
          <br /><br />

          <button onClick={criticalBreach}>☠️ Critical Breach</button>
          <br /><br />

          <button onClick={networkAttack}>🌊 Network Attack</button>

          <br /><br />
          <button onClick={() => setSelected(null)}>Close</button>
        </div>
      )}
    </div>
  );
}