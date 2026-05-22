import React, {
  useEffect,
  useRef,
} from "react";

import * as d3 from "d3";

const IdentityGraph = ({
  validators,
  onSelectNode,
}) => {

  const svgRef =
    useRef();

  useEffect(() => {

    if (
      !validators
      || validators.length === 0
    ) return;

    const width = 900;
    const height = 900;

    const svg =
      d3.select(svgRef.current);

    svg.selectAll("*").remove();

    const centerX =
      width / 2;

    const centerY =
      height / 2;

    // =========================
    // 🌐 TOPOLOGY CLUSTERS
    // =========================
    const clusterCenters = [

      { x: 250, y: 250 },
      { x: 650, y: 250 },
      { x: 250, y: 650 },
      { x: 650, y: 650 },
    ];

    const nodes =
      validators.map((v, i) => {

        const cluster =
          v.topology_cluster || 0;

        const center =
          clusterCenters[
            cluster % clusterCenters.length
          ];

        const angle =
          (i / validators.length)
          * Math.PI * 2;

        const instability =
          (v.drift || 0)
          + (v.regional_pressure || 0);

        const radius =
          60
          + instability * 1.5;

        return {

          ...v,

          x:
            center.x
            + Math.cos(angle)
            * radius,

          y:
            center.y
            + Math.sin(angle)
            * radius,
        };
      });

    // =========================
    // 🌊 LINKS
    // =========================
    const linkGroup =
      svg.append("g");

    nodes.forEach((source, i) => {

      nodes.slice(i + 1)
        .forEach((target) => {

        const strength =
          (
            (source.trust || 0)
            + (target.trust || 0)
          ) / 200;

        const entropy =
          (
            (source.entropy_output || 0)
            + (target.entropy_output || 0)
          ) / 2;

        const healing =
          (
            (source.healing_wave || 0)
            + (target.healing_wave || 0)
          ) / 2;

        let color =
          "#00ffff";

        if (entropy > healing) {
          color = "#ff4444";
        }

        if (healing > entropy) {
          color = "#66ffee";
        }

        linkGroup
          .append("line")

          .attr("x1", source.x)
          .attr("y1", source.y)

          .attr("x2", target.x)
          .attr("y2", target.y)

          .attr("stroke", color)

          .attr(
            "stroke-opacity",
            0.05 + strength * 0.3
          )

          .attr(
            "stroke-width",
            0.5 + strength * 2
          );
      });
    });

    // =========================
    // 🌐 NODE GROUP
    // =========================
    const nodeGroup =
      svg.append("g");

    const nodeEnter =
      nodeGroup
        .selectAll("g.node")

        .data(nodes)

        .enter()

        .append("g")

        .attr(
          "class",
          "node"
        )

        .attr(
          "transform",
          d =>
            `translate(${d.x}, ${d.y})`
        )

        .style(
          "cursor",
          "pointer"
        )

        .on(
          "click",
          (event, d) => {

          if (onSelectNode) {
            onSelectNode(d.id);
          }
        });

    // =========================
    // 🌊 INFLUENCE RADIUS
    // =========================
    nodeEnter
      .append("circle")

      .attr(
        "r",
        d =>
          30
          + (
            d.influence_radius
            || 0
          )
      )

      .attr(
        "fill",
        "none"
      )

      .attr(
        "stroke",
        d => {

        if (
          d.status === "fractured"
        ) {
          return "#ff4444";
        }

        if (
          d.status === "immune"
        ) {
          return "#66ffff";
        }

        return "#00ffff";
      })

      .attr(
        "stroke-opacity",
        0.12
      )

      .attr(
        "stroke-width",
        2
      );

    // =========================
    // 🔵 CORE NODE
    // =========================
    nodeEnter
      .append("circle")

      .attr("r", 12)

      .attr(
        "fill",
        d => getNodeColor(d)
      )

      .attr(
        "stroke",
        "#ffffff22"
      )

      .attr(
        "stroke-width",
        1
      )

      .style(
        "filter",
        d => getGlow(d)
      );

    // =========================
    // 🧬 LABELS
    // =========================
    nodeEnter
      .append("text")

      .attr(
        "text-anchor",
        "middle"
      )

      .attr("dy", "0.35em")

      .attr("fill", "white")

      .attr(
        "font-size",
        "10px"
      )

      .attr(
        "font-weight",
        "bold"
      )

      .text(d => d.id);

  }, [
    validators,
    onSelectNode,
  ]);

  return (
    <svg
      ref={svgRef}
      width={900}
      height={900}
      style={{
        background: "#020c1b",
      }}
    />
  );
};

export default IdentityGraph;

// =========================
// 🌈 COLORS
// =========================
function getNodeColor(d) {

  if (
    d.status === "fractured"
  ) {
    return "#ff4444";
  }

  if (
    d.status === "quarantined"
  ) {
    return "#ff8800";
  }

  if (
    d.status === "immune"
  ) {
    return "#66ffff";
  }

  if (
    d.status === "recovering"
  ) {
    return "#ffee66";
  }

  return "#00ff88";
}

// =========================
// ✨ GLOW
// =========================
function getGlow(d) {

  if (
    d.status === "fractured"
  ) {
    return "drop-shadow(0 0 10px #ff4444)";
  }

  if (
    d.status === "immune"
  ) {
    return "drop-shadow(0 0 10px #66ffff)";
  }

  if (
    d.status === "recovering"
  ) {
    return "drop-shadow(0 0 10px #ffaa00)";
  }

  return "drop-shadow(0 0 10px #00ff88)";
}