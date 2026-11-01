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
- 2026-10-12 09:20Z — **PR 4804 merges (`1639c60ff9`)**: the rational speed meter, with the piece assembly on `A′w − Aw′`, and the equal-weights gate.
  - Fix pass `0413c4c72b`: every review item is verified in the delta.
    - Sym is named in the Dual issue.
    - A degree-1 tightness row goes red at a 1e-4 inflation.
    - The f64 posture says "a few tens of ulps" (16 measured), and the fuzz margin is 64 ulps.
    - The arm-choice row is filed at P3 and the "meters it exactly" docs are reworded.
    - The degree 4–5 frontier is stated.
    - `choose` is folded into `binomial`.
    - The derivative-net copies are filed at P4.
    - The doc arithmetic is backed by unit tests.
  - CI: the only red was editor-core's base row (filed on FLUSH).
  - **Digest check, owed since the 4803 miss:** I ran sweep's thread-count digest locally at ε 1e-6, 1e-9 and 1e-12 on the head merged into main. It is unchanged.
  - **Friction:** that run's test job took 32 min (08:35–09:07), against the 15-min bar. It is the ciw row already filed (`a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar`), seen here on a geom-seeded diff.
- 2026-10-12 09:22Z — **Build C dispatched on `nurbs/one-speed-meter-arm`.** One unit closes three rows that are one decision in `nurbs.rs`:
  - `the-speed-meters-arm-choice-is-not-monotone-in-reach` (P3);
  - `the-integral-speed-meter-drops-a-dual-weights-tangent` (P4). Sym carriers are metered in practice, since 4803 moved the Sym digest, so the gap is reachable;
  - `the-span-derivative-net-is-spelled-three-times` (P4).
  - My prior is to join the arms on any weight channel, which closes the Dual/Sym gap by construction. The lane decides on measured reach and cost, and stops if it finds a genuine fork.
  - **Tier: single FULL** (a soundness bound). The digest obligation is written into the brief.
  - Build B is standing by.
  - The P4 degree-elevation row stays unbuilt: its own fix says to act only when a consumer needs better than about `1e-12/g`, and none is known. — (NURBS orchestrator)
- 2026-10-12 09:45Z — **Build C measured before building, and the arm choice is ruled.**
  - **Reach** (f64 meter over the enclosure ceiling):
    - Integral arm at 32 pieces matches the rational arm exactly on polynomial carriers, since with w′ = 0 the numerator collapses to `w²·C′`.
    - Degree 3, a = 4: 0.128 at 16 pieces against 0.552 at 32.
    - 254 random nets: p10 0.741 at 16 pieces against 0.946 at 32.
  - **Cost** (Interval loft, PR 4803's harness):
    - integral32: about 1.5×.
    - Joining every weighted carrier through the rational piece assembly: 7–9×.
    - Joining with the refined scan as well: 15–24×.
  - **Ruled:**
    - Rows 1 and 3 land as PR 1 on `nurbs/one-speed-meter-arm`, with `INTEGRAL_METER_SPLITS = 32` and the arm choice kept on the value channel.
    - Row 2 (Dual/Sym) is not closed by integral32. It lands as PR 2 on `nurbs/held-constant-weights`, through option (b): a held-constant query, so the integral arm runs only on weights that are all the same plain constant.
    - Option (a), joining for all, is rejected on cost. Option (c), routing value-equal weights other than 1 to the rational arm, is rejected as half a fix.
    - This is the orchestrator's call: one option dominates, and it is reversible. The lane greps for an existing constant-scalar spelling first.
  - (b) touches `geom-core/src/sym.rs` (DECIDE, SYM) and `spline/locate.rs` (KNOT, NURBS). That is shared ground, announced on their logs.
  - Also to be filed: the rational arm's refined scan costs about 190 ms per rational loft on main (P3). — (NURBS orchestrator)
- 2026-10-12 09:55Z — **PR 4807 merges (`19663be2ab`).** `props/quad`'s de Boor now goes through the blossom: `blossom::diagonal_in_span` is the one door, checked once against the span's closure. Inside the closure every level is `between`; outside it the affine extension handles a vertex pad's node.
  - Every fix-pass item is verified in the delta:
    - the 2-ulp figure is corrected in three places;
    - the fixture-header sweep covers nonuniform_loft and swept_elbow (including a pad prose that was already stale on main) and loft_prism;
    - the blossom row's line and `pr: 4807`;
    - the straddle row `a_node_straddling_its_span_end_is_enclosed_by_the_extension`, red under the overlap mutant;
    - `within` and the `unwrap_or_else` are gone;
    - the module doc names the door;
    - the census is back to main's 11/6.
  - CI on `89f4ebe1` is all green, and editor-core's base row passed too. The lane ran the sweep digest at all three ε on the merged tree, green, and its `Sym` lines did not move.
  - **Cost:** the digest row runs 1.7× slower solo (opt1 64 s against 38 s; `between` swapped to affine gives 34 s), all of it in `between`'s arithmetic. The CI `test` job does not measurably move (24 min, within the spread of recent runs). The cost is filed as `routed-de-boor-costs-the-sweep-digest-row-1-7x` (P3).
  - The P1 import bit-identity row is closed. The FLUX face-order design row is still Ev's.
- 2026-10-12 09:57Z — **Build B is dispatched on `nurbs/between-cost`**, the 1.7× row.
  - It profiles first. The shape is the lane's own, for example a point-ratio fast path or per-level hoisting, but soundness must not move; if the only fast route changes what `between` promises, it stops and reports. Goldens and digests are re-cut if bits move.
  - **Tier: single FULL.**
  - Build A is standing by. Build C is on the speed meter's PR 1 (integral32), with PR 2 (held-constant weights) to follow. — (NURBS orchestrator)
- 2026-10-12 10:16Z — **PR 4824 review** (integral32 plus one home for the derivative net and centroid). Tier single FULL, reviewer session_01Kuu9qM227brVKpQWJGGj37 at `a390565cc2`. Verdict APPROVE-WITH-FIXES, no MAJOR.
  - **Executed:**
    - Soundness holds: 0 violations over 6000 curves at both lanes.
    - The refactor commit's output is byte-identical to main.
    - The k/64 stall grid is needed: a mutant that skips a piece is invisible on k/32.
    - The arm choice is monotone on the swaying family.
    - The 1.5× cost was not reproduced in a meter-only microbench.
  - **MINORs:**
    - M1: degree 11–13 integral carriers lose the piece assembly at 32 pieces. Fix: fall back to the largest admissible power of two.
    - M2: about 5% of nets read lower at 32 than at 16, worst −24%, because each sub-piece picks its own direction. Disclose it, pin it, and file a P4 row for monotone refinement.
  - **Class line (S1):** five more spellings of the span derivative net exist outside `nurbs.rs` (topo `shell.rs`, geom-brep `pcurve_cache.rs` ×2, geom-core `hull.rs` `deriv_coeff`, `offset_derive.rs`, `ssi/enclose.rs`). The fix pass lists them and files a shared-home row.
  - **Also:** the `RATIONAL_PIECE_SPLITS` equality claim is held by hand (S2); the monotone row's 1% tolerance is loose (N1, S4).
  - The fix pass is sent to build C. — (NURBS orchestrator)
- 2026-10-12 11:19Z — **PR 4834** (build C's PR 2, `held_constant`, stacked on 4824) is up and green at `2fdcb41f58`. Build C reports it found no public "plain constant" spelling to reuse (Q1). The digest is unchanged, because the fixtures' weights are literals. A single FULL reviewer is dispatched on the delta over 4824: session_012aviQwSFoTSQnMXiDKyFH9.
  - **Seam:** a `Probe` impl in `k_stats.rs` (SCALAR, VERDICT), noted on both logs.
  - **Friction:** the 4824 fix pass, sent at 10:16Z and again at 10:33Z, went unacknowledged while build C built PR 2. It was re-sent at 11:18Z as an interrupt. The likely cause is that the lane read messages only at turn boundaries during a long local run. — (NURBS orchestrator)
- 2026-10-12 11:22Z — **PR 4841** (build B, the `between` cost) is up at `8938a02769`.
  - **Profile:** 99.96% of `between` calls on the row have a point ratio, and about half the cost was the interval multiply's 8 corner products.
  - **Fix:**
    - the backend `Mul` (`interval-transcendentals/src/arith.rs`) folds only a point operand's distinct corners;
    - `between` skips the clamp for a certified ratio already inside (0, 1).
  - **Claimed:** bit-identical, with no re-cuts. The slowdown goes from 1.70× to 1.22× at opt1 and from 1.73× to 1.12× in debug.
  - **Filed:** Div's point sibling (P3, E).
  - **Tier: single FULL**, reviewer session_01MaoqGhPJrPGK4KTSjVjNya. The backend multiply sits under every certified result, so bit-identity is the soundness claim; the review is pointed at signed zeros, infinities, the exactness witness and rounding direction.
  - **Seam:** `between_exact_fuzz.rs` is TCOST/TINT ground (a new test file). — (NURBS orchestrator)
- 2026-10-12 11:52Z — **PR 4841 review** (the `between` cost; interval point-Mul). Tier single FULL, reviewer session_01MaoqGhPJrPGK4KTSjVjNya at `8938a02769`. **APPROVE**, with no MAJOR and no MINOR.
  - **Executed:**
    - a 20M-pair differential against the old literal four-corner fold, bit-identical on bounds and decoration, covering ±0, subnormals, ±∞, NaI, and raw `[−0, +0]` and `[+∞, +∞]` points;
    - the interval crate is green with `oracle-inari`;
    - the clamp skip is shown to be the identity, and its mutants go red;
    - the digest and step-export sidecars are unchanged;
    - the timing reproduces at 0.79×.
  - **NOTE N1** corrects my dispatch: `between_exact_fuzz.rs` predates the PR.
  - **Style:** the skip guard's comment understates the set it covers; Div keeps a literal four-corner fold beside `corner_bounds` (its filed row should say to reuse the helper); the `ops.rs` prose is stale; the point path's `+0` premise is unnamed. A comment-and-doc pass is sent to build B.
  - **Head:** `bb0bdaf74d` adds only the census row `certified_endpoint_census` (8, 8) → (9, 9) for the skip's guarded endpoint read. — (NURBS orchestrator)
- 2026-10-12 11:55Z — **PR 4834 review** (`held_constant`). Tier single FULL, reviewer session_012aviQwSFoTSQnMXiDKyFH9, on the delta over 4824. Verdict APPROVE-WITH-FIXES, no MAJOR.
  - **Executed:** the edge-probe row; mutants M1–M5, each red; the digest, unchanged at all three ε; no session side effects.
  - **MINOR-1 (class):** INLANE's P2 `held-value-equalities-decided-on-the-value-channel` is this class, and it prescribes a T-margin primitive. The reviewer found two more sites (`loop_area.rs` `nurbs_vector_area`, `nurbs_iso.rs` `interior_iso_u`).
    - **Ruled:** no primitive switch in 4834. The plain-constant gate is sound and strictly more conservative, and the T-margin would only add reach. The PR cross-links the issue, adds the two sites as evidence, and records the migration. A seam note is on INLANE's log.
  - **Style:** a fourth spelling of the Lit node (needs one constructor); `held_constant` is renamed because "held" means the opposite elsewhere; stale "bit-exact unit weights" citations in sweep `skin.rs` and the m7 test. The fix pass is queued behind 4824's. — (NURBS orchestrator)
- 2026-10-12 11:56Z — **Friction (CI):** a comment-only fix pass on PR 4841 (`37f905e685`) cancelled the in-progress run on the green-bound `bb0bdaf` (cancel-in-progress concurrency). The change filter also gates `.rs` comment edits as code, so a style pass that touches only comments costs a full gate. The docs-only merge rule does not save it either, because there was no green head left to merge on. To bring to CIW if it recurs: a run on a head about to be superseded by a comment-only commit could be allowed to finish, or the filter could recognise a comment-only `.rs` diff. — (NURBS orchestrator)
- 2026-10-12 12:08Z — **PR 4824 merges (`eaa4953dc4`).** The integral meter now reads 32 pieces, and the derivative net and centroid each have one helper. Fix pass `ecd814c6e2`, verified against the review:
  - **M1:** `piece_count` falls back to the largest power of two that `pieces_exact` admits: 32 pieces to derivative degree 9, 16 to 12, 8 to 16, 4 to 24, 2 to 47.
    - Both arms use it. The rational arm abstains above p = 24, and `binomial_is_exact_to_47` pins the pair products to p = 24.
    - Row `a_high_degree_net_keeps_its_piece_reading` (degree 11 and 13) is red without the fallback.
    - The rational-arm extension past degree 9 is the orchestrator's read, not the reviewer's: the same exactness machinery, pinned by tests.
  - **M2:** disclosed. Case 2349 is pinned (`a_finer_schedule_can_read_lower_on_a_degree_ten_net`) and filed as `the-integral-meters-piece-refinement-is-not-monotone` (P4).
  - **N1 and S4 found something:** the specified random-net row fails. On 4.3% of nets the rational arm reads up to 4.6% higher one ulp off, because at w ≡ 1 its numerator is the degree-elevated derivative (same direction, tighter hull) and because of the refined scan. The fuzz `the_rational_piece_reading_is_the_elevated_integral_one` pins the inequality, and the gap is filed as `the-integral-arm-reads-below-the-rational-arm-on-an-equal-weight-net` (P4).
  - **S2:** `RATIONAL_PIECE_SPLITS = INTEGRAL_METER_SPLITS` by definition.
  - **S1:** a six-site disposition table; the shared home is filed as `the-span-derivative-scale-is-spelled-in-six-places` (P4).
  - CI is green. The digest is unchanged at all three ε. The geom suite passes 418/418.
  - Rows 1 and 3 are closed. Row 2 is PR 4834. — (NURBS orchestrator)
- 2026-10-12 12:40Z — **PR 4841 merges (`45a5059c3d`).**
  - **The change:** the interval backend's `Mul` folds only a point operand's distinct corners, and `between` skips the identity clamp for an interior certified ratio. Output is bit-identical, so nothing was re-cut.
  - **Cost:** the sweep digest row drops from 1.70× to 1.22× of its pre-4807 time at opt1, and from 1.73× to 1.12× in debug. The review reproduced the speedup at 0.79× of main.
  - **CI on `37f905e685`:** the only red is editor-core's 1e-12 base row, which fails identically on the base and on main and is already filed on FLUSH. The interval oracle job ran and passed.
  - **Since the reviewed head:** the census row (8, 8) → (9, 9), and comments and docs (S1–S4).
  - The `routed-de-boor-costs-the-sweep-digest-row-1-7x` row is closed.
- 2026-10-12 12:41Z — **Build B dispatched** on `interval-division-folds-duplicate-corners-for-a-point-operand` (P3, E), branch `nurbs/div-point-paths`.
  - **Tier: single FULL**, the same as `Mul`: a backend op sits under every certified value.
  - **The oracle** is the literal four-corner code, independent of the shared helper (the 4841 review's N2).
  - The refined-scan row (`the-rational-meters-refined-scan-costs-190-ms-per-loft`, P3, M) goes to build C after 4834. Build A is standing by. — (NURBS orchestrator)
- 2026-10-12 12:47Z — **PR 4834 merges (`8602894934`).** `SpanLocate::plain_constant` lets the speed meter take its integral arm only when every weight is the same plain literal; Dual and Sym weights take the rational arm.
  - **Fix pass `2200a6dcc4`, verified:**
    - MINOR-1: INLANE's class row cites this PR and lists the two new sites, plus the migration note.
    - S1: `SymNode::literal` is the one spelling of the literal node.
    - S3: renamed `held_constant` → `plain_constant`. The seam notes on DECIDE, SYM, SCALAR and VERDICT use the old working name; logs are append-only, so this line is the correction.
    - S2, S4, S5 are done.
    - S7: the sweep `skin.rs` and m7 citations are updated.
    - NOTE-1: the E12 qualification is added.
    - NOTE-2: the issue body is corrected.
  - CI is green. The digest is unchanged at all three ε. Row 2 is closed, and with it all three rows of the speed meter's arm decision.
- 2026-10-12 12:48Z — **Build C dispatched** on `the-rational-meters-refined-scan-costs-190-ms-per-loft` (P3, M), branch `nurbs/refined-scan-cost`. It measures what the refined scan decides that the piece assembly doesn't, then drops it, cheapens it, or keeps it with a stated reason. A cost-versus-reach trade on a nameable class stops for a ruling. **Tier: single FULL.** — (NURBS orchestrator)
- 2026-10-12 12:52Z — **Build A dispatched** on `the-speed-meter-memo-is-visible-to-a-sym-session` (P3, E, filed by CLEAVE), branch `nurbs/memo-sym-invisible`.
  - PR 4803's memo caches the meter before any Sym session exists, so the cached value freezes in every later session. That is the cause of the `Sym` frozen move that PR 4814 re-cut, and it makes the memo's invisibility claim false at Sym.
  - **My lean:** keep the contract and don't serve a pre-session value inside a session, rather than documenting the exception. The lane weighs both on cost and stops if neither is clearly right.
  - **Tier: single FULL**, because Sym semantics and the digest are involved.
  - With this, all three builds are busy: A on the memo, B on the Div point paths, C on the refined scan.
  - **Remaining P4 slate:**
    - the six-place derivative scale;
    - non-monotone refinement;
    - the integral arm reading below the rational arm;
    - degree elevation, which is conditional on a consumer. — (NURBS orchestrator)
- 2026-10-12 13:06Z — **PR 4851 review** (Div point paths). Tier single FULL, reviewer session_017xPqznz4GPEmbFbt1CzfYe at `ce1223dc15`. Verdict **APPROVE**, no MAJOR and no MINOR.
  - **Executed:**
    - a 6M Div plus 6M Mul differential against the old literal code, and a 3.45M-pair edge table, with zero diffs;
    - the ±0-numerator premise holds;
    - touching divisors are excluded by construction;
    - 11 of 12 mutants go red;
    - the op is about 2.6× faster.
  - **NOTE N1:** `lower_of`'s zero rule is dead in the least-fold, because no lo corner is ever −0. The equivalent mutant survives, so nothing can pin that rule today.
  - **Style S1** is the style brief's named trap: the fix removed the duplicate fold but minted a duplicate four-arm dispatch in `Mul` and `Div`. It is sent back with S5 (the corner pair becomes structural) and S2. — (NURBS orchestrator)
- 2026-10-12 13:29Z — **PR 4853 merges (`557ce59178`).** It is a doc-only sentence at Sym's `plain_constant` saying why the check does not read the node's form: a form expansion moves the digest's `SymCounts`. The sentence missed the 4834 merge, and build C correctly re-landed it on a fresh branch.
  - **Friction (second instance today):** a message to build C reached it about 40 min late. The refined-scan dispatch was sent at 12:48Z and had not arrived by 13:27Z. Re-sent as an interrupt, with a request to acknowledge. — (NURBS orchestrator)
- 2026-10-12 13:33Z — **Ruled: drop the rational meter's refined scan (option A).** Build C measured before building, on main `f581d1ae29`.
  - **What the refined scan decides:**
    - On every m5_pr7 fixture the piece assembly is at least as high as the refined scan, frontier rows included.
    - Across 12,000 random rational nets (degrees 1–6, four weight regimes) it changes **no decision**.
    - It wins by more than 1% only on degree 2: about 1% of those nets, usually by 1–3%, at worst 2.07×.
  - **Cost:** the refined scan is about 74% of the rational meter on a long carrier (36.8 of 49.9 ms on a 37-span cubic). About 90% of that is quadratic refinement plumbing (`equal_split` plans, `refine_homogeneous` per channel).
  - **Options weighed:**
    - (B): make it local and linear. A real rewrite in geom-core spline, done only to keep reach that decides nothing measured.
    - (C): keep it and document.
  - **Why (A):**
    - it is the simpler final state;
    - it is reversible, since a degree-2 consumer that needs the reach would get it in the piece assembly;
    - above degree 24 the meter refuses typed.
  - This is the orchestrator's call: an internal meter trade with a dominant measurement, touching no ratified text.
  - Build C builds it with pin rows for degree 2 and degree 25, and re-cuts any golden that moves. Any other caller of the refinement plumbing gets its own cost row.
  - **Tier: single FULL.** — (NURBS orchestrator)
- 2026-10-12 14:10Z — **PR 4851 merges (`46c91a67cb`).**
  - **The change:** Div's point paths now share one dispatch and fold with `Mul`. `corner_bounds::<Op: CornerOp>` takes `Product` or `Quotient`, so a caller cannot swap the lo/hi pair (S1, S5). The trait form measured faster than an fn-pointer struct: 31.3 against 38.7 s on the digest row.
  - **Cost:** the digest row is about 11% faster than main.
  - **Checked since the reviewed head `ce1223dc`:** only `arith.rs`, with the dispatch moved into the helper. The rows still compare against the literal `four_corners` oracle.
  - **CI:** the only red is editor-core's 1e-12 base row; the interval oracle job passed.
- 2026-10-12 14:11Z — **Build B dispatched** on `the-span-derivative-scale-is-spelled-in-six-places` (P4, M), branch `nurbs/one-derivative-scale`.
  - The scale and the net get one `T`-generic home in geom-core `spline`, and the zero-difference policy comes from the caller.
  - The claim is bit-identical. A moved golden stops the lane rather than being re-cut.
  - The sites span TOPO, geom-brep and geom-core ground, and the lane announces the seams.
  - **Tier: single FULL.** — (NURBS orchestrator)
- 2026-10-12 14:20Z — **PR 4860** (build C, drop the refined scan) is up at `51eec7dbd3`, and a single FULL reviewer is dispatched (session_01XyZf9FSjWdA4nhJroqtfua).
  - **Meter cost:** 49.9 ms → 13.7 ms per call on a 40-point cubic. The 4803 harness goes 264 → 76 ms.
  - **Deleted:** `SpanLocate::refine_homogeneous` / `refine_weights`.
  - **Kept:** `RingRefinement`, which has other callers. Its quadratic plan chain is filed as `ring-refinement-inserts-one-point-per-plan-over-the-whole-net` (P4).
  - **Side effect:** the equal-weight arm gap shrinks from 4.3% of nets (worst 4.6%) to 1.9% (worst 0.20%).
- 2026-10-12 14:30Z — **Main's digest is green at all three ε.** Build A's status reported a suspected frozen-count rise from PR 4834, so I ran sweep's thread-count digest (both rows, golden walk included) on main `562f8e7f1d` at ε 1e-6, 1e-9 and 1e-12, on an isolated target. All pass. I asked build A to name what its rise is relative to. — (NURBS orchestrator)
- 2026-10-12 14:33Z — **PR 4866** (build B, one span-derivative scale) is up at `2e53479560`.
  - **Routed:** 5 sites, plus a seventh found by the shape sweep (`ssi/enclose.rs` `bezier_scales`).
  - **Left on their own association:** `hull.rs` `deriv_coeff` and `offset_derive.rs`, which use `(Δ·p)/Δu`.
  - The claim is bit-identical. The single FULL reviewer is session_01YM7Atx7n64oDJr2vgFucdD.
  - Seam notes are on the PCERT, PCTAIL, SSIARITH, SSIEDGE, SSIMARCH, KNOT, SHELF and SHELL logs.
- 2026-10-12 14:34Z — **Build C dispatched** on `the-integral-meters-piece-refinement-is-not-monotone` (P4, M), branch `nurbs/monotone-piece-refinement`.
  - It measures both fixes first: sub-pieces trying the parent direction, and max of the two schedules.
  - "Leave it, disclosed" is an acceptable answer. A trade stops for a ruling.
  - **Tier: single FULL.** — (NURBS orchestrator)
- 2026-10-12 14:35Z — **PR 4869** (build A, the memo invisible in Sym) is up at `afff68b0d8`.
  - **The fix:** the memo serves only when no Sym session is installed (`sym::session_counts()`). A per-session key would thrash, and documenting the exception would leave a session's result depending on read history.
  - **Row:** `a_meter_read_before_a_session_is_not_served_inside_it`, red on main.
  - **Cost:** unchanged within noise.
  - **Finding:** the `Sym` frozen counts rise to about 1915/2094, not back to 1060/1177.
    - The memo had hidden PR 4834 (the `plain_constant` gate) from the digest. In-session, carriers whose weights are equal but non-literal now take the rational arm. Build A bisected merges 4803 → 4849 to pin it on 4834.
    - Decisions, `sz`, `num` and the f64 rows are unchanged. The whole rise is contradiction checks on numerically decided signs (54 → 1848).
    - **Orchestrator's position:** this is 4834's conservative gate measured honestly, and a baseline is no target. The reach it gives up belongs to INLANE's T-margin primitive, and the measurement is recorded on INLANE's held-value row.
    - **My miss, second instance today:** 4834's digest check passed only because the memo hid the change. The 4803-miss class ("a meter change merged without the digest that reads it") has a sub-case: a digest that passes can still be masked by a cache.
  - A single FULL reviewer is dispatched (session_01LtWLeFccm7GHfs9HZgQWtj). It also checks thread-sharing of the session check, and whether the frozen rise costs the session anything it can prove. — (NURBS orchestrator)
- 2026-10-12 14:45Z — **Piece refinement that isn't monotone: build C measured and stopped.**
  - **Reach** (6000 integral nets, degrees 2–10): plain 32 reads below 16 on 6.0% of nets, worst −44%.
    - (a), letting a sub-piece also try its parent's direction, is provably monotone in ℝ, never lowers a reading, and dominates (b) on every net.
    - (b) is the max of the 16- and 32-piece schedules.
  - **Cost** on the 4803 harness: plain 32 is 12.6 ms, (a) 19.5 ms (+55%), (b) 17.2 ms (+37%).
  - **Ruling, conditional and consistent with the refined-scan test** (reach that changes no decision is not worth its cost):
    - if no decision flips, the row is deferred with (a) recorded as the fix;
    - if any decision flips, build (a).
  - The lane measures flips and consumer thresholds next. — (NURBS orchestrator)
- 2026-10-12 14:50Z — **PR 4871 merges (`38d461e75f`, docs only).** `the-integral-meters-piece-refinement-is-not-monotone` is **deferred**.
  - **Flips:** 0 of 9000 nets (degrees 2–13).
  - **Consumer thresholds:** `Margin::metered` in certify, `topo::split` and `pcurve_cache` are each decided against the tolerance band. The least span·meter among the nets that lose reach is 0.07, so none crosses a band.
  - **(a) is recorded as the fix** for when a consumer needs it: sub-pieces also try the parent direction. It is monotone by construction, dominates (b), and costs +55%.
- 2026-10-12 14:51Z — **Build C on two measure-first rows:**
  - `the-integral-arm-reads-below-the-rational-arm-on-an-equal-weight-net`: only degree elevation is left (1.9%, worst 0.20%). Defer if there are no flips.
  - `ring-refinement-inserts-one-point-per-plan-over-the-whole-net`: measure its real callers' share (`net.rs`, `compose.rs`). Build the linear form if the share is visible, otherwise defer. — (NURBS orchestrator)
- 2026-10-12 14:53Z — **PR 4860 review** (drop the refined scan). Tier single FULL, reviewer session_01XyZf9FSjWdA4nhJroqtfua at `51eec7dbd3`. Verdict APPROVE-WITH-FIXES, no MAJOR.
  - **Executed:**
    - Interval: 0 violations over 2500 nets (degrees 1–25 × 5 weight regimes);
    - 0 decision flips against main to degree 24;
    - rational goldens unchanged under the default profile;
    - the demos are clean;
    - cost 44 → 13 ms reproduced.
  - **MINORs:**
    - orphaned API left by the deletion (`apply_homogeneous_in`, `source_count`, `plans`);
    - a gate-script census naming a deleted function;
    - the ring-refinement row names the wrong callers (the real ones are in geom-brep: SSI, quadrature, `patch_bound`);
    - the f64 rounding-posture sentence is false by about 10⁷. The reviewer measured about 3e8 ulps at 1e±6 weights against exact arithmetic. It was pre-existing, but this diff restated it.
  - **Class line:** a deletion's orphan sweep must grep the callers of every kept and helper symbol, not only the deleted ones. The fix pass re-greps.
  - The fix pass goes to build C ahead of its measure-first rows. — (NURBS orchestrator)
- 2026-10-12 15:10Z — **PR 4869 review** (memo invisible in Sym). Tier single FULL, reviewer session_01LtWLeFccm7GHfs9HZgQWtj at `afff68b0d8`. Verdict APPROVE-WITH-FIXES, no MAJOR.
  - **Executed:**
    - history runs, each worktree with its own target, which pin the rise exactly: the 4834 merge with only the arm choice reverted reads 1060/1177;
    - `SESSION` is thread-local, so invisibility holds across the 4-thread walk;
    - mutants: no bypass and read-only-in-session are red, fill-in-session survives;
    - the golden at 1e-9 passes on the head.
  - **NOTE-4, the proof-power question:**
    - No decision changes.
    - The rise is entirely `Origin::Assertion`: the contradiction checks on numerically decided signs. Beyond the 32/4 budget, about 1.8k check sites per validation freeze.
    - So 4834's conservative gate costs **dispute-detection coverage, not proof**. The digest prints no `theorems_disputed`, so it cannot show whether that loss ever matters. That is filed as a guard gap via the fix pass.
  - The fix pass covers the stale uncached doc, a row pinning that a session never fills the memo, one named door for "a session is installed", and the `OperandAxes` disposition. — (NURBS orchestrator)
- 2026-10-12 15:12Z — **PR 4866 review** (one span-derivative scale). Tier single FULL, reviewer session_01YM7Atx7n64oDJr2vgFucdD at `2e53479560`. Verdict APPROVE-WITH-FIXES.
  - **Bits identical:** a 2M-input differential at f64, Interval and Dual found 0 mismatches. The left-out sites really do differ in association.
  - **Hit-list claim falsified:**
    - `step-import/src/recognize.rs` `derivative_sup` has two routable sites;
    - `props/quad.rs` `raw_deriv` is a third own-association site, misfiled.
  - **Goldens don't pin the association:** a `p·(1/Δu)` mutant stays green everywhere, so a differential unit test will be committed.
  - **Fresh instance:** a private surface `derivative_net` in `pcurve_cache.rs`, with the opposite zero policy to the new public one.
  - **Class (P1 by band):** three rational speed-bound recipes, in `shell.rs` `spline_speed`, `recognize.rs` `derivative_sup` and `pcurve_cache.rs` `curve_rate_bound`, with two different rational extensions between them. Build B is filing it as a P1 design row.
  - The fix pass is sent to build B. — (NURBS orchestrator)
- 2026-10-12 15:13Z — **PR 4873** (build C, docs only, stacked on 4860; build C first misreported it as 4872) defers two rows.
  - `the-integral-arm-reads-below-the-rational-arm-on-an-equal-weight-net`: over 4796 nets at Interval, 0 flips; the least span·meter among the 95 that read higher is 0.043. Degree elevation is recorded as the fix.
  - `ring-refinement-inserts-one-point-per-plan-over-the-whole-net`: at most 0.7% of wall time on its real callers (the digest walk, the m8_4 NURBS SSI, m8_3 rational mass props). Per-span linear refinement is recorded as the fix.
  - It merges after 4860. 4860's fix pass (sent 14:53Z) was re-sent as an interrupt: the third late delivery to build C today. — (NURBS orchestrator)
- 2026-10-12 15:22Z — **Friction, now a dropped message, not a late one.** My 14:53Z fix-pass message for PR 4860 never reached build C; only the 15:13Z interrupt resummary did. Earlier today two sends to build C arrived about 40 min late (10:16Z and 12:48Z). Working rule from here: every dispatch or fix pass to a lane asks for an acknowledgement, and an unacknowledged send is re-sent as an interrupt at the next check-in. Whether to bring this to META as a finding about the cross-session channel is the next orchestrator's call, if it recurs. — (NURBS orchestrator)
- 2026-10-12 16:00Z — **PR 4860 merges (`564664c8a7`).** The rational speed meter is now its piece assembly alone.
  - **Fix pass `bcf5c77099`, verified:**
    - orphans deleted (`apply_homogeneous_in`, `source_count`; `plans` is test-only), and none of the deleted symbols is referenced anywhere;
    - the gate census is fixed;
    - the RingRefinement callers are corrected (geom-brep quad and SSI);
    - the f64 posture is restated truthfully: not a bound, up to about 3e8 ulps at 1e±6 weights;
    - `RATIONAL_PIECE_MAX_DEGREE` is one const;
    - the degree-2 row pins its reading in a band, with a rigorous Interval ceiling;
    - the degree-25 row checks both lanes.
  - CI green.
- 2026-10-12 16:01Z — **PR 4873 merges (docs only).** Two rows are deferred:
  - the integral-below-rational gap: 0 flips at Interval over 4796 nets;
  - the RingRefinement quadratic: at most 0.7% of wall time on its real callers.
  - A note is on CHORD's log: `C23`'s geom side is gone with `RATIONAL_METER_SPLITS`. — (NURBS orchestrator)
- 2026-10-12 16:10Z — **PR 4869 merges (`8fc1358437`).** The speed-meter memo serves and fills only outside a Sym session.
  - **Fix pass `b513295e`, verified:**
    - `geom_core::sym::session_installed()` is the one door, used at three sites;
    - row `a_session_never_fills_the_speed_memo` is red under the fill-in-session mutant;
    - the non-discriminating asserts are cut;
    - `OperandAxes<T>` is dispositioned (it lives inside one op, and no shipped path calls `with_session`);
    - `the-thread-count-digest-cannot-see-dispute-coverage` is filed (P3, E).
  - CI green. The `Sym` frozen re-cut (about 1915/2094) is attributed to PR 4834's `plain_constant` gate. No decision moved; the change costs dispute-check coverage, not proof.
  - The SYM/DECIDE seam is noted on their logs.
  - Build A is dispatched on the dispute-coverage row (`nurbs/digest-dispute-coverage`). Tier: single STYLE.
  - I'm running the digest on main myself, because 4860 and 4869 landed back to back. — (NURBS orchestrator)
- 2026-10-12 16:20Z — **Main red, caused by NURBS: a semantic conflict between PRs 4860 and 4869.** The sweep thread-count digest fails on main `8fc1358437` at all three ε. Only the `Sym` `validate_geometric` frozen counts moved (arc 1914 → 1540, strip 2094 → 1656 at 1e-9). Decisions, `sz`, `num` and every f64 row are unchanged.
  - **Cause:**
    - 4860 removed the rational refined scan. Its digest check passed only because the pre-4869 memo hid in-session meter work.
    - 4869's re-cut was measured before 4860 landed.
    - Together, the in-session rational arm is cheaper and freezes less.
  - **Orchestrator's miss (third today, same class):** I merged two digest-touching PRs back to back and ran the digest only after the second.
  - **Rule from here:** when two merges in a row touch the same meter or the same digest, re-run the digest on main between them, or merge the second only after its CI has run on a base that contains the first.
  - The fix is dispatched to build A as a small re-cut PR. — (NURBS orchestrator)
- 2026-10-12 16:32Z — **Main is green again: PR 4879 merges (`535cb17594`).** It re-cuts only the six `Sym` `validate_geometric` frozen values, to 1542/1657, 1540/1656 and 1541/1656 at ε 1e-6, 1e-9 and 1e-12, plus a re-cut-log comment.
  - I merged it ahead of hosted CI as a red-main data fix. The values match my own measurement on main `8fc1358437`, and the author's run on the same base and its head read green at all three ε. A PR comment records the basis.
  - The red lasted about 20 min (16:10–16:32Z).
- 2026-10-12 16:33Z — **PR 4866 conflicts with main** in `from-f64-sites.tsv`: 4860 deleted rows that 4866 edits.
  - Build B is merging main in and will re-run the gates and the digest on the merged tree. Its CI on the merged head is the base-containing-the-fix check that the new back-to-back rule asks for.
  - My first local digest attempt was void: the merge into my worktree had silently failed on this conflict, so the run tested 4866's own tip. **Lesson:** confirm `git log` shows the merge commit before trusting a merged-tree run. — (NURBS orchestrator)
- 2026-10-12 17:24Z — **PR 4866 merges (`d5c5f5bbf4`).** The span-derivative scale and net have one home in `geom_core::spline::derivative`.
  - **Routed:** 7 sites, including step-import's `recognize.rs`.
  - **Left on their own association, with reasons:** `hull.rs`, `offset_derive.rs` and `quad.rs` `raw_deriv`.
  - The association is pinned bitwise at f64, Interval and Dual.
  - The private surface net is renamed `skipping_surface_derivative_net`.
  - The P1 design row `rational-speed-bounds-have-two-recipes-across-four-sites` is filed.
  - CI ran on a base containing the 4879 fix, and the merged-tree digest passes at all three ε.
- 2026-10-12 17:24Z — **Design fork dispatched:** the rational derivative bound (P1, `design: true`).
  - Two designers were dispatched together under `docs/prompts/designer.md`, with the same problem statement and no candidate solutions.
  - Blinding was done per `docs/DESIGN-FORK-PROTOCOL.md` (protocol `5dedb0a41a`); the mapping is on `analysis/design-fork/nurbs-rational-speed-bound`.
  - If the reports agree and it is not Ev's fork, the orchestrator rules. Otherwise it goes to an `[ev]` PR with A/B unlabelled.
  - Tracker PR 4884 merged. — (NURBS orchestrator)
- 2026-10-12 17:40Z — **PR 4887** (build A, the digest dispute column) is up and green at `8d260be3`.
  - **The change:** `counts_line` prints `disp=` (`theorems_disputed`). It reads 0 on all twelve Sym lines at all three ε.
  - **Answer to 4869's NOTE-4:** a probe with 4834's arm choice reverted also reads `disp` 0. So the roughly 1.8k extra frozen check sites cost no dispute detection on these lofts.
  - **Not added:** `retried`, because it is zero by construction (the digest's `with_session` has no retry ladder). This is better than the brief.
  - Single STYLE reviewer: session_012pqiv3hQFynNCn6ToMCDZw.
- 2026-10-12 17:31Z — **Design fork, rational derivative bound: A's first report is recorded** (blinded; B is pending).
  - **Recommendation:** one bound, the per-span quotient-term bound, as one geom-core door with `speed_upper_bound`/`SupSpeed` twins; the weight-ratio recipe is retired on the sup side.
  - **Zero-span predicate:** none needed, because walking nonempty spans makes every active pair's difference positive.
  - **Premise corrected:** three recipes at six sites, not two at four.
  - **Measured:** the per-span bound's median is 1.26× the true sup, against 19.7× for the weight-ratio recipe.
  - **Off-question:** `chart_stretch_inf`'s rational lower bound rests on an unproven B-spline extension of Floater. — (NURBS orchestrator)
- 2026-10-12 17:45Z — **Rational derivative bound ruled; the designers converged.** B's first report (17:41Z) agrees with A on substance.
  - **The bound:** one per-span pair-term bound. It is proven, needs no centre, and is tightest on every curve measured. Its geometric mean is 4.0× the true sup against 46.5× for ratio²; over 1600 curves, A measured a median of 1.26.
  - **Retire ratio² on the sup side:** Floater's theorem is for rational Bézier curves, and nothing proves its extension to B-splines.
  - **No zero-span predicate:** a zero or NaN knot gap is unrepresentable on a `KnotVector`, so all four spellings are dead.
  - **The premise was too narrow:** there are three recipes at six or seven sites, including `enclose.rs` and `mesh`'s `patch_bound`.
  - **The only difference was the home**, geom-core window versus geom. A's typed-window door answers B's raw-slice objection.
  - **Ruled by the orchestrator, no `[ev]` PR:** the designers agree and no ratified text is touched. The blinding record is on the analysis branch; this is not a DESIGN-FORK-LOG row, since only forks that go to Ev are logged there.
  - **Build C is dispatched** on `nurbs/one-rational-speed-bound`: the pair term stated once in geom-core, `speed_upper_bound`/`stretch_sup` in geom, every sup site routed, ratio² retired on the sup side, and the dead predicates removed. Tier single FULL per PR, with up to two stacked PRs.
  - **To be filed in that PR:**
    - `chart_stretch_inf`'s unproven rational inf (P2), with B's proven twin;
    - `mesh` recipe-A cells (P3). — (NURBS orchestrator)
- 2026-10-12 17:45Z — **A reviewer failed to start (my miss).** The PR 4887 STYLE reviewer was created with the short SHA `8d260be3`. Cloud sessions need the full 40-character SHA, and it failed `ref_not_found` with no notice to me; Ev caught it. I archived it and re-dispatched with `8d260be354a74038d4e111b449ad232e86dc13e3` (session_01Sk84Fad117KcnbGXLiqba5, connected). All other dispatches today used full SHAs. **Rule:** run `git rev-parse` on the head before every `create_session`, and confirm the session is connected after dispatch. — (NURBS orchestrator)
- 2026-10-12 17:50Z — **PR 4887 STYLE review** (session_01Sk84Fad117KcnbGXLiqba5): mergeable.
  - `disp` is a live assertion: `count_theorem_disputed` fires on contradiction checks.
  - The re-cut is exactly 12 lines, inserting `disp=0` only.
  - The `retried` omission is right.
  - **Class findings, to be filed via build A:**
    - the `expected()` re-cut log has accreted about 25 dated entries, a changelog in a doc comment (Q8);
    - `disp=0` is monotone in the wrong direction: freezes starve dispute recording.
  - A comment-and-docs-only commit (trimming the off-tree probe and moving the `retried` reason to `counts_line`) then merges on the green head. — (NURBS orchestrator)
- 2026-10-12 18:16Z — **PR 4887 merges (`c1514de8c7`).** The digest gains a `disp=` column, which reads 0 on all Sym lines.
  - The style pass is comments only: the off-tree probe is trimmed and the `retried` reason sits at `counts_line`.
  - **Filed:**
    - `the-thread-count-digests-re-cut-log-is-a-changelog` (P4, E);
    - `disp-zero-can-be-satisfied-by-freezes` (P4, M; a SYM/DECIDE seam when built).
  - Build A is dispatched on the changelog trim (`nurbs/digest-log-trim`). Tier: orchestrator's read. It lands before build C's rational-bound re-cut, to keep their conflict small. — (NURBS orchestrator)

- **2026-10-12 18:40Z — PR 4892 (rational speed bound, part 1) FULL reviewer dispatched.**
  - Session: session_01L9BiRwMrCxHYyp24sk3b5t, at `d7484a0edcd8d7358a44fd88270c05c1e649dc0e`. The full SHA came from `git rev-parse`, and the session is confirmed connected.
  - Tier: single FULL.
  - Claims to falsify:
    - door soundness at f64 and Interval, swept against a sampled sup;
    - the window variant;
    - the NaN poison and the Sym memo rule;
    - the routed sites (shell, offset_derive, pcurve_cache) keep their guarantees;
    - the tests go red under three mutants;
    - the digest is unchanged;
    - deletion completeness. — (NURBS orchestrator)

- **2026-10-12 19:20Z — PR 4892 FULL review: APPROVE-WITH-FIXES, no MAJOR.**
  - **The bound holds.** The reviewer ran its own sweep:
    - 1500 rational curves, degree 1–6, weight spreads up to 10^±3, against a certified Interval floor of ‖C′‖;
    - 5432 windows;
    - all were bounded, with a worst ratio of 0.9999999999999983.
    - The suites gave 5652/5652, and the 1e-9 digest was green.
  - **Fix pass sent to build C:**
    - delete `speed_upper_bound_on`: it has no caller and a std min/max that drops NaN;
    - delete the unreachable weight-poison arm (construction already refuses);
    - pin a carrier whose max sits at j ≠ i, since 80√5 survives the drop-Δw and pairs-only mutants;
    - add a locality row, since the sound-but-global mutant survived everything because "≤ recipe A" is monotone the wrong way;
    - add a rational-carrier row in offset_derive for the refusals turned into answers;
    - un-`pub` the unpaired `rational_span_speed_sup`.
  - **Class:** the issue's four-site census, a hand-written list, missed `mesh` chords `rational_carrier_m_bound` (recipe A on C′), plus `patch_bound` `rational_cells` and `offset_fit` `m_tilde_sup`. Part 2 takes them or files a row for each, with a hit list in its body.
  - **Brief correction (mine):** `weight_ratio_factor` is not inf-only until part 2 (pcurve_cache `nurbs_stretch_bounds` is a surface sup). — (NURBS orchestrator)

- **2026-10-12 20:08Z — 4892 fix pass read (head `48801e68342514cb2ee8e16f307c0366b3f30589`).** All 7 items landed:
  - `speed_upper_bound_on` deleted;
  - weight arm deleted;
  - pin at j outside the pair (14 exactly; the mutants read 2 and 8);
  - locality row;
  - rational offset_derive rows reaching `level_row` and `plane_spline_root`;
  - `rational_span_speed_sup` private;
  - one `session_memo` helper.
  - "Lift once" is declined (perf only, needs a caller-side cache).
  - `offset_fit` `m_tilde_sup` is accepted as out of class: a polynomial product, no quotient.
  - **Build C corrected the reviewer:** the weight arm was not unreachable. Knot insertion can round a subnormal weight to 0, and `from_validated_parts` admits it. The bound reads non-finite there (pinned), so deleting the arm stays sound. The admission itself is a latent invariant breach. Build C files `knot-insertion-can-round-a-weight-to-zero` (P3) in this PR, and I post a seam note on KNOT (co-owner of `algebra.rs`). — (NURBS orchestrator)

- **2026-10-12 20:40Z — PR 4904 (rational surface bound, part 2, stacked on 4892) up; single FULL reviewer dispatched.**
  - The reviewer is session_01TTzHgrMCC5fMeXpHU6eyTA at `b56fe691f1b831b13034a76f4ccbae1f903e39d0`, confirmed connected.
  - Tier FULL, because the surface twin of a soundness bound carries its own index arithmetic.
  - Claims to falsify: surface soundness per direction; the routed sites (recognize, the pcurve sups, enclose bit-identical); the six-cell `frozen=` digest re-cut with disp=0; red-under-mutation including locality; the dispositions of patch_bound/chords (filed) and offset_fit (out of class).
  - **4892 harness red explained:** `lo + (hi−lo)·n/n` rounded one ulp past the domain. The bound was not unsound there. The clamp is in 0e26675ae0.
  - **4892 merge waits on one push:** main merged (contacthold 4809 and 4820 also touch `topo/src/shell.rs`) plus the zero-weight row. — (NURBS orchestrator)

- **2026-10-12 21:00Z — PR 4904 FULL review: APPROVE-WITH-FIXES.**
  - **What holds:**
    - the surface statement is sound per direction (hand-checked, plus a 300-surface sweep, 0 failures);
    - enclose is bit-identical (200k Interval cases, to_bits);
    - the recognize None arms were dead;
    - the digest re-cut reproduces at 1e-9 with disp=0.
  - **The finding that matters:** the "j over the pair's own points only" mutant is UNSOUND (the true sup is up to 136× above it, on a coincident pair with a large weight step), and NO row catches it: not fuzz, not the named carriers, not recognize. This also refutes part 1's "no cubic Bézier beats pairs-only" search. Part 1's j-outside-the-pair pin catches it on curves by value only.
  - **Class, logged as a rule I will brief from now on:** for a soundness bound, random fuzz does not witness each clause. Every clause whose removal is unsound owes a named witness row that goes red under the mutant that drops it, and every locality clause owes a row that goes red when the bound degrades.
  - **Fix pass sent to build C:**
    - the witness row (surface, plus a curve twin);
    - a locality row;
    - the stale "sup multiplies by the weight ratio" prose (a class: sweep `ratio`/`Floater`);
    - `offset_meters` `patch_regularity` and mesh `nurbs_cert` `cell_readings` read patch_bound's recipe-A first-order sup → named in row (b) or routed;
    - enclose `about` restates `RationalPairTerm::at` (a fresh copy of the duplication the PR closes);
    - one per-pair builder;
    - the tautological `the_two_doors_report_one_sup`;
    - widen the fuzz;
    - correct "fewer frozen ⇒ tighter".
  - **Handed to build B's P2 floor row:** the inf side's `skipping_surface_derivative_net` with its own `denom == 0` skip. — (NURBS orchestrator)

- **2026-10-12 21:50Z — PR 4892 merges** (`47226e9fb248e5c78a2a1e7cbe3273d39b248ac4` → main `9c414f94e8`).
  - Its gate ok was green on a base containing contacthold 4809 and 4820 (which touch `shell.rs`).
  - Main took 4831 and germ/conic after that run. I checked: no shared files, `merge-tree` clean, and none of main's new code calls an API 4892 removed. I merged rather than re-run.
  - Seam notes are posted to SHELF/SHELL, PCERT/PCTAIL, TCOST/TINT and KNOT.
  - Build C merges main into 4904 before its fix pass. — (NURBS orchestrator)

- **2026-10-12 22:25Z — PR 4904 fix pass pushed** (head `064fcbb6fce841007133fdf468340051449110ee`; base is now main).
  - The pair-only counterexample is pinned on the surface and on the curve. `check` asserts bound ≥ sampled floor, so it is red under the j-over-pair mutant.
  - Part 1's search had drawn weights from {0.25…8} only. That was the blind spot.
  - Also landed:
    - a locality row;
    - the stale ratio prose (m9_2 re-measured);
    - row (b) names `patch_regularity` and `cell_readings`;
    - `RationalPairTerm::about` as the one spelling, with private fields;
    - one `pair_scale`;
    - the tautological test deleted;
    - the fuzz widened (degree 1–5, 10^±3, `expect` on KnotVector);
    - the body corrected on `frozen`.
  - Tier for the fix pass: orchestrator's read. Every item answers a named finding, and the refactors are covered by the rows that pinned the review's mutants.
  - It merges on green. — (NURBS orchestrator)

- **2026-10-12 23:10Z — PR 4904 merges** (`064fcbb6fce841007133fdf468340051449110ee` → main `51d913c3ca`).
  - gate ok was green on a base containing main with 4892. Main's later drift shares no file with it, and no new main code reads a symbol it changed.
  - The digest's six `frozen=` cells moved with it, and no other digest-touching merge is in between.
  - The rational sup is now one statement for curves and surfaces. The design row `rational-speed-bounds-have-two-recipes-across-four-sites` is closed in the PR.
  - Seam notes are posted to PCERT/PCTAIL, SSIARITH/SSIEDGE/SSIMARCH, EXCH and KNOT.
  - Next: build B on the P2 inf floor. Build C is archived: its context was at about 700k, and its unit is done. — (NURBS orchestrator)

- **2026-10-12 23:10Z — friction (message delivery).** Build C reports that six of my messages queued between 18:19Z and 21:49Z reached it only at about 23:09Z, up to five hours late.
  - Its work kept pace only because I re-sent each ask as a `now` interrupt.
  - The acknowledge-and-interrupt rule is what made the lane workable. A `next`-priority send to a lane polling CI in a long tool loop can sit for hours.
  - From now on, the default for any ask that gates a merge is `now`. — (NURBS orchestrator)

- **2026-10-12 23:15Z — build A dispatched on `knot-insertion-can-round-a-weight-to-zero`** (`nurbs/insert-weight-collapse`).
  - Tier: single STYLE. It is a guard with one shared check, and the witness rows carry its correctness.
  - Scope:
    - every weight writer (insert, refine, elevate, surface twins) refuses with `WeightCollapse` through one shared check;
    - the existing pin becomes the refusal;
    - the hit list of weight-writing plans goes in the body.
  - It does not collide with build B's `pcurve_cache` work. The KNOT seam is announced in the PR. — (NURBS orchestrator)

- **2026-10-12 23:56Z — PR 4923 (zero-weight writer guard, build A) green-ready.**
  - One shared admission, `written_weight`, guards insertion, refinement, elevation, removal and the surface twins.
  - Plan, curve and surface rows each go red with a guard removed.
  - The old pin is deleted; it read a curve that can no longer be built.
  - Single STYLE reviewer session_01RSRJMupUMGh3L6aJcd1hrJ is dispatched at `c147c29b52181564f85f03a7f4a14fad4c3124f1`, confirmed connected. — (NURBS orchestrator)

- **2026-10-13 00:01Z — PR 4923 STYLE review: mergeable.**
  - Both guard-removal mutants turn all three rows red, and the reviewer's own weight-arithmetic sweep found every `f64` writer guarded.
  - **Fix pass sent to build A:**
    - the `from_validated_parts` premise doc is false for the plan path (the code had drifted from the invariant, and is now repaired in code but not in the sentence);
    - removal's reinsertion is a new, unlisted route to `WeightCollapse`, and its index means a reinsertion slot;
    - the doors' `# Errors` omit the new refusal (a class: `insert_knot`, `refine_knots`, `split_at`, surface twins);
    - `written_weight` is a fourth spelling of positive-and-finite, so `check_weights`, compose, tensor and hull get one predicate;
    - the surface row pins the index;
    - optionally, a subnormal-positive least-weight row for `speed_upper_bound`.
  - The T-channel note and the unreachable `?` stay as they are. — (NURBS orchestrator)

- **2026-10-13 00:33Z — PR 4928 (proven rational stretch and area floors, build B) up.**
  - The floors replace `weight_ratio_factor`. The area floor reads both cross factors at one active point: `T_i(S)×U_j(S)` is affine in S. That beats the M̃ product spline the brief suggested.
  - 14 mutants red. The digest is unchanged at 3 ε.
  - **Reach loss:** nets with an in-cell weight spread past about 1.6× now refuse. `r2_probe_cert8`'s rational charts were moved to ≤1.6×, and a frontier row pins the steep refusal. The reviewer is asked whether the probe edit is legitimate.
  - Filed:
    - `rational-inf-floor-reach-refines-before-reading` (P2, design);
    - `rational-inf-speed-has-two-recipes-curve-and-surface` (P3).
  - Single FULL reviewer session_014ErZjhFLejDn28dwMteLFc is dispatched at `699a591e426706c03d7c7d5c6e8f27424c4c9b70`. — (NURBS orchestrator)

- **2026-10-13 00:47Z — PR 4923 merges** (`f7e9c013ddf428f46f83a97a18ff6c46089695fb` → main `50d2297d9a`).
  - gate ok was green. Main's drift overlaps only `pcurve_cache.rs`: PCERT's harmonic sups, disjoint hunks, `merge-tree` clean.
  - The fix pass folded every spelling of positive-and-finite into `weight_in_regime` / `check_weight_values`, across `compose`, `tensor`, `hull`, `net.rs`, mesh chords, `patch_bound` and `pcurve_cache`.
  - Seam notes posted to KNOT, CHORD, TESS, ENCL, PCERT and PCTAIL.
  - Build A is starting `rational-second-derivative-bounds-have-two-recipes` on `nurbs/second-order-pair-term`. It derives first and stops if no clean second-order statement exists. Tier FULL.
  - Build B is warned of the `weight_ratio_factor` conflict on 4928. — (NURBS orchestrator)

- **2026-10-13 01:01Z — PR 4928 FULL review: MERGEABLE.**
  - Both floor proofs check by hand. The Interval per-cell sweep found 0 violations in 65,622 checks, across weight spreads up to 1e3.
  - **The witness rule caught one more gap.** The "every active column" clause has no FLOOR row. Its mutant is unsound (4,022 violations) and goes red only through the sup's rows, via the shared builder. A clause guarded only by a sibling's tests is unguarded once they diverge.
  - **The reach figure is wrong.** The frontier is about 1.7×, and the AREA floor binds. The frontier row pins 3.7×+ and lumps refusal reasons together.
  - **The `r2_probe_cert8` weight edit is judged legitimate.** It keeps the anti-vacuity class and masks no failure. Above-frontier soundness stays covered by the random rows.
  - **Fix pass sent to build B:**
    - a floor row for every active column;
    - frontier rows at about 1.6×/1.8× with the refusal reason asserted, and the numbers corrected;
    - option one named as the alternative in the reach row;
    - the stale `work/knot` citation fixed;
    - `floor_weights_admissible` deleted, now that #4923 guards producers;
    - `net_direction` (a fourth derivative-net walk) routed through `derivative_net`, or filed;
    - doc fixes.
  - The f64 one-ulp overshoot of the floors predates this PR. It is documented, not fixed. — (NURBS orchestrator)

- **2026-10-13 02:19Z — friction: CI test-job wall time.**
  - The PR test jobs on this program's spline PRs (4892, 4904, 4928) ran for 30–45 min each. The bar is 15 min.
  - These diffs seed geom-core, geom, geom-brep, topo, sweep and step-import, so the "slow set of the touched crates" step pulls in nearly the whole slow suite.
  - I have not measured which step dominates. If the next spline PR repeats it, I'll pull the job timings and file a row for CIW. — (NURBS orchestrator)

- **2026-10-13 02:45Z — PR 4928 merges** (`202610195d4f41247c8d6d324df37bfd3d2f83d9` → main `03a0fec33c`).
  - gate ok was green.
  - **The back-to-back digest rule applied.** FLUX #4839 re-cut the thread-count digest on main at 01:26Z, after 4928's base.
    - I built main + 4928 in a fresh worktree; the merge commit was confirmed with 2 parents, and it compiled fresh.
    - The digest passed 2/2 at 1e-9, 1e-6 and 1e-12.
  - The `pcurve_cache` overlap with PCERT's spiric work merged clean.
  - The rational infs now rest on a proof, and the unproven ratio² bracket is gone from the tree.
  - Seam notes posted to PCERT/PCTAIL, KNOT, CHART and TCOST/TINT. — (NURBS orchestrator)

- **2026-10-13 02:38Z — design fork dispatched: the rational lower-bound statement and its reach.**
  - It covers two rows:
    - `rational-inf-floor-reach-refines-before-reading` (P2, design);
    - `rational-inf-speed-has-two-recipes-curve-and-surface` (P3, design).
  - Both are ONE question: what single rational inf statement serves curves and surfaces, and how reach is recovered.
  - One Opus and one Fable designer received the same problem statement and no candidate solutions. The blinding record is on `analysis/design-fork/nurbs-rational-inf-floor`.
  - A log row is owed only if the fork goes to Ev.
- **Build B dispatched on `disp-zero-can-be-satisfied-by-freezes`** (`nurbs/dispute-coverage`, tier single STYLE). It is a guard that goes red when dispute coverage degrades, with a SYM/DECIDE seam on `SymCounts`. — (NURBS orchestrator)

- **2026-10-13 02:57Z — build A paused on `nurbs/second-order-pair-term`** (WIP at `f22f608d`, no PR).
  - **Why paused:** the open inf-floor design fork may replace the pair-term statement on both sides, including second order. Building a second-order pair-term would then be built only to be replaced.
  - **Where it stands:**
    - The proven second-order statement is `W·C″ = Σ N·k²(b_{j+1}−b_j)(C) − 2W′C′`, with the surface twins.
    - 12 of 12 mutants go red.
    - patch_bound and chords both read it. The rational sups and m_sup are tighter, so the offset goldens fell.
    - The digest is unchanged.
  - It resumes or redirects once the fork is ruled. — (NURBS orchestrator)

- **2026-10-13 03:00Z — RULING: one Bernstein quotient bound for every rational derivative bound** (fork `nurbs-rational-inf-floor`; designers A and B convergent; no Ev needed).
  - **Why no Ev:** no DESIGN or README text governs this, and the sup half revises MY 2026-10-12 17:45Z ruling. Both designers confirmed that.
  - **The statement.** It lives in `geom_core::spline::derivative`.
    - On an exact dyadic Bézier piece of a span or cell, each quantity is a numerator over a power of the weight: `C′ = N/W²`, `S_u = N_u/W²`, `S_u×S_v = M̃/W³`. Second order has the same form.
    - Raise the numerator and the weight power to one degree.
    - Then `inf (·)·d ≥ min_k (a_k·d)/b_k` and `sup ‖·‖ ≤ max_k ‖a_k‖/b_k`.
    - It has no weight range, no sign split and no hull. It needs only positive weights.
  - **Pieces:**
    - curves keep their piece count;
    - surfaces get a D9 per-direction constant, measured on the chart rows, at least 2;
    - pieces come from exact restriction weights, never refined knots.
  - **Ruled details:**
    1. **Per-piece direction: yes.** The doors return norm floors (`inf ‖S_u‖`, `inf ‖S_u×S_v‖`) like the curve. `net_direction` and `floor_along` retire.
       - Why: A measured a rational half-cylinder reading 0 under one global direction and 2.0 with per-piece directions. B leaned the same way. The doors keep their names.
    2. **`RationalPairTerm`:** SSI `enclose` moves to the quotient form if its rigid-map carriage transfers (a rotation plus a weight-scaled translation of the numerator coefficients). Otherwise it stays, scoped to enclose only, with a row filed. No bound door may read it.
    3. **Second order** follows the same statement. Build A's pair-term WIP is not merged; it is redirected onto the quotient form, and its rows and mutants carry over.
    4. **`offset_meters`** is the third inf recipe. It goes with the second-order unit, where its cells also live.
    5. **Degree ceiling:** the statement states and pins its own f64 ceiling, from exact binomials. Interval is outward.
  - **Rejected:** knot refinement then pair-term (four times the pieces for the reach, Interval-only widening); the M̃ numerator over `w_max³` (loses a factor of `w_min/w_max` per weight power); keeping two recipes.
  - **Build plan:**
    1. **Unit 1 (build A, FULL):** the statement plus the first-order readers. That covers the curve sup and inf (the piece assembly's divisor and both arms in one call), surface `stretch_sup` / `stretch_floor` / `area_floor`, and `chart_stretch_inf` with per-piece directions. It also handles the enclose disposition and memoising `chart_stretch_inf`.
    2. **Unit 2 (build A):** second order plus `offset_meters`, on the quotient form.
  - The witness rule binds both. — (NURBS orchestrator)
- **2026-10-13 03:00Z — dispatch.**
  - **Unit 1** goes to a fresh lane, build D (session_01GNJrUW7PtMDWKbGraZXaWP), on `nurbs/quotient-bound` from main `fc37c7b698dcb15bc3b26bdc683b93061bb56312`. Tier: FULL.
    - Fresh, because build A sits at about 550k of context, and unit 1 is the program's widest change since the speed bound.
  - **Build A** stands by for unit 2 (second order and `offset_meters`) after unit 1 merges.
  - **Designers A and B** archived. — (NURBS orchestrator)

- **2026-10-13 04:15Z — PR 4944 (dispute coverage, build B) up.**
  - What it adds:
    - `SymCounts` splits contradiction checks into expanded and frozen;
    - a digest `chk=` column;
    - a guard with a 3/4 floor on the expanded share;
    - a starved-budget witness: `disp` stays 0 while the guard trips.
  - It filed `work/sym/far-placement-zero-dispute-cells-can-be-satisfied-by-freezes` on SYM's slate.
  - Single STYLE reviewer: session_01UBoURR7eLxiVDFVNN41uHK at `56f39dc332f2f1c0287f392f06a317e346893d6f`.
  - **Note for the merge order:** 4944 re-cuts the digest (the new column), and build D's unit 1 will likely move it too. The back-to-back digest rule applies to whichever merges second. — (NURBS orchestrator)

- **2026-10-13 04:39Z — PR 4944 STYLE review: style fine; one finding blocks the merge.**
  - **The blocking finding:** the classifier counts nodes minted before the session as expanded, and the digest builds its bodies before the session. Counting them as frozen, the golden reads 14/110 rather than 89/110, so the 3/4 floor may rest on a mis-measure.
  - Build B investigates whether a pre-session freeze blinds a check, reports, and then fixes. The candidate fixes are: classify those nodes by kind, or build the digest's bodies inside the session.
  - **Also sent:**
    - one indeterminate walker, since `reads_a_freeze` copies `keeps_an_opaque_atom` and they have drifted;
    - poisoned forms count as not-expanded;
    - the floor constant cites its measurement and handles small receipts;
    - the SYM row's cell count is fixed and the blind spot noted there.
  - **Class:** a guard whose measured baseline depends on a classifier's blind spot guards the blind spot. Its baseline needs the same adversarial check as the bound it guards. — (NURBS orchestrator)

- **2026-10-13 04:54Z — 4944 finding 1 measured and ruled.**
  - The pre-session nodes are literal and lofting arithmetic, not free variables. Freezing them DOES blind the checks: built inside the session, 364 and 306 more decisions become theorems.
  - True coverage is 103/110 and 104/116, and disp is still 0. The old 89/110 was a mis-measure.
  - **Fix (c), approved.** The session names the free indeterminates it minted, and every other indeterminate charges `checks_frozen`, which is conservative for every caller. The digest builds its bodies inside the session and receipts the door alone.
  - **Added:**
    - a witness row that fails under the old classifier, using out-of-session bodies, which must read frozen;
    - an explicit statement in the body that the digest receipt changes meaning. — (NURBS orchestrator)

- **2026-10-13 05:50Z — PR 4954 up (one Bernstein quotient bound, unit 1, build D).** About +2.1k/−2.2k lines over 27 files.
  - The statement is in `geom_core::spline::derivative`, with the pieces in `spline/pieces.rs`. SSI enclose is rewritten onto dyadic pieces. Both inf rows close. The digest is re-cut (the sym frozen column).
  - **Reviewer:** single FULL, session_01DvtA5zTszfNoi2q2nXXwWS at `8f4f058b9b03e587d03605bab1f840f7cc459921`.
  - It is told to check whether "the flat-wall row stands down at 1e-9" weakens a test. Skipping or weakening a test to get green is never allowed.
  - **Build B** is still validating its 4944 fix. PRs 4944 and 4954 both re-cut the digest: the second to merge gets my merged-tree digest run. — (NURBS orchestrator)

- **2026-10-13 06:15Z — the 4944 fix pass read** (head `5e4cccc2a7d19bd6fed1ddfb33d99fe40bdda2b5`).
  - All items have landed:
    - `Session::free` names the minted free indeterminates; everything else (and poison) charges frozen;
    - `any_indet` is the one walker shared with `keeps_an_opaque_atom`, with `push_atoms` argued separate;
    - the digest builds its bodies in-session and receipts the door alone. arc gives chk 103/110, strip 104/116, disp=0, with serial = 4-thread;
    - the floor is 4/5 at ≥20 checks, with its measurement cited;
    - the witness is an out-of-session build: 14/110, disp=0, trips, and is red under the old classifier;
    - 11 of 11 mutants red;
    - the SYM row is corrected;
    - #4953's census fix is ported.
  - Tier for the fix pass: the orchestrator's read.
  - **Base red:** `geom-core` `bounds_census::every_sole_bracket_bound_door_is_in_the_roster` fails on main `ffa0264c4e`; I reproduced it locally. It is BAND's, already filed as the `reach-displaces` / `worse` sole-bracket-door rows. 4944's test job will show it, and I merge with it as the only red. — (NURBS orchestrator)

- **2026-10-13 06:25Z — PR 4954 FULL review: MERGEABLE after MINORs, no MAJOR.**
  - **Evidence:** soundness checked by hand; an Interval sweep of 600 curves and 150 surfaces with 0 violations; every unsound mutant red in its own floor or sup row; reach holds to 1000×. The "flat-wall stands down" row turned out to be an honest premise change, not a disable, but it was inert at the default ε.
  - **Fix pass sent to build D:**
    - m1: the per-axis floors capped at degree 18 by the area pieces (a reach regression); decouple them;
    - m2: assert the specific refusal at 1e-9;
    - m3: name ENCL as a seam;
    - fix the f64 note (5e-12, not an ulp) and the stale prose;
    - binomial/product and blossom twins in `geom-core::spline`: fold them or file one row;
    - guard the measured shares;
    - delete the vestigial clause;
    - make `QuotientPiece::new` private.
  - **Design collision with ENCL #4891**, which homes the pair-term reduction 4954 retires. I commented on #4891 and posted on ENCL's log proposing that 4954 merges first and #4891 re-scopes. 4954's merge is held for ENCL's answer. — (NURBS orchestrator)
- 2026-10-13 09:05Z — **4944's red is FLUSH's 1e-12 P1, not 4944's.** The only red on run 38366610178 is `editor-core coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`, in the ε 1e-6/1e-12 step. The default-ε step passed, and so did `bounds_census` this run. I reproduced it on main a5229515 with `CAD_TOLERANCE_EPS=1e-12`: it panics at `coincidence_door.rs:44`. Build B had spent a round failing to reproduce it at the default ε.
  - **Class, a friction finding:** the lane named the step as "default-ε `ci`" from the log tail. A red's ε row has to be read from the job's step list (`get_workflow_job`), not inferred.
  - The merge got a 405: main moved, and `.config/nextest.toml` conflicts. Build B is merging main in. Because that file is test-selection config, the merge re-earns CI.
- 2026-10-13 09:05Z — **Friction: CI wall time.** On the PR `test` job:
  - the `build` step alone takes about 20 min (07:11→07:31, 08:04→08:23);
  - 4954's slow-set step took 55 min (07:58→08:53), against the 15-min whole-run bar, so a full `test` job ran 1 h 43 min.
  The slow set of geom-core is the likely cost on a diff that seeds geom-core. I have not filed a row yet; I'll file one once build D names what ran long.
- 2026-10-13 09:45Z — **4954 status, and the digest interaction with 4944.**
  - **4954's step-8 red** was BAND's `bounds_census` roster row. Main has carried its fix since PR 4959; build D merged main a5229515, giving head d7ae1e3f.
  - **Fix pass verified:** m1 (per-axis floors to degree 28, M9 red), m2 (the flat-wall row asserts `BoundaryTangent` on the v-low side at 8e-9), m3, `QuotientPiece::new` private, and the degree ceilings.
  - **Back-to-back digest on the merged tree:** I built local merge 94f5d3eb (d7ae1e3f + 4944's 0281ec6d) and resolved the conflicts in all three `eps-*.txt` sym-session blocks to 4944's lines exactly. `mass_props_are_thread_count_invariant` passes at 1e-9, 1e-6 and 1e-12, 4/4 each, with sweep compiled fresh.
  - With bodies built in-session, 4954's `frozen` growth (×4.9 / ×3.6) is absent, and 4944's 4/5 guard holds.
  - **Plan:** 4944 merges first. 4954 then merges main, takes main's digest lines, re-runs the digest, and corrects its PR body. It re-earns CI, because the digest goldens are test data.
- 2026-10-13 09:50Z — **PR 4944 merged** (dispute coverage) at `89f044e92e` on head 0281ec6d.
  - The only red was step 9's FLUSH 1e-12 `coincidence_door` ring P1, which reproduces on main a5229515. Build B's PR comment names it.
  - Seam notes are posted on sym, decide, tcost and tint.
  - Build D was told to bring main into 4954 and take main's digest lines.
- **2026-10-13 — unit 2 (build A): what it touches in `patch_bound.rs`, ahead of ENCL #4891's re-scope.** On `nurbs/quotient-second-order` from main `092b3022`. As of this note there is no ENCL reply on #4891 or `work/encl/log.md`.
  - **Touches:** `rational_cells` (its whole body), the rational arm's module-doc section, the rational halves of the `PatchCell` field docs, and `RATIONAL_CERT_SPLITS`' doc.
    - Each cell's boxes and sups, first and second order, come from `geom_core::spline::derivative`'s quotient pieces on the original cells' exact dyadic pieces: `S_u`, `S_v`, `S_u×S_v`, `S_uu`, `S_uv`, `S_vv`.
    - The arm's interval refinement and the recentred hulls it fed are deleted: `window_tilde_hull`, `tilde_entry`, and the rational use of `refine_split`.
    - `rational_split_points` goes once mesh chords stops refining.
  - **Does not touch:** the integral arm (`integral_cells*`, `DNets`, `comp_nets`); the unit-arm helpers #4891 reworks (`vectors_norm_sup`, `window_norm_sup`, `cross_norm_sup`, `centre`, `coordinate_hull`); `PatchCell`'s layout; `PatchBoundError`.
  - The rational arm stops calling the unit-arm helpers. #4891's re-scope keeps them for the integral arm.
  - Also touched, outside `patch_bound`: mesh chords' `rational_carrier_m_bound`, `offset_meters`' cell inf (the area-floor quotient), and `geom_core::spline::derivative`, which gains the second-order statement. — (NURBS, build A)
- 2026-10-13 11:30Z — **PR 4954 merged** (quotient bound, unit 1) at `092b30227f` on head d6bb50c4.
  - CI was fully green, with no FLUSH red on this head. The `test` job took 80 min, which adds to the CI friction already logged.
  - Before merging I commented on #4891 (comment 6144508945). ENCL did not answer between 06:21Z and 11:28Z, so the ruling stands and #4891 re-scopes.
  - Seam notes are posted on pcert, pctail, ssiarith, ssiedge, ssimarch, knot, encl, tcost, tint, shelf, shell and chart.
  - `rational-inf-floor-reach-refines-before-reading` and `rational-inf-speed-has-two-recipes-curve-and-surface` are closed by the PR.
  - Next: build A on unit 2.
- 2026-10-13 11:33Z — **Build A dispatched on unit 2 of the quotient ruling.** Session session_01WzFKtLWBVWNnnS3M5ezaMB, branch `nurbs/quotient-second-order` from 092b3022.
  - **Scope:**
    - `patch_bound` `rational_cells`: first- and second-order boxes, and `m_sup`;
    - `mesh` chords `rational_carrier_m_bound`;
    - `offset_meters` `cell_normal` / `patch_regularity`, whose inf reads the area-floor quotient;
    - `second_derivative.rs`, if it carries a rational recipe.
  - Closes `rational-second-derivative-bounds-have-two-recipes`. The pair-term WIP f22f608d stays unmerged; its rows carry over.
  - **ENCL seam:** before touching `patch_bound.rs`, the lane reads #4891 as it stands and logs what it will touch. If ENCL has countered, it stops.
  - Tier: FULL, because the second-order numerators and the witness-per-clause mutants carry real correctness risk.
- 2026-10-13 15:39Z — **PR 4984** (unit 2, head 5d2a7a4a) is up. The FULL reviewer is session_01J87p3xUa5Wf1XjMEjnKAzm. It has seven claims to falsify: the numerators over W³; the mutant rows; the `m_inf` floor; chords' `accel_upper_bound`; goldens with stated causes; the ENCL seam; and no second blossom or binomial home.
- 2026-10-13 15:39Z — **Build C dispatched on CI wall time** (session_01DyXAveAT58f6rcbLPEojAD, branch `nurbs/ci-wall-time`, on CIW ground), with Ev's go-ahead. Today's numbers: the `build` step takes about 20 min; 4954's slow-set step took 55 min; one `test` job ran 1 h 43 min.
  - **Order of work:** measure first, then fix within the per-PR selection standard (shard, cut the build path, cheaper tests).
  - **Limit:** dropping a seeded row to the nightly comes back as a proposal, because it contradicts implementer-discipline §2.
  - The lane updates or closes CIW's open row `a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar`.
- 2026-10-13 16:05Z — **Build C's first findings** (CI wall time).
  - **(1) Red main.** Sweep's tests don't compile on main after a 4918/4197 semantic conflict. I fixed it in PR 4992 (test-only) and posted seam notes on sym and contacthold.
  - **(2) PR 4954, our own merge, regressed hard.** Unit 1 took `shelling_the_curved_loft_refuses_at_a_walls_fit` from about 60 s to 1530–1926 s, `r1_lane0_e2e::the_f64_seam_answers_every_public_door` from about 17 s to 650–820 s, and the geom-brep slow rows to 470–1000 s each, roughly 30–50×. That, not CI shape, is what made 4954's slow-set step 55 min.
    - **Missed:** neither the FULL review nor I measured run time.
    - **Class:** a bound that swaps a cheap reading for a tighter, costlier one needs a wall-time row, or the slow rows' timings read before merge.
    - **Next:** build C is profiling the cause. The 4984 reviewer has claim 8 (unit 2's added cost). Who lands the fix is decided once the cause is known.
- 2026-10-13 16:33Z — **The cause of the 4954 regression** (build C, from gdb samples, 25/25 in one place).
  - **Cause:** since 4954, `ssi::enclose`'s `CellNet` builds the full Bernstein quotients (`N_u`, `N_v` over `W²` on 2×2 pieces, in interval arithmetic) eagerly on every construction, and nothing caches them. Before, `derivative_box` was an O(n) pair-term hull.
  - **Why it multiplies:** the four-variable walk's `rect_box` → `fold_cells` → `NurbsBoxes::cell` rebuilds the same cell's quotients 2–3× per residual or jacobian evaluation.
  - **Measured locally:** the shell test took 35.3 s before 4954, and was still running past 30 min after it.
  - **Fix:** dispatched to build C on `nurbs/enclose-quotient-memo` from 3ad2c1bf.
    - Memoise per `NurbsBoxes`, keyed by `(su, sv)`, and make `CellNet.quotients` lazy, with answers bit-identical.
    - Guarded by a counting row (one build per cell), with the removed memo as its named mutant.
    - It also sweeps 4954's other readers for the same build-per-call shape. Build A was told to check 4984 for it.
  - `enclose.rs` is ssiarith/ssiedge/ssimarch ground; seam notes go out when it lands.
- 2026-10-13 17:00Z — **PR 4992 merged** (red-main fix) at `b13167f9`; sweep's tests compile on main again.
  - **The enclose memo** (build C) is in progress, measured locally at `--test-threads 1`:

    | Build | Shell test | `r1_lane0` |
    |---|---|---|
    | Before 4954 | 35.3 s | 12.6 s |
    | 4954 | 1692 s | 608 s |
    | With the whole-cell memo | 284 s | 193 s |

  - The remaining 8–15× is in the cut nets, which build both axes per call. Next is a cut-net memo, or an additive per-axis entry point in `derivative.rs` (where it overlaps 4984). The target is within about 2× of before.
- 2026-10-13 17:05Z — **The PR 4984 FULL review is in.**
  - **What held:** no unsound bound. The numerators match `deriv2`/`ders` to 1.2e-14, `m_inf` was never above the truth over 22k cells, and `accel_upper_bound` bounds at p up to 18.
  - **MAJOR:** an undisclosed refusal at degree ≥ 19 on both arms. The integral arm reads the rational area form at 3p and refuses with the kernel-defect text, where main certifies the face.
  - **Broken seam:** `coordinate_hull` is deleted (#4891 adds callers), the integral arm is touched, and `PatchCell` gained `m_inf`. The touch list posted to ENCL was wrong; I corrected it on ENCL's log.
  - **MINORs:** the S1 schedule witness, a dead variant and a dead order-2 arm, `RefinementFailed` meaning two things, Q1 duplications (`accel_pieces`/`speed_pieces`, the cross term, `integral_area_floor`) and Q4 refinement-era prose.
  - **Fix pass dispatched to build A.** It moves the integral arm to the A_u×A_v numerator, adds typed ceiling refusals, and runs the perf check the review skipped.
  - **Class:** a touch list posted before the work is a prediction. Re-check it against the diff before review and correct it where it was posted.
- 2026-10-13 17:15Z — **PR 5001** (enclose whole-cell memo, build C, head 019d3d3c).
  - **Tier lowered to the orchestrator's read:** a mechanical memo of a pure function with no reading change, guarded by a counting row whose mutant goes red. I read the diff: correct, and empty cells are cached as `None`. It merges on green CI.
  - **Remaining 8–15×** sits in the cut nets: 80k builds over 59k distinct keys, so caching won't help. Filed by build C as `a-cut-cell-nets-quotients-cost-the-walk-eight-times-its-pre-4954-time`.
    - Levers 1 (an axis selector on `cell_quotients`) and 2 (a binomial table in `product`, bit-identical) are approved, to start AFTER 4984 lands, because 4984's fix pass is refactoring `derivative.rs`.
    - Lever 3 (cheap hull first) changes readings and is held as a possible design fork.
- 2026-10-13 17:55Z — **Friction: CI queueing.** PR 5001's run 38436425383 was created about 17:15Z, but lint and test started only at 17:34Z, about 19 min queued. Earlier today every job started within 2 s. That points to runner-pool pressure, under the free plan's 20-concurrent-job cap per `work/ciw/latency-cut.md`. Passed to build C's CI wall-time measurements.
- 2026-10-13 18:00Z — **Build C's CI-shape measurements, and my decisions.**
  - **Measurements:**
    - Cold build floor: 18m19s locally, 16–20 min hosted. Throughput-bound, with parallelism 3.5 on 4 cores; the tail runs topo → editor-core → editor-core `all`. topo's unit tests force a second full topo compile (531 cpu-s).
    - Tour: release build 15m55s, 9.5 min of it the demo-tour crate's own codegen; tests 8 min.
    - Queueing: up to 19 min, on #5001's run.
  - **Decided:**
    - (3) the workspace-crate cache and (5) demo-tour's own crate at opt-1 go together in one PR. The kernel stays at opt-3, and the tour's outputs must stay unchanged.
    - (4) topo's unit-test recompile is filed on TOPO.
    - (2) splitting `test` is deferred until after (3)+(5), because of the extra queue slot.
    - **[ev] proposal:** the 15-min bar is unreachable for deep-kernel PRs on 4-vCPU hosted runners. Build C writes it after (3)+(5), with measured options: persistent or self-hosted build dir, larger runners, or a revised bar. It edits `memories/orchestration-model.md`.
- 2026-10-13 18:16Z — **PR 5001 merged** (enclose whole-cell memo) at `eec79d69`. The hosted `test` job took 21m22s (build 13m20s, slow set 2m59s), against 4954's 55-min slow set. Seam notes are posted on ssiarith, ssiedge and ssimarch.
- 2026-10-13 — **Seam note from ENCL: #4891 is closed unmerged, and your quotient ruling stands.** Sorry we missed the proposal on #4891 and this log; ENCL has no counter to the split.
  - #4891 filed these on its branch, and they never reached main. They are handed to you to take or drop:
    - `topo/src/shell.rs` `spline_speed` reads a relaxed quotient-rule speed;
    - step-import `recognize` `derivative_sup` does the same;
    - mesh `chords.rs`'s `s1` relaxation;
    - `props/quad.rs`'s rational cross is a candidate consumer of the one form.
  - Pieces of #4891 unit 2 might want (branch `encl/one-chart-speed` @ adb4df370e):
    - `the_meter_sups_bound_every_sampled_magnitude`'s `SPEED_CEILING` tightened to the box (`1 + 1e-12`);
    - the unit-arm `screened_max`/`vectors_sup` f64 screen, about 5% cheaper on the integral arm, with its fuzz;
    - `a_part_carried_below_the_normal_range_screens_soundly`.
  - The ENCL row `the-rational-chart-speed-is-read-twice-in-patch-bound-and-enclose` is closed as superseded. — (ENCL orchestrator)
- 2026-10-13 19:20Z — **ENCL closed #4891 unmerged**, superseded by the quotient ruling, with no counter. Its handover is filed as `other-rational-speed-readings-read-relaxed-quotient-recipes`: shell `spline_speed`, step-import `derivative_sup`, the chords `s1`, `props/quad`'s cross, plus three pieces from `encl/one-chart-speed`. It is picked up after 4984. **Build A's 4984 fix pass** is pushed at 1c58397c and CI is running; the delta check went to the 4984 reviewer.
- 2026-10-13 19:40Z — **PR 4984 delta check: MERGEABLE** (on 1c58397c).
  - Closed: the degree-19 MAJOR (integral to p=28, typed rational/chords refusals), the seam, S1, the pointwise rows, the dead code, `RefinementFailed` and Q1/Q2.
  - Perf against base c16eaf57: the nurbs_cert row is 2.3×, `budget_faces` is 1.0×, and the m_inf soundness sweep is unchanged.
  - **Last commit, docs and tests, to come from build A:**
    - the perf row: the face bound is still 11× main and the random sweep 14×, from per-piece restriction;
    - the stale `cert10_r1_probes` row and prose;
    - `QuotientRefused`'s note naming the face kind correctly;
    - `cut_both`'s coupling made explicit.
  - Merge after CI on that head.
- 2026-10-13 19:45Z — **Build C's final CI wall-time report.**
  - A deep, geom-brep-seeded PR's test job went from 37–103 min to **21 min** (#5001). The 55-min run was 4954's own regression, not CI shape.
  - The build floor is 13–20 min hosted, throughput-bound.
  - Declined: the workspace-crate cache. Deferred: the test-job split.
  - **#5010:** the opt-1 tour change saved only about 45 s hosted, under the 3-min bar, so I asked for it to be dropped. The CIW row (with `needs_ev`) and the topo row stay.
  - **#5011 `[ev]`:** the CI bar question.
    - Recommended: 15 min, or 25 min for PRs that seed geom-core, geom, geom-brep or topo, with queue time excluded.
    - Alternatives: larger runners (B) or a self-hosted incremental build (C).
    - Before Ev reads it, I asked build C to measure (C)'s incremental rebuild and update the body in place.
- 2026-10-13 21:52Z — **PR 4984 merged** (unit 2, second-order quotient) at `2639b944`, on head 3d23bdcc. CI was green; the FULL review and its delta check were mergeable, and the delta MINORs are folded in.
  - Seam notes are posted on chord, tess, encl, offset, shell, guard, knot, tcost and tint.
  - `rational-second-derivative-bounds-have-two-recipes` is closed by the PR.
  - **Next:**
    - build C on cut-net levers 1+2;
    - `other-rational-speed-readings-read-relaxed-quotient-recipes` (ENCL's handover) goes to build A next;
    - the perf row `second-order-quotient-pieces-cost-the-face-bound-eleven-times-main` is shared with build C's levers.
- 2026-10-13 21:55Z — **Dispatched after 4984.**
  - **Build C:** cut-net levers 1+2 on `nurbs/cut-net-quotient-cost`, from 2639b944. It covers both perf rows (cut nets, and the face bound's per-piece restriction), bit-identical, with counting guards. The target is within about 2× of before 4954.
  - **Build A:** `other-rational-speed-readings-read-relaxed-quotient-recipes` on `nurbs/remaining-rational-readings`, with the tier decided at PR time.
- 2026-10-13 22:40Z — **The quotient bound's cost cannot be closed without changing readings.**
  - Build C's levers 1+2 are bit-identical and guarded. They take the shell test from 286 to 216 s and r1 from 183 to 113 s, still 6–9× the pre-4954 times. The face bound barely moves (0.96 → 0.93 s).
  - The remainder is interval arithmetic in the O(n²) quotient products, against the old O(n) hull.
  - (a) L1+L2 lands as its own PR. (b) The interval scale fast path, at most 25%, is filed, not done.
  - (c) **Design fork dispatched:** "what the code should be given the quotient bound's per-call cost". Two designers, concurrent, same problem statement, no candidate options. The blinding record is on `analysis/design-fork/nurbs-quotient-cost`. A designer convergence needs no Ev; a split goes to an `[ev]` PR.
- 2026-10-13 22:58Z — **Red main from 4984** (build C found it during its 3-ε sweep).
  - **Failure:** `sweep saddle_wall_offset_fit::a_saddle_wall_certifies_on_a_grid_longer_in_one_direction_than_a_square_cap` certifies at 3.850263672224892e-10 against the pinned 3.8534e-10 ± 1e-13, at all three ε.
  - **Why the gate missed it:** it's a sweep slow row, and 4984's gate didn't seed sweep.
  - **Assigned to build A, ahead of its handover unit:** confirm the cause, check the new value is right on its own terms (re-baseline with its cause, or fix a bug), and sweep sibling pins on the paths 4984 moved in crates its gate didn't seed.
  - **Class:** a change to a shared bound (`patch_bound`, offset) moves pinned values in crates the PR gate doesn't seed. The reviewers ran the seeded suites only.
  - **Next time:** run the slow sets of the bound's downstream consumers (sweep, topo) locally before merging, or seed them.
- 2026-10-13 22:58Z — **Build C's L1+L2** passes 427 of 428 rows clean across enclose, patch_bound, nurbs_cert, offset, derivative, stretch and the digest at three ε. The one red is the main red above. The PR is opening.
- 2026-10-13 23:00Z — **Design fork CONVERGED: the cost of the quotient bound.** It ran two rounds and crossed once in round 1, as the protocol anticipates. The record is on `analysis/design-fork/nurbs-quotient-cost`. No Ev and no fork-log row: the statement is unchanged and no ratified text moves.
  - **The finding beneath both:** `NurbsBoxes` is rebuilt per rung (`certify`) and per `boundary` call, so the 5001 memo lives one probe. Windows nest and never repeat, so no window-keyed cache and no tree.
  - **The final state:**
    - an operand-lifetime `NurbsBoxes`;
    - one raised quotient per cell, with `restrict(a, b)` by de Casteljau and `pieces` by halving;
    - the piece tables and the exactness ceiling retired;
    - the cut retired, becoming restriction;
    - the meet turned into a debug containment check.
  - **Dispatched to build C as three PRs:**
    - lifetime (my read);
    - products-once with halving (FULL);
    - cut becomes restriction (FULL).
    - It measures after each step, and the L1+L2 PR is closed as superseded.
  - **Process note:** the 22:40Z brief's "bit-identical levers are exhausted" was one level off. Both designers found the cause in ownership and order of operations, not in arithmetic.
- 2026-10-14 00:08Z — **A second red on main from 4984:** the sweep thread-count digest's `tilted_cut_upper` pads read slightly tighter than the table, at all three ε. It is the same class as the saddle-wall pin (sweep not seeded) and is routed to build A's fix PR; the table is re-cut with its cause.
- 2026-10-14 00:08Z — **Quotient-cost step 1 is PR 5047** (`NurbsBoxes` lifetime, head b23d0f02).
  - `ChartedNurbs` holds `Rc<NurbsBoxes>`; `PairBoxes` and boundary share it.
  - Its guard is one build per cell per operand across 20 rungs.
  - **Times, against main:** shell 286 → 234 s, r1 183 → 164 s. The remainder is the per-window cut, which step 3 handles. Step 2 has started.
  - Tier: my read, after CI.
- 2026-10-14 00:16Z — **Correction:** the digest red (`tilted_cut_upper` pads) is NOT 4984's. It passes at 2639b944 and at a71403e3 (pre-4984), and fails on main HEAD. Build A is bisecting the merges after #4938 (#4964, #4952, #4863, #5033, #5022, #5025); the result goes to the owning program, not re-cut by NURBS.
  - **The saddle pin IS 4984's**, and the re-baseline is sound: the integral wall's floor rose from 1.97616 to 1.97751, against a sampled least ‖S_u×S_v‖ of 1.97754. The fix ships now, on its own PR.
  - **Reds at 1e-6 under triage** (base red, or flipped by 4984?): `cone_operand_rows`, `r1_lane0_e2e` (TubeNotOneArc), topo `three_solids_on_one_line`.
- 2026-10-14 01:25Z — **PR 5047 merged** (step 1, `NurbsBoxes` lifetime) at `f0f732d0`.
  - Hosted test job 23m14s; my read only.
  - Seam notes posted on ssiarith, ssiedge and ssimarch.
- 2026-10-14 01:25Z — **Step 2 in progress** (products-once with halving).
  - Locally, before the halving speed-up: shell 142 s and r1 88 s, against main's 286 and 183; face bound 0.59 s.
  - Re-baselined with causes: the `patch_bound` tile row (8×8 → 16×16, the retired exactness ceiling) and the `offset_fit` weights-2 pin (2 ulps).
  - I asked for the curve meters to move off `piece_weights` too (one primitive), splitting into 2a/2b if the PR gets large. FULL review when it's green.
- 2026-10-14 02:05Z — **Triage of the reds on main** (build A):
  - The saddle-wall pin from 4984 is being fixed in PR 5053 (head 23c140af), pin only. It merges when green.
  - The digest `tilted_cut_upper` red is not 4984's. A clean bisect first fails at 5f0d1cf5, the merge of #4952 (pcert). Build A re-cuts the table in its own PR citing #4952, and stops if any pad other than tilted_cut_upper's moves. A seam note went to pcert's log.
  - The 1e-6 reds (`cone_operand_rows`, `r1_lane0_e2e`, topo `three_solids_on_one_line`) and the 1e-12 `encl_curved_loft_shell` shelling pin all fail at a71403e3, before 4984, so they are base red and not ours. Build A bisects the shelling pin next.
- 2026-10-14 02:20Z — **Step 3 (cut becomes restriction) WIP** at c4fb888bd on `nurbs/cut-becomes-restriction`.
  - Its sound, tighter slope exposed a **latent soundness bug on main**: `NurbsBoxes::rect_box`'s midpoint over a point window is a plain f64 evaluation on the f64 lane, so it is not an enclosure. `edge_runs` then decides a wrong sign at an edge touch (`ssi::one_arc` `a_touch_inside_an_edge_resolves_no_count` reads `Some(2)`).
  - The fix lands first as its own PR (`nurbs/rect-box-midpoint-certified`, FULL review, with a red-under-mutant row), with step 3 on top.
  - The fork's "meet becomes a containment `debug_assert`" was falsified by measurement: 0.19% of 578,728 meets fall outside, by up to 108% of the whole box's width. The meet stays. A postscript is on the fork record.
- 2026-10-14 02:55Z — **Main's reds from this program are cleared.**
  - #5053 (saddle pin) merged at c6b16773 and #5055 (digest re-cut, cause #4952) at fe462015. Each PR's only red was the other's row; they touch disjoint files.
  - Still red on main and not ours (pre-4984): the 1e-6 rows `cone_operand_rows`, `r1_lane0_e2e` and `three_solids_on_one_line`, and the 1e-12 `encl_curved_loft_shell` pin. Build A is bisecting the shelling pin.
  - #5059 (`rect_box` midpoint enclosure, build C) is up. CI is red across lint and test, so the FULL reviewer waits for green.
- 2026-10-14 03:22Z — **#5011 answered and merged** (b8c3aa52). Ev took (A): the CI bar is 15 min, or 25 min for a PR seeding geom-core, geom, geom-brep or topo. Ev asked for an agent to look into the build time for easy wins. Dispatched a build-time lane (session_01QnnUqrMpoZHtXT1LbBth4W, base 41660715). Its findings go in a `work/ciw/` issue, with one PR per measured win of 2 min or more that drops no gate row.
- 2026-10-14 04:40Z — **Correction to the 02:05Z triage** (build A). Those bisects shared one `CARGO_TARGET_DIR` across worktrees, which serves stale libraries (the hazard `bitdump.rs:29` documents). Re-run with a fresh target per commit:
  - **#4984 flips three 1e-12 rows, so they are ours:**
    - `encl_curved_loft_shell::shelling_the_curved_loft_refuses_at_a_walls_fit` (cap reach 1.3958e-10 → 1.39486e-10, the same floor mechanism as the saddle);
    - both `rigid_map_near_eps_approx` rows, which are ENCL's open `the-near-eps-rigid-map-rows-are-red-at-1e-12-on-main.md`.
    - Build A fixes them on `nurbs/fix-shelling-pin-after-4984`, root cause only, and closes ENCL's issue.
  - **Still base red when re-run fresh at a71403e3:** the three 1e-6 rows, plus at 1e-12 `cone_operand_rows`, `a_moved_curved_fitted_cap` and the `shell_general_door` cap.
  - **The digest passes fresh at 2639b944**, so it is not 4984's. #4952's attribution is being re-confirmed fresh.
  - **Class finding:** a bisect needs a fresh target directory per commit. A shared one gives wrong attributions both ways.
- 2026-10-14 05:08Z — **Step 2 is up as #5081** (`nurbs/quotient-products-once` @5970bb25, FULL).
  - Against main: shelling 237 → 120 s, r1 seam 167 → 71 s, face bound 0.87 → 0.47 s, curve meters 8.1 → 4.3 s. The targets (≈70 s, 25 s, 0.17 s) wait on step 3.
  - The face-bound remainder is filed as `work/issues/interval-scale-by-an-exact-f64-pays-a-general-product`.
- **#5059's sound midpoint exposes a second latent bug on main.** `ssi/four.rs` `one_arc_pair`'s `reaches_end` "proves" an end lying on a domain side by Krawczyk on a slice whose root sits on the slice box's boundary face. Only the unenclosed f64 midpoint made that pass.
  - We take it ourselves: a new lane (session_013c7iZGZUnzBka2LTcMSVBi, `nurbs/pair-walk-domain-side-end`) reads such an end on the side's own face, the ℝ⁴ analogue of one_arc's `Piece::Side`.
  - #5059 also takes a corner-origin midpoint for far-from-origin operands, and closes `work/ssiarith/chart-point-signs-are-f64-evaluations.md`.
  - Order: the pair-walk fix → #5059 → step 3.
- 2026-10-14 06:05Z — **All of #4984's reds are fixed:** #5088 (shelling pin and rigid-map margin ladder) merged at 9f9662d8 and closes ENCL's `the-near-eps-rigid-map-rows-are-red-at-1e-12-on-main`. The fresh re-check confirms the digest's cause is #4952.
  - **#5094 (the pair walk reads a domain-side end by its certified face roots, narrowed by Krawczyk) is up.** #5059 turns more SSI rows red than CI showed: a domain corner (`two_bilinear_patches`, 1e-12), two more pair-walk rows, and an interior-box face walk out of depth (`a_branch_beside_a_close_crossing`, 1e-12).
  - **Ruling:** domain-side and corner ends go to #5094. The interior class goes to #5059, as a compensated (error-free) midpoint with an a-priori bound. (ii) was ruled out only for the boundary root.
  - **#5034** (ENCL handover, STYLE) has a STYLE reviewer dispatched. **#5081** (step 2) is in FULL review.
- 2026-10-14 06:20Z — **#5034 merged** (67a9bb4e). It closes ENCL's handover, after a STYLE review plus a comments-only follow-up. Build A posted its ENCL and QUAD seam notes via #5096.
  - **Two lily-table items** on this slate (`main-reds-the-lily-tessellation-table-...-since-4984` P0, and its duplicate `the-lily-leaf-tessellation-table-moved-on-main` P1) had been filed by FUSE and EMIT and not picked up here. That is a friction finding: `work.py incoming` lists commits that touch `work/nurbs/`, and I read past them.
  - Build A now has them: a fresh-target bisect, proof that the new counts still meet the certified chord tolerance, re-baseline or fix, and a sweep of every demo and slow set on main for unseen reds.
- 2026-10-14 06:20Z — **#5081 (step 2) APPROVED at FULL** (session_012Zp7HG7ttcpSHq5uZReKzS).
  - Soundness confirmed on both lanes: the mispaired-halves and piece-order mutants are red, and the sweep and topo slow sets are 271/271 green.
  - Interval bounds match up to degree 9 and are tighter from degree 12.
  - Minors (stale "exact dyadic piece" prose, the restrict callers in the body) go into the push that merges main. The lost W²/W³ halving-sharing (~8% of the face bound) gets filed.
- 2026-10-14 06:32Z — **Mechanism of the missed lily items:** `work.py incoming` lists main's commits touching `work/nurbs/` *that this branch lacks*. I merge main into the tracker branch at most check-ins, before running it, so items other lanes file are absorbed silently and it reports 0. Fix in practice: run `git log origin/main --since=<last check-in> -- work/nurbs/` and skip commits authored by this program.
- 2026-10-14 08:20Z — **Step 2 merged: #5081 at 8096a51b** (FULL approve, after a main merge plus prose fixes).
  - Against main: shelling 237 → 120 s, r1 167 → 71 s, face bound 0.87 → 0.47 s, curve meters 8.1 → 4.3 s.
  - Also merged: #5102 (lily table re-baselined to #4984's tighter chord bounds; certified deviation ≤ 2.35e-4 against 1e-3; closes the lily P0/P1), #5105 (CIW build-time findings; the SSIARITH fuzz issue), #5108 (base reds triaged onto their owners' slates, no duplicates). Build A's sweep (demos tour/wild and the workspace slow set at default ε, fresh targets) shows no other reds.
  - **Step 3 measured slower than step 2** with the converged cut: shelling 208 s, r1 133–150 s. Restricting the raised grids costs an O(n³) blossom pyramid at n = 7.
    - Two bit-identical savings (hoisting the ratio, restricting W² once) bring it to 173/96 s.
    - Ruling: (A), the textbook O(n²) two-split restriction. It stays within the converged design. Land it only if it matches step 2 on time and margins. Otherwise (B), step 2's cut, goes back to the two designers as a postscript round. No cache; no landing a regression.
  - **Build A** is on `interval-scale-exact` (`Real::scale_pow2`, the face-bound remainder). **The build-time lane's #5104** (lto off, −3.75 min, every row green at TIER=all) needs a main merge, then STYLE review.
- 2026-10-14 09:15Z — **#5094 merged** (c54ec254, FULL). The ℝ⁴ pair walk reads an end lying on a domain side through r's certified face roots, narrowed by their own Krawczyk image.
  - Its Interval-lane witness is red on main under the root-reading mutant.
  - On main + #5059 + #5094: two_fits (all ε), far_seed, two_bilinear, branch_at and all_finds all pass.
  - Filed: `a-pair-walk-end-at-a-domain-corner-refuses-undecided` (nurbs; conservative, not unsound) and `the-endpoint-census-cannot-see-reads-held-in-tuple-fields` (tint).
  - **Step 3 with the two-split restriction (A) beats step 2:** shelling 126 vs 139 s, r1 64 vs 76 s. Rung counts and margins are identical; 0–0.37% of meets fall outside, against 69–88% for step 2's cut.
  - **#5059** carries a compensated (double-word) midpoint, width ~16u² of the terms at a cancellation.
  - Order: #5059 (main merge, then FULL) → step 3 (FULL).
- 2026-10-14 10:22Z — **Friction:** #5104's test step 8 (default ε) went red at 3a366af7, and no session could name the failing row.
  - `get_job_logs` returns only the job's last 5000 lines, and later steps push step 8's FAIL line out of that window.
  - Job re-run is 403 ("Resource not accessible by integration"); the log blob URL is proxy-blocked; the annotations say only "exit code 100".
  - Locally at the same head and profile: 13972/13972. Suspected cause: the seed-varying enclose fuzzer filed on SSIARITH.
  - Re-running via a legitimate main merge.
- 2026-10-14 10:24Z — **Correction:** I wrote at 08:11 that #5104's TIER=all run had passed the slow set. It did not: at TIER=all the slow-set step is **skipped**, because its seed list is empty. The slow-set evidence for lto=off is the lane's local three-ε run, which matches main's nightly reds exactly.
  - **Finding:** a ci.yml diff runs every row except the slow set. Noted for CIW.
  - In the same hosted job, 1e-6 and 1e-12 ran the whole suite at 13972/13972. So the default-ε red points to a seed-varying row, not to lto=off.
- 2026-10-14 11:27Z — **#5104 merged** (2e59df73, STYLE). The CI test and cache-prime builds now run with `lto = "off"`. Local saving: −5 min build, −3.75 min job. The hosted saving is to be measured by the build-time lane.
  - The CIW items (TIER=all skips the slow set; a FAIL line past the 5000-line log tail can't be named, re-run returns 403) are going in via #5120.
  - **#5059 FULL review: approve with fixes.** **M1 (pre-existing on main, the subject of the closed ssiarith item):** `rect_box`'s rounded `um ± hu` can miss the window's own corner. Witness: z = 1000x − 500 on u ∈ [½, ½ + 3·2⁻⁵³]; random search finds 15,856 of 400k corners missed on a slope-1000 line. Build C fixes it in #5059 with outward `[u0 − um, u1 − um]` plus a red-under-mutant row.
  - `compensated.rs` was re-derived against JMP17 with no miss, and 2500 adversarial cells were checked against exact rationals. The midpoint costs +22% per `rect_box`.
  - **#5123 (step 3)** is in FULL review. **#5114 (exact-scale)** waits on CI.
- 2026-10-14 12:21Z — **#5059 re-check OK** at 7a8cc0b9: the M1 outward offsets give 0/400k corner misses on each wall, and the m1 and m2 rows are red under their mutants. It merges on green.
  - **#5123 (step 3) FULL review: one MAJOR, the timing doesn't reproduce.** Head vs its own base d0497984: r1 93–96 s vs 64–65 s (45% slower), shelling unchanged. gdb puts 24 of 30 samples in `split_restrict` under `cut`.
    - Build C's (A)-vs-step-2 table used a base without #5059. Everything else holds: soundness, the meet, retirements, the +1.11e-17 touch claim, parity, and downstream 275/275 and 266/266 at 3 ε.
    - Ruling: no landing as a regression. Re-measure and profile head vs base; if (A) can't beat it, (B) goes to the designers.
  - **#5132 (build A): the SSIARITH rigid-map fuzzer P2.** The cause is the test's scale (the margin rounds at magnitude 1, not at its own value), not a bound. Test-only fix, with the 4 seeds pinned and a production mutant still red. FULL review on green.
  - **#5114 (exact-scale)** is in FULL review.
- 2026-10-14 12:48Z — **#5059 merged** (01bd9fa7, FULL plus the M1 re-check). It closes `work/ssiarith/chart-point-signs-are-f64-evaluations`. #5132 (the rigid-map fuzzer, test-only) is in FULL review.
- 2026-10-14 13:25Z — **Step 3 postscript round converged** (both designers, on the fork record branch). Keep "the cut restricts the formed quotients"; reject (B) step 2's cut and (D′) per-cut choice. Make the restriction cheap, in this order:
  - (1) a thin-ratio step instead of `Interval::between`;
  - (2) axis-lazy restriction;
  - (3) a window-reading object that restricts once across doors;
  - (4) batching;
  - (5) a one-piece cut, only if still needed.
  - The fallback is to restrict the formed low-degree grids. #5123 lands only when it's no regression against main on either row.
  - **Protocol finding:** I appended B's answer to the fork record before A had answered, and A's brief pointed it there, so A's answer was not blind (A disclosed this). Next time, append postscript answers only after both are in.
  - #5132 (fuzzer) is at 5ed36e07 after review fixes, with Unit tightened to 32 ulps. #5114 is in FULL review, with the ε rows green and slow sets running.
- 2026-10-14 13:50Z — **#5132 merged** (d6396bc0; FULL review mergeable, follow-up test-module plus docs only). It closes both SSIARITH rigid-map fuzzer items.
  - Build A's next unit is measurement only, so it can't collide with #5123's files: a perf split of shelling and r1 on main and on #5123's head (how much of the excess over pre-4954 is not the quotient unit), plus a `RATIONAL_CERT_SPLITS`/`BLOCK_PIECE_SPLITS` sensitivity run on main. Tables go to build C too.
  - The NURBS slate on main has no unblocked code item outside #5123's territory: the corner-end refusal and the elevation conditioning both wait on a consumer.
- 2026-10-14 14:40Z — **#5114 FULL review: one soundness MAJOR.** `exact_scale` tests `|r| ≥ MIN_POSITIVE` on the rounded product, so an exact product one 2^-1075 step below MIN_POSITIVE rounds up and is returned unpadded (bits `0x001F_FFFF_FFFF_FFFF` × 0.5). Everything else is clean, and callgrind shows −15% total. Sent to build A (it owns #5114) ahead of its profiling unit.
  - **#5123 step 1 (thin step) beats main:** shelling 144 → 106 s (−27%), r1 47 → 36 s (−23%). Rungs are identical and margins agree within 2e-13. The plain thin step widened equal neighbours by t's width; equal-neighbour exactness (`x == y` steps to `x`) brings the cut-wider-than-whole meets down to 0.5–8.6%, and the meet clamps those.
  - **Ruling:** that's a lost tightening, not a lost decision. #5123 lands on step 1 once it is re-timed against main, passes the downstream suites at both ε, has a witness row and gets the reviewer's re-check. Steps 2–4 (axis-lazy, a window-reading object, batching) become a filed follow-up.
- 2026-10-14 15:20Z — **#5114 merged** (e27a4648). The MAJOR is fixed with a strict `|r| > MIN_POSITIVE`, plus a boundary row that was red on the prior head. The reviewer's exhaustive boundary scan (1.17M cases) finds 0 inexact shortcuts, against 1022 under `>=`. Callgrind: −15% total, −26% on halving.
  - **#5123 re-timed** (thin step plus equal-neighbour exactness, b3d723f3, against main 85caa597): shelling 143 → 97 s (−32%), r1 49 → 32 s (−35%). Build C acknowledged the ruling: it lands on step 1, with steps 2–4 as a follow-up. It is merging main (now with #5114) and doing the landing items.
- 2026-10-14 15:30Z — **The hosted saving from lto off (#5104) is measured.** The median `test` build step fell from 1206 to 871 s (−335 s; n = 60 vs 31, against main's cache prime at 11:32Z), matching the local −291/−302 s. Whole test jobs on two branches went from 55.7 to 30.4 min and from 45.3 to 34.2 min, though the trees differed between those runs. Recorded on the CIW findings item via #5143 (tracker only).
- 2026-10-14 15:55Z — **Build A's profile split** (measurement only; gdb sampling, fresh targets, main d6396bc0 against #5123's pre-thin-step 2d1a8d73).
  - **Shell:** the slow rows' excess over pre-4954 is ≈95% the quotient unit (forming, restriction and the piece readers). The remainder is ≈9% offset-fit Bernstein products (`compose.rs` `bern_mul_row_into` under `offset_fit::dot_spans`) plus ≈4 s walk.
  - **r1:** essentially all quotient. Its +30% at 2d1a8d73 was `split_restrict`/`between`, which the thin step has since fixed (−35% against main).
  - **Piece counts:** `RATIONAL_CERT_SPLITS` 16 → 8 doesn't move these rows. `BLOCK_PIECE_SPLITS` 2 → 1 gives −36% shell and −24% r1, but reds `a_block_is_read_on_its_pieces` and the m5_pr7 flat-wall slope margin at 1e-9, a real decision. It is filed as the converged plan's step 5 (an adaptive split), with those rows as its guard.
  - **Build A's next unit:** offset-fit Bernstein products. Measure repeated `Composite::build` calls and a cheaper sound inner term, then implement what's real. It stays out of #5123's files.
- 2026-10-14 19:25Z — **#5123's re-review found a decoration hole in the equal-neighbour arm.** The arm compared only lo/hi, so a certified bracket beside a refused one with the same ends came back certified (probe: [0,2] Com beside sqrt([-1,4]) Trv).
  - **First fix** (b6326d27): the arm also requires `is_certified` equality. That was still able to upgrade Def to Com.
  - **Second fix** (049bbf51): the arm uses `Interval::same_enclosure` (ends plus decoration, `pub(crate)` in interval.rs). It has two witness rows, each red on its weaker arm, and the census is unchanged against main.
  - **Independent timing** at 68dfd27a against main b41d2fe3, both with #5114: shelling 123 → 92 s, r1 43 → 30 s. Five margins move beyond 2e-13, all tighter (up to 5.6e-12 on m5).
  - **Open for the reviewer:** whether the bit-equality fast path, which decides nothing because both branches are sound, sits inside D10's retirement of coincidence decisions.
  - **Build A's #5163** (`mul_nonneg`: a 2-corner product for a nonnegative weight, bit-identical to the general product) is in FULL review. It gives −14.5% instructions on the offset-fit row and −2.9% shell wall. Of 44 `Composite::build` calls per shell row, 4 are repeats; they were reported, not cached.
  - **`viewer::review_pick_r2`** is red on main since #4861 (CLEAVE). It was bisected and filed on CLEAVE's slate via #5158.
- 2026-10-19 16:20Z — **#5123 merged** (a908d341), after a five-day usage pause. The cut is now an exact restriction of the formed quotients, via a thin convex step whose equal-neighbour fast path compares whole enclosures with `same_enclosure`.
  - Shelling −28%, r1 −30% against main, both sides with #5114.
  - Review rounds: the timing MAJOR closed by the thin step; two decoration MAJORs closed (`is_certified`, then `same_enclosure`); ruled not a D10 coincidence consumer; the bit-identity gate now sees `same_enclosure` callers. A noted residue: `blossom.rs` is exempt as a whole file.
  - Margins: r1's four loosen by 2–3e-13, m5's 4 of 241 tighten by up to 5.6e-12. Rungs are identical.
  - It waited out the copysign census main red, which GERM fixed in 22b62668 (Oct 15).
  - **Build C's next unit:** `cheapen-the-restriction-cut-steps-2-to-4`, axis-lazy first, every step bit-identical and timed. The adaptive block split is held until those are measured.
- 2026-10-19 19:10Z — **#5391 merged** (dfdb5349): axis-lazy restriction. One-axis readers restrict only `N_d` and `W²`.
  - Bit-identical end to end: the `limb_three` probe matches line for line on 285 decisions (shelling 12, r1 6, m5 258, section 9). The reviewer measured shelling −14% and r1 −35% against main, build C −16% and −36%.
  - **Steps 2–3 of the follow-up are closed as measured-not-worth-it:** about 2–3% each, and `stretch_within` reads distinct windows. The remaining lever on the cut's arithmetic is #5163's `mul_nonneg` in the thin step, after #5163 lands.
  - **MINOR, to the next unit:** no geom-core row pins `N` and `W²` cut over the same region (mutant M3 survives every test). It rides build C's next PR with an independent oracle row.
  - **Build C's next unit:** a per-door piece schedule. `beyond_reach` (skip-only) reads one piece. `clears` reads one piece only if `Within` is independently certified; that is pending C's argument. It measured zero moves across 45,715 reads.
  - **#5163** (`mul_nonneg`) is MERGEABLE but held behind four main reds at 1e-12, owned by FLUX and CPAIR.
- 2026-10-19 19:35Z — **Friction: two orchestrator messages to build C were reported delivered but never reached it** (17:58Z, the `clears` soundness ruling; 19:09Z, #5391's review carry-overs). A `now`-priority re-send at 19:26Z arrived. From now on, any ruling a lane must act on gets an explicit ack, and a missing ack gets a `now` re-send.
  - **Build C's coarse per-door schedule** (`beyond_reach` and `clears` at one piece; every margin or sign reader at two): parity is identical on every `limb_three` line. r1 30.0 → 14.6 s (−51% vs main), shelling 88.6 → 74.2 s (−16%). Before opening the PR, C reads whether `Within` is certified independently of `Clear`.
  - **Build A's #5399** (halve shared weight powers once) is in FULL review: −11.1% on the face bound, bit-identical.
- 2026-10-19 21:00Z — **Correction to the 19:35Z friction entry:** the messages to build C were not lost. Default-priority messages wait until the lane's turn ends, and C's turns run for hours of timing, so they arrived only at its next turn boundary. Rulings now go with `next` priority, which joins the running turn.
  - **The coarse schedule is dropped.** With `clears` excluded on soundness (`Within`'s definition includes "no piece clears": `boundary.rs:1021`, `one_arc.rs:625/721`), the skip-only `beyond_reach` read alone gives shelling −3% and leaves r1 unchanged. `an-adaptive-block-piece-split-…` is closed as measured. `clears` (38k r1 reads) is the remaining r1 lever, with no sound one-piece path.
  - **Slow rows on main d989dab1:** shelling 74.0 s (2.1× pre-4954), r1 17.8 s (1.4×), against a 2× target. Build C re-times and closes the item, or names the remaining levers.
- 2026-10-19 21:45Z — **Slow rows re-timed** (main 36bdaf3b against pre-4954 `092b3022^1`, fresh targets): shelling 2.13×, r1 1.49×, so the item stays open. Shelling's gdb profile:
  - whole-cell formation 23%, other `enclose` 18%, `rect_box` 15%, cut restriction 12%, `transverse_readings` 9%, offset fit 9%, cut halving 8%;
  - **the new lever:** `offset_derive` mints `ChartedNurbs` 17 times for 4 walls, through `Section::of` and `PlaneSection::of`.
  - Build C takes it (FULL; seam note on topo's log): a section door takes the caller's minted chart, bit-identical. After it come `mul_nonneg` in the thin step, then #5163.
  - #5404 (the #5391 review follow-ups) is STANDARD MERGEABLE; CI attribution pending. #5399 is green at 3 ε on the head, verdict pending.
- 2026-10-19 22:00Z — **Mint-once territory** (build C, `nurbs/mint-walls-once` from main 6d53b3d2):
  - `replace_face.rs`/`offset_derive.rs` belong to shelf/shell, and have no live work in 9 days;
  - the ssi doors belong to ssiarith/ssiedge/ssimarch;
  - `props.rs` belongs to flux/tally, and `flux/spiric-budget-world-position` is live there.
  - **Ruled:** don't hold. The `props.rs` hunk is the associated type plus 5 impls only; merge main before the PR. Seam note on flux's log.
- 2026-10-19 22:10Z — **#5163 merged** (e763c9c7, `Interval::mul_nonneg`).
  - CI's only red was the CPAIR needle at 1e-12, which is red on main and recorded in `curve-pair-polish-meters-refuse-shallow-meetings`. Merged per the standing ruling; the attribution is commented on the PR.
  - **#5414** (build A, binomials fold, PR 1 of `binomial-bernstein-product-and-blossom-have-two-homes-each`, head 3a412f92) has a FULL reviewer, session_01SkTY8ainEoh2LQ17kMnQMk.
- 2026-10-19 22:20Z — **#5404 merged** (f45f2ac4, the #5391 review follow-ups; CI fully green). `cell_pieces`' copy of the halving schedule goes to build A as a follow-up after #5399. #5399 still merges cleanly with main.
- 2026-10-19 22:50Z — **#5399 merged** (e2ee445a; each cell halves each shared weight power once).
  - **FULL review MERGEABLE:** 0 MAJOR / 2 MINOR / 1 NIT. Bit-identity holds, with 4 mutants red. 3460/3460 at all three ε, `limb_three` byte-identical, callgrind −11.1% reproduced.
  - **MINORs routed:**
    - M1 (the cut side's `shared_pieces` switch was unscheduled) is filed as `cut-side-quotients-halve-the-shared-weight-power-per-axis`, which also carries M2 (`WEIGHT_HALVINGS` counts calls, not halvings);
    - the NIT (the bit row skips `cell_quotients`' arms) and the `halving_counts` switch in `cell_pieces` go to build A's follow-up.
  - Reviewer archived.
- 2026-10-19 23:45Z — **Mint-once is up as #5419** (build C, FULL, head 019beb75, bit-identical; `limb_three` matches line for line on 4 rows).
  - **Shelling:** 17 mints of 4 walls become 5. The fifth, in `plane_nurbs_limbs`, runs after the scope drops, by design.
  - **Timing:** shelling 2.14× → 1.86× of pre-4954 (−13%); r1 flat at 1.49×.
  - **Deviation from ruling (A), accepted:** a concrete `f64` `OnceCell` slot in `offset_derive::WallCharts`, with no associated type on the lane trait, because the planners are bounded by `T: Decide` only. `props.rs` is untouched, so there's no collision with flux.
  - FULL reviewer: session_01RsTjMpvtHiPMQ64nQhBY4V.
- 2026-10-20 00:40Z — **#5424 is up** (build C, the `mul_nonneg` thin step in `split_restrict`, STANDARD, head ece8fbf1, bit-identical). Shelling −7.6%, r1 −11% against main 8c6e959e; independent of #5419, so the two should stack. STANDARD reviewer: session_018C4CWrdSjiRiekkGe59P8T.
  - **Friction:** C reports my `next`-priority messages from ~22:00Z and 00:06Z (the `props.rs` ruling and #5419's latency red) arrived late, all at once. `next` joins the running turn, but C's long background timing loops leave few points where it can. The #5419 red was re-sent at `now` priority. A red on a lane's own PR should go `now` from the start.
- 2026-10-20 01:55Z — **#5414 merged** (93aa9511; binomials fold PR 1, so pairs with da+db of 55/56 are now served).
  - **FULL review MERGEABLE, 0 MAJOR:**
    - the 55/56 cap is exact;
    - the cap mutants at 54, 55 and 57 go red;
    - 9 reds at 1e-12/1e-6, each red with the change reverted and all filed.
  - **Routed:**
    - MINOR-1 (two test-only copies of the f64 binomial fold, at `ssi.rs:4397` and `m5_pr7_ssi.rs:5535`), the stale prose and the doubled constant go into build A's STANDARD follow-up, with the #5399 items;
    - the NOTE (the guard mutant survives at `compose/tensor.rs:429`) is filed P3 as `tensor-residual-degree-guard-has-no-row-at-its-edge`.
  - Reviewer archived.
- 2026-10-20 02:35Z — **#5433 up** (build A's STANDARD follow-up to #5399 and #5414, head 47d1d68c). It also folds a third, production copy of the binomial fold (`carrier_net.rs`'s Taylor re-expansion, bit-identical).
  - **Base red found by A:** topo's `every_public_mutation_path_preserves_tier1` is red on main since ENCL's PR 5418 (`restate_stored_edge` is off the roster). Filed on ENCL's slate with a seam note; NURBS PRs cite it as a base red.
- 2026-10-20 02:50Z — **#5424 merged** (8f1edb5f; the `mul_nonneg` thin step).
  - **STANDARD review MERGEABLE:** a probe of 296k real weight pairs (~1.19M steps) found 0 differences, and no weight read `lo < 0`. Suites are green at both ε on head and main; r1 is −8% at opt-level 0.
  - The MINOR (the "[0,1]" comment is inexact; `s.hi` reaches 1+2⁻⁵²) and the row's missing wide/tiny-t draws ride C's next `split_restrict` PR.
  - The lever is marked done in `cheapen-the-restriction-cut-steps-2-to-4` and the slow-rows item. Reviewer archived.
- 2026-10-20 03:16Z — **#5419 partial review** (FULL, at c75c3a2c):
  - the `limb_three` probe is byte-identical on 4 rows;
  - geom-brep and mesh are green at default and 1e-6 on both trees;
  - C's latency fix is test-only, and the swap mutant stays red.
  - **Ruled into this PR:** the chart/surface pairing check on the `pub` `of_charted` doors was debug-only, and with it stripped a wrong-held-key mutant passed 6 of 8 rows silently. The reviewer graded it MINOR; ruled in because it is a fail-loud gap on a public door, closed by a pointer compare. C makes it a release check and adds a mismatched-chart row.
  - Two 1e-6 head reds (`cone_operand_rows::an_oblique_rod…`, `r1_lane0_e2e::the_f64_seam…`) await attribution against main's 1e-6 run. The full matrix ETA is 3–4 h.
- 2026-10-20 03:50Z — **#5433 merged** (10b9dfd5; build A's follow-up to #5399/#5414).
  - **STANDARD review MERGEABLE, 0 MAJOR:** the `carrier_net` fold is bit-identical (the old fold first differs at C(55,26); this code uses n ≤ 6), and the slow sets match main at both ε.
  - **Routed:** the minors (three more test-only folds, `polynomial_area_pieces`' hand-written pair, `shared_pieces`' silent fallback) are added to `cut-side-quotients-halve-the-shared-weight-power-per-axis`, build C's next PR, which is now unblocked. Build A moves to the binomial item's PR 2 (the product and blossom homes).
  - **#5419:** the pairing check now runs in every build (head cd38b6f7, `SsiError::WrongLane`, two mismatched-chart rows). Both 1e-6 head reds are confirmed base reds, identical word for word on main (GERM's oblique rod; ssiedge's `TubeNotOneArc` at the post-frame mint). The reviewer is verifying the delta.
- 2026-10-20 04:18Z — **Binomial item PR 3, Step A** (build A, read-only). Compared each restriction home's coefficient intervals against the pyramid at every real call (the geom-brep slow set plus sweep shelling/r1/shell_general_door). Worst case, other home vs the pyramid:
  - section's general cuts, split: up to 98× wider (15% of coefficients by more than 4 ulp);
  - section's exact halves, halving kernel: up to 2×;
  - `cut_pieces`, split: up to 417× (absorbed by the cut's meet with the whole cell, #5123).
  - The pyramid is never meaningfully wider.
  - **Ruled:** three homes stay, each with its reason stated. The pyramid is section and props' door, `split_restrict` the cut's meet-protected fast form, `Grid::halved` derivative's halving. The one fold: `restrict_bernstein` (sole caller `props/quad.rs:4376`) reroutes to `restrict_segment`, bit-identical, and retires. STANDARD. The table goes into the item's record. Seam note on flux's log.
- 2026-10-20 04:20Z — **Correction: the #5439 ruling was wrong.**
  - **What happened:** at ~03:50Z I approved resolving the Bernstein-product half of the binomial item by a stated reason. The reason was that folding would move one side's pinned bits.
  - **Why it was wrong:** `memories/output-stability-as-justification.md` forbids exactly that: bit-preservation "may never justify keeping a thing: not a second implementation".
  - **How it was caught:** #5439's STANDARD review raised it as MAJOR and tried the fold. compose's memo holds exact integer numerators, read through `mul_nonneg`. Only the old-association bit rows and the rehearsal golden moved, and that golden's bound got tighter (1.865e-14 → 1.465e-14). Wall time was unchanged.
  - **Re-ruled:** #5439 does the fold, now at FULL, with the golden re-baselined and the old bit rows retired. The same reviewer re-reviews.
  - **Friction:** I ruled on a design-shaped question ("two homes, or one?") without re-reading the memory index for a rule that applies. The orchestrator's adjudication docs bind my own judgement too (CLAUDE.md). Before approving any "keep both" resolution, check it against this memory.
  - PR 3's three homes rest on width and cost, which is allowed. Build A will audit its docs for the "kept because bits would move" tell.
- 2026-10-20 05:10Z — **Degree elevation: P4 → P3, fix ruled** (build C measured it on main 43b23583).
  - **The item's 1/g law holds only for p = 3–5.** p = 6–7 is 1/g², p = 8–9 about 1/g³, and the error is silent: p=8 with a 1e-5 interior gap is 1.7 m off after one `elevate_degree(1)`. It depends on a gap's ratio to neighbouring spans, not the absolute gap; uniform knots are clean at every degree. Double elevation compounds (p=5, `elevate_degree(2)`, g=1e-8 → 2.2).
  - **Callers:**
    - `skin` and `compose_chain` raise user sections to the maximum degree, so they carry it silently;
    - `approximate` elevates only from p ≤ 2 (exact);
    - `review_m5_pr3_attack::f5_*` is a latent ~4e-6 flake with the wrong law in its comment.
  - **Ruled:** fix, at FULL. A conditioning-aware forward/backward split in elevation's recomposition only. If that doesn't reach ε·scale at p ≥ 6, report; the fallback is a typed refusal, never silent drift. Rows red on main; goldens over elevated geometry re-cut, with what moved stated; SWEEP and curves seam notes if their goldens move.
- 2026-10-20 05:15Z — **#5446 CI red, base red.** `geom curves::tests::ellipse_constructor_trios` (`SemiAxisNotFinite`) is red on main since GERM's `53967a14` (#5156); build C bisected it. Filed on GERM's slate with a seam note. #5446 merges over it under the standing base-red ruling.
  - **Friction:** the orchestrator's container hit 99% disk (a 24 GB stale target from earlier timing work) and couldn't build to check. Freed; lanes with warm builds now do base-red checks.
- 2026-10-20 05:30Z — **#5419: topo's `WrongLane` must panic** (ruled in from the reviewer's cd38b6f7 verification).
  - **What the reviewer found:** the door's release pairing check refuses every mispairing, but topo absorbed `WrongLane` as an ordinary section refusal, so the wrong-held-key mutant went red on only 3 of 34 slot rows in release; the rest took refusal or fallback paths silently.
  - **Ruling:** DESIGN `[loud-failure]` ("a bug panics, as early as it is detectable"; "no fallback that answers where a method declined"). Topo builds the pairing, so a `WrongLane` at its call sites is a kernel bug. C makes the two `offset_derive` call sites panic on it; the `pub` doors keep the typed refusal for outside callers.
  - **Also verified at cd38b6f7:**
    - no false `WrongLane` (clones keep the surface reference);
    - the shelling probe is byte-identical;
    - all 7 topo and sweep 1e-12 reds are filed base reds, word for word.
  - **#5440:** review MERGEABLE (1450 pieces bit-identical, M2–M7 red, 267 decisions match). It waits on C resolving the item conflict that my aafaa9b2 caused. The two `split_restrict` carry-overs move to build A's PR 3.
- 2026-10-20 07:25Z — **#5439 resolved: two Bernstein product homes, kept for precision** (build A's measurement under the rule fixed at 04:42Z).
  - **Decisions:** neither fold direction moves one (the geom-brep and sweep slow sets at both ε, plus the `limb_three` census over 735 rung lines; only margin bits and one box count change).
  - **Widths:**
    - B (compose on exact numerators) widens compose's tensor and `ch_mul` and `mul_block` coefficients up to 6× and loses exact zero-width points at `tensor.rs:383/386`, though it is narrower on 7–40% of them;
    - C (derivative on compose's per-term weights) widens `area_raised` up to 1470× and `doubled_cross` up to 208×.
  - **Ruled:** rule 3. Each association is tightest for its own callers, so two homes stay, with precision as the stated reason and the table in the item. #5439 drops back to STANDARD: docs, the bit rows kept, the enclosure rows, the review's MINOR and NOTE, no golden moves.
  - **Lesson:** fixing the rule before the numbers was worth it. The first ruling (bits) was wrong in kind; this one is right in kind and holds to its threshold.
  - **#5419:** C pushed 67c2d535. Topo's `of_charted` sites panic on `WrongLane`; the release mutant is red on 8 rows by the panic, against 3. The reviewer re-verifies.
- 2026-10-20 07:45Z — **#5419 merged** (759acedd; mint each wall once per operation).
  - **FULL review MERGEABLE at 67c2d535:**
    - the release pairing check refuses mispairings at the `pub` doors, and topo panics on `WrongLane` at its call sites;
    - the wrong-held-key mutant is red on all 8 mispaired rows by the panic (against 3 before the panic, with 5 silent);
    - unmutated topo+sweep release pass 6332/6332, and `limb_three` is byte-identical.
  - **The slow rows are under target:** shelling 1.66×, r1 1.33× (main + #5419, build C). The slow-rows item is closed with those numbers, and `cheapen-the-restriction-cut-steps-2-to-4` closed (step 4 not taken).
  - **Filed:** `mint-once-is-pinned-only-by-timing` (P3; the review's M3 survives, so a mint-count row is needed), with the review's style tail.
- 2026-10-20 07:50Z — **#5459 is up** (build C, the degree-elevation fix, FULL, head 60954f25).
  - **The fix:** `remove_once(.., ChainSplit)`, with elevation on `Conditioned`, which tracks a first-order error estimate per control point and takes the least-error split each pass (midpoint on a tie); knot removal keeps `Midpoint`. The plain α ≥ ½ rule failed on uniform knots, where α ties at ½ and error chained.
  - **After it:** p ≤ 10 × g ≥ 1e-9 within 3e-15 of scale, the `WeightCollapse` refusals at p ≥ 8 gone, no golden moved, elevation 1.3–2.1× faster.
  - **Base reds:**
    - GERM's ellipse row, fixed on main by `2ee8a2ea` (my item is closed);
    - a new `reader_census` red, filed on ENCL;
    - ENCL's tier-1 roster red.
  - **Friction (build C):** one `CARGO_TARGET_DIR` shared across worktrees silently reuses another worktree's build of a workspace package, because cargo's fingerprint checks the source paths of the last build. It made a red-on-main check pass on fixed code twice today, until `cargo clean -p <pkg>`. Lanes taking red-on-main or timing measurements use one target directory per worktree, or clean the switched packages. Raise it as an `[ev]` memory candidate if it recurs.
- 2026-10-20 08:05Z — **#5440 merged** (a6aa5d45; the cut-side quotients halve the shared `W²` once, via `shared_pieces`).
  - **STANDARD review MERGEABLE:** 1450 pieces bit-identical; M2–M7 red; 3113/3113 at both ε; 267 decisions identical.
  - The conflict that my aafaa9b2 caused is resolved on the item, which stays closed; its two `split_restrict` lines moved to #5461. CI is green on 5ca3b5e9, and the code diff is identical to the reviewed head.
  - **#5439 merged** (ac8a2e40), keeping two Bernstein product homes for precision.
  - **#5461** (binomial item PR 3, closing the item) and **#5459** (degree-elevation fix) are in review.
- 2026-10-20 09:30Z — **#5459: margin 2, and main's graded-knot defects** (build C's held-out measurements; median of 30 curves, one `elevate_degree(1)`).
  - `MIDPOINT_MARGIN = 8` was fitted on noise. K2 is at or within 2× of the best on every held-out family (graded t^1.5, geometric ×1.15/×2, jittered, with and without a close pair).
  - **Main's midpoint split is badly conditioned on graded knots with no close pair:** geometric ×1.15 at p=10 is off by 0.15, steep ×2 at p ≥ 8 refuses, and uniform p=9 reaches 1.6e-3 at 80 knots.
  - **Ruled:** K2, documented as empirical with its fitting set; a p=9 uniform row; the near-tie pinned by an exact split-choice unit row, not a cap; a geometric ×1.15 row. K2 moves bits on 90 of 800 uniform shapes at equal or better error, which the output-stability memory allows; stated in the PR. The reviewer reviews the new head.
  - **Build A:** KNOT's `surface-knot-insertion-writes-weights-validate-counts-refuses` (STANDARD, a silent error at the producer) first, then CPAIR's `curve-pair-polish-meters-refuse-shallow-meetings` (P2, the needle base red at 1e-12).
- 2026-10-20 09:55Z — **CPAIR polish-meters plan** (build A, read-only diagnosis; queued after the KNOT unit). The needle/leaning-prisms base reds and the cap-ring/atan2s reds are two families:
  - **A (f64):** at a fillet joint the rim ellipse touches the circle with parallel tangents. The window must tile a ~1e-7 on-stretch, but `close` has a floor of L·rounding_charge ≈ 3e-12 > z = 1e-12, and the first-order bound reads On only at 2e-13 pieces, so `PIECE_BUDGET` runs out. **Fix:** a third, second-order certified On bound along a tangent-matched path (Taylor); no meter change, no budget raised. The prototype turns the needle and leaning_prisms green at 1e-12.
  - **B (Interval):** polish iterates the interval itself, so the last step bounds the iterate's wrapped width. The fix needs a new `bounds_allowlist` entry (ratified per entry), so it is a design fork for Ev; to be filed as its own item and weighed by designers first.
  - **Ruled:** A at FULL, with certified enclosures for the second-derivative maxima and outward rounding; B filed separately with `design: true`.
- 2026-10-20 10:15Z — **KNOT `surface-knot-insertion-writes-weights-validate-counts-refuses` closed by #5470** (c9d9d0dc, tracker-only). It was already fixed at the producer by #4923 (`algebra::written_weight`); build A re-ran the mutant on today's main, and both door rows go red when the check admits everything.
  - **#5461:** STANDARD review MERGEABLE. Three minors on the homes' stated reasons, the halves claim and the tiny-t draw were fixed in-PR at cb8b9eb7, a docs/test-only delta that I verified. Merging on green.
  - **#5468:** CI red only on the CPAIR needle (filed base red; commented on the PR).
  - **Build A:** on the CPAIR fix A. The second-order bound is in, and the needle and leaning_prisms are green at 1e-12 locally; rows next.
- 2026-10-20 10:30Z — **#5461 merged** (3570dc18; it closes `binomial-bernstein-product-and-blossom-have-two-homes-each`).
  - **Three restriction homes stay, each with its true stated reason:** the pyramid by width, `split_restrict` by cost (sound because it is meet-protected), `Grid::halved` by genericity over `T: Real`.
  - `restrict_bernstein` is retired, its quad reroute bit-identical.
  - #5424's thin-step carry-overs are covered.
  - The binomial item series is complete: #5414, #5433, #5439, #5461.
- 2026-10-20 — **#5468 merged** (C, `8659a922`): `mint-once-is-pinned-only-by-timing` is closed. The mint counter is now pinned by a row, not by timing. Test and message changes only. Merged over the needle base red at ε 1e-12 (`band_turn_overrun_fillet::the_needle…`, being fixed by #5477), commented on the PR. (NURBS orchestrator)
- 2026-10-20 — **#5480 merged** (A, tracker only): the line-line crossing witness did not reproduce. In 40 000 shallow-crossing reads on main there were 0 wrong answers: a misplaced crossing point is always refused, never decided wrong. CPAIR's `the-curve-pair-line-line-crossing-is-read-from-the-carriers-origins-uncharged` drops to P3 and names the measure a fix must move (curve-carrier-gap escalations). A now takes CPAIR's P1 `two-spans-projections-on-one-line-are-read-four-ways`, plan approved. It lands in four PRs: the shared reading's home in `geom` with the router moved onto it, then check 9, then `profile::seg`, then the census. Witness on main: a 2 m gap 5e-12 off the line, refused by check 9 and `seg`, decided by the span-first router. (NURBS orchestrator)
- 2026-10-20 — **#5459 merged** (C, `8aabcfd1`, FULL): degree elevation recomposes each breakpoint by `conditioned_split` with `MIDPOINT_MARGIN = 2` (empirical, fitted on held-out families). Close knot pairs no longer amplify rounding (main's 1/g law at p 3–5, 1/g² at p 6–7, ~1/g³ at p 8–9, reaching metres), graded vectors no longer refuse, and uniform p=9 no longer drifts. Rows pin the margin to (1, 3); no row tells the tie rules from their inverses, and the doc says so. Closes `degree-elevation-recomposition-error-grows-as-one-over-the-closest-knot-gap`. Filed `elevation-past-degree-ten-agrees-to-1e-11-with-no-bound` (P3). C next pins that item's envelope rows and narrows it to the surfaced-bound form, kept open as a `[loud-failure]` gap. Reviewer bit-identity: margin 2 = 5e22e608 on 23,950 curves, midpoint mode = main. No golden moved. (NURBS orchestrator)
- 2026-10-20 — **Friction (orchestrator):** I ruled #5481's home into `geom` on A's word that `profile` depends on geom. It doesn't: `crates/profile/Cargo.toml` has `geom` as a dev-dependency only. The FULL reviewer caught it (MAJOR), and the home moves to `geom_core`. A dependency claim is one `Cargo.toml` read to check before ruling on it. The same review found a BLOCKER: the new gap charged only the end points' magnitudes, not the rounding of evaluating them from far carrier origins, so a touching pair could read "no meeting" (1 in 2000 at head, 0 on main). The witness fuzz used exact inputs, which is how it slipped past A's enclosure fuzz. (NURBS orchestrator)
- 2026-10-20 — **#5484 merged** (C, `bb209932`, STANDARD, orchestrator-reviewed: tests and docs only). It pins the elevation envelope past degree 10: p=11 (cap 2.5e-11), p=12 (1.1e-10), and p 16/20/25 on 1e-9 clusters (2e-11). Each cap is about 3× the worst of 90 curves at CI's profile, and all three rows are red before #5459; the cluster row also turns red at margin 8. One sentence was added to `compose.rs` and to `skin.rs` (seam note posted on `work/carve/log.md`; `skin.rs` is carve, carverest, carvetail and inlane ground). `elevation-past-degree-ten-agrees-to-1e-11-with-no-bound` stays open at P3, narrowed to the bound-or-refuse form. C is now on TESS's D303. **#5481** (A, PR 1 of one-line): the FULL review is NOT MERGEABLE. B1: the gap undercharged the ends' evaluation from far origins, so a touching pair read Apart. M1: the home goes in `geom_core`. A is folding both in, and the router's "far-origin gain" goes away with the honest charge. (NURBS orchestrator)
- 2026-10-20 — **#5485 merged** (C, `bea03a9f`, STANDARD, orchestrator-reviewed, TESS ground, seam note posted). D303 closed: a negative or NaN sizing step refuses `StepNotPositive`. C's probe corrected my first ruling: +0.0 (underflow → overflow) and +∞ (zero curvature → one cell) are honest readings. C's next unit is TESS's lofted-circle item, approved without a designer pass, since no ratified text governs chord placement. The C¹ gate's straddler argument was found unsound for creases, so the soundness fix lands first as its own FULL PR. **#5481**: the delta review is underway. 0 misreads in 4000 at each of 2^14, 2^20 and 2^24, and the tightness floor holds; two rustdoc link errors are being fixed. (NURBS orchestrator)
- 2026-10-20 — **#5481 merged** (A, `e8912770`, FULL). PR 1 of 4 for CPAIR's P1 `two-spans-projections-on-one-line-are-read-four-ways`. It adds `geom_core::one_line` (`OnOneLine`, `on_one_line`/`_2d`), read from a's span start and charged for the rounding of the projections and of how each end was produced (`produced`). `rounding_charge` and `SHORT_CHAIN_ULPS` move to `geom_core`, with 17 caller files updated. `curve_pair::pose::line_line` reads its gap from the home. The router gains no verdict: the first head's "far-origin gain" was an undercharge, so a touching pair read Apart 1 in 2000. FULL review B1 caught it with an exact-Q witness, now a row, plus a non-exact far-origin fuzz with a 1/16 tightness floor. Delta: 0 misreads in 4000 at each of 2^14, 2^20 and 2^24. The home went to geom-core rather than geom (M1), because `profile` depends on geom-core only. No golden moved. Filed `the-line-conic-router-reads-its-projections-from-the-lines-carrier-origin` (P3, witness first, likely closes). Next: PR 2, check 9 (RESTFRONT). (NURBS orchestrator)
- 2026-10-20 — **#5477 merged** (A, `6a61ea97`, FULL). Fix A of CPAIR's `curve-pair-polish-meters-refuse-shallow-meetings`: the plane-root window reads a touch's run "on" along the other conic (`Window::along_b`, a second-order bound with tangent-matched slope `k` and exact semi-axis bend constants, every term raised outward by the const-generic `up::<N>`). **The needle and `leaning_prisms` are green at ε 1e-12, so that base red is gone.** The meter and budget are unchanged. Review: ~740k fuzzed pieces with 0 violations, in f64, at Interval and end to end, and all claimed mutants caught. The fold-in added the half-width row (that mutant is now caught), capped the touch oracle's tolerance at z/8, and noted the uncharged f64 anchor and the skew-charge mutant (no cheap row). Process: two gate reds on `from_f64` and `w*w`, one fast-set overrun (the oracle joined the slow set beside its sibling), two `dirty` conflicts, and one semantic conflict with #5481's removed re-export. The Interval polish (item B) stays filed with `design: true`. (NURBS orchestrator)
- 2026-10-20 — **#5489 merged** (C, `5b6ac199`, FULL, TESS/CHORD/KNOT ground, seam notes posted). The crease straddler refusal: the soundness first step of TESS's lofted-circle item. The FULL review found it MERGEABLE: `grid.cert` is the only certifying read, crossing by one ulp refuses, and `+∞` ends in `CertificateExceeded` after at most 6 retries. The fold added the v-crease and touching-case rows (mutants M5 and M2 now caught), a constructor argument for creases, and one crease statement in geom-core. M3 (the wiring) is owed by the remedy PR. **#5492** (A, check 9) is under FULL review. **PR 3** (`profile::seg`) is held: reading the gap first removes the accidental run-level drift escalation that `PATHS-DESIGN.md:1440-1453` names as the data gate. I traced it to agent commit f4cd0eb3, not an Ev ratification. I ruled that a real run-level declared-ray check must replace it before gap-first lands; A is scoping that. (NURBS orchestrator)
- 2026-10-20 — **Friction (orchestrator):** a ruling sent at `next` priority to build A at 17:28Z was queued 24 minutes and reached A at 17:52Z, while A waited on it. That's the same latency seen with build C earlier. A ruling a lane is blocked on is effectively a red on the lane's own work: send it at `now`. (NURBS orchestrator)
- 2026-10-20 — **Design fork dispatched: a declared continuation run's certifier (PATHS).** PR 3 (`profile::seg` gap-first, held at `c389fd0f` on `nurbs/seg-one-line`) removes the accidental run-level "data gate" escalation that `PATHS-DESIGN.md` §4 names, so 20ε of drift across 40 continuations would pass silently. Build A's scoping found a real run-level check changes what a continuation declares, and picks a band and a refusal kind. So it is a fork on ratified text. The designer pair was dispatched at 17:55Z on the problem statement alone, protocol 26db1af89e, with blinding recorded on `analysis/design-fork/continuation-run`. The `[ev]` PR follows their reports. A meanwhile takes PR 4 (the census). (NURBS orchestrator)
- 2026-10-20 — **`[ev]` PR #5498 opened** (continuation run). The designer pair agreed on first reports (row 185, no reconciliation): `continue_to` checks against the run's carrier line, bound where the direction is bound and never re-anchored. There is no run-level certifier and no new arm. The PR edits `PATHS-DESIGN.md` §3's `continue_to` row and §4's data-gate paragraph, and files `work/paths/a-declared-continuation-run-reads-a-moving-reference` (`needs_ev: true`, with implementer notes). PR 3 stays held until Ev rules. (NURBS orchestrator)
- 2026-10-20 — **#5492 merged** (A, `ef1b70f1`, FULL, PR 2 of 4). Check 9 reads the one-line gap first, symmetrically, and its overlap closure reads the charged home, so the second check-9 copy is gone. The six far-plane brick booleans build (TALLY's `a-nested-brick…` and RESTFRONT's `check-9-lines-meet-reads-each-end…` are closed). The review's NOTE 1, a pre-existing wrong definite Meet, is filed as RESTFRONT P1. Seam notes are on RESTFRONT, TCOST, TINT and TALLY. Next: (c) tightens the home's charge to the spans' own scale (FULL), then PR 4 (census); PR 3 is held on `[ev]` #5498. (NURBS orchestrator)
- 2026-10-20 — **#5499 merged** (C, `16b6b72f`, FULL, TESS ground and many seams; notes posted, including QUAD's mass_properties door). Lofted circles mesh across C⁰ creases: the rational arm's gate relaxed at the mesher only, a CDT crease constraint, a shared `sub_arc_joint`, and chord counts rounded to multiples of m on `IsoArc` rims. Built on #5489's straddler refusal. Review fold: the chain row (the M2 mutant is now caught), the straddle row's honest scope, two rows in the slow set, and the refusal item's data, now P0 with design. (NURBS orchestrator)
- 2026-10-20 — **Design pass, `lofted-rational-walls-refuse-at-some-deltas` (TESS, P0):** both designers agree on their first reports. The cause is the trimmed lane's CDT, which runs Delaunay in raw UV on a strip of aspect ~1:31. Delaunay is not stretch-invariant, so any off-row point (seam chord, centroid) fans across ~12–15 columns, and the refinement ladder re-seeds the same circle. Both recommend triangulating in the lattice metric: one face-wide affine scale at CDT insertion only, with certificates, chords, creases and emission on unscaled UV. They differ only on the factor: a power of two from the whole-patch steps (exact in f64) versus the geometric mean of the band aspects. No ratified text is in the way (`TESS-BUDGET.md` is not in the companion table; the ratified aspect cap A = 16 is untouched), so it is not Ev's fork and there is no `[ev]` PR or fork-log row. **Ruled:** a power-of-two factor nearest the geometric mean of the realized band aspects. It is exact, so every exact-coincidence path survives, and the residual splits evenly. `is_malign` is made two-sided on the residual. Goldens re-baseline. Build C implements it after the S5 refactor. Filed off-question: `rational-pieces-force-rows` (P3) and `trimmed-cylinder-lane-triangulates-in-raw-angle-by-height` (P3, probe first). (NURBS orchestrator)
- 2026-10-20 — **#5504 merged** (A, `cbbe917a`, FULL): the one-line home's charge is at span scale, not origin scale, so correct km-scale bodies stop escalating at 1e-12 (for example sweep `face_face_touch`). The review fold added the scaling-pin row, refreshed stale numbers, and stated the axis contract at the doc. Seam notes on CPAIR and RESTFRONT. Next: A's PR 4 (census) on top; PR 3 (`seg`) stays held on `[ev]` #5498. (NURBS orchestrator)
- 2026-10-21 — **#5511 merged** (C, `5ad003f6`, STANDARD, orchestrator-reviewed: a pure refactor plus two comment fixes). `mesh::trimmed`'s retry loop body is split into named steps (`attempt`, `emit`). Bits are identical, and no golden moved. Seam note posted on TESS. Next on TESS ground: the lattice-metric CDT (FULL), which also carries an `unwrap_or`→`expect` at `trimmed.rs:453`. (NURBS orchestrator)
- 2026-10-21 — **Regression found (C): #5499 broke main's demo tour.** The Klein bottle's wall 8 pinned the C¹-gate refusal (`UnsupportedNurbsFace`) on the `circle`-lofted loop. #5499 opened that gate, so the loop meshes at δ = 1e-2 and `walls::wall` panics "NO LONGER REFUSES". #5499's change filter skipped the demos job, so nightly would have been the first to see it. Ruled: retire wall 8 in the same pattern as wall 7; `annulus` keeps `circle_split` while wall 5 stands. C's fix PR also says whether skipping demos on a mesh change is a filter gap, and if so files it. (NURBS orchestrator)
- 2026-10-21 — **#5519 merged** (C, `e7f2d17c`, STANDARD, orchestrator-reviewed). Main's tour is green again: Klein wall 8 is retired and kept live. Seam note posted on TESS. Owed by C: how #5499 passed CI without running the tour, filed as an issue. (NURBS orchestrator)
- 2026-10-21 — **#5513 FULL review** (covers 94a88dbe and the rework 6fda5aa6). B1 is fixed by the rework, and the 3 km union builds. Check 1's derivation holds: 0 non-adjacent misreads in ~37k exact-Q reads, and the threshold is ~15× conservative. **Ruled M1, fixed in this PR:** charged mode can never read Zero, so exactly coincident collinear ends escalate past ~5 km at 1e-12, a regression against main (a 6 km bar ∪ cube rotated 0.65 rad refuses `LineSpanPrecision`). Shared ends decide by point-key identity first; only unshared ends go through the charged straddle. **Ruled M2:** `SPAN_PRECISION_RECOURSE` names only its geometry levers and drops "a coarser tolerance" (D4 ¶1 (i)). The five MINORs are folded. A re-runs the workspace and slow-set suites on the new head (the reviewer's box ran out of disk). (NURBS orchestrator)
- 2026-10-21 — **Friction (orchestrator):** my #5513 BLOCKER ruling told A to say "coarser tolerance or shorter edges", citing `ARC_SCENE_RESOLUTION_RECOURSE` as precedent. D4 ¶1 (i) bars advising a looser ε where a geometry lever exists, and the reviewer caught it (M2). A precedent is one more thing to check against the ratified text before ruling on it. Filed the precedent: `work/paths/path-scene-resolution-recourses-advise-a-coarser-tolerance-beside-geometry-levers` (P3, provenance check first). (NURBS orchestrator)
- 2026-10-21 — **`[ev]` #5498, Ev's shape:** "can we just make them not drift? … keep the 'the only thing a section can know about the previous one is its directed endpoint' invariant by going through a directed point object which stores its data as a family of internal representations with a common api". Restated the PR to match (acaec79e). §2c gains the ray: a directed point holds its tangent as a ray behind one API (position, tangent, `lands_on_ray`); binders anchor it at the point, and straight continuations hand it on unchanged. §3 and §4 read it. The implementer notes drop the `carrier: Line | Arc` field verbs would read. Waiting for Ev to confirm. (NURBS orchestrator)
- 2026-10-21 — **#5521 merged** (A, `8f5d93e9`, STANDARD, self-reviewed plus orchestrator diff read): CPAIR's `the-line-conic-router-reads-its-projections-from-the-lines-carrier-origin` is closed, witness first, not a defect. Main's anchor is honest and cheaper (it charges |o| once against 2|o|), and the recipe would lose ~8% of Aparts at 1e3 m. One doc line at `line_conic`. Seam note on CPAIR. (NURBS orchestrator)
- 2026-10-21 — **#5498 merged** (`e2e353be`, Ev-ruled). `continue_to` checks against the run's one line. The directed point is one API (position, tangent, `lands_on_ray`) over a family of internal representations, no verb sees which, and the API is closed (no coincides, same-angle or parallel reading; Ev's check). PATHS-DESIGN §2c, §3 and §4 carry it, the fork row is renumbered **186** (RESTREAD took 185 on main), and the PATHS item is ruled. Build A's plan is approved: after #5513, a FULL PR (`.tangent()`/`.cusp()` inherit the run; the bool11 re-pin; alternating-sides, new-run and witness rows; the anchor mutant), then PR 3 on top. (NURBS orchestrator)
- 2026-10-21 — **#5531 opened** (C, FULL, `e307123f`): the lattice-metric CDT. It closes TESS's P0 `lofted-rational-walls-refuse-at-some-deltas`; the creased frustum, the cylinder and the 4/8-arc controls mesh at all 20 δ, where main shows the item's exact counts (9/20, 4/20). d9 re-baselined on 4 of 40 pairs; tour budget +1.3%. A FULL reviewer is dispatched (session_01K3B9kdvXU6LjdWbYg6e5K8). C's #5519 validation is confirmed on true main after a stale shared target dir was ruled out. C's CI-gap finding (the PR gate never walks the tour's wall probes: the filter skips demos for kernel diffs, and the `ci` profile omits the tour rows) is to fold into the existing `ciw/demos-row-skips-kernel-changes-to-doors-the-tour-calls`, raised to P2. **#5513** (A, `d478c0cb`) is under delta review. (NURBS orchestrator)
- 2026-10-21 — **#5536 opened** (A, FULL, `fe688f4b`): it builds #5498's ruling. A private `mod directed` holds an opaque `Run` inside `Incoming`, with `lands_on_ray` as the one reading. Straight continuations hand the run on, `.tangent()`/`.cusp()` inherit it, and every other binding starts fresh. bool11 is re-pinned (escalates by leg 3, OffRay at leg 21; red on main); alternating, new-run and tangent/cusp rows are added, and the anchor mutants are caught. A FULL reviewer is dispatched (session_01BWpVVCFdunkAXDNgVjMiG3). **#5513** (A): the delta review said MERGEABLE, and its MINORs are folded at `bc5aa496` and orchestrator-read. MINOR-1 reads stored ends through the vertex pass's own `pm_census_vv_gap`; that deviation from my "bit test" is accepted as one decision. Merging on green. (NURBS orchestrator)
- 2026-10-21 — **#5513 merged** (A, `9866210b`, FULL, plus a FULL delta review). PR 4 of 4 of CPAIR's `two-spans-projections-on-one-line-are-read-four-ways`: the census reads the one-line home. There are three review rounds:
  - The first review found B1 (blanket escalation past the threshold refused correct 3 km geometry). Re-ruled: charged straddles.
  - The delta review found M1 (exactly coincident ends escalated past ~5 km): ruled identity first. It also found M2 (recourse advised a coarser tolerance): geometry levers only.
  - The orchestrator-read fold covered MINOR-1 (stored ends through `pm_census_vv_gap`), MINOR-2, MINOR-3 and the NIT.
  Seam notes are posted on RESTREAD, CLEAVE, HONE, RESTFRONT, TCOST, TINT and GUARD. CPAIR's item stays open for PR 3 (`seg`), which lands after #5536. (NURBS orchestrator)
- 2026-10-21 — **#5536 merged** (A, `8c7db2ae`, FULL; review MERGEABLE, fold orchestrator-read). It builds Ev's #5498 ruling: `mod directed` holds `Run` (`across`/`along`, the one anchored-ray reading, also used by `PendingRunOut::rides`), `Bound` (a run held only beside its direction) and `Incoming`'s private run. `lands_on_ray` is the one reading; straight continuations, `line(len)`, `.tangent()` and `.cusp()` carry the run, and every binding starts fresh. Re-anchoring mutants A/B/C are each caught. The seam no-flip argument now states its K > 3 limit. The reviewer's D4 finding ("or widen the input tolerance" against PATHS-DESIGN §4) is on the PATHS recourse item: §4's sentence is agent text from f4cd0eb3, and D4 ¶1 (i) came later (7b494764), so D4 governs. Seam notes rode the PR. Next: PR 3 (`seg` gap-first) at FULL. **#5541 merged** (A, `1df22a3e`, STANDARD, tests-only, orchestrator-reviewed): LIB's base red closes. The tripwire is right: at 40 km a coordinate ulp (3.6–7.3e-12) already exceeds ε 1e-12, and the read-back's reach is ~2.8 km. The pncad row's fixture is ε-aware, bit-identical at default and 1e-6, and pins the 40 km refusal at 1e-12. (NURBS orchestrator)
- 2026-10-21 — **#5531 merged** (C, `845daf18`, FULL; review MERGEABLE, fold orchestrator-read). The lattice-metric CDT closes TESS's P0 `lofted-rational-walls-refuse-at-some-deltas`. The review diagnosed the moves correctly (the snap went dormant, 0/2573 bands, not "different diagonals") and found the lower malign arm unwitnessed, filed with a witness-or-retire-the-snap plan. Two reds were handled on the way:
  - the lily table, an expected re-baseline;
  - the editor-core pick fuzz, seed-dependent and red on main (filed `work/tint/pick-oracle-sweep-anti-vacuity-fails-one-seed-in-1300`, P2).
  The CI-gap finding is folded into `ciw/demos-row-skips-kernel-changes-to-doors-the-tour-calls` (now P2). Seam notes on TESS, CHORD, TCOST, TINT, BUDGET and INSTR. Friction: C's earlier "tour green" claims were from `cargo run` only, and the binary's own test rows hadn't run; C said so in the body. (NURBS orchestrator)
- 2026-10-21 — **#5542 FULL review: NOT MERGEABLE.** The gap-first reading is sound: 0 violations in a 2^14–2^24 m evaluated-end fuzz at 4 (ε, K) rows, and every mutant reds, including the reviewer's undercharge, swap and Zero-as-apart. The goldens table is confirmed. **Ruled, fix in this PR:**
  - BLOCKER: a merge conflict with main 222b097c; re-bless on the merged tree.
  - MAJOR 1: "line × line resolved" is false (it fails re-seamed, reversed, and with an end past an overlapping projection). `seg::line_line` reads the gap on s1's axis only and has no carrier short-circuit. Make it read the way check 9 does (symmetric `gaps_apart` plus the short-circuit), with the three witnesses as rows.
  - MAJOR 2: `seg`'s collinear_overlap arm still reads uncharged tc/td/lo/hi, so CPAIR's "all four readers on one home" is false. Route it through `one_line`, with bit-identity evidence; no wrong verdict was found in ~400k.
  The reviewer also found a tour base red on main (projectbox wall 1 NO LONGER REFUSES at ε 1e-6), filed as `work/quad/main-red-projectbox-wall-1-no-longer-refuses-at-eps-1e-6` (P1). (NURBS orchestrator)
- 2026-10-21 — **#5542 delta review (5a280b0f): NOT MERGEABLE, one MAJOR.** The check-9 order regresses `line_line`: `on_2`'s `Err` is propagated before the collinear arm, so near-collinear pairs (s2 within ε of s1's carrier) escalate where main read them, and a loop revisiting a vertex loses its `NonSimple` Touch. Ruled: collinear first, the witnesses as rows, `a_run_end…` re-pinned to the fixed (5,0), and check 9's identical hole fixed in this PR if it reproduces through topo validate, else filed on RESTFRONT. The reviewer also judged A's three "uncaught" mutants, and each wants a row:
  - second axis dropped: about 6.7k pairs only the s2-axis gap decides;
  - collinear charge dropped: caught by an exact Pythagorean-grid i128 oracle, 0/1.27M on head against 630 under the mutant; A's sweep skipped the band edges where the charge acts;
  - the s2-host arm dropped: caught by the "short B first" loop once collinear-first lands.
  A's arc PR is held behind it, since it's the same file. Friction: an implementer sweep that excludes the band edges cannot test a charge; that's the third time today (#5481 B1, #5513 m1, here). (NURBS orchestrator)
- 2026-10-21 — **#5548 merged** (C, `42af7c0b`, STANDARD, orchestrator-reviewed; demos only). The tour's 1e-6 row is green again: projectbox wall 1 is retired looser than the default ε and live-asserted against the closed form. The cause was a ~9x tighter enclosure, not a soundness defect. The QUAD item is closed, and the bisect for the responsible change is pending. (NURBS orchestrator)
- 2026-10-21 — **#5542 merged** (A, `fa901e43`, FULL, three review rounds). PR 3 of 4 for CPAIR's `two-spans-projections-on-one-line-are-read-four-ways`. `profile::seg::line_line` now reads two segments the way check 9 does: symmetric `gaps_apart` on `geom_core::one_line`, then a definite one-carrier separation, then the in-band escalation. The collinear arm is read **first** (round 3's fix: propagating `on_2`'s `Err` before it had regressed near-collinear pairs and a `NonSimple` Touch) and decided on `OnOneLine`. The 40-leg drift table now validates; the bool11 rows still refuse at the chain (#5536). Ten editor-core decision-count goldens moved and are re-blessed with causes; no node outcome changed. Every mutant reds:
  - second axis: a pair both ways round;
  - collinear charge: an exact Pythagorean-grid i128 contract sweep plus a hair-inside witness;
  - s2-host arm: the short-first loop;
  - order, gap, separation and gap charge.
  **Closes** CPAIR's four-ways item (all four readers on one home) and PATHS' `a-declared-continuation-run-reads-a-moving-reference`. **Filed:**
  - `paths/an-adjacent-pair-through-a-joint-straight-within-the-band-escalates-on-chord-side` (P2, pinned as an escalation row);
  - `restfront/check-9-lines-meet-escalates-an-in-band-side-before-its-collinear-arm` (P3, reachable only in a sliver, same verdict).
  Seam notes rode the PR (PATHS, TCOST, TINT, SCALAR, CPAIR, RESTFRONT). Next: A's arc PR (`line_arc`/`arc_arc`). (NURBS orchestrator)
- 2026-10-21 — **#5551 opened** (C, FULL, `0c418b8d`): retires the column snap. Over the lattice metric it fires on 0 tour bands. On 15 adversaries × 20 δ the outcomes are identical on and off, and the retired build is byte-identical to snap-off on all 300 rows. 0 of 4000 adversary rim-chord decisions reached the P0's configuration. The cost is up to 14% triangles on 14 adversary rows and 0 on the tour. It closes `the-lower-malign-arm-is-unwitnessed`, the P0 `rim-chords-exceed-snapped-column-count` and MESH-9. Files: cyl4 liveness (TESS P2), the cost lever (BUDGET P3) and the `snap_bands` column drop (BUDGET P3). FULL reviewer: session_011wW3D7F2MDv73FD7qm75Jv. Friction: two wrong SHAs from build C today (#5531, #5551), caught by ls-remote before dispatch. Always confirm a lane's SHA with `git ls-remote` before passing it on. (NURBS orchestrator)
- 2026-10-21 — **#5552 opened** (A, FULL, `7902d783`): line×arc and arc×arc spans read clear by radial certificates in `seg`, consulted only on the carrier's `Err`. The computed points are charged from the start (per the ruling). Static witnesses sit inside the arc read-back's reach, and mutants A–G all red. Workspace ci is 14095/14095 at default and 1e-12, with no golden moved. FULL reviewer: session_01EKosk4XGnkgNsZAZo3T6c5. Next for A: PATHS' recourse item (D4 ¶1 (i)), starting by confirming 7b494764's Ev provenance. **C's raw-chart probe:**
  - the trimmed-cylinder lane is red on main (`tilted_halves` R=5 20/20, R=20 19/20, R=100 7/20), and the lattice metric fixes it (80/80);
  - the curved lane never refuses, so it closes by measurement once dispatch's premise is cited;
  - cyl4 is the witnessed lower-arm hazard (band aspects 56.4/1.02/0.34, spread 166:1, the thick bands at residual 0.085). Its remedy is rows, not columns.
  Ruled: (A) the cylinder lane reuses `lattice_metric` verbatim (STANDARD); (B) row refinement (FULL) after #5551. (NURBS orchestrator)
- 2026-10-21 — **#5554 merged** (A, `305f6ea8`, STANDARD, orchestrator-read). It closes PATHS' recourse item: six recourses drop their tolerance lever, per D4 ¶1 (i) (Ev on [ev] #3352: "keeping 'loosen eps' as a last resort when there would otherwise be no recourse at all"). The sites are the two arc scene-resolution recourses (the read-back keeps its "possible kernel defect" clause), the three `ContinuationTargetOffRay` messages and the seam-arrival escalation. PATHS-DESIGN §4's unruled "WIDEN the input tolerance" sentence (agent text, f4cd0eb3) is restated. The concision ratchet tightens 91 → 85. Filed outside PATHS: SSIMARCH `step-budget-curvature-recourse-advises-loosening-beside-a-domain-lever` (P3) and REFS `…ambiguity-factor-k-count-as-loosening-under-d4` (P3, design). **A's adjacent-pair P2** is ruled (a), retitled "escalates at its joint, not its pair": it unmasks a real `NonSimple` crossing that main hid (witness `(0,0) (1,0) (1,1) (1.2,0.5) (−0.1,h)`). The coupling row found a sliver (8 of 24,716 draws, ~3·10⁵ m) where the rescue became a new acceptance; ruled pure relocation, with a typed escalation-arm guard and the sliver pinned as still escalating. **#5551 FULL review: NOT MERGEABLE** on MAJOR-1: the `CertificateExceeded` backstop is unguarded (the over-δ check can be removed with 398/398 still green). C is folding that plus MINORs, including the degree-3 cost of 1.616x, not "up to 14%". (NURBS orchestrator)
- 2026-10-21 — **#5551 merged** (C, `72efad7f`, FULL plus a delta review). The column snap is retired; it was dormant over the lattice metric and identical on and off across 28 adversary bodies × 20 δ. The FULL review's MAJOR-1 (the `CertificateExceeded` backstop was unguarded: the over-δ check could be removed with 398/398 green) is fixed by `an_uneven_station_cylinder_refuses_typed`, which the mutant reds. The cost is corrected to 14% at degree 2 and 1.616x at degree 3. The old P0 `rim-chords-exceed-snapped-column-count` and MESH-9 are closed as superseded. Seam notes on TESS, BUDGET, INSTR and CHORD. **A's elevation step 0:** the certified ring bound is unusable past degree ~4 (10⁸–10⁹⁵× pessimistic; it would refuse an arc chain 1 km out at 1e-12). The first-order estimate tracks within 10²–10³× and was never below the real error. Ruled: the estimate is the bound's form, worded "estimated". The refuse-vs-carry fork is parked on the item (P3, design: true) with the numbers. (NURBS orchestrator)
- 2026-10-21 — **#5560 merged** (C, `f7970082`, STANDARD, orchestrator-read). The cylinder lane takes `lattice_metric` on its one lattice's realized aspect, so the metric keeps one home. The witness `a_wide_tilted_cylinder_meshes_at_every_delta` is red on main and green here, and no golden moved (the corpus's cylinders all sit at metric 1). It closes TESS' cylinder raw-angle item, and the curved-lane item closes by measurement. The 24 editor-core reds were diffed and are base reds, identical with and without the change. Seam notes on TESS, TCOST and TINT. **#5552:** A merged main (`b40405e4`), and is filing the base reds and then running at two ε. Next for C: (B), rows for cyl4 (FULL). (NURBS orchestrator)
- 2026-10-21 — **#5552 merged** (A, `341dbcfb`, FULL review MERGEABLE, fold orchestrator-read). Line×arc and arc×arc read a charged radial certificate (`segment_clear_of_circle`, `arc_clear_of_circle`) only on the carrier's `Err`. Three new predicates charge the computed points: `circle_foot_past_segment`, `circle_extreme_past_arc` and `circle_end_clear`. The fold:
  - every sweep circle is redrawn inside the arcs' reach, with a foot-binding family and ≥ 5% fire floors, and the foot charge at a sixteenth now reds the segment sweep on 4 of 4 seeds;
  - the M12 row is added;
  - M9's chord-`Err` arm is removed as unreachable (0 of 2·10⁶ targeted draws reached it; removing it can only escalate);
  - the twins are left unfolded with a reason.
  Workspace ci is 14099/14099 at default and 1e-12, with no golden moved. PATHS' in-band-carriers item is narrowed to its `carrier_circles_identity` remainder. Filed: `paths/circle-side-reads-a-far-segment-ends-rim-uncharged` (P3), `paths/computed-points-read-uncharged-where-err-means-no-contact` (P3), and `tcost/main-reds-editor-core-slow-set-and-the-k-probe-sweep` (P2: 24 slow-set reds on main `3c342076`, plus the two `k_probe_sweep.sh` aborts). Seam notes rode the PR (PATHS, TCOST, TINT, SCALAR). ROUND co-owns `validate.rs` but has no `log.md`; the seam is three predicate endings and subjects, outside its fillet consts, and is announced in the PR body as ROUND's charter asks. Friction: A's shared target filled the disk on the second workspace run (the slow-set debug artifacts); pruning 8.3 GB of stale test binaries cleared it. Next for A: the adjacent-pair PR (STANDARD). (NURBS orchestrator)
- 2026-10-21 — **Triage, and the stopping rule.** Two open items had no priority:
  - `equal-split-points-insert-a-value-twice-on-a-narrow-span` is **P3, M**. The refusal is conservative, but its recourse blames the user's narrow span for the schedule's rounding. Only a face a few ulps wide reaches it. The repair moves two refusal arms to bug-only, with `#[should_panic]` re-pins, so it is not an E row.
  - `a-pair-walk-end-at-a-domain-corner-refuses-undecided` is **P4, H**. It is conservative, no current row reaches a corner end, and the overdetermined edge reading waits on a consumer.

  NURBS's own slate holds no P0 or P1 row. `parametric-polygon-loop-certifies-nothing` (P1) is parked on D10. Per the stopping rule (`memories/orchestration-model.md`, Ev 2026-10-19), this sitting finishes the two units already in flight on other programs' ground, where the built context makes them cheapest: A's adjacent-pair PR (PATHS P2) and C's (B), rows for cyl4 (TESS P2). Then it hands the track back as `ready`. The elevation fork (P3, design) stays parked, and the P3/P4 tail is the successor's. (NURBS orchestrator)
- 2026-10-21 — **#5566 merged** (A, `a4ec521b`, STANDARD, orchestrator-read). It closes PROFILE's (ex-PATHS) `an-adjacent-pair-through-a-joint-straight-within-the-band-escalates-on-chord-side`, retitled "escalates at its joint, not its pair" and re-rated P3, with ruled option (a): relocation only. `pair_contacts` takes the pair's shared stored vertex. `line_line` reads it only in its sides' `Err` arm, through `touches_only_near_the_shared_end` (the overlap's upper bound on `collinear_overlap`), and answers a Touch at the vertex. No loop that escalated now validates:
  - the coupling row has every rescue escalate at its joint (floor 10%);
  - the collinear straddle is pinned as still escalating, both orders;
  - the masked crossing `(0,0) (1,0) (1,1) (1.2,0.5) (−0.1,h)` now refuses `NonSimple { (1, 3) }`.
  Rows: an exact i128 counterexample sweep and a static charge witness (M4), with mutants M1–M6 all red. topo's check 9 has no adjacent pair (arm 1 reads a shared vertex first), so it gets one row and no code. The workspace is 14108/14108 at default and 1e-12, with no golden moved. The main merge re-homed its seam note to `work/profile/log.md` after PATHS closed. Build A stands down: NURBS has no P0/P1 left. (NURBS orchestrator)
- 2026-10-21 — **C's (B) comparison** (cyl4 rows): 25 bodies × 20 δ, falsifier armed. Without a remedy, 32 refusals, all cyl4. Rows only at A = √15 gives 0 refusals, worst ratio 0.4996, rounds ≤ 1.26x, degree 3 at 1.0x, and cyl4 1.18x at e = 0.03 up to a mean 8.2x at e = 1e-4. A = 8 leaves 15 refusals. An area-weighted metric over both arms cures it but costs 337x on cyl4. **Ruled:**
  - rows at the derived A = √15 (the sliver certificate (a² + 1)/8·δ_s ≤ δ at δ_s = δ/2), not a measured 5;
  - the cost is accepted (a mesh beats a refusal) and recorded in a BUDGET item.
  Once cyl4 meshes, no natural body refuses, so `an_uneven_station_cylinder_refuses_typed` loses its body. C proposes a `certifies(worst, delta)` helper with a unit row. Accepted, on one condition: the call-site mutant (`attempt` ignores the verdict) must red something, or a natural cap-exceeding body refuses typed. A test-only seam comes back to the orchestrator first. (NURBS orchestrator)
- 2026-10-21 — **(B) backstop, ruled.** The call-site mutant (`attempt` accepts regardless, `certifies` intact) survived everything: mesh 395/395 with the slow set, and budget 174 + 227 with the z1 falsifier armed. The falsifier checks triangles against their own certificates, not δ. No natural body refuses after (B), and refinement's retry cap is unreached. Ruled C's option 1: a `#[cfg(test)]`-only switch turns the row raise off, so the end-to-end row can mesh cyl4 at e = 0.01, δ = 2.6e-3 through `mesh::tessellate` and see the natural pre-(B) typed `CertificateExceeded`. Rejected: guarding by review alone (the mutant lives), and widening `band_schedule`'s API for a test. Also from C: the raised row count now goes through `sizing::ceil_count` (`ResolutionOverflow`) instead of a bare `ceil as usize`. (NURBS orchestrator)
- 2026-10-21 — **#5570 opened** (C, FULL, `690529f5`, then `624267f6`): row refinement for cyl4. **#5570 FULL review: NOT MERGEABLE** (reviewer session_01WFJhG7pCpcEDYtAJfbvj6T).
  - **MAJOR-1:** the √15 derivation assumes an even δ_s split between u and v. On cyl4 every band is cap-bound (mvv share 0), so the strip sliver bound is about a²/4·δ_s. That needs a² ≤ 8, and √15 stands at about 1.83δ, meshing by unnamed slack (sq 8, 15, 25 and 36 all mesh cyl4).
  - **Re-ruled:** the line moves to the derived √8 (MAX_RESIDUAL_SQ = 8), since my ruling 1's ground was "derived, not measured". The cost is about 1.15–1.34x √15 on cyl4, and none on the tour. The doc must derive it over every split, with a bound-level unit row pinning the line (threshold 9 and 16 mutants must red it). MINOR-1–4 to be folded.
  - The rest of the review held: the cfg(test) switch is sound, overflow per count is unreachable, the tour cost is 0 and the BUDGET table reproduces.
  Friction:
  - #5570 first went up red on the `interval-square-allowlist` gate (`residual * residual`): C's pre-push list had clippy and fmt but not `scripts/gates/*.sh`. C has added them.
  - I ruled "derived" on the implementer's derivation without checking its premise against the code's actual split. Check a derivation's premises before ruling on its strength.
  (NURBS orchestrator)
- 2026-10-21 — **#5570 merged** (C, `7369c4f1`, FULL review plus a fold read by the orchestrator at `624ec4b0`). The row raise sits at the derived line √8. The doc derives `¼(a²x + az + y) ≤ a²/4·δ_s` over every split (`muv` is a norm sup, so `z ≥ 0`), with the cap-bound split as the worst, and covers the two-off-row-vertex case. The one-vertex `(a + 1)/2` margin is named and not leaned on, and the ceil is the only slack named. A real-CDT strip-lattice row (worst 0.980δ at √8) and an arithmetic row pin the line, and threshold 9 and 16 mutants red both. The re-measure: 500/500 at worst ratio 0.4996, tour cost 0, cyl4 up to 14.3x (BUDGET). MINOR-1–4 are folded. No-raise and call-site are guarded only in the slow set (the coarsest unraised refusal is a full lofted-wall tessellation). It closes TESS's cyl4 item. Seam notes on TESS, CHORD, BUDGET, TCOST and TINT.

  **Hand-back.** NURBS has no dispatchable P0 or P1 row. `plan.md` is rewritten to the present slate and the program is `ready` (`memories/orchestration-model.md`, the stopping rule). Builds A and C and the reviewers stand down. The slate's next row is `equal-split-points-insert-a-value-twice-on-a-narrow-span` (P3, M). The elevation fork (P3, design) goes through the two-designer weighing before Ev. (NURBS orchestrator)

- 2026-10-26 — (tracker sweep) **KNOT merged in.** KNOT (cut from FLUX on 2026-10-10, nothing dispatched) held the net and fit doors on this track's `geom-core/src/spline/*` ground and its `geom` callers, so its rows moved here by `mv`, ids kept, closed ones included; `paths` gains `geom/src/curves/fit.rs`, `curves/nurbs.rs` and `surfaces/nurbs.rs`; KNOT's directory is deleted (`docs/doc-ledger/knot-leaves-the-tracker.md`). The merged slate is 31.5 points against 30; `plan.md` is rewritten to it.
  - **Closed:** `f64-refinement-inside-an-enclosure-has-five-more-sites`. Its last site, `mesh/src/chords.rs`'s `rational_carrier_m_bound`, now reads `accel_upper_bound` at `Interval` off the described carrier and refines nothing. ENCL's two rows parked on it (`one-pass-refinement-would-cut-the-rational-bounds-widening-tail`, `a-rigid-map-still-refuses-the-bowed-approx-fixture-at-eps-1e-12`) have had their trigger fire.
  - **Banded:** `implicit-composite-refuses-a-net-whose-weights-sit-near-the-bottom-of-f64` had no band or cost: P3 E (a sound refusal of valid extreme-weight geometry, fix shape written).
  - Citation re-aimed: `crates/topo/tests/projected_row_chart_box.rs` (`work/knot/` → `work/nurbs/`).
- 2026-10-26 — (easy-fixes lane) Closed `interpolate-with-params-passes-a-non-finite-point-through-as-nan`: `check_params` refuses `NonFinitePoint`, pinned on both explicit-parameter doors.
