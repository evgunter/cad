---
id: should-topos-dual-doors-take-tier-3-evidence-by-type
kind: ruling
title: Should topo's doors at a dual take the record's tier-3 evidence by type (option B), or is the record-bound gate the contract's holder?
status: open
opened: 2026-11-10
refs: [topos-doors-at-a-dual-take-no-tier-3-evidence-by-type]
priority: P4
---



The question `topos-doors-at-a-dual-take-no-tier-3-evidence-by-type` parks on. It is Ev's: fork row 196 (`docs/DESIGN-FORK-LOG.md`) records Ev's pushback, "it was supposed to be impossible at the type level for a dual to enter into certification", and option B changes the shape of DUAL-DESIGN DL3's contract. No `[ev]` PR is open for it yet.

**Today the premise is held by a gate:** `scripts/gates/dual-doors-only-through-the-record.sh`. Production code names a concrete dual only on the record-bound path. A gate is not "impossible at the type level", so the issue row stays open.

**Option B, the type-level form.**
- **The witness.** At a dual, `AtRestPolicy::finish` takes the record's finished body for the same node: a topo `RecordWitness<B>` wrapping `&AtRestBody<B>`, pairing-checked by value bits against the dual's body.
- **What carries it.** `Replay` keeps the witness. The premise-bearing readers take a `Tier3Held` bound, which `AtRestBody` and a witnessed `Replay` both meet. The dual still certifies nothing; the evidence is the record's.
- **The ripple.**
  - Every door that finishes a body needs the witness, and the Boolean finishes internally, with no node in hand.
  - editor-core needs a per-node witness lookup into the record.
  - Each body gets a pairing check.
- **Cost:** L.

**Options set aside:**
- each premise site checking its own premise at every scalar, which restates tier 3's checks at about nine sites;
- closing on the contract with no enforcement.
