function generateKey() {
  return Math.random().toString(16).substring(2, 10);
}

// =========================
// 🔗 VALIDATION
// =========================
function validateChain(node) {
  const { identityChain, trust, drift, tainted } = node;

  if (!identityChain || identityChain.length === 0) {
    return { valid: false, reason: "missing identity chain" };
  }

  for (let i = 1; i < identityChain.length; i++) {
    const prev = identityChain[i - 1];
    const curr = identityChain[i];

    if (Math.abs(curr.trust - prev.trust) > 50) {
      return { valid: false, reason: "identity discontinuity detected" };
    }
  }

  const last = identityChain[identityChain.length - 1];

  if (Math.abs(trust - last.trust) > 60) {
    return {
      valid: false,
      reason: "current state deviates from identity history",
    };
  }

  if (tainted && trust < 75) {
    return {
      valid: false,
      reason: "identity recovering from compromise",
    };
  }

  if (drift > 120) {
    return { valid: false, reason: "critical instability" };
  }

  return { valid: true, reason: "chain valid" };
}

// =========================
// ⚖️ EPOCH WEIGHT
// =========================
function epochWeight(age) {
  return Math.min(1, 0.3 + age / 50);
}

const STABILITY_THRESHOLD = 15; // 🔒 enforced window

export function createNetwork(size = 20) {
  const nodes = [];

  for (let i = 0; i < size; i++) {
    const key = generateKey();

    nodes.push({
      id: i,
      trust: 70 + Math.random() * 20,
      drift: Math.random() * 5,
      status: "healthy",
      connections: [],
      compromised: false,
      recoveryTimer: 0,
      immunityTimer: 0,
      recoveryPenalty: 0,

      epoch: i,
      epochAge: 0,

      key,
      identityChain: [{ key, trust: 100 }],

      chainValid: true,
      chainReason: "init",

      tainted: false,
      taintTimer: 0,

      // 🔥 NEW
      stabilityCounter: STABILITY_THRESHOLD,
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
      epoch,
      epochAge,
      key,
      identityChain,
      tainted,
      taintTimer,
      stabilityCounter,
    } = node;

    let status = node.status;

    epochAge += 1;
    const weight = epochWeight(epochAge);

    if (immunityTimer > 0) immunityTimer--;

    // =========================
    // COMPROMISE
    // =========================
    if (compromised) {
      recoveryTimer++;
      drift *= 0.995;
      trust += 0.02 * weight;

      tainted = true;
      taintTimer = 50;

      stabilityCounter = 0; // 🔒 reset

      status = "attacked";
    }

    // =========================
    // RECOVERY
    // =========================
    if (compromised && recoveryTimer > 25) {
      drift *= 0.94;
      trust += 0.3 * weight;
      status = "warning";
    }

    // =========================
    // EXIT COMPROMISE
    // =========================
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

      stabilityCounter = 0; // 🔒 restart window

      immunityTimer = 10;
      status = "recovering";
    }

    // =========================
    // NORMAL
    // =========================
    if (!compromised) {
      drift *= 0.97;

      const penalty = recoveryPenalty > 0 ? 0.4 : 1;
      const taintFactor = tainted ? 0.5 : 1;

      if (drift < 25) {
        trust += 0.6 * penalty * weight * taintFactor;
      } else {
        trust -= drift * 0.015;
      }
    }

    // =========================
    // TAINT DECAY
    // =========================
    if (tainted) {
      taintTimer--;

      if (taintTimer <= 0 && trust > 80) {
        tainted = false;
      }
    }

    if (recoveryPenalty > 0) recoveryPenalty--;

    trust = Math.max(0, Math.min(100, trust));
    drift = Math.max(0, drift);

    // =========================
    // VALIDATION
    // =========================
    const validation = validateChain({
      trust,
      drift,
      identityChain,
      tainted,
    });

    // =========================
    // 🔒 STABILITY WINDOW LOGIC
    // =========================
    if (validation.valid) {
      stabilityCounter++;
    } else {
      stabilityCounter = 0;
    }

    // =========================
    // FINAL STATUS CONTROL
    // =========================
    if (!validation.valid) {
      status = tainted ? "recovering" : "drifting";
    } else if (stabilityCounter >= STABILITY_THRESHOLD && !tainted && drift < 25) {
      status = "healthy";
    } else {
      status = "recovering";
    }

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
      tainted,
      taintTimer,
      stabilityCounter,
      chainValid: validation.valid,
      chainReason: validation.reason,
    };
  });
}