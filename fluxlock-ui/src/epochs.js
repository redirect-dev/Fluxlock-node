// epochs.js
// NO GLOBAL STATE — FULLY DETERMINISTIC

function createEpochForNode(node) {
  return {
    epochId: node.id, // stable identity
    epochAge: 0,
    epochWeight: 1,
  };
}

export function ensureNodeEpoch(node) {
  if (
    node.epochId !== undefined &&
    node.epochAge !== undefined &&
    node.epochWeight !== undefined
  ) {
    return node;
  }

  return {
    ...node,
    ...createEpochForNode(node),
  };
}

export function ensureAllEpochs(nodes) {
  return nodes.map(ensureNodeEpoch);
}

export function runEpoch(nodes) {
  return nodes.map((node) => {
    const n = ensureNodeEpoch(node);

    return {
      ...n,
      epochAge: n.epochAge + 1,
      epochWeight: Math.max(0.1, n.epochWeight * 0.995),
    };
  });
}

export function rebalanceTrust(nodes) {
  return nodes.map((node) => {
    const n = ensureNodeEpoch(node);

    return {
      ...n,
      trust: n.trust * n.epochWeight,
    };
  });
}