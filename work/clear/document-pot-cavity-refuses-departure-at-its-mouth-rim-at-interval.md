---
id: document-pot-cavity-refuses-departure-at-its-mouth-rim-at-interval
kind: issue
title: the editor-core corpus pot's cavity refuses Departure at its mouth rim at Interval, though the same pot built in sweep shells Ok, so SHELL-4's gate over-refuses a valid document body
status: dispatched
opened: 2026-10-31
priority: P1
cost: M
---


## Finding

SHELL-4's lane found this on resume, on `clear/shell4` at `3a6c20799f`,
which has main merged in after FRUSTUM, SPIRIC and EPSROWS.

- **The body.** The pot in editor-core's corpus `vessel::document()`,
  evaluated at `Interval`
  (`lib_g17_shell_node::both_documents_evaluate_green_at_the_interval_scalar`).
- **The operand** certifies: Holds, `cornered` 0, refused 0.
- **The shell refuses**, sealed or opened:
  `CavityUncertified { Uncertified { a: 2v1, b: 6v1, shared: Edge(12v1), arm: Departure } }`.
  - `2v1` is the mouth plane, `y = 1/8`.
  - `6v1` is a sphere belly band, centre `y = 1/16`, `r = 5/64`.
  - `Edge(12v1)` is the mouth rim circle, `r = 3/64`.
  - All the geometry is exact and dyadic.
- **The same pot built in sweep shells Ok** at `Interval` and at
  `Sym<Interval>`. That is CAVITYDIAG's `corpus_pot`, which uses
  `bulge_from_center` and a sweep-side revolve.

So the refusal depends on how the document builds the pot, not on its
surfaces. The candidates are the `ProgramArcData::Center` arc and the
document's revolve: where the seam sits, how the band is split, or which
way it is oriented.

Over-refusing a valid shell is ruled out (SHELL-4's ruling), so SHELL-4
is parked on this row.
