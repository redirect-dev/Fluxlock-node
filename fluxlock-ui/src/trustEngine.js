// ==============================
// FLUXLOCK TRUST ENGINE (PHASE 36)
// ==============================

export function createNetwork(size = 20) {
  const nodes = [];

  for (let i = 0; i < size; i++) {
    nodes.push({
      id: i,
      trust: 90 + Math.random() * 10,
      drift: 0,
      behavior: 90 + Math.random() * 10,
      influence: 50 + Math.random() * 50,
      status: "normal",
      connections: [],
      x: Math.random() * 800,
      y: Math.random() * 500,
    });
  }

  // Create random connections
  nodes.forEach((node) => {
    const connectionCount = 3 + Math.floor(Math.random() * 3);

    for (let i = 0; i < connectionCount; i++) {
      const target = Math.floor(Math.random() * nodes.length);
      if (target !== node.id && !node.connections.includes(target)) {
        node.connections.push(target);
      }
    }
  });

  return nodes;
}

export function simulateStep(nodes) {
  const updated = nodes.map((n) => ({ ...n }));

  // Pick attacker
  const attacker = updated[19]; // keep consistent for demo

  if (attacker.status === "normal") {
    attacker.status = "drifting";
  } else if (attacker.status === "drifting") {
    attacker.status = "attacked";
  }

  attacker.trust -= 10;
  attacker.drift += 20;
  attacker.behavior -= 10;
  attacker.influence -= 15;

  // Propagation
  updated.forEach((node) => {
    if (node.id === attacker.id) return;

    const isConnected = node.connections.includes(attacker.id);

    if (isConnected) {
      node.trust -= 5;
      node.drift += 8;

      if (node.drift > 25 && node.status === "normal") {
        node.status = "warning";
      }

      if (node.drift > 60) {
        node.status = "drifting";
      }
    }

    // clamp values
    node.trust = Math.max(0, node.trust);
    node.influence = Math.max(0, node.influence);
  });

  return updated;
}