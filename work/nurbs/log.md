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
