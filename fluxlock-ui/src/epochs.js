// epochs.js
// PHASE 44 — TRUST DIFFUSION ENABLED

function hashString(str) {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    const chr = str.charCodeAt(i);
    hash = (hash << 5) - hash + chr;
    hash |= 0;
  }
  return Math.abs(hash).toString(16);
}

function createEpoch(node) {
  return {
    epochId: node.id,
    epochAge: 0,
    epochWeight: 1,
    epochKey: "",
    epochValid: true,
    recovering: false,
  };
}

export function ensureNodeEpoch(node) {
  if (
    node.epochId !== undefined &&
    node.epochAge !== undefined &&
    node.epochWeight !== undefined &&
    node.epochKey !== undefined &&
    node.epochValid !== undefined
  ) {
    return node;
  }

  return {
    ...node,
    ...createEpoch(node),
  };
}

export function ensureAllEpochs(nodes) {
  return nodes.map(ensureNodeEpoch);
}

export function generateEpochKey(node) {
  const base = `${node.id}|${node.epochAge}|${node.epochWeight.toFixed(
    4
  )}|${Math.round(node.trust)}|${node.status}`;

  return hashString(base);
}

// =======================
// EPOCH
// =======================
export function runEpoch(nodes) {
  return nodes.map((node) => {
    const n = ensureNodeEpoch(node);

    const epochAge = n.epochAge + 1;
    const epochWeight = Math.max(0.1, n.epochWeight * 0.995);

    const updated = {
      ...n,
      epochAge,
      epochWeight,
    };

    return {
      ...updated,
      epochKey: generateEpochKey(updated),
      epochValid: true,
    };
  });
}

// =======================
// VALIDATION
// =======================
export function validateEpochs(nodes) {
  return nodes.map((node) => {
    const expectedKey = generateEpochKey(node);
    const isValid = node.epochKey === expectedKey;

    return {
      ...node,
      epochValid: isValid,
      recovering: isValid && node.trust < 20, // 🔥 earlier recovery trigger
    };
  });
}

// =======================
// TAMPER
// =======================
export function tamperNode(nodes) {
  return nodes.map((node) => {
    if (node.id === 19 && node.epochAge > 10 && node.epochAge < 40) {
      return {
        ...node,
        epochKey: "tampered_key",
      };
    }
    return node;
  });
}

// =======================
// ENFORCEMENT
// =======================
export function enforceEpochRules(nodes) {
  return nodes.map((node) => {
    if (node.recovering) return node;

    if (!node.epochValid) {
      return {
        ...node,
        trust: node.trust * 0.5,
        influence: node.influence * 0.2,
      };
    }
    return node;
  });
}

// =======================
// DISCONNECT
// =======================
export function disconnectInvalidNodes(nodes) {
  const invalidIds = new Set(
    nodes.filter(n => !n.epochValid).map(n => n.id)
  );

  return nodes.map((node) => {
    const filteredConnections = node.connections.filter(
      (id) => !invalidIds.has(id)
    );

    return {
      ...node,
      connections: node.epochValid ? filteredConnections : [],
    };
  });
}

// =======================
// RECOVERY
// =======================
export function recoverNodes(nodes) {
  return nodes.map((node) => {
    if (!node.epochValid && node.epochAge > 40) {
      return {
        ...node,
        epochKey: generateEpochKey(node),
        epochValid: true,
      };
    }
    return node;
  });
}

// =======================
// 🧬 TRUST DIFFUSION (NEW)
// =======================
export function diffuseTrust(nodes) {
  return nodes.map((node) => {
    if (!node.connections.length) return node;

    let total = 0;
    let count = 0;

    node.connections.forEach((id) => {
      const neighbor = nodes[id];
      if (!neighbor) return;

      total += neighbor.trust;
      count++;
    });

    if (count === 0) return node;

    const avgNeighborTrust = total / count;

    // 🔥 Pull node toward neighbor average
    const adjustment = (avgNeighborTrust - node.trust) * 0.05;

    return {
      ...node,
      trust: node.trust + adjustment,
    };
  });
}

// =======================
// REHABILITATION
// =======================
export function rehabilitateNodes(nodes) {
  return nodes.map((node) => {
    if (node.recovering) {
      return {
        ...node,
        trust: node.trust + 4,      // 🔥 stronger push
        influence: node.influence + 2,
      };
    }

    if (node.epochValid && node.trust < 50) {
      return {
        ...node,
        trust: node.trust + 1,
        influence: node.influence + 0.5,
      };
    }

    return node;
  });
}

// =======================
// FINAL CLAMP
// =======================
export function stabilizeNetwork(nodes) {
  return nodes.map((node) => ({
    ...node,
    trust: Math.max(-100, Math.min(100, node.trust)),
    influence: Math.max(-1000, Math.min(1000, node.influence)),
  }));
}