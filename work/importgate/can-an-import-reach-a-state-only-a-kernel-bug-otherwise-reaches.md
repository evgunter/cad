---
id: can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches
kind: issue
title: design: can an import reach invalid states that are otherwise reachable only by a kernel bug, and when and how is an import validated?
status: open
opened: 2026-10-14
priority: P2
cost: M
---

(Split out at Ev's request on `[ev]` PR 5134, 2026-10-14: "it might make sense for 'can imports reach otherwise unreachable invalid states' / 'when/how are imports validated' to be a separate design question (basically the same question just came up on another PR)". Filed by the ENCL orchestrator; EXCH owns adoption, D7.)

## Why it matters

D2 says the kernel never panics on input, and that a state reachable only by a kernel bug must panic. Which side of that line a stored contradiction falls on depends on whether an import can reach it:

- a reversed parameter span;
- a stored tangency where the edge says the faces cross;
- a negative dihedral arm;
- more generally, any state the kernel's own constructions never mint.

`[ev]` PR 5134 waits on this answer.

## Current state (read-only survey, 2026-10-14; read from source, not executed)

**What adoption certifies.** Every edge is certified at ε twice:
- `step_import::adopt::adopt_edges` runs the description ladder through `Body::set_edge_curve`;
- `gate3` runs the full tier-3 battery.

A refusal refuses the import. Nothing is healed after certification.

**What adoption keeps from the file.** It does not rebuild the file's data. It keeps the carriers, vertex points and surfaces verbatim or promoted at ε_in (D4 ¶1's "until adoption rebuilds its caches"). Most other passes interpret or heal:
- units and directions;
- ε_in recognition and promotion;
- apex, band and sphere re-mints;
- the outer-loop inference.

**The import door is read "as at rest".** `ReadAt::File(_).reading()` returns `Reading::AtRest`. So file-reachable refusals and kernel-bug-only refusals share one reading.

**Known holes:**
- **Negative radius:** a cylinder, sphere or torus radius below zero passes `entities.rs` unchecked and reaches edge certification before check 1. Filed as `step-import-reads-a-negative-surface-radius-unchecked-before-edge-certification`.
- **Unrebuilt data:** the tier-3 deferrals apply to imported bodies as to native ones. Data adoption doesn't rebuild is certified against the kernel's *guessed* description, not one the file stated.

**Other doors.**
- **Document load** rebuilds from the recipe (D8).
- **pncad-py** builds bodies only by evaluation, assembly or import.
- **A Rust caller** of `topo` gets certifying setters, apart from `set_face_sense` and an un-gated `flipped_face_sense_for_tests`. Filed as `flipped-face-sense-for-tests-is-not-feature-gated`.

## The question

When and how should an import be validated? Can a body that passed import hold a state the kernel otherwise reaches only by a bug? Specifically:
- Should adoption rebuild the file's data from its descriptions, or validate it fully before any certification reads it?
- Is the import door its own reading, distinct from at rest?
- Which refusals belong there, typed and naming the file, rather than past it, where they panic?

Weigh it with the designer pair before it goes to Ev.

## Waiting on this

- `[ev]` PR 5134 (ENCL, the proven-zero fork): whether a definite contradiction past the import door panics. **Settled 2026-10-19 (merged):**
  - `[tolerance.recourse]` (iv) now says that past the import door a stored contradiction or an unminted side panics, and that at the door it is refused typed, naming the file.
  - ENCL's panic arms (`a-proven-zero-is-definite-yet-ends-as-no-size-at-rest`) wait on this row closing the import gaps: carriers kept verbatim, and a negative radius unchecked. Until then a damaged file would panic. (ENCL orchestrator)
- `[ev]` PR 5113 (PCERT, "what certification owes a row at a band-sized lever"): its decision 2 "waits on your design conversation about imports" (Ev named it as the same question, 2026-10-14).

## Ev, 2026-10-17 (review of `[ev]` PR 5230)

> "imported geometry should never be distinguishable from native geometry--there should be no states that are only reachable via import"

PR 5230 states this at DESIGN `[tolerance.boundary]`, with a pointer from `[import]`. That answers the first half of this row's question: import reaches no state the kernel's own constructions never mint, and it refuses such a state at the door. When and how an import is validated is still open here.
