import React, { useEffect, useRef } from "react";
import * as d3 from "d3";

const IdentityGraph = ({ validators, onSelectNode }) => {
  const svgRef = useRef();

  useEffect(() => {
    if (!validators || validators.length === 0) return;

    const width = 800;
    const height = 800;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    const centerX = width / 2;
    const centerY = height / 2;
    const radius = 300;

    // ================= BUILD NODE POSITIONS =================
    const nodes = validators.map((v, i) => {
      const angle = (i / validators.length) * 2 * Math.PI;

      // 🔥 DRIFT DISTORTION (instability visualization)
      const instabilityOffset = (v.drift || 0) * 0.5;

      return {
        ...v,
        x: centerX + (radius + instabilityOffset) * Math.cos(angle),
        y: centerY + (radius + instabilityOffset) * Math.sin(angle),
      };
    });

    // ================= LINKS (INFLUENCE) =================
    const linkGroup = svg.append("g");

    nodes.forEach((source, i) => {
      nodes.slice(i + 1).forEach((target) => {

        // 🔥 TRUST-BASED STRENGTH
        const strength = ((source.trust || 0) + (target.trust || 0)) / 200;

        linkGroup.append("line")
          .attr("x1", source.x)
          .attr("y1", source.y)
          .attr("x2", target.x)
          .attr("y2", target.y)
          .attr("stroke", "#00ffff")
          .attr("stroke-opacity", 0.05 + strength * 0.25)
          .attr("stroke-width", 0.5 + strength * 1.5);
      });
    });

    // ================= NODE GROUP =================
    const nodeGroup = svg.append("g");

    const nodeEnter = nodeGroup
      .selectAll("g.node")
      .data(nodes)
      .enter()
      .append("g")
      .attr("class", "node")
      .attr("transform", d => `translate(${d.x}, ${d.y})`)
      .style("cursor", "pointer")
      .on("click", (event, d) => {
        if (onSelectNode) {
          onSelectNode(d.id);
        }
      });

    // ================= NODE CIRCLE =================
    nodeEnter.append("circle")
      .attr("r", 12)
      .attr("fill", d => getNodeColor(d))
      .attr("stroke", "#ffffff22")
      .attr("stroke-width", 1)
      .style("filter", d => getGlow(d));

    // ================= RECOVERY PULSE =================
    nodeEnter.each(function (d) {
      if (d.recovery_timer > 0) {
        const circle = d3.select(this).select("circle");

        function pulse() {
          circle
            .transition()
            .duration(800)
            .attr("r", 16)
            .transition()
            .duration(800)
            .attr("r", 12)
            .on("end", pulse);
        }

        pulse();
      }
    });

    // ================= ATTACK / REJECT FLICKER =================
    nodeEnter.each(function (d) {
      if (d.decision === "REJECT" || d.status === "attacked") {
        const circle = d3.select(this).select("circle");

        function flicker() {
          circle
            .transition()
            .duration(100)
            .style("opacity", 0.3)
            .transition()
            .duration(100)
            .style("opacity", 1)
            .on("end", flicker);
        }

        flicker();
      }
    });

    // ================= HOVER EFFECT =================
    nodeEnter
      .on("mouseover", function () {
        d3.select(this).select("circle")
          .transition()
          .duration(150)
          .attr("r", 18);
      })
      .on("mouseout", function () {
        d3.select(this).select("circle")
          .transition()
          .duration(150)
          .attr("r", 12);
      });

    // ================= LABELS =================
    nodeEnter.append("text")
      .attr("text-anchor", "middle")
      .attr("dy", "0.35em")
      .attr("fill", "white")
      .attr("font-size", "10px")
      .attr("font-weight", "bold")
      .text(d => d.id);

  }, [validators, onSelectNode]);

  // ================= FALLBACK =================
  if (!validators || validators.length === 0) {
    return (
      <div style={{ color: "white", padding: 20 }}>
        No validators loaded
      </div>
    );
  }

  return (
   <svg
       id="identity-graph"
       ref={svgRef}
      width={800}
      height={800}
      style={{ background: "#020c1b" }}
    />
  );
};

export default IdentityGraph;

// ================= HELPERS =================

function getNodeColor(d) {
  if (d.decision === "REJECT") return "#ff4d4d";
  if (d.decision === "WEIGHTED") return "#ffaa00";
  if (d.decision === "ACCEPT") return "#00ff88";

  if (d.status === "attacked") return "#ff4d4d";
  if (d.status === "warning") return "#ffaa00";
  if (d.status === "healthy") return "#00ff88";

  return "#3399ff";
}

function getGlow(d) {
  if (d.decision === "ACCEPT") return "drop-shadow(0 0 8px #00ff88)";
  if (d.decision === "WEIGHTED") return "drop-shadow(0 0 8px #ffaa00)";
  if (d.decision === "REJECT") return "drop-shadow(0 0 8px #ff4d4d)";

  if (d.status === "attacked") return "drop-shadow(0 0 6px #ff4d4d)";
  if (d.status === "warning") return "drop-shadow(0 0 6px #ffaa00)";
  if (d.status === "healthy") return "drop-shadow(0 0 6px #00ff88)";

  return "none";
}