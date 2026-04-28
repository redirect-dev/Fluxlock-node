import React, { useEffect, useRef } from "react";

export default function FluxlockVisualizer({ node, nodes, authTrigger, authData }) {
  const canvasRef = useRef(null);
  const consensusRef = useRef([]);

  useEffect(() => {
    if (!nodes || nodes.length === 0) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");

    const GRAPH_SIZE = 800;
    canvas.width = GRAPH_SIZE;
    canvas.height = GRAPH_SIZE;

    let frame;

    const centerX = GRAPH_SIZE / 2;
    const centerY = GRAPH_SIZE / 2;
    const radius = 300;

    // =========================
    // 📍 NODE POSITION
    // =========================
    const getNodePosition = (n, index, total) => {
      const angle = (index / total) * Math.PI * 2;
      const driftOffset = (n.drift || 0) * 0.5;

      return {
        x: centerX + (radius + driftOffset) * Math.cos(angle),
        y: centerY + (radius + driftOffset) * Math.sin(angle),
      };
    };

    // =========================
    // ⚡ AUTH → BUILD WEIGHTED WAVE
    // =========================
    if (authTrigger && authData) {
      consensusRef.current = [];

      const confidence = authData.confidence || 0;

      nodes.forEach((n, i) => {
        const trust = n.trust || 0;
        const drift = n.drift || 0;

        // 🔥 strength formula
        let strength = (trust / 100) * confidence - drift * 0.2;
        strength = Math.max(0.05, Math.min(1, strength));

        // 🔥 delay influenced by trust
        let delay = i * 35 - trust * 0.15;

        // recovering adds instability
        if (authData.resolvedStatus === "recovering") {
          delay += Math.random() * 30;
        }

        // denied breaks early + weakens
        if (authData.resolvedStatus === "denied" && i > nodes.length * 0.4) {
          return;
        }

        consensusRef.current.push({
          id: n.id,
          delay,
          strength,
          alpha: 1,
          active: false,
          start: performance.now(),
        });
      });
    }

    // =========================
    // 🌐 NODE FIELD (unchanged base)
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

      ctx.beginPath();
      ctx.arc(x, y, base + trust * 0.25, 0, Math.PI * 2);
      ctx.stroke();

      ctx.beginPath();
      ctx.arc(x, y, base + 35 + drift * 1.2, 0, Math.PI * 2);
      ctx.stroke();

      ctx.beginPath();
      ctx.arc(x, y, base + 70 + (epoch % 50), 0, Math.PI * 2);
      ctx.stroke();

      ctx.shadowBlur = 0;
    };

    // =========================
    // ⚡ WEIGHTED CONSENSUS DRAW
    // =========================
    const drawConsensus = () => {
      const now = performance.now();

      consensusRef.current.forEach((c) => {
        const n = nodes.find(n => n.id === c.id);
        if (!n) return;

        const index = nodes.findIndex(n2 => n2.id === c.id);
        const { x, y } = getNodePosition(n, index, nodes.length);

        if (now - c.start > c.delay) {
          c.active = true;
        }

        if (!c.active) return;

        // 🎨 color by state
        let color = "0,255,255";
        if (authData?.resolvedStatus === "recovering") color = "255,200,0";
        if (authData?.resolvedStatus === "denied") {
          color = "255,60,60";
          c.alpha *= 0.95;
        }

        const strength = c.strength;

        // 🔥 dynamic visuals
        const radius = 18 + strength * 20;
        const alpha = 0.3 + strength * 0.7;

        // glow
        ctx.beginPath();
        ctx.arc(x, y, radius + 6, 0, Math.PI * 2);
        ctx.fillStyle = `rgba(${color}, ${0.08 * alpha})`;
        ctx.fill();

        // core ring
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.strokeStyle = `rgba(${color}, ${alpha})`;
        ctx.lineWidth = 2 + strength * 1.5;
        ctx.stroke();
      });

      // cleanup fade
      consensusRef.current = consensusRef.current.filter(c => c.alpha > 0.05);
    };

    // =========================
    // 🎬 MAIN LOOP
    // =========================
    const draw = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      nodes.forEach((n, i) => drawField(n, i));
      drawConsensus();

      frame = requestAnimationFrame(draw);
    };

    draw();

    return () => cancelAnimationFrame(frame);
  }, [node, nodes, authTrigger, authData]);

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