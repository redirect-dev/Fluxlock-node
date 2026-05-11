import React, {
  useEffect,
  useRef,
} from "react";

export default function FluxlockVisualizer({
  node,
  nodes,
  authTrigger,
  authData,
}) {

  const canvasRef =
    useRef(null);

  const consensusRef =
    useRef([]);

  const orbitalRef =
    useRef([]);

  useEffect(() => {

    if (
      !nodes ||
      nodes.length === 0
    )
      return;

    const canvas =
      canvasRef.current;

    const ctx =
      canvas.getContext("2d");

    const GRAPH_SIZE = 800;

    canvas.width =
      GRAPH_SIZE;

    canvas.height =
      GRAPH_SIZE;

    let frame;

    const centerX =
      GRAPH_SIZE / 2;

    const centerY =
      GRAPH_SIZE / 2;

    const radius = 300;

    // =========================
    // 📍 NODE POSITION
    // =========================
    const getNodePosition = (
      n,
      index,
      total
    ) => {

      const angle =
        (index / total) *
        Math.PI *
        2;

      const driftOffset =
        (n.drift || 0) * 0.5;

      return {

        x:
          centerX +
          (radius + driftOffset)
            * Math.cos(angle),

        y:
          centerY +
          (radius + driftOffset)
            * Math.sin(angle),
      };
    };

    // =========================
    // 🌊 BUILD CONSENSUS
    // =========================
    if (
      authTrigger &&
      authData
    ) {

      consensusRef.current = [];
      orbitalRef.current = [];

      const confidence =
        authData.confidence || 0;

      nodes.forEach((n, i) => {

        const trust =
          n.trust || 0;

        const drift =
          n.drift || 0;

        let strength =
          (trust / 100)
          * confidence
          - drift * 0.2;

        strength =
          Math.max(
            0.05,
            Math.min(1, strength)
          );

        consensusRef.current.push({

          id: n.id,

          delay:
            i * 30,

          strength,

          alpha: 1,

          active: false,

          start:
            performance.now(),
        });

        // 🔥 ORBITALS
        orbitalRef.current.push({

          id: n.id,

          angle:
            Math.random()
            * Math.PI * 2,

          speed:
            0.004
            + Math.random()
            * 0.01,

          radius:
            30
            + Math.random()
            * 40,

          size:
            2
            + Math.random()
            * 4,
        });
      });
    }

    // =========================
    // 🧠 STATUS VISUALS
    // =========================
    const getLifecycleVisuals =
      (status) => {

      switch (status) {

        case "emerging":

          return {
            scale: 1,
            glow: 10,
            pulse: 1,
          };

        case "maturing":

          return {
            scale: 1.4,
            glow: 18,
            pulse: 1.5,
          };

        case "established":

          return {
            scale: 1.8,
            glow: 28,
            pulse: 2,
          };

        case "sovereign":

          return {
            scale: 2.5,
            glow: 40,
            pulse: 3,
          };

        case "recovering":

          return {
            scale: 1.2,
            glow: 12,
            pulse: 0.6,
          };

        default:

          return {
            scale: 1,
            glow: 8,
            pulse: 1,
          };
      }
    };

    // =========================
    // 🌊 FIELD
    // =========================
    const drawField = (
      n,
      index
    ) => {

      const {
        x,
        y,
      } =
        getNodePosition(
          n,
          index,
          nodes.length
        );

      const isSelected =
        node &&
        n.id === node.id;

      const lifecycle =
        getLifecycleVisuals(
          authData?.status
        );

      const trust =
        n.trust || 0;

      const drift =
        n.drift || 0;

      const pulse =
        Math.sin(
          performance.now()
          * 0.002
          * lifecycle.pulse
        ) * 6;

      let color =
        "0,255,220";

      if (
        authData?.status
        === "recovering"
      ) {

        color =
          "255,180,0";
      }

      ctx.shadowColor =
        `rgba(${color},0.9)`;

      ctx.shadowBlur =
        lifecycle.glow;

      const base =
        isSelected
          ? 55
          : 20;

      // 🔥 MAIN FIELD
      for (
        let i = 0;
        i < 4;
        i++
      ) {

        ctx.beginPath();

        ctx.arc(
          x,
          y,
          base
          + i * 35
          + pulse
          + trust * 0.08
          + drift,
          0,
          Math.PI * 2
        );

        ctx.strokeStyle =
          `rgba(${color},
          ${0.12 - i * 0.02})`;

        ctx.lineWidth =
          2 - i * 0.3;

        ctx.stroke();
      }

      // 🔥 CORE
      ctx.beginPath();

      ctx.arc(
        x,
        y,
        isSelected
          ? 16
          : 10,
        0,
        Math.PI * 2
      );

      ctx.fillStyle =
        `rgba(${color},1)`;

      ctx.fill();

      ctx.shadowBlur = 0;
    };

    // =========================
    // 🔗 ORBITALS
    // =========================
    const drawOrbitals =
      () => {

      orbitalRef.current
        .forEach((o) => {

        const n =
          nodes.find(
            n => n.id === o.id
          );

        if (!n)
          return;

        const index =
          nodes.findIndex(
            n2 => n2.id === o.id
          );

        const {
          x,
          y,
        } =
          getNodePosition(
            n,
            index,
            nodes.length
          );

        o.angle += o.speed;

        const ox =
          x
          + Math.cos(o.angle)
          * o.radius;

        const oy =
          y
          + Math.sin(o.angle)
          * o.radius;

        ctx.beginPath();

        ctx.arc(
          ox,
          oy,
          o.size,
          0,
          Math.PI * 2
        );

        ctx.fillStyle =
          "rgba(0,255,255,0.9)";

        ctx.fill();

        // 🔥 ORBIT TRAIL
        ctx.beginPath();

        ctx.moveTo(x, y);

        ctx.lineTo(ox, oy);

        ctx.strokeStyle =
          "rgba(0,255,255,0.08)";

        ctx.stroke();
      });
    };

    // =========================
    // ⚡ CONSENSUS
    // =========================
    const drawConsensus =
      () => {

      const now =
        performance.now();

      consensusRef.current
        .forEach((c) => {

        const n =
          nodes.find(
            n => n.id === c.id
          );

        if (!n)
          return;

        const index =
          nodes.findIndex(
            n2 => n2.id === c.id
          );

        const {
          x,
          y,
        } =
          getNodePosition(
            n,
            index,
            nodes.length
          );

        if (
          now - c.start
          > c.delay
        ) {

          c.active = true;
        }

        if (!c.active)
          return;

        const radius =
          25
          + c.strength * 28;

        ctx.beginPath();

        ctx.arc(
          x,
          y,
          radius,
          0,
          Math.PI * 2
        );

        ctx.strokeStyle =
          `rgba(
            0,
            255,
            255,
            ${0.5 * c.alpha}
          )`;

        ctx.lineWidth =
          3;

        ctx.stroke();

        c.alpha *= 0.992;
      });

      consensusRef.current =
        consensusRef.current
          .filter(
            c => c.alpha > 0.05
          );
    };

    // =========================
    // 🎬 MAIN LOOP
    // =========================
    const draw = () => {

      ctx.clearRect(
        0,
        0,
        canvas.width,
        canvas.height
      );

      nodes.forEach(
        (n, i) =>
          drawField(n, i)
      );

      drawOrbitals();

      drawConsensus();

      frame =
        requestAnimationFrame(
          draw
        );
    };

    draw();

    return () =>
      cancelAnimationFrame(
        frame
      );

  }, [
    node,
    nodes,
    authTrigger,
    authData,
  ]);

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