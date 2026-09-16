---
id: pick-accepts-uncertified-barycentrics-on-a-certified-determinant
kind: unit
title: the exact test accepts barycentrics whose rounding error exceeds the acceptance interval when the determinant is certified but small
status: review
opened: 2026-09-16
branch: edit/pick-barycentrics
pr: 2746
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

## RULED (EDIT orchestrator, 2026-09-16) — the conjunction, spec'd as EDIT-PICK2

A sequencing decision with a recommendation, taken per the program's
posture rather than put to Ev: of the three shapes, the first kills
grazes and the second and third each leave half the defect, so the
ruling is their conjunction from one derived bound — a candidate is
accepted iff its barycentric interval MEETS the closed range and does
not COVER it. `docs/EDIT-PICK2-SPEC.md` binds the unit; block EDIT-B1
slot 1. This row and `pick-closed-acceptance-loses-a-graze-to-rounding`
close together.

## Built (2026-09-16) — the ruled conjunction, built, measured and REVERTED

The ruling was implemented in full and executed; three of
`docs/EDIT-PICK2-SPEC.md`'s six acceptances are unreachable under it,
and the spec's premise 1 is false on the tree. Nothing of the
acceptance change landed. The implementation is recoverable at
`git show f096d84c5` (this branch): the derived interval
(`barycentric_intervals`, one forward bound per barycentric in the
`DETERMINANT_ERROR_UNITS` style, sharing the triple product's bound
with the certification), the MEET ∧ INFORM acceptance, the re-stated
boundary pins with their three mutants, the uninformative fixture and
the example ray's row.

**Premise 1 is false.** The spec says the example candidate carries
"a barycentric error bound near 100" and "refuses at INFORM". It does
not. On the bumped gallery ring the `−y` ray through the vertex
`(0.24519632010080758, 0, 0.04877258050403218)` at `reach = 1.48` has
flat triangle 35 as its winner, `det = 1.66e-19`, and the derived
intervals are `u = 0.367 ± 0.466`, `v = 0.459 ± 0.218`,
`u + v = 0.827 ± 0.684`. None of the three covers `[0, 1]`, so the
candidate is INFORMATIVE by the ruling's own definition and is
admitted; the ray still answers `t = 1.4487652724897624`, `0.031`
before the vertex. No reading of INFORM as ruled refuses a bound of
`0.47`, so acceptance 1 is unreachable under the conjunction, under
INFORM alone and under MEET alone.

**The three rules measured**, over `index_memo`'s tie-break aim at
every landing of the corpus and the gallery ring (19 296 rays;
`git show f096d84c5:crates/viewer/tests/index_memo.rs`, `probe_tie_aim`):

| acceptance | aimed rays answering beyond the aim or missing | `Pruned` ≠ `Every` | winners with a bound ≥ 1 | widest winner bound |
| --- | --- | --- | --- | --- |
| the closed comparison (`main`) | 149 | 0 | 45 | 7.35 |
| MEET ∧ INFORM (ruled) | 129 | **2** | **3** | 1.99 |
| the closed comparison ∧ INFORM | 149 | 0 | 0 | 0.684 |

So: acceptance 3 (`Pruned == Every`, "order independence is not
traded") is broken BY the ruling — MEET admits a barycentric outside
`[0, 1]` and the hit point then leaves the triangle, filed as
`pick-hit-point-from-an-out-of-range-barycentric-leaves-the-triangle`.
Acceptance 5 ("zero winners with bound ≥ 1") is false under the
conjunction and true only under the closed comparison ∧ INFORM, where
a value inside `[0, 1]` with a bound ≥ 1 necessarily covers the range.
MEET's own gain is real but small: 20 of 149 aimed rays recovered,
so the carried row `pick-closed-acceptance-loses-a-graze-to-rounding`
does not close either.

**What the measurement says the ruling should be**, as a
recommendation and not a decision: the two halves do not compose the
way the ruling assumed. INFORM is the half that removes the noise
class, and it does so only while the comparison that bounds `u` and
`v` keeps them inside the range — MEET is what lets a wide interval
sit outside `[0, 1]` and still win. Closing the graze row as well
needs MEET plus an answer to the box-entry row above, and closing
THIS row needs a line on the bound that `0.47` falls the wrong side
of, which the ruling's "covers the admissible range" is not.
