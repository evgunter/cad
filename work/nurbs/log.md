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

## 2026-10-09 — PR 4441 delta review: the 1.91× was accepted on a false premise

The first fix pass stored the restricted range in angle units and accepted up to 1.91× main's width on (½,1)-type chains, saying no single stored form keeps both ends flat. The delta review (APPROVE-WITH-FIXES) showed a form that does: keep `(u0, du)` in the original normalized parameter and apply the angle once at eval. Dyadic splits are then exact; it is ≤ main in every measured row and bit-identical on a whole range. My bar was "no worse than main", so the PR takes that form.

The reauthor fix still stores a width from `R(−0)`'s documented subnormal dust at Interval (2.3e-13 far, where main stored 0). Moved starts on tilted far placements are 2.5–3× worse than main at f64 (the tilted_lune class). Both are required fixes, along with independent reference spellings in the width rows (the ratio row's comparison shared production eval and could not see an eval mutant). Second fix pass dispatched. A container restart lost the first delta reviewer; the redo reported. (NURBS orchestrator)
## 2026-10-09 — PR 4438 merges: grid points clear knots by a fraction of the spacing

The ruling above, built at all seven grid sites. A delta review of the fix pass approved it with fixes, and those are in. Changes after the review:
- production and the tests share one `grid_clearance` helper;
- the no-cliff rows take the production span counts;
- the clearance doc states the trade as measured (about +0.2% width at large gaps, measured to g ≈ 1e-8) instead of "costs nothing";
- m5_pr7's ratio bound was re-derived as `√G`.

Walls 15 and 17 retired, so the lily's swept leaves are now cubic. A container restart killed the cleanup lane after it pushed; the redo found the work already on the branch and gated a fresh merge of main. (NURBS orchestrator)

## 2026-10-10 — PR 4441 merges: restriction narrows a range in the sweep's own parameter

The second fix pass adopted the delta review's form: the range is stored in the whole sweep's normalized parameter, and the angle is applied once at eval. Against main, every width row is at or below main, except two far interior chains at 1.06× and 1.03×, within the ≤1.25× allowance. Each of those chains moves the start inexactly at every split. End-anchored chains are flat from both ends. Unrestricted output is bit-identical at every scalar, so the `m10` ledger returns to main's.

Reauthor: an unmoved start stores main's point (width 0). A turned start reads through main's composite (Interval equal to main; f64 `eval(0)` 1.2× at 1e3, and 3 ulps against 2 at 1e5, disclosed). The rows compare against a hand-written composed spelling that shares no code with production, and both the two-step-rotation mutant and the endpoint-form mutant turn them red.

I accepted this without a third review: it implements the reviewer's own proposal, and every claim is measured against an independent reference.

Friction: two container restarts each killed an in-container lane. Neither lost pushed work. The second-pass lane ran as its own cloud session and survived. Long lanes go to cloud sessions from now on. (NURBS orchestrator)

## 2026-10-10 — next slate dispatched

The previous slate is fully merged (PRs 4438, 4441, 4442). Dispatched, each as its own cloud session (the container restarts of 2026-10-09 killed two in-container lanes):

- **`coefficient-vector-pairing-survivors`** (P1, H): the row's per-site dispositions, built. Class H, so the review is a DUAL concurrent pair.
- **`row-space-reflection-compares-rounded-knots`** (P2, E), plus a re-measure of **`an-exact-pcurve-image-certifies-worse-than-an-interpolated-one`** (P3). PR 4442 rewrote the `cells_touched` the latter's hypothesis blames, so it may already be closed; it is re-measured before any fix.
- **Designer pair (one Opus, one Fable)** on how a restricted description evaluates. This covers `sketch-segment-restrict-re-derives-endpoints-per-split`, where `SketchSegment` is the profile's canonical form and a window changes what `a`/`b` mean to every reader, and `revolved-point-eval-levers-angle-width-by-the-coordinates`, where eval's spelling trades f64 accuracy against Interval width by up to four orders of magnitude. They get one problem statement and no candidates.

Not picked: `parametric-polygon-loop-certifies-nothing` is parked and is PROPS's subject. The P3/P4 rows (`a-swaying-loft-corner…`, `certified-blossom-primitive…`, `degree-elevation-recomposition…`) wait for capacity. (NURBS orchestrator)

## 2026-10-10 — designer pair on restricted descriptions: Q1 converged and is building; Q2 split, round 2

**Q1 converged; both designers were sure.** Restriction lives on the description: `MappedCurve { source, range }`, and `SketchSegment::restrict` is deleted. A sketch segment is never restricted, so `a`/`b` always mean the authored endpoints. No ratified text changes. Both designers independently found that the Line arm's `lerp` re-derives both ends per split, so Interval width compounds exponentially (8e3 m after 64 nested (0.3, 0.7) splits at a far centre). Build dispatched.

**Q2 split; both rated it likely.** Both reject the shipped `R·p + (I−R)·q`, whose width grows with distance from the world origin. Both agree on the weighing rule: width that scales with the geometry is the floor; width that scales with the origin or the split count is a defect; a few f64 ulps do not decide. They split on the anchor:
- one picks the axis point, `q + R(p−q)`: bit-identical at an origin axis, but an uncertain axis reaches the start sample;
- the other picks the point, `p − (I−R)(p−q)`: the start sample is exact, but it is 2–3× wider at an origin axis.

Round 2 gives each the other's argument. Filed: `tilted-lune-sits-at-the-f64-floor-of-its-band`, the fixture both designers found measuring ulps, not merit, with an unexplained hosted/local split. (NURBS orchestrator)

## 2026-10-10 — Q2 converged after round 2; W1 goes to Ev; reviews dispatched

**Q2 converged on `q + R·(p − q)`.** In round 2 one designer held that answer, conceding that the point form's axis-free start sample is real. It measured `arc_of_circle`, the only producer that hands a wide axis: there the point form wins the start sample by 3×, and the axis form wins the widest sample, which certification meters, by 1.5×. The other designer moved to it. Against an exact 300-bit reference the axis form is never worse, and the case the point form wins needs a wide axis with an exact point, which no producer makes. This was not a crossover: one designer held its position and the other moved to it. The build is queued behind PR 4491, since both touch `mapped.rs`.

**The decision rule both designers stated** is new binding text for geom-core's README, so it goes to Ev as [ev] PR 4492 (W1), fork row 103. The blinding byte was drawn late; the lapse is disclosed in the row's analysis-branch record and in the PR.

**Reviews dispatched:**
- PR 4485 (coefficient pairing): DUAL concurrent pair on frozen head `94a8eee822`, identical briefs.
- PR 4479 (row-space reflection): one FULL review.

(NURBS orchestrator)
- 2026-10-10 — Seam note from FLUX: FLUX's priority-seam cut moved `the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column` and `the-projective-applier-still-lerps-so-a-nurbs-refined-at-t-interval-pays-twice` onto this slate (P3 M +design and P2 M; +5 points, 26/30). The first is what `certified-blossom-primitive-in-geom-core-spline` waits on, and the second is the same lerp-against-convex combine in `CurvePlan::apply_points`. FLUX's other spline rows went to the new KNOT (`geom-core/src/spline/*`, shared with you). (FLUX orchestrator)

## 2026-10-10 — PRs 4491 and 4489 merge; PR 4485's delta review finds a hole its fix pass minted

**PR 4491** (restriction lives on the description; `SketchSegment::restrict` deleted) merged after one FULL review (APPROVE-WITH-FIXES, no MAJOR) and a fix pass.
- Widths: main was red on 8 of 12 width rows, up to 1.9e8 m on a far line. The head is flat at one rounding far out.
- Fix pass: the payload-rung sweep now reads one rung into struct payloads, so it sees `MappedSource` again; it also surfaced 13 undecided rungs, filed on lib. The sphere-recut outcome move at ε = 1e-13 is measured and re-aimed.
- `SubRange::is_whole` stays structural: an exactness reading needs a `Bounds` ratification, and no production path restricts by exactly (0, 1).

**PR 4489** (exact pcurve image) merged.
- Its review refuted the "1.5× floor": a depth-p blossom cut makes the exact image certify bit-identically to the interpolated one, and the row closed.
- The lane built ENCL's scripted-bound seam for the stall row rather than re-hunting it a third time.

**PR 4485** went back for a second fix pass. Its first fix pass made the `DerivLadder` sound: no misses on 1M checks against Gauss–Legendre, and bit-identical elsewhere. But it deleted the `derived_knots` gates as dead, and they were reachable: equal-split points collide on a span a few ulps wide, the multiplicity reaches p after the gate, and `S_uu` silently reads zero where main refused. This is the fix minting a fresh instance of the defect class it closes.

**PR 4479** is in a delta review. Its fix pass made `reversed_column` reflect through 0 (exact, infallible) instead of exact-or-refuse, which would refuse common lofts that build on main. That moves one-segment strut domains to [−1, 0], so the review checks ratified text, STEP export and readers for a negative domain.

**Q2 build dispatched:** `q + R(p − q)`, citing W1. (NURBS orchestrator)

## 2026-10-10 — PR 4479 merges; PR 4485's delta 2 approves, small fixes applied by the orchestrator

**PR 4479** (one exact reflection rule; `reversed_column` reflects through 0) merged at `1366b92992` with CI green.
- Fix pass 2 closed the delta review's M1. `RowSpace` admits forward on equal knots and run back only on an exact reflection, which is what the certificate accepts. The reviewer's probe is now a row: red on `8015fdf33`, green here.
- Filed off this slate:
  - the 8-section STEP re-import failure, on EXCH (`one-segment-loft-at-eight-sections-fails-step-reimport`), identical on the previous head;
  - the two reversal policies for a spline net, on KNOT (`two-reversal-policies-for-a-spline-net`).

**PR 4485** delta review 2 of `2ac0994f80`: APPROVE-WITH-FIXES, no MAJOR.
- Every claim held under execution:
  - the colliding-split row is red on `c5d1c7ae` and gives `DerivedKnots` / `RefinementFailed` at head, as main does;
  - degree-1 `Zero` is sound: 40 random interior-knot degree-1 faces all refuse `Degree1Crease`;
  - patch and face outputs are bit-identical across main, c5 and head on 400 surfaces;
  - the 1-D ladder moves only at multiplicities p, p−1 and p−2, and 6000 constant-`u` cases show main excluding the truth 1288 times and head 0 times.
- The orchestrator applied the small findings on the branch rather than spending a lane on them:
  - the `DerivedKnots` doc and note no longer claim a valid face never reaches it;
  - the `Second::Zero` comment;
  - the collision row asserts its variants (Q3);
  - `whole_net_bound`'s missing second partial panics off degree 1 rather than reading zero (Q4);
  - the `compose.rs` re-wrap.
- Left as is:
  - Q7, A/w shape held by `unreachable!` across two `DNets`;
  - Q1/Q2, `Loose::Const` read whole-domain in `Level` vs per span in `Dir`.

  These are refactors of a sound structure, worth noting but not blocking.
- Pre-existing NOTEs carried to the user, not filed yet:
  - `nurbs_patch_face`'s A2 area-gauge `debug_assert` panics on a caller perimeter below the truth;
  - `offset_fit` refuses `DerivedKnots` on any degree-1 base direction.
- Recorded:
  - DR-140: tally 1, R1's ladder zeroing, which predates the PR;
  - the row is closed in the PR.
- Merges when CI is green on `695dc165ce`. (NURBS orchestrator)
