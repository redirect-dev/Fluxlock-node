# Fluxlock Trust Decay Model Specification
Version: v0.1 (Draft)
Status: Normative

This document defines the behavior of trust as a continuous,
time-dependent function within the Fluxlock protocol.

This specification is subordinate only to:
- Fluxlock Protocol Axioms
- Fluxlock System Model
- Node Lifecycle Specification
- Network Lock State Specification

---

## 1. Overview

Trust in Fluxlock is a continuously evaluated, time-dependent value.

Trust is not binary and not permanent.

Trust MUST:
- Decay over time without reinforcement
- Respond to adversarial pressure
- Recover only through verified behavior
- Never instantly increase

---

## 2. Design Intent

Trust decay is designed to reflect real-world system entropy.

Fluxlock assumes:
- Long-lived trust without reinforcement is unsafe
- Attackers exploit static trust relationships
- Time should reduce blind trust
- Recovery should require observable behavior

---

## 3. Trust Properties

Trust MUST be:

Continuous — Not binary  
Local — Evaluated per node  
Time-dependent — Always evolving  
Pressure-sensitive — Reacts to attack intensity  

---

## 4. Trust Decay Function

Trust is modeled conceptually as:

Trust(t+1) = Trust(t)
             - BaseDecay
             - PressureDecay
             + VerifiedRecovery

Where:

BaseDecay = natural trust entropy over time  
PressureDecay = additional decay under attack  
VerifiedRecovery = behavior + proof-based trust restoration  

---

## 5. Base Decay

Base decay MUST:
- Always be present
- Prevent permanent trust
- Scale with elapsed time

---

## 6. Pressure Decay

Pressure decay increases when:
- Network lock escalates
- Attack indicators increase
- Node anomaly rate increases

---

## 7. Recovery Trust Gain

Recovery trust gain MUST:
- Require time
- Require compliant behavior
- Require cryptographic proof
- Be slower than decay under pressure

---

## 8. Trust Threshold Interaction

Trust values determine:
- Node lifecycle state eligibility
- Lock participation eligibility
- Recovery eligibility

---

## 9. Trust Security Invariants

The following MUST hold:

- Trust cannot instantly increase
- Trust cannot be self-asserted
- Trust decay never fully stops
- Trust recovery always requires proof + time

---

## 10. Security Goal

Trust decay ensures:

> Trust must be continuously earned.

And:

> Long-term adversarial persistence becomes ineffective.
