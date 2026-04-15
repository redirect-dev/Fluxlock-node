// Fluxlock Core API
// This wraps your internal engine into a usable external interface

import { simulateStep } from "./trustEngine";
import {
  ensureAllEpochs,
  runEpoch,
  validateEpochs,
  enforceEpochRules,
  disconnectInvalidNodes,
  recoverNodes,
  rehabilitateNodes,
  stabilizeNetwork,
} from "./epochs";

/**
 * Runs one full Fluxlock evaluation cycle
 */
export function evaluateNetwork(nodes) {
  let updated = simulateStep(nodes);

  updated = ensureAllEpochs(updated);
  updated = runEpoch(updated);

  updated = validateEpochs(updated);
  updated = enforceEpochRules(updated);
  updated = disconnectInvalidNodes(updated);

  updated = recoverNodes(updated);
  updated = rehabilitateNodes(updated);
  updated = stabilizeNetwork(updated);

  return updated;
}

/**
 * Get trust score for a specific node
 */
export function getTrustScore(nodes, nodeId) {
  const node = nodes.find((n) => n.id === nodeId);
  return node ? node.trust : null;
}

/**
 * Get full node state (for inspection / UI / external use)
 */
export function getNodeState(nodes, nodeId) {
  return nodes.find((n) => n.id === nodeId) || null;
}

/**
 * Verify identity integrity (epoch validity)
 */
export function verifyIdentity(node) {
  return node.epochValid === true;
}

/**
 * Get network health summary
 */
export function getNetworkHealth(nodes) {
  const total = nodes.length;

  const healthy = nodes.filter(n => n.status === "healthy").length;
  const warning = nodes.filter(n => n.status === "warning").length;
  const attacked = nodes.filter(n => n.status === "attacked").length;
  const drifting = nodes.filter(n => n.status === "drifting").length;

  const avgTrust =
    nodes.reduce((sum, n) => sum + n.trust, 0) / total;

  return {
    total,
    healthy,
    warning,
    attacked,
    drifting,
    avgTrust,
  };
}