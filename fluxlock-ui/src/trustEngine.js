// =========================
// 🔐 PSEUDO-DILITHIUM KEY GEN
// =========================
function generateKeypair() {
  const rand = () =>
    Math.random().toString(36).substring(2, 10);

  return {
    publicKey: `dil_${rand()}${rand()}${rand()}`,
    privateKey: `priv_${rand()}${rand()}`,
    fingerprint: rand().substring(0, 8),
  };
}

// =========================
// 🔗 VALIDATION
// =========================
function validateChain(node) {
  const chain = node.identityChain;

  if (!chain || chain.length === 0) {
    return { valid: false, reason: "missing identity chain" };
  }

  const last = chain[chain.length - 1];

  if (Math.abs(node.trust - last.trust) > 60) {
    return {
      valid: false,
      reason: "current state deviates from identity history",
    };
  }

  if (node.drift > 120) {
    return {
      valid: false,
      reason: "critical instability",
    };
  }

  return { valid: true, reason: "chain valid" };
}

// =========================
// 🧠 CREATE NETWORK
// =========================
export function createNetwork(size = 20) {
  const nodes = [];

  for (let i = 0; i < size; i++) {
    const kp = generateKeypair();

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

      ...kp,

      identityChain: [
        {
          publicKey: kp.publicKey,
          fingerprint: kp.fingerprint,
          trust: 100,
          sig: "GENESIS",
        },
      ],

      chainValid: true,
      chainReason: "init",

      stabilityCounter: 0,
    });
  }

  // Dense mesh
  for (let i = 0; i < size; i++) {
    for (let j = 0; j < size; j++) {
      if (i !== j && Math.random() < 0.5) {
        nodes[i].connections.push(j);
      }
    }
  }

  return nodes;
}

// =========================
// ⚙️ SIMULATION (FIXED STATE MACHINE)
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
      fingerprint,
      identityChain,
      status,
    } = node;

    epochAge++;

    if (immunityTimer > 0) immunityTimer--;

    // =========================
    // ENTER COMPROMISE
    // =========================
    if (!compromised && drift > 70 && trust < 60) {
      compromised = true;
      recoveryTimer = 0;
      status = "attacked";
    }

    // =========================
    // COMPROMISED BEHAVIOR
    // =========================
    if (compromised) {
      recoveryTimer++;

      drift *= 0.97;     // stronger decay
      trust += 0.2;

      status = "attacked";
    }

    // =========================
    // RECOVERY PHASE
    // =========================
    if (compromised && drift < 50 && trust > 50) {
      status = "warning";
    }

    // =========================
    // FULL RECOVERY → KEY ROTATION
    // =========================
    if (compromised && drift < 25 && trust > 65) {
      const kp = generateKeypair();

      identityChain = [
        ...identityChain.slice(-5),
        {
          publicKey: kp.publicKey,
          fingerprint: kp.fingerprint,
          trust: Math.round(trust),
          sig: "ROTATED",
        },
      ];

      publicKey = kp.publicKey;
      privateKey = kp.privateKey;
      fingerprint = kp.fingerprint;

      epoch++;
      epochAge = 0;

      compromised = false;
      recoveryTimer = 0;
      immunityTimer = 10;

      status = "healthy";
    }

    // =========================
    // NORMAL BEHAVIOR
    // =========================
    if (!compromised) {
      drift *= 0.96;

      if (drift < 25) {
        trust += 0.4;
        status = "healthy";
      } else {
        trust -= drift * 0.015;
        status = "warning";
      }
    }

    trust = Math.max(0, Math.min(100, trust));
    drift = Math.max(0, drift);

    // =========================
    // VALIDATION
    // =========================
    const validation = validateChain({
      trust,
      drift,
      identityChain,
    });

    return {
      ...node,
      trust,
      drift,
      compromised,
      recoveryTimer,
      immunityTimer,
      epoch,
      epochAge,
      publicKey,
      privateKey,
      fingerprint,
      identityChain,
      status,

      chainValid: validation.valid,
      chainReason: validation.reason,
    };
  });
}