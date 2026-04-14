import { useEffect, useState } from "react";
import * as d3 from "d3";
import "./index.css";
import { createNetwork, simulateStep } from "./trustEngine";
import {
  ensureAllEpochs,
  runEpoch,
  validateEpochs,
} from "./epochs";

function App() {
  const [nodes, setNodes] = useState(() =>
    ensureAllEpochs(createNetwork())
  );
  const [selectedId, setSelectedId] = useState(null);

  useEffect(() => {
    const interval = setInterval(() => {
      setNodes((prev) => {
        let updated = simulateStep(prev);

        updated = ensureAllEpochs(updated);
        updated = runEpoch(updated);

        // 🔥 VALIDATION STEP
        updated = validateEpochs(updated);

        return updated;
      });
    }, 1500);

    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    const svg = d3.select("#graph");
    svg.selectAll("*").remove();

    const width = 800;
    const height = 600;
    const radius = 220;

    nodes.forEach((node, i) => {
      const angle = (i / nodes.length) * 2 * Math.PI;
      node.x = width / 2 + radius * Math.cos(angle);
      node.y = height / 2 + radius * Math.sin(angle);
    });

    nodes.forEach((node) => {
      node.connections.forEach((targetId) => {
        const target = nodes[targetId];
        if (!target) return;

        svg.append("line")
          .attr("class", "link")
          .attr("x1", node.x)
          .attr("y1", node.y)
          .attr("x2", target.x)
          .attr("y2", target.y);
      });
    });

    const nodeSelection = svg.selectAll(".node")
      .data(nodes)
      .enter()
      .append("circle")
      .attr("class", "node")
      .attr("cx", d => d.x)
      .attr("cy", d => d.y)
      .attr("r", 10)
      .attr("fill", d => {
        if (!d.epochValid) return "#ff00ff"; // 🔥 INVALID = PURPLE
        if (d.status === "attacked") return "#ef4444";
        if (d.status === "drifting") return "#f97316";
        if (d.status === "warning") return "#facc15";
        return "#22c55e";
      });

    nodeSelection.on("click", function (event, d) {
      setSelectedId(d.id);
    });

    svg.selectAll(".label")
      .data(nodes)
      .enter()
      .append("text")
      .attr("x", d => d.x)
      .attr("y", d => d.y + 4)
      .attr("text-anchor", "middle")
      .attr("font-size", "10px")
      .attr("fill", "black")
      .text(d => d.id);

  }, [nodes]);

  const selectedNode = nodes.find(n => n.id === selectedId);

  return (
    <div>
      <h1>FLUXLOCK NETWORK GRAPH</h1>
      <svg id="graph" width="800" height="600"></svg>

      {selectedNode && (
        <div className="panel">
          <h2>Validator {selectedNode.id}</h2>
          <p>Trust: {selectedNode.trust.toFixed(2)}</p>
          <p>Drift: {selectedNode.drift.toFixed(2)}</p>
          <p>Status: {selectedNode.status}</p>
          <p>Influence: {selectedNode.influence.toFixed(2)}</p>

          <hr />

          <p>Epoch ID: {selectedNode.epochId}</p>
          <p>Epoch Age: {selectedNode.epochAge}</p>
          <p>Epoch Weight: {selectedNode.epochWeight.toFixed(4)}</p>

          <p><strong>Epoch Key:</strong></p>
          <p style={{ wordBreak: "break-all", fontSize: "12px" }}>
            {selectedNode.epochKey}
          </p>

          <p>
            <strong>Valid:</strong>{" "}
            {selectedNode.epochValid ? "YES ✅" : "NO ❌"}
          </p>

          <button onClick={() => setSelectedId(null)}>Close</button>
        </div>
      )}
    </div>
  );
}

export default App;