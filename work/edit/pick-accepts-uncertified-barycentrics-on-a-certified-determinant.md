---
id: pick-accepts-uncertified-barycentrics-on-a-certified-determinant
kind: unit
title: the exact test accepts barycentrics whose rounding error exceeds the acceptance interval when the determinant is certified but small
status: closed
opened: 2026-09-16
branch: edit/pick-barycentrics
pr: 2746
closed: 2026-09-16
---


## The finding

`ray_triangle` (`crates/editor-core/src/resolve/pick.rs`) certifies
the determinant and takes `t` from the hit point's projection, but
accepts `u` and `v` on their rounded values alone. When the
determinant is certified and small — a ray a few units of roundoff
out of a planar face's plane, not through a vertex — the numerators
`s·p` and `d·q` carry absolute errors of order `u·|s|·|p|` that the
division by the tiny determinant magnifies past the width of the
acceptance interval: `u` and `v` land in `[0, 1]` by chance, the
projected `t` is the parameter of a point that is not the crossing,
and the candidate can win.

Measured at the fix pass of
`pick-grazing-ray-answer-depends-on-candidate-order` with a derived
bound on the barycentric error (`(err_N + |u|·bound_det) /
(|det| − bound_det)`, `err_N` from the operation count): over the
tie-break row's aim at every landing of the corpus and the ring,
44 of 36 578 winners and 340 of 247 483 accepted candidates carry a
barycentric error bound of 1 or more (125 winners at 1e-2 or more);
over the wide aim (`review_pick_r2`, 441 126 rays), 203 winners and
4 014 accepted candidates at 1 or more (1 394 winners at 1e-2). One
on the ring after its bump: the −y ray through
`(0.24519632010080758, ·, 0.04877258050403218)` answers the flat face
at `t = 1.4488` — 0.03 before the aimed vertex at `1.48` — from
`u`, `v` with an error bound near 100. `main`'s kernel answered the
same class with its own noise (`1.4694` there), so nothing regressed;
nothing closed either.

The fix is a ruling before it is a unit, because the three shapes
change the closed-boundary contract differently:

- **certify inside** (`u − err ≥ 0`, `u + err ≤ 1`, and the same for
  `v`): refuses every genuine vertex and edge graze, whose true `u`
  or `v` is `0` and cannot be certified either side — the tie-break
  row's whole class;
- **accept the interval** (`[u − err, u + err] ∩ [0, 1] ≠ ∅`): the
  closed contract with its rounding made explicit, which also
  answers `pick-closed-acceptance-loses-a-graze-to-rounding`, and
  which the spec's trap names because it moves ULP-level tie
  behaviour;
- **refuse the uninformative** (the interval covers the whole
  admissible range): no constant, but a candidate whose interval
  reaches outside on one side only is still accepted on a rounded
  value.

Whichever is ruled, the bound is derived at the site from the
operation count, never tuned, and the rows that pin it are the
`index_memo` differential and `review_pick_r2`'s tally.

## RULED (EDIT orchestrator, 2026-09-16)

**A candidate is admitted iff each of `u`, `v` and `u + v` is in the
closed range `[0, 1]` AND its derived interval does not COVER that
range** — the item's third shape, alone. The two halves compose into
what this row was opened for: a value inside the range whose bound is
`1` or more necessarily covers it, so no admitted barycentric carries
a bound that wide. The bound is derived at the site from the operation
count, never tuned.

This row does NOT close
`pick-closed-acceptance-loses-a-graze-to-rounding`: closing that one
needs MEET, and MEET was built and measured to trade order
independence. A sequencing decision taken under the program's posture
rather than put to Ev; block EDIT-B1 slot 1.

*How it got here, in one line: the first ruling was the conjunction of
MEET and INFORM, the lane built it and measured it wrong at its
centre, and the orchestrator re-ruled on that measurement.
`docs/EDIT-PICK2-SPEC.md` §"Amended at the fix pass (2026-09-16)"
carries the table and the two mis-stated premises.*

## Built (2026-09-16)

**Landed.** `ray_triangle` reads each barycentric as the interval its
own rounding bound gives it and refuses any whose interval COVERS
`[0, 1]`. What that buys, in this row's own terms: the item's counts
of winners at a bound of 1 or more — 44 over the tie-break aim, 203
over the wide aim — are zero, and zero by construction rather than by
measurement, because a value inside `[0, 1]` with a bound that wide
necessarily covers it. Asserted per ray on both aims
(`index_memo`'s `reference_answers`, `review_pick_r2`'s sweep) rather
than pinned as a count, since a pinned `0` reads as a baseline.

- **One door.** `crossing(ray, tri)` answers
  `Crossing { det, bound_det, barycentrics: [(x, err); 3] }` or `None`
  on an uncertified determinant, computed once; `ray_triangle` and
  every corpus row read it. `certified_determinant` and
  `barycentric_intervals` are gone — three public doors onto one
  evaluation were three chances to disagree.
- **The bound is derived at the site**, sharing the certification's
  arithmetic: `triple_bound(a, b, c)` serves all three triple products
  the test evaluates, `certify` answers the determinant WITH its
  bound, and two more counted constants cover the quotient's two
  roundings (`QUOTIENT_ERROR_UNITS = 2`) and the sum's one
  (`SUM_ERROR_UNITS = 0.5`). The derivation lives at one site
  (`crossing`'s doc), says what the bound is a bound ON (this
  evaluation's rounding, the operands taken as exact — not the mesh's
  own coordinates), and `quotient`'s doc accounts for the bound's own
  roundings against the slack in the constants.
- **The mutants are mis-read bounds, not a second predicate.**
  `Door::bound(i, err)` hands the REAL `admits` a scaled bound — `0`
  for a door that ignores the interval, `err/2`, `2·err`, and `0` on
  the sum alone — so a row cannot drift from the acceptance it
  mutates.
- **Rows**, and which mutant each kills: the one-ULP boundary pins
  kill MEET (`u` one ULP above `1` carries an eight-ULP bound, so MEET
  admits it); `near_tangent(8)` kills drop-INFORM; `near_tangent(32)`
  kills halve-the-bound; `near_tangent(64)` kills double-the-bound (a
  candidate the door admits that a `2×` bound refuses — the tight side
  of the bound, which nothing pinned before); a `k = 18` fixture aimed
  at `(0.25, 0.125)` kills INFORM-without-the-sum, which
  `near_tangent`'s symmetric `u = v = 0.5` cannot show. Adopted from
  the review lanes: that sum fixture, the ends-of-`admits` row, and
  the corner-labelling row.
- **`review_pick_r2`'s tally re-baselined**: rays answered at the
  aimed vertex `141 094` → `141 106`. The other three columns do not
  move — they count refusals AT the determinant, untouched — so the
  spec's premise 2 named the wrong pair.

**What it costs.** The `+12` is a NET of `+15` and `−3`
(`crates/viewer/tests/review_pick2_r1.rs`): 133 answers over the wide
aim moved against `main`, every one FARTHER, and three aimed-vertex
grazes on `cut_cylinder` are lost. The bound does not vanish where a
barycentric does, so a corner graze on a candidate at the
certification's floor is refused too. That is now stated at
`ray_triangle`'s door instead of the false claim that it costs no
graze, appended to
`pick-closed-acceptance-loses-a-graze-to-rounding`, and its
labelling asymmetry filed as
`pick-a-corner-graze-verdict-depends-on-the-corner-labelling`.

**What did not land, and where it went.** The example ray is a
measurement row, not a fix: its winner sits at the certification's own
floor (`det = 1.66e-19`, conditioning `7.19e-16`,
`|det| / bound_det = 5.72`) with intervals `0.367 ± 0.466`,
`0.459 ± 0.218`, `0.827 ± 0.684` — wide but informative, and admitted
by every shape of the ruling. The class is
`pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`.
It, the graze-loss row and
`pick-hit-point-from-an-out-of-range-barycentric-leaves-the-triangle`
are parked on the ruling row
`what-t-the-pick-door-answers-and-with-what-width`, which is the one
question all three name from different sides.

Verified by CI run `35076081918` on head `23fe58c8b` (the fix pass):
code-tier, twelve `test (…)` jobs and five `k-lint (gate, …)` jobs,
33 green and 6 skipped by the change filter, none failed. The
re-ruling's own run was `35069024406` on `7d6222b4c`.

## Closed (2026-09-16, EDIT orchestrator)

Merged as PR #2746 after the v6 dual (ordinal 4801, sample #214; both
reviews APPROVE-WITH-FIXES and convergent, no tally candidate) and the
union fix pass. The residue is three rows parked on the ruling row
`what-t-the-pick-door-answers-and-with-what-width`, plus the
labelling-dependence row this unit's review found.
