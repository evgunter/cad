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

## Recommendation (2026-09-16, EDIT orchestrator) — on the `[ev]` PR

**One shape, four parts.**

(a) **Interval, not bare `f64`.** `ray_triangle` answers `[t_lo, t_hi]`
derived from the barycentrics' certified bounds through the projection
(`err_u·|e1| + err_v·|e2|` along the ray, rounded outward), beside the
rounded value. No factor is chosen: every width is the operation
count's, as EDIT-PICK and EDIT-PICK2 require.

(b) **Order.** Candidate A precedes B when `t_hi(A) < t_lo(B)`. Two
candidates whose intervals overlap are a CERTIFIED tie — the geometry
does not order them — and fall to the tie-break, exactly as an exact
tie does today.

(c) **Tie-break: the narrower interval first, then `(target position,
flat position)` as now.** When the geometry cannot say which is in
front, the door prefers the better-certified claim. Derived (the same
bounds), not tuned. It does what the parked rows ask: the noise-floor
winner (width ≫ its two transversal neighbours', hit `0.031` short of
the vertex they answer exactly) loses to a neighbour; a ray down a
shared edge still ties two narrow intervals and the existing order
decides, and both answers are true there. The one case it decides
against a user's likely intent: a face the ray meets nearly edge-on
(wide) in front of a transversal face (narrow) — the wide one loses.
Argued right: a face edge-on to the ray is what the user is aiming
past. The alternative is (c′) the existing order alone, with the hit
reporting `decided_by: Geometry | TieBreak`. Refusing on overlap is
rejected: every shared-edge pick overlaps.

(d) **The hit point is clamped into the closed triangle** (nearest
admissible barycentrics), so `t` is the parameter of a point OF the
triangle. Consequence, not guard: a point of the triangle is inside
the triangle's box, so "a true hit's parameter is never below its
box's entry" becomes a theorem of the clamp in exact arithmetic, and
the traversal's early-out compares `t_hi(best) < t_enter(cand)`. The
box does not otherwise enter the test — EDIT-PICK's withdrawn guard
refused a hit on that comparison and was false in `f64`; this uses it
only to stop scanning, and the cost of its rounding is a near-tie at
ULPs, the class the door's docs already state.

**Left to the unit, measured, not ruled:** whether the acceptance
returns to MEET ∧ INFORM. What broke MEET (2 `Pruned ≠ Every`) was an
admitted candidate whose projection preceded its box entry; the clamp
removes that mechanism. The unit runs the three-rule table (19 296
rays) and the wide aim (441 126) for closed vs MEET under (a)–(d) and
takes MEET only at `0 Pruned ≠ Every` and no lost aim; otherwise
closed stays and the 149 grazes remain the stated class.

**Not answered here:** `pick-a-corner-graze-verdict-depends-on-the-corner-labelling`
— Möller–Trumbore's `(a, e1, e2)` is not symmetric in the corners and
the certified bound inherits that; it stays its own row unless the
unit's measurement shows it vanish under the clamp.

## RULED (Ev, on the `[ev]` PR #2764, 2026-09-16): the recommendation stands

"Ok cool, sounds decided then." The four parts above are the ruling:
(a) `ray_triangle` answers a `t` INTERVAL derived from the barycentrics'
certified bounds through the projection, no factor chosen; (b) A
precedes B when `t_hi(A) < t_lo(B)`, overlap is a certified tie; (c)
the tie-break is the narrower interval first, then `(target position,
flat position)` — the viewer's GPU id pass already picks what is
displayed by construction, and this is what makes the kernel's ray
path agree with it in the edge-on class instead of raising the two
paths' disagreement; (d) the hit point is clamped into the closed
triangle, and the box enters only as the traversal's early-out bound.
Closed-vs-MEET is the unit's to measure under the acceptance written
above. This row is now the unit (kernel unit, v6 dual, block EDIT-B1
slot 2); the three rows parked on it unpark at its merge; the
corner-labelling row stays its own.
