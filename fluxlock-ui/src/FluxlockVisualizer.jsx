import React, { useEffect, useRef } from "react";

export default function FluxlockVisualizer({ node, nodes, authTrigger, authData }) {
  const canvasRef = useRef(null);
  const pulsesRef = useRef([]);
  const waveRef = useRef(null);

  useEffect(() => {
    if (!nodes || nodes.length === 0) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");

    const GRAPH_SIZE = 800;

    const resize = () => {
      canvas.width = GRAPH_SIZE;
      canvas.height = GRAPH_SIZE;
    };

    resize();

    let frame;

    // =========================
    // 📍 SAME AS GRAPH
    // =========================
    const centerX = GRAPH_SIZE / 2;
    const centerY = GRAPH_SIZE / 2;
    const radius = 300;

    const getNodePosition = (n, index, total) => {
      const angle = (index / total) * Math.PI * 2;

      return {
        x: centerX + radius * Math.cos(angle),
        y: centerY + radius * Math.sin(angle),
      };
    };

    // =========================
    // 🔥 TRIGGER WAVE
    // =========================
    if (authTrigger && node) {
      const index = nodes.findIndex(n => n.id === node.id);

      if (index !== -1) {
        const origin = getNodePosition(node, index, nodes.length);

        waveRef.current = {
          x: origin.x,
          y: origin.y,
          radius: 0,
          max: 600,
          alpha: 1,
        };

        // core pulse
        pulsesRef.current.push({
          x: origin.x,
          y: origin.y,
          radius: 0,
          alpha: 1,
        });
      }
    }

    // =========================
    // 🌈 COLOR FROM AUTH
    // =========================
    const getAuthColor = () => {
      if (!authData) return "0,255,200";

      if (authData.resolvedStatus === "healthy") return "0,255,200";
      if (authData.resolvedStatus === "recovering") return "255,170,0";
      if (authData.resolvedStatus === "denied") return "255,80,80";

      return "0,255,200";
    };

    // =========================
    // 🌐 NODE FIELD REACTION
    // =========================
    const drawNodeField = (n, index) => {
      const { x, y } = getNodePosition(n, index, nodes.length);

      const isSelected = node && n.id === node.id;

      const baseColor = getAuthColor();

      // distance from wave origin
      let reaction = 0;

      if (waveRef.current) {
        const dx = x - waveRef.current.x;
        const dy = y - waveRef.current.y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        const diff = Math.abs(dist - waveRef.current.radius);

        if (diff < 40) {
          reaction = 1 - diff / 40; // smooth falloff
        }
      }

      const intensity = isSelected ? 1 : 0.2 + reaction * 0.8;

      ctx.strokeStyle = `rgba(${baseColor}, ${0.4 * intensity})`;
      ctx.shadowColor = `rgba(${baseColor}, ${0.6 * intensity})`;
      ctx.lineWidth = isSelected ? 2.5 : 1;
      ctx.shadowBlur = isSelected ? 25 : 8 + reaction * 20;

      const base = isSelected ? 40 : 18;

      ctx.beginPath();
      ctx.arc(x, y, base + reaction * 30, 0, Math.PI * 2);
      ctx.stroke();

      ctx.shadowBlur = 0;
    };

    // =========================
    // ⚡ DRAW CORE PULSE
    // =========================
    const drawPulses = () => {
      pulsesRef.current.forEach((p) => {
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
        ctx.strokeStyle = `rgba(0,255,255,${p.alpha})`;
        ctx.lineWidth = 2.5;
        ctx.stroke();

        p.radius += 5;
        p.alpha *= 0.96;
      });

      pulsesRef.current = pulsesRef.current.filter(p => p.alpha > 0.05);
    };

    // =========================
    // 🌊 DRAW WAVE
    // =========================
    const drawWave = () => {
      if (!waveRef.current) return;

      const w = waveRef.current;

      const color = getAuthColor();

      ctx.beginPath();
      ctx.arc(w.x, w.y, w.radius, 0, Math.PI * 2);
      ctx.strokeStyle = `rgba(${color}, ${w.alpha * 0.25})`;
      ctx.lineWidth = 8;
      ctx.stroke();

      ctx.beginPath();
      ctx.arc(w.x, w.y, w.radius, 0, Math.PI * 2);
      ctx.strokeStyle = `rgba(${color}, ${w.alpha})`;
      ctx.lineWidth = 2;
      ctx.stroke();

      w.radius += 6;
      w.alpha *= 0.97;

      if (w.radius > w.max || w.alpha < 0.05) {
        waveRef.current = null;
      }
    };

    // =========================
    // 🎬 LOOP
    // =========================
    const draw = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      nodes.forEach((n, i) => {
        drawNodeField(n, i);
      });

      drawWave();
      drawPulses();

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