---
id: an-atom-over-a-gated-argument-is-another-indeterminate
kind: issue
title: An atom is keyed by its argument's gate, so atan(max(x, 3)) and atan(3) are two indeterminates: a zero the read would reach through an atom stays numeric
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [DECIDE-10, the-read-at-its-node-relabels-a-cancellation-above-it]
---


Found by DECIDE-10's Phase 1 (the item
`the-read-at-its-node-relabels-a-cancellation-above-it`, "What Phase 1
found", rows S8 and C6).

An opaque atom is keyed by its argument's DIGEST (`combine`'s `atom1`
and the two-argument atoms in `crates/geom-core/src/sym.rs`), and
`Form::digest` (`crates/geom-core/src/sym/form.rs`) hashes the `gated`
flag with the polynomial. So an atom over a gated argument is a
different indeterminate from the same atom over the same polynomial
ungated, though the two stand for one real at every point of the box.

Measured on DECIDE-10's probe (`decide/10-phase-1-probe`,
`decide10_probe::decide10_shapes`), `x ∈ [1, 2]`:
- `atan(max(x, 3)) − atan(3)` is NUMERIC with the read on and with it
  shut. With the read on, `max(x, 3)` reads `3` gated, and `atan(3
  gated)` is not `atan(3)`. With gate-free keys the difference would be
  the zero form, gated: `sign_gated`.
- `atan(max(x+Z, 3) − max(x, 3) + x) − atan(x)` was refused with the
  read on before DECIDE-10, for the same reason. DECIDE-10 answers it
  through the walk with the reads shut, not through the key.
- `sqrt` does not show it: rule G keys a root by its argument's value
  class (`sym/root.rs`).

The direction is conservative (a missed discharge, never a wrong one),
so it is a reach row, not a soundness one. A fix keys atoms on the
polynomial alone and lets the minted form carry the gate (`atom1`'s
`gate` already does). It would move counts from `numeric` to
`sign_gated` wherever a document has such a pair, so it is measured
first on the documents.
