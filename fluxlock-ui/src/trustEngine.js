function generateKeypair() {
  const key = Math.random().toString(16).substring(2, 10);

  return {
    publicKey: "pub_" + key,
    privateKey: "priv_" + key,
  };
}

// =========================
// SIGNING
// =========================
export async function requestSignature(message, validatorId) {
  try {
    const res = await fetch("http://127.0.0.1:3001/sign", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        message,
        validator_id: validatorId,
      }),
    });

    const data = await res.json();
    return data.signature;
  } catch (err) {
    console.error("Signing error:", err);
    return null;
  }
}

// =========================
// VERIFY
// =========================
export async function verifySignature(message, signature, validatorId) {
  try {
    const res = await fetch("http://127.0.0.1:3001/verify", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        message,
        signature,
        validator_id: validatorId,
      }),
    });

    const data = await res.json();
    return data.valid;
  } catch (err) {
    console.error("Verify error:", err);
    return false;
  }
}

// =========================
// 🔥 VALIDATION (FINAL FIX)
// =========================
function validateChain(node) {
  const { identityChain, trust, drift, tainted } = node;

  if (!identityChain || identityChain.length === 0) {
    return { valid: false, reason: "missing identity chain" };
  }

  let hasPending = false;
  const lastIndex = identityChain.length - 1;

  for (let i = 1; i < identityChain.length; i++) {
    const prev = identityChain[i - 1];
    const curr = identityChain[i];

    // skip genesis
    if (curr.signature === "genesis") continue;

    // 🔥 ONLY LAST ENTRY CAN BE PENDING
    if (!curr.signature) {
      if (i === lastIndex) {
        hasPending = true;
      }
      continue;
    }

    // 🔥 HARD FAIL
    if (curr.invalidSignature) {
      return {
        valid: false,
        reason: "invalid cryptographic signature",
      };
    }

    // continuity checks
    if (curr.publicKey === prev.publicKey) {
      return { valid: false, reason: "key reuse detected" };
    }

    if (Math.abs(curr.trust - prev.trust) > 50) {
      return {
        valid: false,
        reason: "identity discontinuity detected",
      };
    }
  }

  const last = identityChain[lastIndex];

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

  // 🔥 NEW: pending only matters if node is NOT stable
  const isStable = drift < 25 && trust > 80 && !tainted;

  if (hasPending && !isStable) {
    return {
      valid: false,
      reason: "awaiting cryptographic signature",
      pending: true,
    };
  }

  return { valid: true, reason: "chain valid" };
}

// =========================
// EPOCH WEIGHT
// =========================
function epochWeight(age) {
  return Math.min(1, 0.3 + age / 50);
}

const STABILITY_THRESHOLD = 15;

// =========================
// NETWORK
// =========================
export function createNetwork(size = 20) {
  const nodes = [];

  for (let i = 0; i < size; i++) {
    const keypair = generateKeypair();

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

      ...keypair,

      identityChain: [
        {
          publicKey: keypair.publicKey,
          trust: 100,
          signature: "genesis",
        },
      ],

      chainValid: true,
      chainReason: "init",

      tainted: false,
      taintTimer: 0,
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

// =========================
// SIMULATION
// =========================
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
      publicKey,
      privateKey,
      identityChain,
      tainted,
      taintTimer,
      stabilityCounter,
    } = node;

    let status = node.status;

    epochAge += 1;
    const weight = epochWeight(epochAge);

    if (immunityTimer > 0) immunityTimer--;

    // COMPROMISE
    if (compromised) {
      recoveryTimer++;
      drift *= 0.995;
      trust += 0.02 * weight;

      tainted = true;
      taintTimer = 50;
      stabilityCounter = 0;

      status = "attacked";
    }

    // RECOVERY
    if (compromised && recoveryTimer > 25) {
      drift *= 0.94;
      trust += 0.3 * weight;
      status = "warning";
    }

    // ROTATION
    if (compromised && drift < 35 && trust > 65) {
      compromised = false;
      recoveryTimer = 0;
      recoveryPenalty = 30;

      const newKeypair = generateKeypair();

      identityChain = [
        ...identityChain.slice(-5),
        {
          publicKey,
          trust: Math.round(trust),
          signature: null,
          needsSignature: {
            message: publicKey,
          },
        },
      ];

      publicKey = newKeypair.publicKey;
      privateKey = newKeypair.privateKey;

      epoch += 1;
      epochAge = 0;

      stabilityCounter = 0;
      immunityTimer = 10;

      status = "recovering";
    }

    // NORMAL
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

    // TAINT DECAY
    if (tainted) {
      taintTimer--;
      if (taintTimer <= 0 && trust > 80) {
        tainted = false;
      }
    }

    if (recoveryPenalty > 0) recoveryPenalty--;

    trust = Math.max(0, Math.min(100, trust));
    drift = Math.max(0, drift);

    const validation = validateChain({
      trust,
      drift,
      identityChain,
      tainted,
    });

    if (validation.valid) {
      stabilityCounter++;
    } else {
      stabilityCounter = 0;
    }

    if (!validation.valid) {
      if (validation.pending) {
        status = "recovering";
      } else {
        status = tainted ? "recovering" : "drifting";
      }
    } else if (
      stabilityCounter >= STABILITY_THRESHOLD &&
      !tainted &&
      drift < 25
    ) {
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
      publicKey,
      privateKey,
      identityChain,
      tainted,
      taintTimer,
      stabilityCounter,
      chainValid: validation.valid,
      chainReason: validation.reason,
    };
  });
}