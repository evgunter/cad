---
id: what-t-the-pick-door-answers-and-with-what-width
kind: ruling
title: what t the pick door answers for an admitted candidate, and with what width
status: open
opened: 2026-09-16
---


## The question

`ray_triangle` (`crates/editor-core/src/resolve/pick.rs`) answers a
bare `f64`, and `pick_face` orders candidates by comparing those bare
`f64`s. Both halves of that are now known to be too little:

- **the value**: `t` is the parameter of `a + u·e1 + v·e2` projected
  onto the ray, which is a point of the closed triangle only while `u`
  and `v` are in range — an acceptance that admits either outside it
  places the point off the triangle, and `t` can then precede the
  parameter at which the ray enters the triangle's own bounding box;
- **the width**: `t` inherits the barycentrics' bounds
  (`err_u·|e1| + err_v·|e2|` along the ray, up to the projection), so
  two candidates whose `t` intervals overlap are not ordered by their
  rounded `t` at all — but the tie-break only recognises an EXACT tie.

Three open rows wait on this one and are parked on it. Each named a
shape; the shapes are the same two or three questions seen from
different sides, which is why they are ruled together rather than one
at a time.

## What the rows ask for

- `pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`
  — a candidate at the certification's noise floor (`det = 1.66e-19`,
  conditioning `7.19e-16`, `|det| / bound_det = 5.72`) places the hit
  `0.031` short of a vertex and beats two transversal neighbours
  (`|det| ≈ 2.8e-5`, bounds near `1e-14`) that answer the vertex
  exactly. Shapes: a `t` interval with the tie-break deciding overlap;
  refusing a candidate whose `t` interval exceeds its own box's extent
  along the ray; or (review lane pick2-r2) refusing where the interval
  covers at a DOUBLED bound, which removes this winner and keeps the
  candidate one behind it (item 45, `t = 1.5112`, sum `0.633 ± 0.252`)
  — cheap, but a factor chosen to fit, which is the tuning the door's
  whole derivation avoids.
- `pick-closed-acceptance-loses-a-graze-to-rounding` — 149 of the
  tie-break aim's 19 296 rays answer beyond their aimed point or miss;
  MEET is the acceptance that would recover them and MEET is what
  breaks the box-entry premise above.
- `pick-hit-point-from-an-out-of-range-barycentric-leaves-the-triangle`
  — the box-entry premise itself, with the repair (place the hit at
  the nearest admissible barycentrics) that makes MEET affordable.

`pick-a-corner-graze-verdict-depends-on-the-corner-labelling` is not
parked on this row — it is a defect with its own shapes — but one of
those shapes is "answer it here", so the ruling should say whether it
does.

## What the ruling must answer

1. Does the door answer a `t` or a `t` interval, and if an interval,
   does the tie-break decide overlap the way it decides an exact tie
   today (a documented total order) or does an overlap become a
   refusal?
2. Is the hit point clamped into the closed triangle, and if so by
   what projection — which decides whether MEET can be taken at all.
3. Does the box the tree already proved the triangle lives in enter
   the test? EDIT-PICK withdrew a box-entry guard whose central
   sentence was false in `f64` (`docs/DOC-LEDGER.md`, "Per-merge
   deletion — EDIT-PICK's spec"); any shape that reads the box owes
   that history an argument, not a re-run.

## What it must not do

Introduce a tolerance. EDIT-PICK's spec forbade a tuned `ε` because it
moves tie behaviour by fiat, and EDIT-PICK2's amendment keeps that:
every width here is derived from the operation count or it is not
taken. A "doubled bound" is a tuned factor unless the doubling is
itself derived (for instance, from the mesh's own coordinate error,
which the current bound explicitly does not cover).

## Measurements it inherits

- EDIT-PICK2's three-rule table over the tie-break aim (19 296 rays):
  `main` 149 beyond-or-miss / 0 `Pruned ≠ Every` / 45 winners with a
  bound ≥ 1; MEET ∧ INFORM 129 / 2 / 3; closed ∧ INFORM (landed)
  149 / 0 / 0.
- The wide aim (441 126 rays), landed against `main`: 133 answers
  moved, all farther; 3 aims lost, 15 gained, net `+12`
  (`crates/viewer/tests/review_pick2_r1.rs`).
- EDIT-PICK's own: the projection replaced Möller–Trumbore's quotient
  because the quotient cancels at a small determinant; whatever this
  ruling does to `t` must keep that.
