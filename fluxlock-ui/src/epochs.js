function randomKey() {
  return Math.random().toString(16).slice(2, 10);
}

function clamp(v, min, max) {
  return Math.max(min, Math.min(max, v));
}

// ------------------------------
export function ensureAllEpochs(nodes) {
  return nodes.map(node => ({
    ...node,
    connections: Array.isArray(node.connections) ? node.connections : [],
    trust: node.trust ?? 0,
    drift: node.drift ?? 0,
    influence: node.influence ?? 0,

    epochId: node.epochId ?? node.id,
    epochKey: node.epochKey ?? randomKey(),
    epochAge: node.epochAge ?? 0,
    epochWeight: node.epochWeight ?? 1,
    epochValid: node.epochValid ?? true,

    epochHistory: node.epochHistory ?? [],
  }));
}

// ------------------------------
export function runEpoch(nodes) {
  return nodes.map(node => {
    const updated = { ...node };

    updated.epochAge += 1;
    updated.epochWeight = Math.exp(-updated.epochAge / 200);

    if (updated.epochAge > 50) {
      updated.epochHistory = [
        ...updated.epochHistory,
        {
          key: updated.epochKey,
          trust: updated.trust,
        },
      ].slice(-10);

      updated.epochKey = randomKey();
      updated.epochAge = 0;
      updated.epochValid = true;
    }

    return updated;
  });
}

// ------------------------------
export function validateEpochs(nodes) {
  return nodes.map(node => ({
    ...node,
    epochValid: node.epochKey !== "tampered_key",
  }));
}

// ------------------------------
export function enforceEpochRules(nodes) {
  return nodes.map(node => {
    if (!node.epochValid) {
      return {
        ...node,
        trust: node.trust - 3,
        drift: node.drift + 5,
      };
    }
    return node;
  });
}

// ------------------------------
export function disconnectInvalidNodes(nodes) {
  return nodes;
}

// ------------------------------
export function recoverNodes(nodes) {
  return nodes.map(node => {
    if (node.epochValid) {
      return {
        ...node,
        trust: node.trust + 1.5,
        drift: node.drift * 0.9,
      };
    }
    return node;
  });
}

// ------------------------------
// 🔥 STRONG GROUP HEAL
// ------------------------------
export function diffuseTrust(nodes) {
  return nodes.map(node => {
    if (!node.connections.length) return node;

    let total = 0;
    let count = 0;

    node.connections.forEach(id => {
      const n = nodes[id];
      if (n && typeof n.trust === "number") {
        total += n.trust;
        count++;
      }
    });

    if (!count) return node;

    const avg = total / count;

    // 🔥 stronger when node is weak
    const strength = node.trust < 40 ? 0.15 : 0.05;

    return {
      ...node,
      trust: node.trust + (avg - node.trust) * strength,
    };
  });
}

// ------------------------------
// 🔥 LINEAGE BOOST
// ------------------------------
function applyLineage(nodes) {
  return nodes.map(node => {
    if (!node.epochHistory.length) return node;

    const avg =
      node.epochHistory.reduce((a, b) => a + b.trust, 0) /
      node.epochHistory.length;

    const strength = node.trust < 40 ? 0.08 : 0.02;

    return {
      ...node,
      trust: node.trust + (avg - node.trust) * strength,
    };
  });
}

// ------------------------------
export function rehabilitateNodes(nodes) {
  return nodes.map(node => {
    if (!node.epochValid && node.drift < 100) {
      return {
        ...node,
        epochKey: randomKey(),
        epochValid: true,
        status: "warning",
      };
    }
    return node;
  });
}

// ------------------------------
export function stabilizeNetwork(nodes) {
  let updated = diffuseTrust(nodes);
  updated = applyLineage(updated);

  return updated.map(node => ({
    ...node,
    trust: clamp(node.trust, -100, 100),
    drift: clamp(node.drift, 0, 1000),
    influence: clamp(node.influence, 0, 300),
  }));
}