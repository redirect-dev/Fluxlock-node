import React, { useEffect, useRef } from "react";
import * as d3 from "d3";

const IdentityGraph = ({ validators }) => {
  const svgRef = useRef();

  useEffect(() => {
    const width = 800;
    const height = 800;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    const centerX = width / 2;
    const centerY = height / 2;
    const radius = 300;

    const nodes = validators.map((v, i) => {
      const angle = (i / validators.length) * 2 * Math.PI;

      return {
        ...v,
        x: centerX + radius * Math.cos(angle),
        y: centerY + radius * Math.sin(angle),
      };
    });

    // Draw links (full mesh)
    nodes.forEach((source) => {
      nodes.forEach((target) => {
        svg.append("line")
          .attr("x1", source.x)
          .attr("y1", source.y)
          .attr("x2", target.x)
          .attr("y2", target.y)
          .attr("stroke", "#00ffff22");
      });
    });

    // Draw nodes
    svg.selectAll("circle")
      .data(nodes)
      .enter()
      .append("circle")
      .attr("cx", (d) => d.x)
      .attr("cy", (d) => d.y)
      .attr("r", 12)
      .attr("fill", (d) => {
        if (d.valid === false) return "red";
        if (d.valid === true) return "green";
        return "blue";
      });

    // Labels
    svg.selectAll("text")
      .data(nodes)
      .enter()
      .append("text")
      .attr("x", (d) => d.x)
      .attr("y", (d) => d.y - 15)
      .attr("text-anchor", "middle")
      .attr("fill", "white")
      .text((d) => d.id);

  }, [validators]);

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