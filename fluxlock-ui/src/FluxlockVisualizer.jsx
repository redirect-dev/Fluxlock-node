import React, { useEffect, useRef } from "react";

export default function FluxlockVisualizer({ node, nodes, authTrigger }) {
  const canvasRef = useRef(null);
  const pulsesRef = useRef([]);

  useEffect(() => {
    if (!nodes || nodes.length === 0) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");

    // =========================
    // 🔒 MATCH SVG EXACT SIZE
    // =========================
    const GRAPH_SIZE = 800;

    const resize = () => {
      canvas.width = GRAPH_SIZE;
      canvas.height = GRAPH_SIZE;
    };

    resize();

    let frame;

    // =========================
    // 📍 EXACT SAME LAYOUT AS D3
    // =========================
    const centerX = GRAPH_SIZE / 2;
    const centerY = GRAPH_SIZE / 2;
    const radius = 300;

    const getNodePosition = (n, index, total) => {
      const angle = (index / total) * Math.PI * 2;

      const instabilityOffset = (n.drift || 0) * 0.5;

      return {
        x: centerX + (radius + instabilityOffset) * Math.cos(angle),
        y: centerY + (radius + instabilityOffset) * Math.sin(angle),
      };
    };

    // =========================
    // ⚡ AUTH PULSE
    // =========================
    if (authTrigger && node) {
      const index = nodes.findIndex(n => n.id === node.id);

      if (index !== -1) {
        const { x, y } = getNodePosition(node, index, nodes.length);

        pulsesRef.current.push({
          x,
          y,
          radius: 0,
          max: 350,
          alpha: 1,
        });
      }
    }

    // =========================
    // 🌐 CONSENSUS FIELD (NEW)
    // =========================
    const drawConsensusField = () => {
      const total = nodes.length;

      const accept = nodes.filter(n => n.decision === "ACCEPT").length;
      const weighted = nodes.filter(n => n.decision === "WEIGHTED").length;
      const reject = nodes.filter(n => n.decision === "REJECT").length;

      const consensusStrength = (accept + weighted * 0.5) / total;
      const instability = reject / total;

      // 🎨 COLOR SHIFT
      let r = 0, g = 255, b = 200;

      if (instability > 0.3) {
        r = 255; g = 80; b = 80;
      } else if (instability > 0.1) {
        r = 255; g = 170; b = 0;
      }

      // 🌊 BREATHING
      const pulse = Math.sin(Date.now() * 0.002) * 8;

      const baseRadius = 70 + consensusStrength * 140;

      ctx.shadowColor = `rgba(${r},${g},${b},0.5)`;
      ctx.shadowBlur = 35;

      // 🌀 FRACTAL RINGS (3 layers)
      for (let i = 0; i < 3; i++) {
        ctx.beginPath();

        ctx.arc(
          centerX,
          centerY,
          baseRadius + i * 45 + pulse,
          0,
          Math.PI * 2
        );

        ctx.strokeStyle = `rgba(${r},${g},${b},${0.18 - i * 0.04})`;
        ctx.lineWidth = 2;

        ctx.stroke();
      }

      ctx.shadowBlur = 0;
    };

    // =========================
    // 🌐 NODE FIELDS
    // =========================
    const drawField = (n, index) => {
      const { x, y } = getNodePosition(n, index, nodes.length);

      const isSelected = node && n.id === node.id;

      const trust = n.trust || 0;
      const drift = n.drift || 0;
      const epoch = n.epoch_age || 0;

      const baseColor =
        n.status === "healthy"
          ? "0,255,200"
          : n.status === "recovering"
          ? "255,170,0"
          : "255,80,80";

      const intensity = isSelected ? 1 : 0.15;

      ctx.strokeStyle = `rgba(${baseColor}, ${0.5 * intensity})`;
      ctx.shadowColor = `rgba(${baseColor}, ${0.7 * intensity})`;
      ctx.lineWidth = isSelected ? 2 : 1;
      ctx.shadowBlur = isSelected ? 20 : 6;

      const base = isSelected ? 42 : 18;

      // trust
      ctx.beginPath();
      ctx.arc(x, y, base + trust * 0.25, 0, Math.PI * 2);
      ctx.stroke();

      // drift
      ctx.beginPath();
      ctx.arc(x, y, base + 35 + drift * 1.2, 0, Math.PI * 2);
      ctx.stroke();

      // epoch
      ctx.beginPath();
      ctx.arc(x, y, base + 70 + (epoch % 50), 0, Math.PI * 2);
      ctx.stroke();

      ctx.shadowBlur = 0;
    };

    // =========================
    // ⚡ PULSES
    // =========================
    const drawPulses = () => {
      pulsesRef.current.forEach((p) => {
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
        ctx.strokeStyle = `rgba(0,255,255,${p.alpha * 0.2})`;
        ctx.lineWidth = 10;
        ctx.stroke();

        ctx.beginPath();
        ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
        ctx.strokeStyle = `rgba(0,255,255,${p.alpha})`;
        ctx.lineWidth = 2;
        ctx.stroke();

        p.radius += 4;
        p.alpha *= 0.96;
      });

      pulsesRef.current = pulsesRef.current.filter(
        (p) => p.alpha > 0.05 && p.radius < p.max
      );
    };

    // =========================
    // 🎬 MAIN LOOP
    // =========================
    const draw = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // 🔥 GLOBAL CONSENSUS FIELD FIRST
      drawConsensusField();

      // nodes
      nodes.forEach((n, i) => {
        drawField(n, i);
      });

      // pulses
      drawPulses();

      frame = requestAnimationFrame(draw);
    };

    draw();

    return () => cancelAnimationFrame(frame);
  }, [node, nodes, authTrigger]);

  return (
    <canvas
      ref={canvasRef}
      style={{
        position: "absolute",
        left: 0,
        top: 0,
        width: 800,
        height: 800,
        pointerEvents: "none",
        zIndex: 0,
      }}
    />
  );
}