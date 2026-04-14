// src/App.jsx

import { useEffect, useState } from "react";
import "./index.css";
import { createInitialNodes, simulateStep } from "./trustEngine";

function App() {
  const [nodes, setNodes] = useState(() => createInitialNodes(20));
  const [selectedId, setSelectedId] = useState(null);

  useEffect(() => {
    const interval = setInterval(() => {
      setNodes(prev => simulateStep(prev));
    }, 700); // slightly faster for visible waves

    return () => clearInterval(interval);
  }, []);

  const radius = 220;
  const centerX = 400;
  const centerY = 300;

  const positionedNodes = nodes.map((node, i) => {
    const angle = (i / nodes.length) * Math.PI * 2;

    return {
      ...node,
      x: centerX + radius * Math.cos(angle),
      y: centerY + radius * Math.sin(angle),
    };
  });

  const selectedNode = nodes.find(n => n.id === selectedId);

  return (
    <div className="app">
      <h1>FLUXLOCK NETWORK GRAPH</h1>

      <svg width="800" height="600">
        {/* connections */}
        {positionedNodes.map(node =>
          node.connections.map(targetId => {
            const target = positionedNodes[targetId];
            if (!target) return null;

            return (
              <line
                key={`${node.id}-${targetId}`}
                x1={node.x}
                y1={node.y}
                x2={target.x}
                y2={target.y}
                className="link"
              />
            );
          })
        )}

        {/* nodes */}
        {positionedNodes.map(node => {
          let color = "#22c55e";

          if (node.status === "warning") color = "#facc15";
          if (node.status === "drifting") color = "#f97316";
          if (node.status === "attacked") color = "#ef4444";

          // size reflects BOTH drift and momentum
          const size = Math.min(30, 8 + (node.drift + node.momentum) * 0.12);

          return (
            <g
              key={node.id}
              onClick={() => setSelectedId(node.id)}
              style={{ cursor: "pointer" }}
            >
              <circle
                cx={node.x}
                cy={node.y}
                r={size}
                fill={color}
                className="node"
              />

              <text
                x={node.x}
                y={node.y + 4}
                textAnchor="middle"
                fontSize="10"
                fill="black"
              >
                {node.id}
              </text>
            </g>
          );
        })}
      </svg>

      {/* side panel */}
      {selectedNode && (
        <div className="panel">
          <h2>Validator {selectedNode.id}</h2>
          <p>Trust: {selectedNode.trust.toFixed(2)}</p>
          <p>Drift: {selectedNode.drift.toFixed(2)}</p>
          <p>Status: {selectedNode.status}</p>
          <p>Influence: {selectedNode.influence.toFixed(2)}</p>
          <button onClick={() => setSelectedId(null)}>Close</button>
        </div>
      )}
    </div>
  );
}

export default App;