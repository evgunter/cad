# TESS-2 — the patch bound encloses the DESCRIBED patch: knot refinement inside the ring (spec)

Binding on the implementer, alongside
`docs/prompts/implementer-discipline.md`. Item: `work/tess/TESS-2.md`.
Carries
`work/tess/rational-cells-hull-the-f64-refined-net-so-the-described-patch-escapes.md`
(read it in full — the measurements are there) and closes
`work/tess/nurbs-face-bound-unsound-on-a-random-rational.md`.
Evidence branch: `origin/tess/nurbs-bound-diag` (`db4cb45a8`) —
`diag/exact_check.py`, `diag/ingredient_check.py` (exact rational
referees), `diag/logs/`, and `mod diag` in `nurbs_cert_fuzz.rs`.

**An announced territory crossing.** `crates/geom-brep/src/patch_bound.rs`
is ENCL's path and `crates/geom-core/src/*` PROPS'; neither has a live
lane and Ev said not to wait. Put `python3 scripts/work.py territory
--base origin/main` output in the PR body.

## The claim

`geom_brep::patch_bound` promises signed enclosures of a patch's
partials per refined cell, outward-rounded. Every step after refinement
is; the refinement is not. `rational_cells` and the integral branch of
`patch_cells_refined` call the plain-`f64`
`NurbsSurface::refine_knots_u/v` and wrap the rounded result as
`RingInterval::point`, so the cells enclose the refined-`f64` patch and
the described patch escapes by insertion rounding, amplified by knot
differencing. Proven in exact arithmetic: the described surface's true
`‖S_uu‖` exceeds `mesh`'s `muu` by 3e-16 relative at a bilinear
rational patch's corner, and `w_u`, `w_v`, `Ã_u`, `Ã_v`, `w` hulls are
each violated on that cell.

After this unit every cell is an enclosure of the DESCRIBED patch's
partial on that cell, with no carve-out.

## The shape of the fix (binding) and what is yours to choose

**Binding.** The refined homogeneous nets (`w`, and `w·P` per channel)
are computed IN THE RING from point intervals of the described net, so
insertion widens outward like every later step. Each insertion ratio
`α = (t − k_j)/(k_{j+p} − k_j)` is itself enclosed — subtraction and
division of point intervals of the knots — never an `f64` quotient
widened by a guessed number of ULPs, and never a padding constant on
the sups: the diagnostic lane could not bound the dust, and a measured
pad is a fudge with a number nobody re-takes. Insertion on the
homogeneous nets is the polynomial (unit-weight) affine combination, so
no rational `λ` is needed. The inserted knots themselves are the `f64`s
`split_points` chose, exactly; the cell extents are unchanged.

**Yours, argued in the PR.** Where the ring insertion lives.
`geom_core::spline::algebra`'s `CurvePlan` already computes the
insertion SCHEDULE (which slots combine, in what order) and carries an
`f64` `lambda` per `Combo`. A second hand-written Boehm loop in
`patch_bound` would be the near-duplicate the style lane exists to
catch — and the first thing to drift. Prefer one schedule with two
arithmetics: e.g. the plan exposing, per `Combo`, the knots its ratio
is made of, so a ring applier re-derives `α` enclosed. If you conclude
a local loop is right, say why the schedule cannot be shared, and
write the row that holds the two in step (same targets, same sources,
on a sweep of knot vectors).

## Phase 1 — before touching the bound (measure-first)

1. Make the defect a red ROW on main's code, not a fuzz draw: the
   diagnostic's bilinear rational patch (bits in
   `diag/logs/diagA.log`; weights ≈ 0.0103–0.0135, knots `[0,0,1,1]²`)
   with the exact referee's true `‖S_uu‖(0,0)` rounded DOWN to an `f64`
   literal, asserted `<= muu`. It must be red before your fix and you
   say so in the PR with the 17-digit numbers. Add the second surface
   (seed `0x5ca58da03160d407`, trial 29) the same way. Say in a comment
   how the literal was produced (exact rational arithmetic; the script
   is on the evidence branch) — a reader must be able to re-derive it.
2. The structural-zero witness: `PatchCell`'s doc records the quarter
   cylinder's `z` enclosure of `S_vv` as `[-4.1e-15, -3.4e-15]` on 6 of
   256 cells, EXCLUDING the true zero. Find the row that pins or prints
   it; after the fix every such enclosure contains 0, and that is a row.
3. Measure what the ring insertion costs on the tour's rational faces
   (wall-clock of `nurbs_face_bound` before/after on a few bodies; perf
   here is reported, never gated — `memories/perf-measurement-lane.md`)
   and how much WIDER the sups get (ratio new/old per component over
   the fuzz census). A widening beyond ~1e-12 relative is a finding
   about your implementation — say so rather than ship it.

## Phase 2 — the fix and what it makes true or false

- `rational_cells` and BOTH branches of `patch_cells_refined`.
  `offset_meters`' consumers (`cell_normal`, `patch_regularity`,
  `cell_curvature`, `patch_collapse`) inherit; re-read their docs,
  which argue the dust harmless at ε scale, and re-word what no longer
  holds. `RefinedWeightLostPositivity` now fires on the weight
  enclosure's `lo`; keep it typed.
- Prose: `PatchCell`'s "What the enclosure encloses" section and its
  "must not use them on the rational arm" warning retire or shrink to
  what is still true; `nurbs_cert.rs`'s header "interval (ring)
  arithmetic end to end" becomes true — check every other sentence in
  both headers against the code while you are there (style Q5). Grep
  for who CITED the caveat (style Q4): any consumer that avoided a
  structural predicate because of it is named in the PR, not changed.
- The falsifier: `r1_random_rational_soundness_sweep` draws bilinear
  patches one trial in nine, which is why two hosted hits took
  thousands of runs. Give the tight stratum its own deterministic
  coverage (read `memories/test-suite-cost.md` on which SHAPE of row
  this is before giving anything a seed), and run a bilinear-only
  census of at least 30,000 trials once, locally, reporting reds (the
  pre-fix rate was 2 in 30,000).
- Out of scope, do not start:
  `exact-zero-second-partial-leaves-cell-component-as-subnormal-dust`
  (the ring's `0 + 0` widening — but do not make it worse: a degree-1
  direction's second-derivative nets stay the exact `None` they are);
  `work/nurbs/refine-dir-hairline-knot-insertion.md`; the recentring
  origins row on ENCL.

## What will move, and whose it is

Sups change in their last bits, so grid counts can change where a
count sat on a ceiling, so **mesh bytes may move**. D9: a moved byte is
re-baselined with the cause named, never weighed against the change.
Report every golden, digest (the MESH-4 digest at three ε rows), demo
pin and tess-budget figure that moved, with before/after. Demo pins:
re-pin in this PR saying what moved. **Do not re-cut
`docs/tess-budget-data/tess-budget-baseline.csv`** — report the diff
and stop; the orchestrator coordinates that. If k-lint fires, do not
touch geometry: K-REPORT runbook or escalate.

## Acceptance

Phase 1's rows red before and green after; the census; hosted CI green
at the full matrix (twelve test jobs, five k-lint jobs, read at the
step level) — the interval lane included, since this is its arithmetic.

## Review

Full v6 dual on your frozen head. NO Co-Authored-By trailer in lane
commits; nothing you write names the model you are.

## Landing

Status `review` on `work/tess/TESS-2.md` when the PR opens (`pr:`,
`branch:`). Do NOT merge, close items or delete this spec; the
orchestrator does at merge.
