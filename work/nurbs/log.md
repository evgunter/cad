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
- Pre-existing NOTEs filed:
  - on QUAD, `patch-face-area-gauge-panics-on-random-bicubic-faces`;
  - on ENCL, `offset-fit-refuses-every-degree-1-base-direction`.
- Recorded:
  - DR-140: tally 1, R1's ladder zeroing, which predates the PR;
  - the row is closed in the PR.
- Merges when CI is green on `695dc165ce`. (NURBS orchestrator)
- 2026-10-10 11:28 — PR 4485 merged at `695dc165ce` (CI green), with DR-140 and the row closed; tracker sync PR 4528 merged. (NURBS orchestrator)
- 2026-10-10 12:15 — Q2 build is up as PR 4518 (`q + R(p − q)` via `Affine3::rotate_point_about_axis`, citing W1). Its sweep also moved `sweep::revolve` partial/full vertices and `offset_axial::reauthor` onto the new spelling, and it re-blessed `m10_p_fence`, `seat7`/`seat8`/`intent_s2_c_world` and `sym11`. A thousand metres out, widths are flat at 2–3 ulps of the coordinates. Class M, dual tier: rule 1 byte 52 (mod 3 = 1) gives SEQUENTIAL, so one FULL Opus review was dispatched on the frozen head `21a59ded5f`. A second review runs only if this one raises a MAJOR. (NURBS orchestrator)

## 2026-10-10 — PR 4518's review approves with fixes; the next slate dispatched

**PR 4518** (Q2, `q + R(p − q)`): the sequential arm's one review returned APPROVE-WITH-FIXES with no MAJOR, so no second review runs.
- **Held under execution:**
  - inclusion on 4000 wide boxes;
  - bit-identity at an origin axis on 200,000 cases, at f64, `Dual` and Interval;
  - against a 300-bit reference, never worse than the old spelling in any bucket;
  - the width table reproduced at 1e3 and 1e5, and flat at 1e7;
  - the corpus moves trace to the vertex sweep alone;
  - every newly built `sym11` washer is closed with 4 faces;
  - the re-posed rows go red under both eval mutants.
- **MINOR-1:** the doc's "up to 2·w" axis bound is false away from the zero turn. The reviewer measured 2.41·w; a turned box grows by up to |cos| + |sin| before `q` is added back.
- **MINOR-2:** the sweep is incomplete. Far-cap edge descriptions and arc centres still turn points through `rot * place`, so a far-cap vertex and its edge are turned by two spellings.
- **Style:**
  - two affine docs argue opposite postures;
  - a bit-for-bit row compares `eval` to the function it calls;
  - a round-trip row stands in for an exact reference;
  - a FAR floor sits above its measurement;
  - a stale cite, and an unwrapped line.
- The fix pass went to the build session. Merge follows the orchestrator's delta check, and DR row (sequential arm) is the PR's last commit.

**Next slate, dispatched 13:19Z:**
- **Fork3:** the certified spline combine. It decides the form of the certified Boehm step (`the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column`, `design: true`), what the projective applier means at `T = Interval` (`the-projective-applier-still-lerps…`), and the one certified primitive (`certified-blossom-primitive-in-geom-core-spline`), which waits on the first.
  - One problem statement, three questions.
  - One Opus and one Fable designer, concurrent on `5bcc5296ce`.
  - Blinding byte drawn before dispatch.
- **One implementer lane, two PRs:**
  - `nurbs-iso-netview-carries-a-surface-unpaired`: read through the surface, delete the re-check;
  - `a-swaying-loft-corner-refuses-as-a-vanishing-span`: a span meter that answers soundly on a swaying corner.
- **Not dispatched:**
  - `degree-elevation-recomposition…` (P4), which sits behind fork3's answer on the applier;
  - `tilted-lune-sits-at-the-f64-floor-of-its-band` (P3 E). Q2 left the lune green at all three ε, because its axis passes through the origin. The hosted/local split it records needs PR 4441's old head to reproduce, so it waits for a nightly reading. (NURBS orchestrator)

## 2026-10-10 — Fork3 converges; its build dispatched; two clarifications to Ev

**Fork3** (the certified spline combine) converged in round 2, without crossing over. A is Opus and B is Fable, by byte 154, recorded on `analysis/design-fork/nurbs-fork3-combine`.
- **Q1, the step's form.** It is the monotone two-point form, met with the sources' hull, as a door `Certification::between`.
  - A moved from midpoint–radius after running B's rows. Monotone is tighter on every row, by 1.4–4×, and midpoint–radius fails the 254-cut row's 2p+1 ceiling at degree 3.
  - A also moved to keeping the hull meet. Without it, either form steps outside the sources' hull, below zero at a subnormal weight, which breaks PR 3737's weight-range guarantee.
- **Row 1's 2.4× is stale.** It was the serial extra-break insertion that PR 4489 removed. Today every form reads the r1 envelope the same to seven digits.
- **Q2, the projective applier.** The knot algebra becomes unrepresentable at `Interval`. B had said no shipped path reached it, and corrected that: `NurbsLane::<Interval>::certified()` → `plane_nurbs_limbs` → `localized` → `refine_knots_u/v` → `apply_points` with a point-lifted `f64` λ, and `ssi/certify::refined` likewise. Both run from editor-core's `Interval` replay. A certified plane × NURBS claim was therefore about a neighbour of the described wall, at ulp scale; no verdict was shown to flip.
- **Q3, the primitive.** One blossom, `CoeffWindow::blossom(args)`, with `restrict` as sugar. `insert_once_ring` and `sub_segment` retire.
- **Asked of Ev,** as `[ev]` PR 4539 with fork row 107: two wording clarifications both designers found.
  - W1's "number of restrictions" is to read "inherited width multiplied per step".
  - C6 is to say a refinement inside a certificate is held as homogeneous enclosures.

  Neither changes a decision, so the build does not wait on them.
- **Build, dispatched 14:59Z on `a80985928917`,** as two lanes:
  - A: `nurbs/monotone-step-and-blossom` closes `the-convex-boehm-step-…` and `certified-blossom-primitive-…`;
  - B: `nurbs/knot-algebra-off-the-ring` closes `the-projective-applier-…`. It measures before choosing between dropping `localized`/`refined` at `Interval` and refining homogeneous channels.
- **Filed** off-question:
  - `work/ssiarith/ssi-cut-derivative-box-is-wider-than-the-whole-cell`;
  - `work/encl/offset-fit-composite-elevates-a-rational-direction-at-f64`.

  The quad sites (`ring_lerp`, `bezier_blocks`, `bezier_bisect`, `de_boor_on`) and `sub_piece` are covered by the two build lanes. (NURBS orchestrator)

## 2026-10-10 — Ev adopts fork3's text (PR 4539); PR 4535 merged; PR 4540 to a HOLDOUT pair

- **PR 4539** merged after Ev's "sounds good". W1 now reads "inherited width multiplied per step", and C6 says a certified refinement is held homogeneously. Fork row 107 is complete: A = Opus, B = Fable, and Ev's decision matched both.
- **PR 4535** (`nurbs_iso` reads the surface; `NetView` and its count re-check deleted) merged on green CI. Tier: the orchestrator's read. The change is mechanical, and bit-identity was shown over 3000 random surfaces with an identical sha256 of 84,156 printed results.
- **PR 4540** (`speed_lower_bound` gains a piece assembly: Bernstein coefficients of `C′` on 16 pieces per span, joined by `max`) goes to the dual tier. A new certified lower bound that read high would certify an edge it should refuse.
  - Class M. Rule 1 byte 18, mod 3 = 0, gives HOLDOUT: a concurrent Opus pair on the frozen head `5ec41ae290`, with identical briefs.
  - The PR filed `a-swaying-rational-corner-refuses-at-the-rational-speed-meter` (P3 M) on this slate.
- **PR 4518** carries DR-143 and merges on green. (NURBS orchestrator)
- 2026-10-10 16:17 — PR 4518 conflicted with main in `docs/DUAL-REVIEW-LOG.md`, because GERM's DR-143 (PR #4522) landed first. I renumbered ours to DR-144 and kept both rows. That makes **twenty M-tier units under rule 1's arms**, so the full readout (rule 9) is owed, and I recorded the triggering row, so I ask Ev.
  - Readout writer: a blind lane, on `analysis/dual-review/readout-4`. I do not read it.
  - Row `the-dual-review-streams-fourth-readout-is-owed` (needs_ev). The `[ev]` PR opens when the writer is done.
  - The arms continue unchanged. (NURBS orchestrator)
- 2026-10-10 16:56 — **Readout 4** is `[ev]` PR 4555, written blind; the orchestrator has not read it.
  **PR 4540's HOLDOUT pair** is in. Both reviews returned APPROVE-WITH-FIXES with no MAJOR, so the tally is 0 pending the blinded coding (coding byte 201).
  - Bilateral:
    - interior-piece coverage is guarded only by chance (the stall fuzz plants stalls only at knots);
    - the join's docs over-claim (dropping the global or per-span assembly turns nothing red);
    - whole-body `Interval` certification costs 3–5× main, undisclosed.
  - Unilateral: `pcurve_cache::chart_stretch_inf` is a missed inf producer, reading 0 on every new loft wall (one reviewer, demonstrated).
  - The fix pass went to the implementer. It also asks for each piece to be cut by blossom from the span row, per W1 as clarified today.
  - **Fork3 builds** are up as PR 4551 (A) and PR 4553 (B), with CI pending. Review tiers at arrival:
    - A, the step door plus the primitive behind every certified spline step: H, dual (concurrent).
    - B: dual if its route changes certification, otherwise single. (NURBS orchestrator)
- 2026-10-10 17:28 — **PR 4518 merged** with DR-144 (Q2, `q + R(p − q)`). The `revolved-point-eval-levers…` row closes with it.
  - **PR 4540's coding** came back: 11 bilateral findings, 8 unilateral to the R2 report and 6 to the R1 report, tally 0. The nearest miss is R1's `chart_stretch_inf`, which fails (a) because it was raised as MINOR. The pair was fair. This matches the pre-note.
  - **PR 4553** (fork3 build B: the knot algebra bounded off `Interval`; `localized` dropped by route (a); `refined` re-routed homogeneously by route (b); the speed meter and quad sites through `apply_certified`) is CI-green.
    - Its tier: dual, class M. It changes the certification route of plane × NURBS and SSI and bounds the knot API across `geom`; the design is settled and the route was measured.
    - Rule 1 byte 182, mod 3 = 2, gives SEQUENTIAL: one FULL review on `e32d58c62d`.
    - The PR reports `topo::rim_dim_review_probes::silent_fixed_predicates_scale_linearly` red on main under `--all-features`. The review is to confirm or refute it before it is filed.
  - **PR 4551** (fork3 build A) is still fixing a red test job. (NURBS orchestrator)
- 2026-10-10 18:06 — **PR 4551** (fork3 build A: `Certification::between`, `CoeffWindow::blossom`/`restrict`; `insert_once_ring` and `sub_segment` deleted) is CI-green on `18b5722731`.
  - The build found and fixed a plane chart's box (`frame_box_on`) that read one span's extension over a multi-span net, with a red-on-main row.
  - Tier: H dual, CONCURRENT. It is the certified step under every spline operation plus a new primitive, across 28 files. Two Opus reviewers with identical briefs, dispatched together on the frozen head.
  - Tracker sync PR 4563 merged. (NURBS orchestrator)
- 2026-10-10 18:21 — Ev ruled on **readout 4** (PR 4555): "the current pattern can continue, iirc 20 more majors or 2 more unilateral majors where the other side didn't catch the issue at all". That is rule 9 as it stands, so the protocol is unchanged.
  - The ruling is recorded in `docs/DUAL-REVIEW-LOG.md`, and the count restarts after DR-144; it stands at 1 (DR-145).
  - The row is closed, and PR 4555 is closed unmerged. (NURBS orchestrator)
- 2026-10-10 18:50 — **PR 4553**'s sequential review returned NOT-MERGEABLE-AS-IS.
  - **MAJOR, demonstrated:** route (a), dropping `localized`, moves `Interval` verdicts. On a steep-weight rational section extruded into a wall, 19 rulings certify 15 → 6 at ε 1e-12, and pads collapse to about 1e-4×. The PR's route measurement was `f64`-only.
  - **The rest** of the review held: the seal; `apply_homogeneous` sound against exact rationals; route (b) for SSI; the speed meter on 430 curves; quad.
  - **MINORs:**
    - a rounded knot difference in `rational_span_bound`;
    - the bound covers λ's scalar, not P's;
    - the stated cause of the quarter turn's 1e-12 is wrong;
    - `offset_fit` elevation (filed today);
    - `apply_homogeneous` is a third certified step at `Interval`.
  - **Next:** the fix pass takes route (b) for the wall too. Per the SEQUENTIAL arm, a second whole-unit Opus review follows. The reviewer confirmed `silent_fixed_predicates_scale_linearly` is red on main under `--all-features`; the lane files it. (NURBS orchestrator)
- 2026-10-10 19:33 — **PR 4551's H pair** is in. Both reviews returned APPROVE-WITH-FIXES with no MAJOR, so the tally pre-note is 0. Blinded coding is dispatched (byte 104).
  - **Soundness held:** about 1M exact-rational checks of `between`, and about 200k of `blossom`/`restrict`.
  - **`frame_box_on` was a real defect on main:** an undersized plane-chart box fed `ChartEdge::Envelope { slack: 0 }`, then `certifies_outside`.
  - **Bilateral:**
    - the r1 row no longer guards the step form; all three mutants stay green at head;
    - the 254-cut ceiling has zero margin;
    - the cost is undisclosed: composite 2.5–3×, quadrature +19–34%, the SSI suite +19%, likely from the O(n²) window minting;
    - duplicated doors, and tests that test copies of production code.
  - The fix pass went to build A. (NURBS orchestrator)
- 2026-10-10 20:46 — **PR 4540**'s fix pass passed its delta check:
  - deterministic piece-coverage rows, with each dropped-piece mutant red;
  - each join arm pinned;
  - pieces as exact combinations of a depth-q row (W1);
  - cost stated: an `Interval` loft build at 3.7–5.5× main;
  - `chart_stretch_inf` filed on PCERT.

  Its review row is DR-149, renumbered twice as GERM's and SHELL's rows landed first; it merges on green. Main's new `from_f64` allowlist gate needed seven stored-knot and stored-weight rows at the merge. I filed `the-speed-meter-runs-three-times-per-corner-carrier` (P3, E): memoise the meter per carrier rather than halve the splits.
- **PR 4551's** blinded coding: tally 0, fair, matching the pre-note. Its fix pass and PR 4553's are both still running. (NURBS orchestrator)
- **2026-10-10 22:10Z.**
  - **PR 4540** merged (`24d342b44b`). Its review row became **DR-150**: INTENT's PR 4583 took DR-149 first. The readout toward-count now also counts INTENT's DR-149, an M-tier unit main's foot had left out, so it reads 4 (DR-145, DR-147, DR-149, DR-150).
  - **PR 4590**, the tracker sync, merged.
  - **PR 4553's second whole-unit review** (sequential arm, byte 182) is in: APPROVE-WITH-FIXES with no MAJOR.
    - MAJOR-1 is confirmed fixed by execution: 8 wall families × 19 rulings × both scalars × 3 ε, with no lost verdict and pads ≥ 1. At 1e-12 Interval the head certifies 19/19 on d3/d2 against main's 15/15.
    - Exact-rational containment: 170,601 checks, 0 misses.
    - New:
      - MINOR-A: the compile_fail rows also emit E0599;
      - MINOR-B: the `ssi_hull_sup` Interval payload guard is no longer carried;
      - MINOR-C: a stale `w′` doc;
      - NOTE-A: `refined_net` has no positivity guard;
      - NOTE-B: cost +4–20%, accepted;
      - S1–S9.
    - Fix pass sent to build B, with S8 not adopted.
  - **PR 4551's** fix pass is pushed (`52ba7469…`); CI is pending before the delta check. (NURBS orchestrator)
- **2026-10-11 02:30Z.**
  - **PR 4553**'s second-review fix pass is checked against MINOR-A/B/C, NOTE-A and S1–S7, S9:
    - one `RingRefinement` owns the refined knots and the extent guard;
    - one `certainly_positive`/`dehomogenize` guard;
    - one `spline::knot_difference`;
    - `BlockRefusal` is typed;
    - `CellGrid` stores `(w·P, w)`;
    - a row reads limb 2's `Interval` margin as an enclosure.
  - **PR 4553's main merges:**
    - Main's PR 4596 extended the `from_f64` gate to geom-core, so the orchestrator listed seven sites (stored knots and weights; the plan's λ, sealed off `Interval`).
    - Build B resolved ENCL's limb-2 subdivision onto the described carrier, cut in the ring at `cert_grid ∪ cuts`.
    - Its review row was renumbered three times, DR-153 → DR-157 → DR-158, as INTENT and GERM rows landed. The toward-count now reads 8.
  - **PR 4551**'s fix pass is accepted (`d968ee5031`). `props/quad::de_boor` ships unrouted: routed through the blossom, it moved step-import's arc-loft volume one ulp apart between native and imported against a bit-identity pin. That is not a soundness loss. Filed as `quad-de-boor-through-the-blossom-breaks-the-import-bit-identity`. 4551 merges main after 4553 lands, then takes its DR row.
  - **Shared review log, not NURBS's:** FLUX's DR-149 and DR-150 and GERM's DR-157 sit below the ruling bullets instead of in the table, and DR-149 and DR-150 are each used twice. Reported to the user, not edited. (NURBS orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4616, merged): a test-only accessor `PatchSpans::cell_coeffs` was added in `geom-core/src/spline/compose/patch.rs` (`cfg(any(test, feature = "test-support"))`) for offset_fit's red-on-main weight row. Its doc (and the `Cargo.toml` feature comment) says "one coefficient", but it returns the whole cell's coefficient slice; that wording is yours to fix or ours on the next touch. (ENCL orchestrator)
- **2026-10-11 04:49Z.** **PR 4553 merged** (`98e620a195`). Its review row is **DR-158**, renumbered from DR-153 and DR-157 as INTENT's and GERM's rows landed first. The readout toward-count now reads 10.
  - Main kept moving under the PR, so it took five more main merges:
    - INTENT's knots-in-T (PR 4608): `SpanLocate` unions `point_value` with this PR's methods, and probe 5's band zero moved from 4.5e-15 to 4.8e-15 because limb 1 now reads about 4.77e-15.
    - The CLEAVE apex row, red on main since PR 4609: PR 4631's re-pin ported verbatim, with one PR comment.
    - The rest were review-log renumberings.
  - The final merge resolved only the review log over a green head (`3c22917974`), so that tree is GitHub's own merge of the green head plus that text. A workspace check, the gates and lint ran on it before the merge.
  - The user caught that one conflict sat unnoticed while I read silence as "CI running". Mergeability is now checked directly with `git merge-tree` against main rather than inferred.
  - Build A is told to merge main into PR 4551 next. (NURBS orchestrator)
- **2026-10-11 05:55Z.**
  - Re-read the three prompts, `work/README.md` and `orchestration-model.md` after the compaction, per the new `CLAUDE.md` clause (PR 4643).
  - Two standing rules had been dropped:
    - `work.py incoming` at each check-in. Now run: 0 incoming.
    - Friction is a finding. Filed `work/issues/the-dual-review-logs-shared-tally-line-conflicts-every-unit.md` (DR-log contention, plus the 35-minute `test` job on PR 4553).
  - PR 4551 is still waiting on build A's push of its main merge. (NURBS orchestrator)
- **2026-10-11 06:27Z.**
  - `incoming` brought INTENT's filing of `main-slow-set-reds-on-two-rows-since-4553` (P0). Two slow-set rows have been red on main since PR 4553: sweep's serial-golden digest, and step-import's arc-loft native-vs-imported volume bits.
  - These are the same two rows PR 4551 had already shown to be quadrature-sensitive. The per-PR gate never selected them, because 4553 touched neither crate.
  - The orchestrator's miss: before merging 4553 it should have run those two rows, since 4551 had just shown they move with quadrature changes.
  - Build B (`session_01VSe9NnecKcXKBB3VP44z5J`) is re-dispatched to root-cause both on `nurbs/main-slow-set-reds-since-4553`:
    - re-baseline only with a stated, measured cause;
    - for the import bits, make both sides compute the same thing rather than re-pin.
  - Build A was told not to attribute these rows to 4551. (NURBS orchestrator)
- **2026-10-11 07:12Z.**
  - PR 4647 (merged by Ev) splits the dual-review log into one file per row, with numbers assigned on main and the foot computed. NURBS's contention issue is closed against it. 4551's row will be `rows/PR-4551.md`.
  - Build A's main merge into 4551 (`11a6b7253a`) has the right `SpanLocate` union, and `knot_difference` is the only spelling.
  - Its probe-5 commit re-aimed 4553's MINOR-B guard from limb 2 (`ssi_hull_sup`) to limb 1, because under the monotone step limb 2 now reads 3.13e-15, under limb 1's 4.77e-15. That re-opens MINOR-B: limb 1 is already pinned by r2 probe 2.
  - Sent back: give `ssi_hull_sup`'s `Interval` payload a row that can still go red (a fixture where limb 2 leads, or a direct `hull_limb` row). Mutation red is to be shown by execution.
  - Build B is working the P0 slow-set reds with a candidate walk-order fix. (NURBS orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4653, merged): `geom_core::spline::compose::sub_segment` is now public and re-exported from `geom_core::spline`. It cuts a Bernstein row on [s0, s1] down to [a, b] through `convex_step`, as an outward enclosure. Its `p` argument is gone. It refuses (returns the refused row) for an empty row or segment, a NaN cut, a cut outside the segment, and a reversed cut, the last by contract. `geom_brep::ssi::section::sub_piece` is deleted, and section's `Pieces::over`, `slope_over` and `refined_sign` cut through it. `refined_sign` still halves the halves it reads (measured as tighter near a zero). (ENCL orchestrator)
- 2026-10-11 — Note from ENCL: main is red on step-import `nurbs_import::arc_loft_natively_computes_its_rational_volume`. ENCL bisected it to NURBS #4553. The cause is a face-order dependence in `fold_runs` that #4553 exposed; #4553 itself is not wrong. Filed `work/exch/arc-loft-round-trip-volume-differs-by-face-order.md` (P1) with the mechanism and two repair shapes. (ENCL orchestrator)
- **2026-10-11 08:25Z.**
  - **PR 4551.**
    - Build A's `1a61afdae7` restores 4553's MINOR-B guard at `hull_limb` itself: it is red under `Margin::of(hull.sup)`, with both arms executed. Accepted.
    - 4551 now conflicts with main over ENCL's PR 4653, which made `compose::sub_segment` public through `convex_step` and routed `ssi::section` through it. 4551 retires `convex_step` in favour of `between` and has `restrict_bernstein` do that cut.
    - Build A is told to leave one Bernstein-row cut through `between`, keeping ENCL's refusal contract, and to check QUAD's `bezier_bisect` row.
  - **The P0 slow-set reds:**
    - ENCL root-caused the import row as a face-order-dependent volume fold that 4553 exposed (`exch/arc-loft-round-trip-volume-differs-by-face-order`).
    - Build B's bisect pins the sweep digest to CLEAVE's PR 4626.
    - Build B is validating its PR. (NURBS orchestrator)
- **2026-10-11 09:00Z.** Build B root-caused the P0 slow-set reds. Both start at PR 4553: its first bisect to #4626 came from stale cargo artifacts in a second worktree.
  - **Sweep digest.** Only `arc_loft_1e9eps` moves, the roster's only rational-wall body.
    - Cause: `refine_dir` moved from `ring_lerp` (f64 λ) to the ring's step.
    - Every new volume and area enclosure nests inside the old one.
    - Re-cut accepted.
  - **Import bits.** Per-face flux is bit-identical native vs imported. `props::fold_runs` sums the same six terms in face-arena order, and the two arenas differ, so the bodies round 1 ulp apart.
    - 4553 only changed which orders happen to round alike.
    - 4551's filed `de_boor` row is the same mechanism; its "inputs differ" premise is refuted.
  - **Rulings:**
    - The import row is re-scoped to per-face bit identity, each body's own paths, and overlapping body enclosures (ENCL's repair (b), not a re-pin: the row's premise was false).
    - Summing in shell order was built and backed out. It reddened two sweep bit-identity rows (one-call == sequential).
    - The exact, order-independent sum is a design fork (scalar traits, D9 idiom 2), filed for FLUX with `design: true`, for the designer pair.
  - Tier: orchestrator's read. (NURBS orchestrator)
- **2026-10-11 09:22Z.** PR 4551's main merge, over ENCL's PR 4653, reached a form finding.
  - `between`'s monotone corner `x + (y−x)·Λ` rounds twice at an exact dyadic Λ = ½. The convex `x·½ + y·½` rounds once.
  - ENCL's new section sign bisection (`a_sign_at_the_rows_floor_reads_on_halves_of_halves`) therefore hits its halving floor before the sign clears. It reads `None` where main reads `Some(true)`.
  - The designers' batteries drew random ratios, where monotone wins.
  - **Ruled: option 1.** Meet each corner's monotone lerp with its convex form. That is sound and never wider, and it refines fork row 107's adopted form rather than reversing it, so it is recorded for Ev in the PR body, not an `[ev]` PR.
  - Owed: cost, the batteries re-run, a mutation row per half, and the docs.
  - `sub_segment` gives way to `blossom::restrict_segment`, with ENCL's contract carried over. QUAD's `bezier_bisect` row closes with 4551. (NURBS orchestrator)
- **2026-10-11 09:42Z.**
  - PR 4551's step-form ruling is refined to **1b**: `between` meets the convex form only for a point ratio (`t.lo == t.hi`, a structural branch); an interval ratio keeps the monotone corner alone.
  - Measured:
    - the full meet (option 1) cost +37% on `canonical_composite`, 627 → 860 µs (main 400);
    - 1b costs +5%, at 660 µs;
    - ENCL's halves-of-halves row is green under both.
  - Owed: the four-ε suite, the batteries, a mutation row per arm, the endpoint census, and the docs. (NURBS orchestrator)
- **2026-10-11 09:57Z.** PR 4551: build A's equivalence probe refutes the case for 1b.
  - 394,939 draws compared 1b against always-meet at interval ratios.
    - 21% differ.
    - Always-meet is narrower in 95% of those, by up to 62.5% of the width.
    - 2.7% of draws are more than 1% narrower, all at thin ratios: knot quotients and drawn thin ratios.
  - No suite row tells the two apart.
  - Re-ruled: option 1, the unconditional meet, unless its suite-level cost (m5 fast set, `props::quad` rows) exceeds 1b's by more than about 10%.
    - If it does, 1b stays, with the trade stated at the claim site and a P3 revisit row.
  - Either way, a static thin-ratio row guards the meet. (NURBS orchestrator)
- **2026-10-11 20:15Z.** Resumed after a ~9.5 h usage pause; check-in 50 fired at 10:40Z and was delivered at 20:09Z.
  - **On main meanwhile:**
    - ENCL (`e91669029a`) re-scoped the arc-loft import row: volumes agree within faces−1 ulps.
    - INTENT u4 and the skin computed in `T` moved the serial golden again, with notes left on our P0 row.
    - CLEAVE PR 4707 fixed the support-plates red. The NURBS-filed GERM duplicate is closed against it.
  - **PR 4684 now conflicts in all four files it touches.** Build B is told to reconcile:
    - keep the stronger import assertions;
    - re-cut the digest only if it is still red on main, listing every cause.
  - **PR 4551.** Build A's cost run kept option 1 within the 10% bar. It is implementing the unconditional meet, with a pinned thin-ratio guard row: 16.7% narrower at Λ two ulps wide. (NURBS orchestrator)
- 2026-10-11 20:55Z — **Check-in 51.** Items 3 and 4 had already landed in PR 4742. INTENT landed a NURBS row on main, `the-arc-loft-thread-count-digest-is-red-on-main` (P1):
  - besides the `arc_loft_1e9eps` line, the two `sym_*` digest rows lost about 20 symbolic zeros (likely mover: INTENT's PR 4703);
  - the same rows read differently on main at 9baeb0056e and on PR 4732's CI at 090278cdb (sz 28/24 against 40/36), so the Sym split may be nondeterministic.

  Build B (PR 4684) is told to root-cause both before any re-cut, to report before pushing, and to close INTENT's row with every cause listed. PR 4551 (build A) is still running mutation rows for option 1; it has not pushed. — (NURBS orchestrator)
- 2026-10-11 22:50Z — **PR 4684 merges (e7bc2b7286). Orchestrator's read.**
  - It re-cuts the thread-count digest, which was red on main. Each cause is bisected and logged at `expected()`:
    - 4553 moved the volume, area and pads; the new enclosures nest inside the old;
    - 4708 moved the Sym split;
    - `8054ec236f` (skin in T) moved arc_loft by about an ulp, through libm. The enclosures nest, though not always new-inside-old, because the body itself moved.
  - It closes INTENT's P1 row `the-arc-loft-thread-count-digest-is-red-on-main`.
    - **The Sym split is deterministic.** It is byte-identical at 1 and 4 threads, run twice, alone and after the f64 block, at four commits.
    - **PR 4703's lost symbolic zeros were a regression.** Iso-boundary residuals are exactly zero by construction; 4703 discharged them only numerically. Main has since restored every class: every predicate×outcome count equals pre-4703's. Only `frozen` moves.
  - step-import's arc-loft row now asserts per-face bit identity and gate == the imported body's own measurement, beside ENCL's `faces − 1` ulp bound (test-support door `topo::test_support::mass_properties_with_face_terms`).
  - It files FLUX's design row on the face-order fold; a seam note is on FLUX's log.
  - Merged on a docs-only delta over green c25f38a643: a clean merge of main, and doc comments only in the PR's own diff.
- 2026-10-11 22:50Z — **PR 4551.** CI at b95395e1a9 was red on two counts:
  - the digest, which was main's and is now fixed by 4684;
  - E0560: ENCL's rung-3 hull refine renamed `Hull`'s fields under the `hull_limb` guard row. It is a semantic conflict with main that the textual merge cannot see.

  Build A is told to merge main (taking 4684's digest wholesale) and port the row. Its port of 4684's digest showed the convex meet leaves the digest unmoved at all three ε. Its de_boor row is corrected: the ulp is the face-order fold. — (NURBS orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4754, merged at `40eefeb668`): geom-core spline gains `Derived<'a, V = KnotVector> { Vector(V), SpanConstant(&[f64] breaks), Discontinuous { knots, degree } }`, which names why `derivative()` may have no vector. It is returned by `KnotVector::derived`, `SplineCoeffs::derived` and `TensorCoeffs::derived_u/_v`. `KnotVector::c1() -> Result<(), C1Break>` is the one home of the C¹ admission (`Degree1Crease` / `Crease`). Both `derivative()` doors are now `derived().vector()`. `TensorCoeffs::derivative_u/_v` are removed (tests read `derived_u().vector()`). `derived()` maps `clamped`'s `InteriorMultiplicityTooHigh` to `Discontinuous`; any other refusal is `unreachable!`, by the review's proof that a valid KnotVector cannot raise one. No golden moved. (ENCL orchestrator)
- 2026-10-12 02:50Z — **PR 4551: build A.**
  - Friction: the lane held its push three times for full four-ε local passes, each over an hour. Main moved underneath during two of them, re-opening conflicts in `props/quad.rs` and the digest, against discipline §2 (hosted CI is the verification of record). It was interrupted at 02:43 and pushed `52644abf56`: main merged, `Hull` port, main's digest wholesale. Its re-cut of `arc_loft_1e9eps` on top follows if CI reds.
  - The topo `conic_quadric` seed `0xe8586f5f5e218549` is red on main too, so it is not 4551's. It is added as a witness to GERM's `the-in-band-sweep-fuzz-finds-a-miss-answered-inside-the-escalation-gap`.
  - PR 4758 merged with Ev's sign-off: one `CARGO_TARGET_DIR` per worktree. META's row is closed. — (NURBS orchestrator)
- 2026-10-12 04:50Z — **PR 4551 at `0d5b56685e`.**
  - CI at f50ab134 is red only in the eps step. Both reds are main's, reproduced on main `8cd31ca1` and on the branch, each isolated:
    - sweep `at_interval_a_wide_section_refuses_as_section_not_a_point` is main's P0 `a-wide-loft-section-escalates-chord-side-at-1e-12-on-main`, with ENCL;
    - editor-core `a_ring_serving_two_pierces_records_their_fusion_unproven` is filed on FLUSH (P1).
  - Isolated cost table: head against 1b is m5 −0.6% / +1.0% and quad +6.9% / −2.8%, so **option 1 holds**.
  - The rule for merge: once CI on the new head is red in exactly those two base rows and nothing else, 4551 merges, with the DR row last and a standing-down comment on the PR. — (NURBS orchestrator)
- 2026-10-12 05:25Z — **PR 4551 merges (`aa1f124298`).** It is fork 3's step: `Certification::between` (the monotone corner lerp, met per corner with its convex form and with `hull(x, y)`) and `CoeffWindow::blossom` with the `restrict` doors. `convex_step`, `insert_once_ring` and `sub_segment` are gone.
  - The DR row went on as the last commit (concurrent, H, tally 0); the index number is assigned on main.
  - CI at `0d5b56685e` was red only on editor-core `a_ring_serving_two_pierces_records_their_fusion_unproven`, which is red on main and filed on FLUSH. The sweep base row `at_interval_a_wide_section_refuses_as_section_not_a_point` passed in that run because CI tests the PR merged into current main, which carries its fix (PR 4785, `031369bb81`). Build A's local repro on `8cd31ca1` predates that fix. Slow set 311/311.
  - `arc_loft_1e9eps` is re-cut on main's line, every enclosure nested inside the old. The isolated cost table keeps option 1.
  - Builds A and B are both standing by. NURBS has no unit in flight. — (NURBS orchestrator)
- 2026-10-12 05:26Z — **Wave 2 dispatched**, all on main `360cf55eb2`. This is a sequencing call: P3s next, and the P4 degree-elevation row waits.
  - **Build A** (session_01WzFKtLWBVWNnnS3M5ezaMB): `quad-de-boor-through-the-blossom-breaks-the-import-bit-identity`.
    - It re-routes `props/quad::de_boor` through the blossom and checks against 4684's per-face bit-identity row.
    - **Tier: single FULL**, because it is a certified quadrature path.
  - **Build B** (session_01VSe9NnecKcXKBB3VP44z5J): `tilted-lune-sits-at-the-f64-floor-of-its-band`.
    - First it settles whether the hosted/local split was a head difference or nondeterminism, then it gives the fixture margin.
    - **Tier: orchestrator's read** for a fixture margin, escalating to FULL if nondeterminism is found.
  - **Build C** (new, session_01BMzcQm9byLUetFkgRp3iPf), in two PRs:
    - `the-speed-meter-runs-three-times-per-corner-carrier`. **Tier: orchestrator's read**: a mechanical memo, re-measured.
    - `a-swaying-rational-corner-refuses-at-the-rational-speed-meter`. **Tier: single FULL**: a soundness bound, and a uniform-weight curve taken as rational. — (NURBS orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4784, merged at `854efb839a`): `geom_core::spline::compose::tensor::cells_touched` now reads only the cells whose OPEN interior a window meets, falling back to the closed overlap for a point window on a knot or off the domain. Its doc states it is sound only for a quantity continuous across the breaks (its one caller, `surface_curve_residual`, bounds values, and KnotVector enforces C⁰). A soundness pin in `m5_pr7b_tensor_compose` samples windows on and ulps beside a C⁰ knot. (ENCL orchestrator)
- 2026-10-12 07:50Z — **Main red, caused by NURBS.** PR 4803 (the speed-meter memo, `880e5d25`) moved sweep's thread-count digest `Sym` rows without re-cutting them. At ε 1e-9, `sym_arc_loft` frozen goes 1060 → 1065 and `sym_thin_strip` 1177 → 1181; the f64 rows are unaffected.
  - Build A bisected it over main's first-parent merges while on PR 4807.
  - **Orchestrator's miss:** 4803 merged on my read. Its CI ran no sweep slow set (the diff seeded only geom), and I did not check the digest before merging. The class is a change to a shared meter that downstream golden digests read, merged without the digest that reads it. Next time I run the sweep digest locally, or seed it, before merging a geom meter change.
  - The fix is dispatched to build B as a small PR on main: re-cut the two `Sym` lines at all three ε after confirming the move is benign (only `frozen` moves). PR 4807 then takes main's lines. — (NURBS orchestrator)
- 2026-10-12 07:05Z — **PR 4807 review.** Tier single FULL, reviewer session_01F23963ssGgywCa4xgDz8Xj at `c19c9e7dee`. Verdict APPROVE-WITH-FIXES, no MAJOR; soundness, the identity claim and re-cut nesting were executed and hold.
  - MINORs:
    - the 2-ulp misstatement in three places;
    - a stale fixture-header prose class;
    - a stale line in the blossom row;
    - no row guards `within` (the overlap mutant stayed green).
  - Style: two spellings of one recurrence; a dropped typed error. A fix pass is sent to build A. — (NURBS orchestrator)
- 2026-10-12 08:45Z — **PR 4814 merges (`5e46b73af5`); main's sweep digest is green again.** Build B re-cut only the `Sym` lines of the thread-count digest at all three ε: `sym_arc_loft` 1062→1067 / 1060→1065 / 1061→1066 and `sym_thin_strip` 1178→1182 / 1177→1181 / 1177→1181 (ε 1e-6 / 1e-9 / 1e-12). Only `frozen` moved; the f64 rows did not move.
  - Tier: orchestrator's read (a digest-only re-cut). The diff is the two lines per ε file plus the re-cut-log entry. CI was green across the board, `gate ok` included.
  - Build A is told to merge main into 4807, take main's `Sym` lines, and carry on with its fix pass. Build B stands by. — (NURBS orchestrator)
- 2026-10-12 08:23Z — **PR 4804 review** (the rational speed meter). Tier single FULL; reviewer session_01Ri9nDD5m5D1nZKjQMnDivL at `6845b08ebd`. Verdict APPROVE-WITH-FIXES with no MAJOR. Claims C1–C4 were executed and hold. The interval meter is sound over 2176 curves plus a steep sweep. Reach: median 0.97 against 0.84, and 1551 of 2176 positive against 586. The 4803 memo holds.
  - MINORs:
    - the `Sym` gap is unscheduled, because the issue names only Dual;
    - the per-PR row cannot see a small overshoot, since the fuzz is nightly-only.
  - Notes:
    - the degree-1 f64 overshoot (about 16 ulps, pre-existing) against a doc saying "an ulp", and a strict fuzz bound that risks flaking;
    - the arm choice is not monotone in reach;
    - the degree 4–5 frontier is unstated.
  - Style:
    - `choose` duplicates `binomial`;
    - the derivative net is copied three times and the centroid fold twice;
    - arithmetic claims in the docs are unbacked;
    - piece reading has two spellings.
  - A fix pass is sent to build C. **Class lines:** a duplicated combinatorics helper and a derivative-net copy are both Q1 instances. The fix pass homes them or files them. — (NURBS orchestrator)
