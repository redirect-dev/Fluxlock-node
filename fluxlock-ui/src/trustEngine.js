// src/trustEngine.js

export function createInitialNodes(count = 20) {
  const nodes = [];

  for (let i = 0; i < count; i++) {
    nodes.push({
      id: i,
      trust: 100,
      drift: 0,
      momentum: 0,
      status: "normal",
      influence: 50,
      connections: [],
    });
  }

  // generate connections
  nodes.forEach(node => {
    const connectionCount = 3 + Math.floor(Math.random() * 4);

    for (let i = 0; i < connectionCount; i++) {
      const target = Math.floor(Math.random() * count);
      if (target !== node.id && !node.connections.includes(target)) {
        node.connections.push(target);
      }
    }
  });

  return nodes;
}

export function simulateStep(prevNodes) {
  const nodes = prevNodes.map(n => ({
    ...n,
    connections: [...n.connections],
  }));

  // -------------------------
  // 1. WAVE ATTACK SYSTEM
  // -------------------------
  if (Math.random() < 0.15) {
    const epicenter = nodes[Math.floor(Math.random() * nodes.length)];

    epicenter.drift += 50;
    epicenter.momentum += 20;

    // blast neighbors
    epicenter.connections.forEach(id => {
      const neighbor = nodes[id];
      if (!neighbor) return;

      neighbor.drift += 25;
      neighbor.momentum += 10;
    });
  }

  // -------------------------
  // 2. PROPAGATION (with momentum)
  // -------------------------
  nodes.forEach(node => {
    // decay but keep some force
    node.drift *= 0.9;
    node.momentum *= 0.85;

    const spread = (node.drift + node.momentum) * 0.05;

    node.connections.forEach(id => {
      const neighbor = nodes[id];
      if (!neighbor) return;

      neighbor.drift += spread / node.connections.length;
      neighbor.momentum += spread * 0.3;
    });
  });

  // -------------------------
  // 3. CLUSTER EFFECT
  // -------------------------
  nodes.forEach(node => {
    const neighborDrift =
      node.connections.reduce((sum, id) => {
        const n = nodes[id];
        return sum + (n ? n.drift : 0);
      }, 0) / (node.connections.length || 1);

    node.drift += neighborDrift * 0.02;
  });

  // -------------------------
  // 4. RECOVERY (RESISTANCE)
  // -------------------------
  nodes.forEach(node => {
    if (node.drift < 10) {
      node.drift *= 0.7; // fast heal
    } else if (node.drift < 40) {
      node.drift *= 0.85;
    } else {
      node.drift *= 0.95; // slow recovery if heavily attacked
    }
  });

  // -------------------------
  // 5. CLAMP + CLASSIFY
  // -------------------------
  nodes.forEach(node => {
    if (node.drift > 150) node.drift = 150;

    if (node.drift > 80) node.status = "attacked";
    else if (node.drift > 50) node.status = "drifting";
    else if (node.drift > 20) node.status = "warning";
    else node.status = "normal";

    node.trust = Math.max(0, 100 - node.drift);
    node.influence = node.trust * 0.8;
  });

  return nodes;
}