---
id: ssi-chart-speed-usability-boundary
kind: issue
title: ssi: wrong diagnoses survive at finite-but-unusable speeds — the usability boundary is ~5.6e-312, not 0, and both guards test only the class
status: closed
opened: 2026-08-29
github: 1238
refs: [762, 1221]
priority: P0
cost: M
branch: ssi/chart-floor
closed: 2026-10-01
pr: 3694
---

## From GitHub issue 1238

Opened 2026-08-29; 0 comments.

**Filed from CERT-2's fix pass (S-CERT). Class issue — deliberately not fixed in PR 1221**, whose fence covers the non-positive-finite *class* guards (issue 762's residue) and not the design question below. Measurements are the blinded reviews' probes; reproduction sources are the two filed reviewer branches **`cert/2r1-probes`** and **`cert/2r2-probes`**.

## The class

Both chart-speed guards (`ssi.rs`'s seeding guard, `ssi/march.rs`'s stepper guard) refuse exactly `!speed.is_finite() || speed <= 0.0`. That is the right *class* test for issue 762's defect, but it draws the usable/unusable boundary at 0 and ∞ when the arithmetic downstream draws it orders of magnitude inside the positive-finite range: a speed can pass the guard and still make `h = (SSI_IDEALIZED_STEP · extent)/speed` overflow, or drive the translated floors below every representable cell.

## Measured (reviewer probes, reproducible from the branches above)

- **march, speed `1e-320` and `5e-324`** (positive finite): the step `h` overflows to `+∞`, the marcher cannot place a sample, and the caller is told `SeedRefinementFailed` — a wrong diagnosis; the speed was never usable.
- **march, speed `1e-300`**: silent `Ok` with a step of **~2e297 m** — no refusal at all, and a number no consumer can mean.
- **the real march usability boundary is ~5.6e-312**, not 0: below it `h` leaves the finite range at this extent; the guard's `> 0.0` admits ~5.6e-312 worth of unusable window.
- **seeding lane, net magnitude `1e150` through the public door**: chart speed ≈ `1e150` is finite, the guard passes it, the translated floors land at ~`1e-152`, and the sweep runs to `CellBudget` — the budget answering in the guard's place, the same wrong-diagnosis substitution issue 762 recorded for `+∞`.

## The design question (this issue's, not PR 1221's)

Two fix shapes, with different reach:

1. **A usability-class predicate at the guards** — refuse when the speed cannot translate this context's floors/steps into the finite range (a function of speed, extent, and the floor constants, not of speed alone).
2. **Guard the derived quantities instead** — `h`, `h_meters`, and the translated floors each check finiteness where they are minted, so the boundary is wherever the arithmetic actually is.

(1) keeps the refusal at one named door but hard-codes the downstream arithmetic's shape into the guard; (2) is local and exact but multiplies refusal sites. Either way the diagnosis must name the speed, not the budget or the seed refinement.

## Where the guards point here

`ssi/march.rs`'s guard comment and PR 1221's body both state the guard's obligation as the non-positive-finite class and cite this issue for the finite-but-unusable window it leaves open.

Refs: issue 762 (the class the guards do close), PR 1221 (CERT-2), reviewer branches `cert/2r1-probes`, `cert/2r2-probes`.

## Home

`work/cert/` — S-CERT's charter names interval-mode honesty and the chart-speed guards explicitly, and the issue was filed from CERT-2's fix pass.

## Re-homed (2026-09-06)

Moved from `work/cert/` to `work/curved/` on S-CERT's exit walk PR
(#1924, its handoffs ledger; merged by Ev 2026-09-06 = ratified), before
`work/cert/` was deleted at sweep 7 of `docs/DOC-LEDGER.md`. Id, body
and header are unchanged; the directory is the claim (`work/README.md`).
The `## Home` section above naming `work/cert/` is superseded by this
line and is kept as the record of why the file was filed there.

## Design (2026-10-01): the chart speed, decided

Two designers weighed this row together with
`limb-3-chart-tube-speed-has-neither-guard-its-sibling-site-has`,
`ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis`
and `ssi-certify-stretch-divides-by-a-norm-with-no-sqrt-up`, as one
problem: what the chart speed is. They converged in three rounds
(`log.md`). Nothing in it changes ratified text, so it goes to no
`[ev]` PR. This section is the spec all four rows build to.

**Premise correction.** The march's "speed" (`ssi/march.rs`,
`tangent_speed`) is not the chart speed. It is the traced point's
pointwise speed and sizes a heuristic step. Its finite-but-unusable
window is guarded on the step `h`/`h_meters` where the step is minted
(the step must be finite and must actually move the state), and the
step refuses naming the speed. The 1e150 m wall is a FLOOR that no
bisection can reach, not an unusable speed.

1. **One outward norm.** `Box3::speed_sup` (round-to-nearest) is
   deleted. `offset_meters::norm_sup` (ring squares and sums, then
   `sqrt_up`) moves to `geom_core` beside `Interval` as the one home.
   All three callers (floor rate, tube pad, transverse stretch) divide
   a bound by it, so every one of them needs the outward bound. Where
   bits move, it is because the old ones were not certified;
   re-baseline and say so.
2. **Typed folds.** `SupSpeed`/`InfSpeed` gain a `max`/`min` that
   propagates NaN, and producers fold tags rather than bare `f64`s.
   This covers `offset_meters::patch_regularity`'s
   `speed_u.max(norm_sup(..))` and `PatchRegularity::speed_lever`,
   which today use the inherent `f64::max` and drop NaN. `new` stays a
   tag that may carry poison: DESIGN.md D4's degeneracy row 3 (poison
   flows through values and is classified `Invalid`) decides this, and
   `certify.rs`'s `nurbs_span_meter` relies on it. `to_param` and
   `to_meters` stay total and one operation each. The rate-pair doc's
   "fold the bare payloads" sentence becomes "fold on the type".
3. **One mint, a per-axis pair.** `NurbsBoxes::chart_speeds` (name the
   implementer's) mints `{u, v}` over the wall's domain and refuses by
   axis: zero means "the wall is constant along u"; non-finite means
   "no finite bound on the chart speed along v". `plane_nurbs_ssi` and
   limb 3 (including via `edge_nurbs::certify_rung3`) both mint
   through it, and the `if m > 0 { m } else { NAN }` closure goes.
   The sweep's single floor takes the pair's NaN-keeping max.
4. **The certificate records what was proved.** `SsiCertificate`'s
   `tube_radius` becomes a per-kind tube: `Spatial { radius }` (metres,
   the ℝ³ arm) and `Chart { rung, pad_u, pad_v }` (the ladder rung that
   was tried, plus the per-axis pad in chart units, which IS the proved
   region). The certificate is a stored claim (`PcurveCache.ssi`, D5
   ¶2). On the chart arm a metre radius over-states the proof, because
   dividing by a sup under-states chart reach. `PlaneNurbsLimbs` takes
   the same type. `chart_tube_windows(pcurve, pad)` builds the windows
   for the probe and for accounting alike, so accounting reads the pad
   rather than re-dividing.
5. **Bad chart lengths stop at the door of the space they land in.**
   - `NurbsBoxes::cells` refuses a NaN or inverted window. A NaN end
     currently passes through `f64::clamp` and lands on the first span,
     which is the silent wrong-region certificate.
   - The floor is minted once as a typed chart floor, and that door
     refuses a floor its domain cannot resolve (non-finite, zero, or
     below the resolution of the domain's endpoints) with a new typed
     refusal naming the rate through `write_chart_length`.
   - The same precondition applies on the ℝ³ lane (D4 ¶1's km headroom
     becomes a check). The exact ulp form is the implementer's to pin
     with a row at the boundary.
   - A pad needs no resolvability guard: under (4) the recorded pad is
     the honest region whatever it rounds to.

Folded in as findings from the weighing:
- `ExhaustivenessRefusal::cell_width_meters`'s doc calls w·max-sup "a
  ceiling", which is off by up to 2×; fix it in the floor unit.
- `plane_nurbs_ssi` swallows `SeedRefinementFailed` (`continue`); handle
  it in the floor unit with the stepper guard.
- `probe_tube_chart` skips a rung when the transverse stretch is zero,
  which no smaller rung can cure; handle it in the tube unit.

README C2/C3 are re-worded to describe the landed code. That is a
description, not a second decision.

## Closed (2026-10-01, PR 3694)

The last of the three chart-speed units. A sweep floor is minted once per lane through `SweepFloor`, which refuses a floor its domain cannot resolve (exactly where bisection stops making progress) with `FloorUnresolvable` naming the rate. The stepper guards its own step (`StepUnusable`; a diagonal cap). Every SSI door validates its domain (`DomainUnusable`). Issue 1238's march claims are measured in the PR body: the "2e297 m step" was state units read as metres through an inconsistent test point map. Review was a single FULL review with one fix pass.
