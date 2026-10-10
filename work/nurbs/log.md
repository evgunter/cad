# NURBS — the log

## 2026-09-20 — opened

Cut out of PROPS, which was carrying 108.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed PROPS's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

6 rows arrived by `git mv` with their ids, bodies and history
unchanged. PROPS keeps its band 2400-2499; band 9300-9399 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## Announced seam from FIX (2026-09-21) — PR 2948

FIX's `recourse-chain-stops-at-the-second-hop-carriers`, the last row
of its wave-4 slate. An arm whose `Display` renders a carried error
whole contributes no recourse of its own, so *"this message names a
repair"* is a claim about the carrier all the way down. Four carriers
gained repairs and an enforcement row each, every repair grounded in the
module's or the variant's own docs rather than invented, and all of them
**proved red by mutation** (run 35548044980 — twelve `test (…)` jobs
red, failure surface exactly the intended rows).

**Your ground:** `crates/geom-core/src/spline/knots.rs` (NURBS and
PROPS) and `crates/geom/src/curves/fit.rs` (PROPS).

**A sixth carrier the row never named.** `KnotVectorIssue` — reached
through `SplineError::KnotVectorInvalid` — had **7 of 7** renderings
stopping at the condition. Without it `SplineError`'"'"'s one delegating
arm could not be asserted transitively and the chain was false at one
remove. Both now carry enforcement rows.

`FitError`'"'"'s `Lsq` and `KnotAlgebra` arms are asserted as
**delegations only**, and **a row is filed on PROPS'"'"'s slate** saying
why: `fit-error-delegates-to-two-carriers-that-name-no-recourse`.
`LsqError` (`linalg/lsq.rs`) and `KnotAlgebraError`
(`spline/algebra.rs`) state their conditions and name no repair, and
`FitError` contributes five characters over them, so whatever they omit
is simply absent from what a caller of `NurbsCurve3::interpolate` reads.
Asserting a recourse over them would have been a claim about the one
payload the test built — the conditional-transitivity rule this class
established.

Signed (FIX orchestrator).

- 2026-09-28 — Seam note from ENCL: PR 3346 (merged `fb0ec473b8`) adds `geom_core::predicate::KERNEL_DEFECT_ENDING` and `KERNEL_OR_FILE_DEFECT_ENDING`, plus hidden `concat!` macros. A forwarded carrier now labels its repair `Recourse:`, and dead ends take the shared ending. It rewords refusal prose on your ground: `predicate.rs` and `geom/src/curves/fit.rs` (props), knots and spline texts (nurbs/props), validate DEFECT and census (restfront), Boolean `ResultVolumeImplausible` (contact), and editor-core concision rows (tcost/tint). No behaviour changed. Rows filed for the hand-spelled endings on your slates are listed in the PR. (ENCL orchestrator)

- 2026-09-28 — Seam note from ENCL: PR 3348 (merged `95b59b9361`) homes the domain-uniform refinement grid in `geom_core::spline::algebra::domain_grid_points(kv, pieces, GridSkip)`. `GridSkip` is `BitEqual` or `WithinUlps(u32)`, and `pub const SLIVER_CLEARANCE_ULPS` replaces `quad.rs`'s private `SLIVER_CUT_ULPS`. It is used at `props/quad.rs` `refine_dir` and `bezier_blocks`, `ssi/certify.rs` `refined`, `edge_nurbs.rs` `localized::breaks`, and one tcost/tint test. Bits are unchanged at every site (pinning rows added first). Each caller still chooses its own skip guard and control-count cut-off, so the NURBS hairline fix is now a one-argument change at each site. (ENCL orchestrator)

- 2026-09-28 — Seam note from ENCL: PR 3354 (merged `260a8d3dba`) adds `geom_core::spline::algebra::range_grid_points(lo, hi, pieces, GridSkip, mandatory)` under `domain_grid_points`, and routes `props::quad::knot_aligned_cuts` (both its grids) through it; `block_edges` is deleted. Two `quad.rs` sites now read `interior_knots()`. The sliver clearance is reassociated so it cannot overflow. Bits are unchanged, with pinning rows added. (ENCL orchestrator)
- 2026-10-01: Seam note from SSI. Filed `an-exact-pcurve-image-certifies-worse-than-an-interpolated-one` on your slate (`surface_curve_residual`; numbers measured, mechanism a hypothesis). (SSI orchestrator)
- 2026-10-08 — Seam note from ENCL (PR 4348, `encl/offset-cert-coefficient-norms`, in review): `geom-core`'s `spline::compose` tensor helpers gain public norm doors (`tensor::coefficient_norm_bound` now takes `[&[Interval]; 3]` and refuses ragged rows; new `coefficient_norm_sup` and `PatchSpans::cell_norm_sup`), and the offset certificate (`offset_fit.rs`, `offset_meters.rs`) reads its vector upper bounds (‖Y‖, M̃, the integral arm's chart speeds) off coefficient norms per D4 ¶2 instead of per-coordinate boxes. The bounds are tighter or equal, and stored offset numbers were re-baselined. (ENCL orchestrator)

## 2026-10-09 — PR 4435: the span-meter row's premise was false

A reversed knot domain cannot be minted, so no arm was added. The
false doc sentence the row grew from is corrected, and the invariant is
stated and tested. The review tier went down from STYLE to the
orchestrator's read, because no refusal arm is left to route. The
row's reach half lives on in `a-swaying-loft-corner-refuses-as-a-vanishing-span`.
## 2026-10-09 — picked up; the D10 hold checked; wave 1

A NURBS orchestrator holds the track (`status: active`).

**The D10 hold.** The hold's covered ground (TOPO's log, 2026-10-03):
the node vocabulary's edges, `Expr` and document parameters, placement
(datums, `Transform`, pattern frames, mates), declared pairs and
contact, the undeclared refusals, axis declarations, `ParamSource`, the
parameter-coincidence lint, and `Measure`/`Assertion`. I read every row
against it, and one stands on that ground:
`parametric-polygon-loop-certifies-nothing`. It is about what
certification sees when an `Expr::param` reaches a polygon vertex, and
D10 stages 1 and 4 rewrite both the parameter and the symbolic tier, so
it is parked on `d10-one-way-to-say-intent-is-unbuilt`. The rest sit on
kernel ground the hold does not cover:

- `MappedCurve`'s `place` is a kernel `Affine3`, not a document
  placement.
- The loft row's fixture uses `Node::Loft`, but its fix is in the
  certify meter.

INTENT's open PRs do not touch `crates/geom-core/src/spline/*`.

**Shared ground.** FLUX (active) claims `crates/geom-core/src/*`, so we
share the spline files. Three of its rows sit beside this slate:

- `the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column`
  (design);
- `the-projective-applier-still-lerps-so-a-nurbs-refined-at-t-interval-pays-twice`;
- `props-collapse-over-lands-a-nan-window-on-the-first-span`, an
  instance of our span-locator class.

Each wave-1 lane runs `work.py territory` and announces its seam on
FLUX's log.

**Pricing.** The legacy `D` rows were re-priced:
`coefficient-vector-pairing-survivors` is H, and `degree-elevation-…`
is M. `span-locator-…` was unpriced; it is now P3, H, with `design`
set. That puts the slate at 32.5/30. Wave 1 takes 12 points off it, so
the track is worked down rather than split. This is a sequencing call,
and splitting first would cost a sitting.

**Wave 1 tiers.**

- `refine-dir-hairline-knot-insertion`: single FULL review. It moves
  enclosure widths at three certification sites and re-baselines them.
- `mapped-curve-restrict-composes-placements-per-split`: single FULL
  review. It is an interval-enclosure argument that has to hold for
  every split count.
- `nurbs-span-meter-…`: single STYLE review. It adds a refusal arm,
  and its routing is the risk.
- `knot-mirror-…`: orchestrator's read. It is a mechanical move with
  a re-export.
- `span-locator-…`: designer pair (Opus + Fable) first.

(NURBS orchestrator)

## 2026-10-09 — span-locator decided by the designer pair; build dispatched

The pair was reconciled over two rounds, with each designer shown the
other's report, and converged. B adopted A's `ParamRange` window type.
A adopted B's refusing `span_at` after checking the public point-hull
doors. In round 2, A conceded B's point that the `Interval` locator
refuses only a bracketless value (NaN or empty), not an uncertified
one.

There was no split, and no ratified DESIGN.md decision moves; the texts
that change are agent-written code docs from `43c940f1c4`. So this is
not a fork that goes to Ev, and no fork-log row is owed. The protocol's
blinding byte stays on `analysis/design-fork/span-locator` in case Ev
asks for the fork to be put to them anyway.

The decision and the final state are in the row's `## Decided` section,
and the row is now `spec`.

**Build tier: DUAL review.** The change is an architectural API change
across `geom-core`, `geom` and `geom-brep` (span location, a new region
type, every window reader), so its impact is broad and it would be hard
to change later.

It supersedes FLUX's `props-collapse-over-lands-a-nan-window-on-the-first-span`.
Filed on FLUX:
`a-loop-area-nurbs-segment-integrates-an-inverted-window-as-zero`.
(NURBS orchestrator)
## 2026-10-09 — PR 4439: the knot-mirror predicate lives on KnotVector

This was a move. The refusal texts gained recourses, and the
`geom::KnotMirrorError` spelling was dropped. The sweep filed
`row-space-reflection-compares-rounded-knots` (P2, E) on this slate.
`topo/src/pcurves.rs`'s row-space test reflects knots with a rounded
`a + b − k` and `==`; it has no other owner. Tier: the orchestrator's
read. (NURBS orchestrator)

## 2026-10-09 — PR 4442: a NaN has no span, and a window is a ParamRange

The span-locator design, built and through a dual review (DR-124: no MAJOR on either review, 11 bilateral findings, tally 0). The fix pass:
- collapsed three hand-written NaN-checked searches into one `Param` + `last_at_or_below`;
- put all of `NurbsBoxes` on windows;
- gave `ParamRange::spanning` to the sites that took a range from `f64::min`/`max`, which drop NaN;
- pinned every poison arm by mutation.

The reviewers found one unswept locator, `cells_touched`, which is now on the shared search. The remainder, the SSI sweep cell type, is filed on SSIEDGE. (NURBS orchestrator)
## 2026-10-09 — PR 4438 review: the ulp clearance narrows the class without closing it

Single FULL review, verdict APPROVE-WITH-FIXES, and it raised one MAJOR. I adjudicated it real.
- `SLIVER_CLEARANCE_ULPS` is 8 ulps of the DOMAIN width, which is 128 knot-ulps at 1/16.
- A knot 129 bits to about 1e-13 from a grid point still costs up to 47× the flux width. The excess goes as ≈1.7e-15/gap, and the ssi box-chain axis error is 1.2e-3.
- Enclosures stay sound; the defect is width.

**Ruling: the clearance becomes a fraction of the grid spacing, at every production grid site.** Grid points are optional refinement, so skipping one costs at most a sliver of extra span width. That makes this a dominant-argument choice, so no designers.

Other adjudications:
- `GridSkip` collapses if every site then takes one value.
- New rows sweep the knot offset across the whole range, so a cliff anywhere goes red.
- Filed in the fix pass:
  - the kernel's own hairline mints (`sweep/src/skin.rs` knot union, the certify composite's break merge);
  - the `refined`/`chart_breaks` grid duplication.

The fix pass runs on hosted CI only, because of disk (below).

**Friction (two findings).**
1. PR 4439's `test` job took 21 min against the 15-min bar. It is a 6-file move in geom-core and geom, so the change filter likely selected most of the suite.
2. This 4-core container has 252 GB but a far smaller per-session writable allowance. Two live lanes hold 17 GB of targets, and one reviewer's local battery hit a full disk (a Bus error at link).

Each lane that builds locally costs 1–10 GB. From here, heavy lanes run CI-only, or in their own cloud sessions (orchestration-model memory, 2026-10-02). (NURBS orchestrator)

## 2026-10-09 — PR 4441 and PR 4442 reviews adjudicated; both in fix passes

**PR 4441 (restrict).** A single FULL review gave APPROVE-WITH-FIXES with one MAJOR, which I adjudicated real.

- **The MAJOR.** `offset_axial::reauthor` composes `rotation_about_axis(.., angles.from)` into the stored point. At Interval, `rotation_about([0,0])` is not the identity, so every re-authored revolved declaration stores a rotation enclosure. The width grows from 0 to 1.1e-11 at a far placement. The fix minted its own defect class, and the PR body's sweep claim missed the `plane` closure.
- **Range arithmetic.** I ruled a required fix where the review rated a MINOR. On interior and alternating nested splits, `SweepRange::at`'s convex form grows faster than main: at N=64 it is up to 12× worse with enclosure parameters. An exact-composition probe shows the floor is flat, so the growth is removable. The fix pass must make every row of the review's table no worse than main.
- **Is the PR still an improvement?** For the kernel's actual call shapes it already is. `split_specs` and `edge_join` keep one end, and those chains are now flat where main grew 9–17×.

**PR 4442 (span-locator), dual pair.** Both reviewers returned APPROVE-WITH-FIXES with no MAJOR, so there is no tally candidate. The blinded pre-note is recorded privately, and the DR row is written at merge. The fix pass takes the union of both reviews:

- poison seeding is pinned at only one of seven arms;
- `piece_controls` returns `from_f64(NaN)`;
- nothing pins the producers' `certified` choice;
- `compose/tensor.rs` `cells_touched` is an unswept hand-rolled locator (found by one reviewer alone, demonstrated);
- NaN-dropping `min`/`max` feeds `ParamRange::new`;
- three copies of one NaN-checked search;
- the ssi `f64`-door narrowing is unscheduled;
- the UV-rectangle refusal is spelled five times.

(NURBS orchestrator)

## 2026-10-09 — PR 4438 merges: grid points clear knots by a fraction of the spacing

The ruling above, built at all seven grid sites. A delta review of the fix pass approved it with fixes, and those are in. Changes after the review:
- production and the tests share one `grid_clearance` helper;
- the no-cliff rows take the production span counts;
- the clearance doc states the trade as measured (about +0.2% width at large gaps, measured to g ≈ 1e-8) instead of "costs nothing";
- m5_pr7's ratio bound was re-derived as `√G`.

Walls 15 and 17 retired, so the lily's swept leaves are now cubic. A container restart killed the cleanup lane after it pushed; the redo found the work already on the branch and gated a fresh merge of main. (NURBS orchestrator)
