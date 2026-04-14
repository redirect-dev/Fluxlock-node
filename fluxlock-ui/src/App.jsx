import { useEffect, useState } from "react";
import * as d3 from "d3";
import "./index.css";
import { createNetwork, simulateStep } from "./trustEngine";

function App() {
  const [nodes, setNodes] = useState(() => createNetwork());
  const [selectedNode, setSelectedNode] = useState(null);

  // simulation loop
  useEffect(() => {
    const interval = setInterval(() => {
      setNodes((prev) => simulateStep(prev));
    }, 1500);

    return () => clearInterval(interval);
  }, []);

  // render graph
  useEffect(() => {
    const svg = d3.select("#graph");
    svg.selectAll("*").remove();

    const width = 800;
    const height = 600;
    const radius = 220;

    // position nodes
    nodes.forEach((node, i) => {
      const angle = (i / nodes.length) * 2 * Math.PI;
      node.x = width / 2 + radius * Math.cos(angle);
      node.y = height / 2 + radius * Math.sin(angle);
    });

    // links
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

    // nodes
    const nodeSelection = svg.selectAll(".node")
      .data(nodes)
      .enter()
      .append("circle")
      .attr("class", "node")
      .attr("cx", d => d.x)
      .attr("cy", d => d.y)
      .attr("r", 10)
      .attr("fill", d => {
        if (d.status === "attacked") return "#ef4444";
        if (d.status === "drifting") return "#f97316";
        if (d.status === "warning") return "#facc15";
        return "#22c55e";
      });

    // 🔥 FIXED CLICK HANDLER
    nodeSelection.on("click", function (event, d) {
      console.log("CLICKED NODE:", d); // debug
      setSelectedNode({ ...d }); // force React update
    });

    // labels
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
          <button onClick={() => setSelectedNode(null)}>Close</button>
        </div>
      )}
    </div>
  );
}

export default App;