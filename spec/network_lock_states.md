# Fluxlock Network Lock State Specification
Version: v0.1 (Draft)
Status: Normative

This document defines network-wide defensive lock states and
their behavior under sustained adversarial pressure.

This specification is subordinate only to:
- Fluxlock Protocol Axioms
- Fluxlock System Model
- Fluxlock Node Lifecycle Specification

---

## 1. Overview

Fluxlock implements coordinated network lock states to reduce
attack surface, increase adversarial cost, and preserve network integrity.

Lock states are defensive, not punitive.

Lock escalation is triggered by:
- Aggregate trust decay
- Sustained adversarial pressure
- Network instability signals
- Coordinated attack indicators

Lock de-escalation requires:
- Verified recovery signals
- Network-wide stabilization
- Proof-backed restoration conditions

---

## 2. Design Intent

Fluxlock network locks are designed under the assumption that
large-scale adversarial pressure will occur repeatedly and over long durations.

Locks intentionally:
- Increase attacker operational cost over time
- Reduce exploitable surface area during attack
- Force adversaries into higher resource expenditure
- Preserve core network integrity even under partial compromise

Locks are not emergency shutdown mechanisms.
They are adaptive defensive postures.

---

## 3. Lock State Ladder

Fluxlock defines four network lock states:

UNLOCKED  
RESTRICTED  
SOFT_LOCK  
LOCKED  

Each state represents progressively stronger defensive posture.

---

## 4. UNLOCKED State

### Description
Normal network operation.

### Behavior
- Full protocol participation allowed
- Normal trust evaluation
- Standard verification cost

### Exit Conditions
Transition to RESTRICTED if:
- Early attack signals detected
- Trust decay rate exceeds baseline
- Coordinated suspicious activity detected

---

## 5. RESTRICTED State

### Description
Early defensive posture.

### Behavior
- Increased verification requirements
- Reduced protocol tolerance for anomalies
- Increased monitoring

### Goals
- Detect escalation early
- Increase adversary cost
- Prevent attack foothold expansion

### Exit Conditions

Escalate to SOFT_LOCK if:
- Sustained attack indicators persist
- Trust decay accelerates
- Coordinated adversarial behavior confirmed

De-escalate to UNLOCKED if:
- Network stability restored
- Attack signals subside

---

## 6. SOFT_LOCK State

### Description
Active defensive posture under sustained pressure.

### Behavior
- Restricted node participation
- Increased recovery proof requirements
- Elevated verification cost
- Reduced propagation tolerance

### Goals
- Limit adversarial lateral movement
- Increase attacker fatigue
- Preserve core trust signals

### Exit Conditions

Escalate to LOCKED if:
- Systemic compromise risk detected
- Network trust collapse threshold approached
- Attack intensity exceeds safe operating margin

De-escalate to RESTRICTED if:
- Sustained recovery signals observed
- Trust decay stabilizes

---

## 7. LOCKED State

### Description
Maximum defensive posture to preserve core network integrity.

### Behavior
- Minimal protocol participation allowed
- Strict message validation
- Recovery proof mandatory for re-expansion
- Maximum adversarial cost enforcement

### Goals
- Prevent systemic compromise
- Preserve minimum viable network integrity
- Force adversaries into unsustainable resource expenditure

### Exit Conditions
De-escalate to SOFT_LOCK only if:
- Network-wide recovery proof thresholds satisfied
- Trust recovery trend confirmed
- Attack pressure measurably reduced

---

## 8. Lock Escalation Principles

### Automatic Response
Lock escalation MUST occur automatically based on
protocol-defined thresholds.

### Pressure Sensitivity
Escalation MUST respond to:
- Duration of pressure
- Intensity of pressure
- Breadth of affected nodes

---

## 9. Lock De-escalation Principles

De-escalation MUST:
- Be slower than escalation
- Require proof-backed recovery signals
- Require sustained stability

Instant de-escalation is forbidden.

---

## 10. Node Lifecycle Interaction

Lock states influence:
- Node transition thresholds
- Recovery difficulty
- Verification burden
- Participation eligibility

Node lifecycle state MUST NOT override lock state restrictions.

---

## 11. Trust Interaction

Lock states modify:
- Trust decay sensitivity
- Recovery trust gain rate
- Verification cost

---

## 12. Security Invariants

The following MUST always hold:

- Locks escalate faster than they de-escalate
- Lock participation is never optional
- Lock bypass via identity reset is impossible
- Lock state cannot be skipped
- Lock enforcement is network-wide

---

## 13. Security Goal

Network locks ensure:

> The network becomes harder to exploit the longer
> adversarial pressure is sustained.

And:

> Sustained attack becomes operationally and economically irrational.
