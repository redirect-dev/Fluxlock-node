// trustEngine.js
// 🔥 FINAL BALANCED SYSTEM

export function createNetwork(size = 20) {
  const nodes = [];

  for (let i = 0; i < size; i++) {
    nodes.push({
      id: i,
      trust: 70 + Math.random() * 20,
      influence: 50 + Math.random() * 20,
      drift: 0,
      status: "healthy",
      connections: [],
    });
  }

  nodes.forEach(node => {
    const numConnections = 4 + Math.floor(Math.random() * 4);
    const connections = new Set();

    while (connections.size < numConnections) {
      const target = Math.floor(Math.random() * size);
      if (target !== node.id) connections.add(target);
    }

    node.connections = Array.from(connections);
  });

  return nodes;
}

export function simulateStep(nodes) {
  return nodes.map(node => {
    let trust = node.trust;
    let drift = node.drift;
    let status = node.status;

    // =========================
    // 🔥 SHORTER ATTACK WINDOW
    // =========================
    if (node.id === 19 && drift < 150) {
      drift += 15;
      trust -= 4; // softer hit
      status = "attacked";
    }

    // =========================
    // 🔥 DRIFT DECAY (FASTER)
    // =========================
    else if (drift > 0) {
      drift *= 0.75;              // faster decay
      trust -= drift * 0.015;     // lighter damage
      status = "drifting";
    }

    // =========================
    // 🔥 STRONGER NATURAL HEALING
    // =========================
    else {
      drift = Math.max(0, drift - 3);
      trust += 2.5;               // 🔥 stronger recovery
      status = "healthy";
    }

    // =========================
    // 🔥 HARD CLAMP
    // =========================
    trust = Math.max(-100, Math.min(100, trust));

    // =========================
    // 🔥 STATUS NORMALIZATION
    // =========================
    if (trust > 60) status = "healthy";
    else if (trust > 30) status = "warning";
    else if (trust > 0) status = "drifting";
    else status = "attacked";

    return {
      ...node,
      trust,
      drift,
      status,
    };
  });
}