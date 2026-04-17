function generateKey() {
  return Math.random().toString(16).substring(2, 10);
}

export function createNetwork(size = 20) {
  const nodes = [];

  for (let i = 0; i < size; i++) {
    const key = generateKey();

    nodes.push({
      id: i,
      trust: 70 + Math.random() * 20,
      influence: 50,
      drift: Math.random() * 5,
      status: "healthy",
      connections: [],
      compromised: false,
      recoveryTimer: 0,
      immunityTimer: 0,
      recoveryPenalty: 0,

      epoch: i,
      epochAge: 0,

      // 🔑 NEW
      key,
      identityChain: [{ key, trust: 100 }],
    });
  }

  nodes.forEach(node => {
    const connections = new Set();
    while (connections.size < 4) {
      const target = Math.floor(Math.random() * size);
      if (target !== node.id) connections.add(target);
    }
    node.connections = [...connections];
  });

  return nodes;
}

export function simulateStep(nodes) {
  return nodes.map(node => {
    let {
      trust,
      drift,
      compromised,
      recoveryTimer,
      immunityTimer,
      recoveryPenalty,
      status,
      epoch,
      epochAge,
      key,
      identityChain,
    } = node;

    epochAge += 1;

    if (immunityTimer > 0) immunityTimer--;

    // COMPROMISE
    if (compromised) {
      recoveryTimer++;
      drift *= 0.995;
      trust += 0.03;
      status = "attacked";
    }

    // RECOVERY
    if (compromised && recoveryTimer > 25) {
      drift *= 0.94;
      trust += 0.4;
      status = "warning";
    }

    // 🔥 EXIT COMPROMISE → ROTATE KEY
    if (compromised && drift < 35 && trust > 65) {
      compromised = false;
      recoveryTimer = 0;
      recoveryPenalty = 30;

      const newKey = generateKey();

      identityChain = [
        ...identityChain.slice(-5),
        { key, trust: Math.round(trust) },
      ];

      key = newKey;

      epoch += 1;
      epochAge = 0;

      immunityTimer = 10;
      status = "drifting";
    }

    // NORMAL
    if (!compromised) {
      drift *= 0.97;

      const penalty = recoveryPenalty > 0 ? 0.4 : 1;

      if (drift < 25) {
        trust += 0.6 * penalty;
        status = "healthy";
      } else {
        trust -= drift * 0.015;
        status = "drifting";
      }
    }

    if (recoveryPenalty > 0) recoveryPenalty--;

    trust = Math.max(0, Math.min(100, trust));
    drift = Math.max(0, drift);

    return {
      ...node,
      trust,
      drift,
      compromised,
      recoveryTimer,
      immunityTimer,
      recoveryPenalty,
      status,
      epoch,
      epochAge,
      key,
      identityChain,
    };
  });
}