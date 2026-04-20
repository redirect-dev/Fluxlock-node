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

    // ---------------- BUILD NODE POSITIONS ----------------
    const nodes = validators.map((v, i) => {
      const angle = (i / validators.length) * 2 * Math.PI;

      return {
        ...v,
        x: centerX + radius * Math.cos(angle),
        y: centerY + radius * Math.sin(angle),
      };
    });

    // ---------------- LINKS ----------------
    const linkGroup = svg.append("g");

    nodes.forEach((source, i) => {
      nodes.slice(i + 1).forEach((target) => {
        linkGroup.append("line")
          .attr("x1", source.x)
          .attr("y1", source.y)
          .attr("x2", target.x)
          .attr("y2", target.y)
          .attr("stroke", "#00ffff22");
      });
    });

    // ---------------- NODE GROUP ----------------
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
          onSelectNode(d.id); // ✅ FIXED
        }
      });

    // ---------------- NODE CIRCLE ----------------
    nodeEnter.append("circle")
      .attr("r", 12)
      .attr("fill", d => {
        if (d.status === "attacked") return "#ff4d4d";
        if (d.status === "warning") return "#ffaa00";
        if (d.status === "healthy") return "#00ff88";
        return "#3399ff";
      })
      .attr("stroke", "#ffffff22")
      .attr("stroke-width", 1);

    // ---------------- HOVER EFFECT ----------------
    nodeEnter
      .on("mouseover", function () {
        d3.select(this).select("circle")
          .transition()
          .duration(150)
          .attr("r", 16);
      })
      .on("mouseout", function () {
        d3.select(this).select("circle")
          .transition()
          .duration(150)
          .attr("r", 12);
      });

    // ---------------- LABELS ----------------
    nodeEnter.append("text")
      .attr("text-anchor", "middle")
      .attr("dy", "0.35em")
      .attr("fill", "white")
      .attr("font-size", "10px")
      .attr("font-weight", "bold")
      .text(d => d.id);

  }, [validators, onSelectNode]);

  // ---------------- FALLBACK ----------------
  if (!validators || validators.length === 0) {
    return (
      <div style={{ color: "white", padding: 20 }}>
        No validators loaded
      </div>
    );
  }

  return (
    <svg
      ref={svgRef}
      width={800}
      height={800}
      style={{ background: "#020c1b" }}
    />
  );
};

export default IdentityGraph;