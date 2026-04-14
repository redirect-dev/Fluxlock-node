// epochs.js
// PHASE 40 — KEYS + VALIDATION + TAMPER + ENFORCEMENT + DISCONNECT

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

// 🔍 VALIDATION
export function validateEpochs(nodes) {
  return nodes.map((node) => {
    const expectedKey = generateEpochKey(node);
    const isValid = node.epochKey === expectedKey;

    return {
      ...node,
      epochValid: isValid,
    };
  });
}

// 🔥 TAMPER TEST
export function tamperNode(nodes) {
  return nodes.map((node) => {
    if (node.id === 19 && node.epochAge > 10) {
      return {
        ...node,
        epochKey: "tampered_key",
      };
    }
    return node;
  });
}

// 🚨 ENFORCEMENT
export function enforceEpochRules(nodes) {
  return nodes.map((node) => {
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

// 🔌 FULL DISCONNECT (BIDIRECTIONAL CLEANUP)
export function disconnectInvalidNodes(nodes) {
  // collect invalid node IDs
  const invalidIds = new Set(
    nodes.filter(n => !n.epochValid).map(n => n.id)
  );

  return nodes.map((node) => {
    // remove ALL connections pointing to invalid nodes
    const filteredConnections = node.connections.filter(
      (id) => !invalidIds.has(id)
    );

    return {
      ...node,
      connections: node.epochValid ? filteredConnections : [], // invalid = no edges at all
    };
  });
}