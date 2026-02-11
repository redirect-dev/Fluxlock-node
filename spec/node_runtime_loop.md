# Fluxlock Node Runtime Loop Specification
Version: v0.1 (Draft)
Status: Phase 2 Implementation Preparation

This document defines the canonical runtime loop for a Fluxlock node.

---

## 1. Overview

A Fluxlock node operates as a continuous runtime process executing:

- Network input handling
- Input normalization
- Deterministic tick execution
- Persistence updates
- Replay logging
- Metrics emission

---

## 2. Runtime Initialization

On node startup:

1. Load configuration
2. Load latest snapshot
3. Replay forward using replay log
4. Enter deterministic runtime loop

---

## 3. Main Runtime Loop

The node MUST execute continuously:

---

### Step 1 — Receive Network Messages

Process:

- Peer signals
- Lock state broadcasts
- Recovery proof submissions
- Adversarial signal telemetry

---

### Step 2 — Normalize Inputs

Convert external messages into normalized tick inputs.

---

### Step 3 — Execute Deterministic Tick

Invoke engine tick execution using:

- Current state
- Normalized inputs
- Protocol constants

---

### Step 4 — Persist State

If snapshot interval reached:

- Write snapshot
- Validate snapshot hash

Always:

- Append replay log entry

---

### Step 5 — Emit Metrics

Emit telemetry for:

- Trust
- Lock state
- Recovery progress
- Resource accumulation
- Attack pressure

---

### Step 6 — Sleep / Wait Until Next Tick

Maintain deterministic tick timing.

---

## 4. Crash Recovery

On restart:

Load snapshot  
Replay log forward  
Resume runtime loop  

---

## 5. Security Goal

Ensure node runtime behavior is deterministic, replayable,
and resilient to crashes and adversarial input patterns.
