# Fluxlock Wire Protocol Message Schema Specification
Version: v0.1 (Draft)
Status: Phase 2 Implementation Preparation

This document defines canonical network message formats for Fluxlock nodes.

---

## 1. Overview

Fluxlock nodes communicate using structured protocol messages.

All messages MUST be:

- Deterministically serializable
- Version tagged
- Hash verifiable
- Replay compatible

---

## 2. Message Envelope

All messages MUST use a common envelope:

Fields:

- protocol_version
- message_type
- sender_node_id
- tick_reference
- payload_hash
- signature

---

## 3. Message Types

---

### Trust Telemetry Message

Purpose:
Share trust-relevant telemetry signals.

Payload:

- trust_signal_type
- anomaly_score
- source_node_id

---

### Lock State Broadcast Message

Purpose:
Broadcast network lock escalation or de-escalation.

Payload:

- lock_state
- trigger_reason
- lock_epoch

---

### Recovery Proof Submission Message

Purpose:
Submit recovery proof evidence.

Payload:

- recovery_proof_hash
- recovery_epoch
- proof_metadata

---

### Adversarial Pressure Signal Message

Purpose:
Signal detected adversarial pressure.

Payload:

- pressure_type
- pressure_intensity
- observation_window

---

## 4. Serialization Requirements

Messages MUST be serialized using deterministic encoding.

Candidate formats:

- bincode
- protobuf (deterministic mode)
- flatbuffers (if deterministic ordering enforced)

---

## 5. Replay Compatibility

Messages MUST be:

- Loggable without loss
- Reconstructable for replay injection
- Hash stable across serialization cycles

---

## 6. Security Requirements

All messages MUST be:

- Signed by sender node
- Hash verifiable
- Rejectable if malformed or mismatched version

---

## 7. Versioning Strategy

Protocol version MUST be included in every message envelope.

Nodes MUST reject incompatible major versions.

---

## 8. Security Goal

Ensure all network communication supports deterministic replay,
tamper detection, and protocol consistency.
