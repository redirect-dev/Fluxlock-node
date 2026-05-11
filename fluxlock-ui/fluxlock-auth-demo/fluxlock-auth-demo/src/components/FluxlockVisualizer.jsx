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

  const orbitalRef =
    useRef([]);

  const pulseFieldRef =
    useRef([]);

  const shockwaveRef =
    useRef([]);

  const scarFieldRef =
    useRef([]);

  const frameRef =
    useRef(null);

  useEffect(() => {

    if (
      !nodes ||
      nodes.length === 0
    ) return;

    const canvas =
      canvasRef.current;

    const ctx =
      canvas.getContext("2d");

    const WIDTH = 900;
    const HEIGHT = 900;

    canvas.width = WIDTH;
    canvas.height = HEIGHT;

    const centerX =
      WIDTH / 2;

    const centerY =
      HEIGHT / 2;

    const networkRadius = 300;

    // =========================
    // 🌐 POSITIONING
    // =========================
    const getNodePosition = (
      validator,
      index,
      total
    ) => {

      const angle =
        (index / total)
        * Math.PI * 2;

      const drift =
        validator.drift || 0;

      const pressure =
        validator.consensus_pressure || 0;

      const instability =
        drift * 1.8
        + pressure * 3;

      const trust =
        validator.trust || 0;

      const resilience =
        validator.resilience_score || 0;

      const coherence =
        trust / 100;

      const breathing =
        Math.sin(
          performance.now()
          * 0.0008
          + index
        ) * 10;

      const radius =
        networkRadius
        + instability
        - (coherence * 18)
        - (resilience * 0.08)
        + breathing;

      return {

        x:
          centerX
          + radius
          * Math.cos(angle),

        y:
          centerY
          + radius
          * Math.sin(angle),
      };
    };

    // =========================
    // 🌊 AUTH EVENT
    // =========================
    if (
      authTrigger &&
      authData
    ) {

      shockwaveRef.current.push({

        radius: 50,

        alpha: 1,

        color:
          authData.status ===
          "recovering"
            ? "255,180,0"
            : "0,255,255",
      });

      nodes.forEach((n) => {

        pulseFieldRef.current.push({

          id: n.id,

          radius: 25,

          alpha: 1,
        });
      });
    }

    // =========================
    // 🪐 ORBITALS
    // =========================
    if (
      orbitalRef.current.length
      !== nodes.length
    ) {

      orbitalRef.current =
        nodes.map((n) => ({

          id: n.id,

          angle:
            Math.random()
            * Math.PI * 2,

          speed:
            0.002
            + Math.random()
            * 0.008,

          orbit:
            35
            + Math.random()
            * 50,

          size:
            1
            + Math.random()
            * 4,
        }));
    }

    // =========================
    // 🎨 COLORS
    // =========================
    const getNodeColor =
      (validator) => {

      const trust =
        validator.trust || 0;

      const drift =
        validator.drift || 0;

      if (
        validator.status ===
        "quarantined"
      ) {

        return {
          core:
            "255,40,40",

          glow:
            "255,0,0",
        };
      }

      if (
        validator.status ===
        "immune"
      ) {

        return {
          core:
            "120,255,255",

          glow:
            "0,255,255",
        };
      }

      if (
        trust < 60 ||
        drift > 20
      ) {

        return {
          core:
            "255,180,0",

          glow:
            "255,180,0",
        };
      }

      return {
        core:
          "0,255,220",

        glow:
          "0,255,255",
      };
    };

    // =========================
    // 🌌 BACKGROUND
    // =========================
    const drawBackgroundField =
      () => {

      const gradient =
        ctx.createRadialGradient(
          centerX,
          centerY,
          80,
          centerX,
          centerY,
          600
        );

      gradient.addColorStop(
        0,
        "rgba(0,255,255,0.04)"
      );

      gradient.addColorStop(
        1,
        "rgba(0,0,0,0)"
      );

      ctx.fillStyle =
        gradient;

      ctx.fillRect(
        0,
        0,
        WIDTH,
        HEIGHT
      );
    };

    // =========================
    // 🌊 COHERENCE FIELD
    // =========================
    const drawCoherenceField =
      () => {

      const avgTrust =
        nodes.reduce(
          (a, n) =>
            a + (n.trust || 0),
          0
        ) / nodes.length;

      const avgDrift =
        nodes.reduce(
          (a, n) =>
            a + (n.drift || 0),
          0
        ) / nodes.length;

      const avgResilience =
        nodes.reduce(
          (a, n) =>
            a + (
              n.resilience_score
              || 0
            ),
          0
        ) / nodes.length;

      const base =
        80
        + avgTrust
        - avgDrift
        + avgResilience * 0.2;

      for (
        let i = 0;
        i < 5;
        i++
      ) {

        ctx.beginPath();

        ctx.arc(
          centerX,
          centerY,
          base + i * 55,
          0,
          Math.PI * 2
        );

        ctx.strokeStyle =
          `rgba(
            0,
            255,
            255,
            ${0.14 - i * 0.02}
          )`;

        ctx.lineWidth =
          3 - i * 0.4;

        ctx.stroke();
      }
    };

    // =========================
    // 🌐 LINKS
    // =========================
    const drawLinks = () => {

      nodes.forEach((a, i) => {

        const posA =
          getNodePosition(
            a,
            i,
            nodes.length
          );

        nodes.forEach((b, j) => {

          if (i >= j)
            return;

          const posB =
            getNodePosition(
              b,
              j,
              nodes.length
            );

          const avgTrust =
            (
              (a.trust || 0)
              + (b.trust || 0)
            ) / 2;

          const avgScar =
            (
              (a.scar_level || 0)
              + (b.scar_level || 0)
            ) / 2;

          let alpha =
            0.05
            + (avgTrust / 100)
            * 0.25;

          alpha -=
            avgScar * 0.03;

          alpha =
            Math.max(
              0.015,
              alpha
            );

          ctx.beginPath();

          ctx.moveTo(
            posA.x,
            posA.y
          );

          ctx.lineTo(
            posB.x,
            posB.y
          );

          ctx.strokeStyle =
            `rgba(
              0,
              255,
              255,
              ${alpha}
            )`;

          ctx.lineWidth =
            0.5
            + (avgTrust / 100)
            * 2;

          ctx.stroke();
        });
      });
    };

    // =========================
    // 🔥 SCARS
    // =========================
    const drawScars = (
      x,
      y,
      scarLevel
    ) => {

      if (scarLevel < 0.2)
        return;

      const count =
        Math.min(
          10,
          Math.floor(
            scarLevel * 2
          )
        );

      for (
        let i = 0;
        i < count;
        i++
      ) {

        const angle =
          (
            Math.PI * 2
          )
          * (i / count);

        const length =
          25
          + scarLevel * 6;

        ctx.beginPath();

        ctx.moveTo(x, y);

        ctx.lineTo(
          x
          + Math.cos(angle)
          * length,

          y
          + Math.sin(angle)
          * length
        );

        ctx.strokeStyle =
          `rgba(
            255,
            60,
            60,
            ${0.12 + scarLevel * 0.05}
          )`;

        ctx.stroke();
      }
    };

    // =========================
    // 🔵 NODE
    // =========================
    const drawNode =
      (
        validator,
        index
      ) => {

      const {
        x,
        y,
      } =
        getNodePosition(
          validator,
          index,
          nodes.length
        );

      const isSelected =
        node &&
        validator.id
        === node.id;

      const {
        core,
        glow,
      } =
        getNodeColor(
          validator
        );

      const trust =
        validator.trust || 0;

      const drift =
        validator.drift || 0;

      const resilience =
        validator.resilience_score || 0;

      const scar =
        validator.scar_level || 0;

      const coherence =
        trust / 100;

      const pulse =
        Math.sin(
          performance.now()
          * 0.003
          + index
        ) * 6;

      // 🌊 OUTER RINGS
      for (
        let r = 0;
        r < 5;
        r++
      ) {

        ctx.beginPath();

        ctx.arc(
          x,
          y,
          35
          + r * 28
          + pulse
          + drift * 0.4
          - resilience * 0.05,
          0,
          Math.PI * 2
        );

        ctx.strokeStyle =
          `rgba(
            ${core},
            ${0.10 - r * 0.015}
          )`;

        ctx.lineWidth =
          2.5 - r * 0.3;

        ctx.stroke();
      }

      // 🛡 IMMUNE HALO
      if (
        validator.status ===
        "immune"
      ) {

        ctx.beginPath();

        ctx.arc(
          x,
          y,
          120
          + pulse,
          0,
          Math.PI * 2
        );

        ctx.strokeStyle =
          "rgba(120,255,255,0.2)";

        ctx.lineWidth = 6;

        ctx.stroke();
      }

      // 🔥 SCARS
      drawScars(
        x,
        y,
        scar
      );

      // 🌟 GLOW
      ctx.shadowColor =
        `rgba(${glow},1)`;

      ctx.shadowBlur =
        15
        + coherence * 22
        + resilience * 0.05;

      // 🔵 CORE
      ctx.beginPath();

      ctx.arc(
        x,
        y,
        isSelected
          ? 18
          : 12
          + coherence * 5,
        0,
        Math.PI * 2
      );

      ctx.fillStyle =
        `rgba(${core},1)`;

      ctx.fill();

      ctx.shadowBlur = 0;
    };

    // =========================
    // 🪐 ORBITALS
    // =========================
    const drawOrbitals =
      () => {

      orbitalRef.current
        .forEach((o) => {

        const validator =
          nodes.find(
            n => n.id === o.id
          );

        if (!validator)
          return;

        const index =
          nodes.findIndex(
            n => n.id === o.id
          );

        const {
          x,
          y,
        } =
          getNodePosition(
            validator,
            index,
            nodes.length
          );

        const drift =
          validator.drift || 0;

        const resilience =
          validator.resilience_score || 0;

        const instability =
          drift * 0.03;

        o.angle +=
          o.speed
          + instability;

        const orbitRadius =
          o.orbit
          + drift * 0.5
          - resilience * 0.03;

        const ox =
          x
          + Math.cos(o.angle)
          * orbitRadius;

        const oy =
          y
          + Math.sin(o.angle)
          * orbitRadius;

        let color =
          "0,255,255";

        if (
          validator.status ===
          "immune"
        ) {

          color =
            "120,255,255";
        }

        if (
          validator.status ===
          "recovering"
        ) {

          color =
            "255,180,0";
        }

        if (
          validator.status ===
          "quarantined"
        ) {

          color =
            "255,60,60";
        }

        ctx.beginPath();

        ctx.arc(
          ox,
          oy,
          o.size,
          0,
          Math.PI * 2
        );

        ctx.fillStyle =
          `rgba(
            ${color},
            0.9
          )`;

        ctx.fill();

        ctx.beginPath();

        ctx.moveTo(x, y);

        ctx.lineTo(ox, oy);

        ctx.strokeStyle =
          `rgba(
            ${color},
            0.08
          )`;

        ctx.stroke();
      });
    };

    // =========================
    // 🌊 PULSES
    // =========================
    const drawPulses =
      () => {

      pulseFieldRef.current
        .forEach((p) => {

        const validator =
          nodes.find(
            n => n.id === p.id
          );

        if (!validator)
          return;

        const index =
          nodes.findIndex(
            n => n.id === p.id
          );

        const {
          x,
          y,
        } =
          getNodePosition(
            validator,
            index,
            nodes.length
          );

        ctx.beginPath();

        ctx.arc(
          x,
          y,
          p.radius,
          0,
          Math.PI * 2
        );

        ctx.strokeStyle =
          `rgba(
            0,
            255,
            255,
            ${p.alpha}
          )`;

        ctx.lineWidth = 2;

        ctx.stroke();

        p.radius += 3;
        p.alpha *= 0.968;
      });

      pulseFieldRef.current =
        pulseFieldRef.current.filter(
          p => p.alpha > 0.03
        );
    };

    // =========================
    // 🌊 SHOCKWAVES
    // =========================
    const drawShockwaves =
      () => {

      shockwaveRef.current
        .forEach((s) => {

        ctx.beginPath();

        ctx.arc(
          centerX,
          centerY,
          s.radius,
          0,
          Math.PI * 2
        );

        ctx.strokeStyle =
          `rgba(
            ${s.color},
            ${s.alpha}
          )`;

        ctx.lineWidth = 4;

        ctx.stroke();

        s.radius += 12;
        s.alpha *= 0.965;
      });

      shockwaveRef.current =
        shockwaveRef.current.filter(
          s => s.alpha > 0.03
        );
    };

    // =========================
    // 🎬 LOOP
    // =========================
    const draw = () => {

      ctx.clearRect(
        0,
        0,
        WIDTH,
        HEIGHT
      );

      drawBackgroundField();

      drawCoherenceField();

      drawLinks();

      nodes.forEach(
        (n, i) =>
          drawNode(n, i)
      );

      drawOrbitals();

      drawPulses();

      drawShockwaves();

      frameRef.current =
        requestAnimationFrame(
          draw
        );
    };

    draw();

    return () => {

      if (
        frameRef.current
      ) {

        cancelAnimationFrame(
          frameRef.current
        );
      }
    };

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
        width: 900,
        height: 900,
        pointerEvents: "none",
        zIndex: 0,
      }}
    />
  );
}