---
id: pick-door-answers-a-t-interval
kind: unit
title: EDIT-PICK3: the pick door answers a t interval, orders by it, breaks a certified tie by width
status: closed
branch: edit/pick-t-interval
pr: 2786
opened: 2026-09-16
closed: 2026-09-17
---


Builds the ruling `what-t-the-pick-door-answers-and-with-what-width`
(Ev, `[ev]` PR #2764, 2026-09-16). Kernel unit, v6 dual, block
EDIT-B1 slot 2.

## Spec (2026-09-16, EDIT orchestrator)

`docs/EDIT-PICK3-SPEC.md` (deleted at merge, recorded in the ledger).
Branch `edit/pick-t-interval`. Closes the ruling row at merge, which
unparks `pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`,
`pick-closed-acceptance-loses-a-graze-to-rounding` and
`pick-hit-point-from-an-out-of-range-barycentric-leaves-the-triangle`
(each then closes on the unit's measurement or stays with a reason);
`pick-a-corner-graze-verdict-depends-on-the-corner-labelling` is
measured, not assumed.

## Built (2026-09-16)

`ray_triangle` answers [`TSpan`] — the rounded `t` with the interval
`[t_lo, t_hi]` the arithmetic certifies around it, derived at `t_span`
from `crossing`'s barycentric bounds through the projection plus the
projection's own rounding (`PROJECTION_ERROR_UNITS`, counted from the
operation list). The hit point is clamped into the closed triangle.
`pick_face` drops every candidate another candidate PRECEDES and breaks
the resulting certified tie by the narrower interval, then by `(target
position, flat triangle position)`; the traversal's early-out compares
the smallest upper end seen against the next candidate's box entry.
`PickHit` carries `t_lo` and `t_hi`; `pncad` re-exports it unchanged and
`pncad-py` spells both (LIB's files, mechanically).

Three premises of the spec were measured before being built on, and two
moved:

- **The order is a rule about a SET, not a pairwise comparison.**
  "A precedes B, else the narrower wins" applied pairwise has
  three-cycles, so a fold over it answers whichever candidate the
  traversal met first. The candidates no other candidate precedes are
  pairwise overlapping, so they are ONE certified tie; width-then-
  position orders that set totally. Pinned by
  `the_certified_order_does_not_depend_on_the_arrival_order`.
- **The gallery ring's wide candidate is not a tie and does not lose.**
  The certified width is relative to the TRIANGLE: the ring's triangles
  are `0.016` on a side, so the noise-floor candidate's interval is
  `0.030` across and lies wholly before the vertex `0.031` further on.
  It PRECEDES; the tie-break never runs; the fixture still answers
  `1.4488`. The class the parked row names is pinned where the
  intervals do overlap — `tube_arc`'s ray, where `main` answered
  `0.0041` short and the tie-break now takes the vertex.
- **Closed ∧ INFORM stays.** Measured both ways under the clamp
  (`crates/viewer/tests/pick3_acceptance.rs`): MEET now costs nothing in
  the early-out column (`0 Pruned ≠ Every`, the clamp removed
  EDIT-PICK2's mechanism) but loses 513 aimed vertices of the wide aim's
  441 126 rays against `main`'s 141 106, gaining none. The closed
  comparison loses none and gains three.

What did not land: nothing the spec asked for. The corner-labelling
asymmetry survives the clamp and stays its own row.

## After the review (2026-09-16, fix pass)

Two blinded reviewers read the frozen head `31cbee19f` and agreed on
one MAJOR. **The early-out was unsound for the width tie-break this
unit added.** The break fired on `t_hi(best) < t_enter(cand)` while
membership of the certified tie is `t_lo(cand) ≤ t_hi(best)`, so a
candidate whose interval reaches back below its own box's entry was
pruned — and when it is the narrower of the two it is the answer the
rule names. `pick3-r2`'s probe showed the consequence outright: the
same two triangles answered differently depending on which target was
offered first, which `pick_face`'s determinism contract forbids.

The repair derives the margin rather than loosening the sentence.
`early_out_margin`, sited beside `t_span`, bounds `t_enter − t_lo` for
every admitted candidate of one triangle, term by term with no chosen
factor: the box's projection spread, the interval's half-width (both
`≤ (|e1| + |e2|)/|d|`, the second because `admits` refuses `err ≥ 1`
and rounding is monotone), twice `t_span`'s `from_rounding`, and the
outward `next_down`. The scan drops a candidate only where that proves
it preceded, so `Pruned == Every` is a theorem, proved in four lines at
`pick_face`. The break became a per-candidate SKIP: `candidates` already
materialises and sorts the whole list, so stopping saves no traversal,
and the only bound valid for every later candidate is the tree's root
diagonal — a margin no scene clears.

Also landed:

- **`TSpan::best_of`** is the one spelling of the certified order.
  `pick_face`'s fold calls it, and so do `index_memo`'s reference,
  `review_pick_r2_probes::nearest` and `review_gui1_r1`'s tie-break.
  GUI-1 R1's exact rational oracle stays independent and says so. The
  `Order`/`Break` mutant enums retired; the mutants are RUN and
  reported in the PR body, not shipped.
- **`retract_to_simplex`** is the clamp, named for what it is — the
  per-coordinate retraction, which fixes the simplex and is not the
  metric projection: `(1, 1)` goes to `(1, 0)`, not `(0.5, 0.5)`. Its
  `v` bound is now the EXACT `1 − u`, because `fl(1 − u)` rounds up at
  `u ≤ 2⁻⁵⁴` and would place the answered point one ulp of `|e2|` off
  the triangle — the premise the early-out proof rests on.
- **Both reviewers' early-out probes are now rows of the unit's suite**
  (`crates/editor-core/tests/pick3_early_out.rs`, authorship noted),
  re-expressed through `pick_face` rather than a hand-run traversal,
  with a new row where two candidates of EQUAL width fall to the
  earlier target.
- **The acceptance suite measures through the real door, once.** The
  MEET arm, its copy of `t_span` with the literal `8.0`, and its copy
  of the traversal are gone; the measurement is recorded below rather
  than asserted as a permanent gate. What stays is `Pruned == Every`
  through `PickIndex::pick` against an exhaustive walk over
  `ray_triangle` and `TSpan::best_of`, and the closed column's
  `aim_lost == 0` and aimed-vertex count. One corpus pass instead of
  three. Wall time, measured three ways because R1's 73 s and a
  release build are not the same number: **9.7 cpu-s in CI**
  (run `35165894343`, `test (eps = 1e-12, 2/2)`'s slowest-tests
  table), **10.6 s** in a local release build, **67 s** in a local dev
  build against R1's 73 s for the three-pass suite in the same
  profile. Roughly flat, not the 7x the release figure alone suggests:
  the pass that went away paid for `PickIndex::pick` resolving a name
  on every one of 460 422 rays.

### The acceptance measurement, as of 2026-09-16

Closed ∧ INFORM stays. MEET ∧ INFORM reaches `0 Pruned ≠ Every` under
the clamp — the clamp did remove EDIT-PICK2's mechanism — and loses
**513** of the wide aim's 441 126 aimed vertices against `main`'s
141 106, gaining none, because the retraction places an out-of-range
candidate's answer on a real point of its own triangle at a smaller `t`
than the vertex the ray was aimed at. The ruling's rule was "MEET only
at `0 Pruned ≠ Every` and no lost aim"; the second half fails. The 149
grazes remain the stated class.

Re-measured after the repair: tie-break aim 19 296 rays, `Pruned ≠
Every` **0**, beyond-or-miss 149, wide winners 0. Wide aim 441 126 rays,
503 answers moved and every one farther, aimed vertices 141 109 against
`main`'s 141 106 — 0 lost, 3 gained.

### Rows filed

- `pick-tie-break-width-key-depends-on-scene-magnitude` (EDIT) — the
  width key reads coordinate MAGNITUDES through `t_span`'s second term,
  so an exact tie between congruent faces is decided by where the scene
  sits. A class the ruling did not discuss; filed, not built.
- `pick-wide-candidate-needs-a-bound-over-mesh-coordinate-error`
  (EDIT) — the scheduled residue of
  `pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`,
  which closes at this merge with nothing else left of it.
- `pickindex-merges-parts-on-a-rounded-t-it-never-converts` (VIEW) —
  `pick_for` carries `t_lo`/`t_hi` through a display frame unconverted,
  merges parts by rounded `t`, and slacks occlusion by a tuned `1e-6`.
  Cited from `PickHit::t` and `pncad.pyi`.
- The CURVED row this unit filed gained its counterexample, copied out
  of run `35152647206`'s job log before it expires (seed, pose, both
  disagreeing roots).

## Closed (2026-09-17, EDIT orchestrator)

Built and merged as PR #2786 (kernel unit, v6 dual on the frozen
head `31cbee19f`: R1 opus 1/6/12 rubric 3/3/2, R2 fable 1/3/6 rubric
4/3/3, both APPROVE-WITH-FIXES and both on the same MAJOR — the
traversal's early-out pruned a narrower member of the certified tie, so
the answer depended on target order; the fix pass derived the margin
with no chosen factor and `Pruned == Every` is a theorem). Recorded as
sample #215 in `docs/MODEL-AB-LOG.md` (ordinals 4802/4803; block
EDIT-B1 concluded and its record folded to main with this merge); the
spec deleted with its `docs/DOC-LEDGER.md` entry. The ruling row closes
with it, and the two rows parked on the ruling. Residue in their own
files: `pick-tie-break-width-key-depends-on-scene-magnitude`,
`pick-wide-candidate-needs-a-bound-over-mesh-coordinate-error`,
`work/view/pickindex-merges-parts-on-a-rounded-t-it-never-converts`;
`pick-a-corner-graze-verdict-depends-on-the-corner-labelling` and
`pick-refuses-a-crossing-within-rounding-of-a-plane` stand as the
mechanism's stated classes. Premise 5's inequality and premise 4's
word were amended at the fix pass with the decision unchanged; Ev is
told on the next `[ev]` PR.
