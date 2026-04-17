function generateKeypair() {
  const key = Math.random().toString(16).substring(2, 10);
  return {
    publicKey: "pub_" + key,
    privateKey: "priv_" + key,
  };
}

// =========================
// COMPAT EXPORTS (dashboard expects these)
// =========================
export async function requestSignature() {
  return "simulated_signature";
}

export async function verifySignature() {
  return true;
}

// =========================
// AUTO SIGN (simulation-safe)
// =========================
function autoSign(identityChain) {
  const last = identityChain[identityChain.length - 1];
  if (last && !last.signature) {
    last.signature = "signed";
  }
  return identityChain;
}

// =========================
// VALIDATION
// =========================
function validateChain(node) {
  const { identityChain, trust, drift } = node;

  if (!identityChain || identityChain.length === 0) {
    return { valid: false, reason: "missing identity chain" };
  }

  let hasPending = false;
  const lastIndex = identityChain.length - 1;

  for (let i = 1; i < identityChain.length; i++) {
    const prev = identityChain[i - 1];
    const curr = identityChain[i];

    if (curr.signature === "genesis") continue;

    if (!curr.signature) {
      if (i === lastIndex) hasPending = true;
      continue;
    }

    // 🔥 KEY PROGRESSION CHECK
    if (curr.publicKey === prev.publicKey) {
      return { valid: false, reason: "key reuse detected" };
    }

    // 🔥 TRUST CONTINUITY
    if (Math.abs(curr.trust - prev.trust) > 50) {
      return { valid: false, reason: "identity discontinuity detected" };
    }
  }

  const last = identityChain[lastIndex];

  if (Math.abs(trust - last.trust) > 60) {
    return {
      valid: false,
      reason: "current state deviates from identity history",
    };
  }

  // 🔥 STATE-BASED VALIDATION
  if (drift > 80) {
    return {
      valid: false,
      reason: "identity unstable (high drift)",
    };
  }

  if (hasPending) {
    return {
      valid: false,
      reason: "awaiting cryptographic signature",
      pending: true,
    };
  }

  return { valid: true, reason: "chain valid" };
}

// =========================
// INIT
// =========================
export function createNetwork(size = 20) {
  const nodes = [];

  for (let i = 0; i < size; i++) {
    const keypair = generateKeypair();

    nodes.push({
      id: i,
      trust: 80,
      drift: 2,
      status: "healthy",
      connections: [],

      compromised: false,
      recoveryTimer: 0,
      immunityTimer: 0,

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
      stabilityCounter: 15,
    });
  }

  // random connections
  nodes.forEach(node => {
    const set = new Set();
    while (set.size < 4) {
      const t = Math.floor(Math.random() * size);
      if (t !== node.id) set.add(t);
    }
    node.connections = [...set];
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
      epoch,
      epochAge,
      publicKey,
      privateKey,
      identityChain,
      stabilityCounter,
    } = node;

    let status = node.status;

    epochAge++;

    if (immunityTimer > 0) immunityTimer--;

    // =========================
    // COMPROMISE DETECTION
    // =========================
    if (drift > 70 && trust < 60) {
      compromised = true;
    }

    if (compromised) {
      recoveryTimer++;
      drift *= 0.995;
      trust += 0.2;
      status = "attacked";
    }

    if (compromised && recoveryTimer > 25) {
      drift *= 0.94;
      trust += 0.3;
      status = "warning";
    }

    // =========================
    // 🔥 KEY ROTATION (FIXED)
    // =========================
    if (compromised && drift < 35 && trust > 65) {
      const newKeypair = generateKeypair();

      identityChain = [
        ...identityChain.slice(-5),
        {
          publicKey: newKeypair.publicKey, // ✅ FIXED (new key, not old)
          trust: Math.round(trust),
          signature: null,
        },
      ];

      publicKey = newKeypair.publicKey;
      privateKey = newKeypair.privateKey;

      epoch += 1;
      epochAge = 0;

      compromised = false;
      recoveryTimer = 0;
      immunityTimer = 10;
      stabilityCounter = 0;

      status = "recovering";
    }

    // =========================
    // AUTO SIGN
    // =========================
    identityChain = autoSign(identityChain);

    // =========================
    // NORMAL BEHAVIOR
    // =========================
    if (!compromised) {
      drift *= 0.97;

      if (drift < 25) {
        trust += 0.4;
      } else {
        trust -= drift * 0.01;
      }
    }

    trust = Math.max(0, Math.min(100, trust));
    drift = Math.max(0, drift);

    const validation = validateChain({
      trust,
      drift,
      identityChain,
    });

    // =========================
    // STABILITY TRACKING
    // =========================
    if (validation.valid) {
      stabilityCounter++;
    } else {
      stabilityCounter = 0;
    }

    // =========================
    // 🔥 STATUS (FIXED)
    // =========================
    if (!validation.valid && !validation.pending) {
      status = "drifting";
    } else if (validation.pending) {
      status = "recovering";
    } else if (drift < 10 && stabilityCounter >= 8) {
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
      status,
      epoch,
      epochAge,
      publicKey,
      privateKey,
      identityChain,
      stabilityCounter,
      chainValid: validation.valid,
      chainReason: validation.reason,
    };
  });
}