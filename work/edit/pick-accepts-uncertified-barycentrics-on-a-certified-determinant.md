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


## RE-RULED (EDIT orchestrator, 2026-09-16) — the closed comparison ∧ INFORM

The conjunction above was built, executed and measured; MEET trades
order independence and the spec's example was mis-stated. The
orchestrator re-ruled on the measurement to the item's THIRD shape,
alone: a candidate is admitted iff each of `u`, `v` and `u + v` is in
the closed range AND its interval does not cover it. The amendment is
`docs/EDIT-PICK2-SPEC.md` §"Amended at the fix pass (2026-09-16)".
This row no longer closes with
`pick-closed-acceptance-loses-a-graze-to-rounding`, whose carry is
dropped.

## Built (2026-09-16)

**Landed.** `ray_triangle` reads each barycentric as the interval its
own rounding bound gives it and refuses any whose interval COVERS
`[0, 1]`. What that buys, in the item's own terms: a value inside the
range whose bound is `1` or more necessarily covers it, so **no
admitted barycentric — and so no winner — carries a bound that wide**,
which is the defect this row was opened for. The item's counts of
winners at a bound of 1 or more (44 over the tie-break aim, 203 over
the wide aim) are zero by construction, asserted per ray rather than
pinned as a number.

- `barycentric_intervals` is a third public door beside `ray_triangle`
  and `certified_determinant`, answering
  `[(u, err_u), (v, err_v), (u + v, err_sum)]` so a corpus row reads
  the door's own numbers instead of an oracle of its own.
- the bound is derived at the site and shares the certification's
  arithmetic: `triple_bound(a, b, c)` serves all three triple products
  the test evaluates (`e1·(d × e2)`, `s·(d × e2)`, `d·(s × e1)`) and
  `certify` now answers the determinant WITH its bound. One further
  counted constant, `QUOTIENT_ERROR_UNITS = 2`, for the division's two
  roundings. No tuned number.
- the acceptance is spelled once, in `admits(x, err)`.
- rows: the closed boundary pins keep their one-ULP-each-way shape
  (nothing on an exact fixture is uninformative); a static `ζ = 2⁻²⁰`
  near-tangent fixture whose `k` dial moves the intervals without
  moving `u = v = 0.5`, at `k = 8` killing the drop-INFORM mutant and
  at `k = 32` killing halve-the-bound; `index_memo`'s
  `reference_answers` asserting per ray that no winner's bound reaches
  `1`, beside the `Pruned == Every` claim it already carried.
- `review_pick_r2`'s tally re-baselined: rays answered at the aimed
  vertex `141 094` → `141 106`. The other three columns do not move —
  they count refusals AT the determinant, which this change does not
  touch, so the spec's premise 2 was mis-stated as well.

Verified by CI run `35066248937` on head `7d6222b4c`: a code-tier
run, twelve `test (…)` jobs and five `k-lint (gate, …)` jobs, green.

**What did not land, and where it went.** The example ray is a
measurement row, not a fix: its candidate's intervals are
`0.367 ± 0.466`, `0.459 ± 0.218`, `0.827 ± 0.684` — wide but
informative, admitted by every shape of the ruling — and the class is
`pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`,
a ruling about `t`'s own interval. MEET's own cost is
`pick-hit-point-from-an-out-of-range-barycentric-leaves-the-triangle`.
The graze-loss row stays open with its carry cleared.
