import { simulateStep } from "./trustEngine";

function applyConsensus(nodes) {
  return nodes.map(node => {
    const peers = node.connections || [];

    let valid = 0;
    let invalid = 0;

    const recentlyUnstable =
      node.drift > 40 || node.trust < 60;

    const instabilityMemory =
      Math.max(0, 1 - node.epochAge / 300);

    const computeConfidence = () => {
      const trustScore = node.trust / 100;
      const driftPenalty = Math.min(node.drift / 150, 1);

      let confidence =
        trustScore * 0.65 +
        (1 - driftPenalty) * 0.35;

      if (!node.chainValid) confidence *= 0.2;

      const recoveryFactor =
        1 - Math.exp(-node.epochAge / 250);

      confidence *= (0.3 + recoveryFactor * 0.7);
      confidence *= (1 - instabilityMemory * 0.5);

      return confidence;
    };

    // -------------------------
    // 🗳️ VOTING
    // -------------------------
    peers.forEach(id => {
      const peer = nodes.find(n => n.id === id);
      if (!peer) return;

      let confidence = computeConfidence();
      confidence += (Math.random() - 0.5) * 0.05;

      let threshold =
        0.55 + (peer.trust / 100) * 0.25;

      if (recentlyUnstable) threshold += 0.15;
      if (node.trust < 50) threshold += 0.1;

      if (confidence > threshold) valid++;
      else invalid++;
    });

    const total = valid + invalid;
    const ratio = total > 0 ? valid / total : 0;

    // -------------------------
    // 🌐 NETWORK
    // -------------------------
    let requiredRatio = 0.6;

    if (recentlyUnstable) {
      const recoveryFactor =
        1 - Math.exp(-node.epochAge / 250);

      requiredRatio =
        0.85 - (recoveryFactor * 0.25);
    }

    const networkAccepted = ratio > requiredRatio;

    // -------------------------
    // 🧠 LOCAL (NOW DERIVED FROM VOTES ONLY)
    // -------------------------
    const localValid =
      ratio > 0.5; // 🔥 MUST HAVE MAJORITY SUPPORT

    // -------------------------
    // 🌍 GLOBAL
    // -------------------------
    const stableEnough =
      node.drift < 15 &&
      node.trust > 70;

    const longEnough =
      node.epochAge > 120;

    const globalValid =
      node.chainValid &&
      networkAccepted &&
      stableEnough &&
      longEnough;

    return {
      ...node,
      peerVotes: { valid, invalid },
      networkAccepted,
      globalValid,
      localValid,
    };
  });
}

export function evaluateNetwork(nodes) {
  let updated = simulateStep(nodes);
  updated = applyConsensus(updated);
  return updated;
}