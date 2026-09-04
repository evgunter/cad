# EXCH-H1 — degree-1 line promotion and the ExtrudedPoint rung (spec)

**Program:** EXCH (`work/exch/plan.md`), unit `EXCH-H1`
(`work/exch/EXCH-H1.md`), closing
`work/exch/step-import-degree-one-line-promotion.md` (#388).
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **M**,
task-class **NUMERIC**.

- **M** — both halves have working precedents beside them (the circle
  limb for recognition, the LINE cap-rim arm for the rung); the labor
  is the cross-file flow and the pin re-derivations, not a new theory.
- **NUMERIC** — the risk-carrying part is a new certificate: a
  certified metre residual from a zero-radius cylinder composite plus
  a convexity-derived SEGMENT obligation. The rung's side/direction
  decisions are folds of existing `decide` doors.

## The claim

**A NURBS carrier that certifiably lies on a chord segment is a
`Curve3::Line`, and an imported line rim has an exact chart image on
its NURBS wall.** Both halves are missing today, and they must land
together because each is unwitnessable without the other:

- Recognition ships exactly one curve kind, the closed full-period
  circle (`crates/step-import/src/recognize_curve.rs`, `recognize` at
  :203). The line certificate is prose at :29-42: the zero-radius
  `ImplicitSurface::Cylinder` composite is exactly `dist(P, line)²`
  over the whole domain (`geom-core/src/spline/compose.rs:927-948`
  forms `r²` as `RingInterval::point(0).sqr()` — the zero interval,
  exact; no radius validation refuses 0), so `√sup` is a certified
  metre residual with no sampling. **No code computes it.**
- A promoted `Curve3::Line` changes the edge's adopted description to
  `MappedCurve::ExtrudedPoint` (non-`nurbs_rim`,
  `step-import/src/adopt.rs:788-792`) or `PlacedSegment` (:779-787),
  and `topo/src/pcurves.rs::nurbs_iso_derive` has NO arm for either —
  the catch-all at :854-859 refuses `IsoUnsupported`, and
  `mint_pcurves` propagates it (`pcurves.rs:1261-1266` swallows only
  `UnsupportedCarrier`), so the import refuses **strictly earlier
  than today** (the #327-measured result; adopt.rs:751-755 names this
  blocker in prose).

The measured gap this retires: dm1's edge `#389` — a two-point
degree-1 `QUASI_UNIFORM_CURVE` polyline — is offered **zero
candidates** at the coarse band, a GAP rather than a refusal, pinned
at `step-import/tests/r1_dm1_probe.rs:115-119`.

**This unit does NOT flip dm1 to a first-class import.** dm1 still
refuses on its rational wall's flux budget (#390 — route 1's dial is
#1315, route 2 is EXCH's unit 3). What flips here: line
self-descriptions for dm1's polyline carriers, the non-rational
pcurve re-mint for them (each line rim re-certified against its wall
through the new rung), exact `LINE(...)` re-export, and `#389` getting
a candidate.

**Territory, announced:** `topo/src/pcurves.rs` is TRIM's file; both
programs' `keep_out`s agree whichever dispatches first builds the
rung — EXCH dispatches first. This unit edits that file at exactly
one seam: the new arm and its tests. `geom-core/spline/compose*` is
S-CERT's glob and is **not edited at all** (the composite is consumed
as-is; the derivative channel is unit 2's row, filed with S-CERT).

## Phase 1 — measure before touching anything

`memories/refusal-text-is-not-cause.md` applies: the catch-all's
sentence is not evidence about what a rung needs.

1. **Reproduce the gap**: run `r1_dm1_probe` at the merge base;
   confirm the coarse-band cell reaches `#389` with an empty attempt
   list, green as pinned.
2. **Census dm1's degree-1 carriers.** The "37 polyline carriers" at
   recognize_curve.rs:32 is prose with no committed count. Off a
   throwaway harness (uncommitted, or committed only as the unit's
   census row), enumerate dm1's degree-1 / `.POLYLINE_FORM.` carriers
   and compute each one's zero-radius composite `√sup` and its
   control-point chord-projection excursions. The table (count,
   residual range, excursion range) goes in the PR body. If any
   genuine polyline carrier fails the certificate, that is a finding
   to report, never a tolerance to widen.
3. **Name the description discriminant.** Identify exactly which
   `EdgeDescription` shape `half_edge_description` yields for an
   adopted `ExtrudedPoint`/`PlacedSegment` edge, and confirm the new
   arm slots after every existing arm (displacing only the catch-all).
4. **Baseline the differential**: record the corpus' import behavior
   at the merge base with existing machinery (the wild.rs rows,
   tier_gate cells, and the STEP round-trip output for a body carrying
   a promotable line), so §Acceptance's "nothing else moved" claim is
   a diff, not an impression.

**Stop clauses.** Stop and report to the orchestrator, before
building, if: the zero-radius composite is not exact on a real
carrier (poison/NaN on the happy path); the rung cannot be one arm
(edits needed to `walk_loop`, `chart_mints`, or another program's
ground); or the segment obligation cannot be discharged from control
values by convexity (INV-C4's shape) without sampling.

## Phase 2 — the change

1. **The rung** (`topo/src/pcurves.rs`, one new arm in
   `nurbs_iso_derive`, placed last before the catch-all): for a
   description carrying `MappedCurve::ExtrudedPoint { .. }` or
   `PlacedSegment` over a line segment, evaluate the mapped curve's
   two endpoints and mint `Pcurve::IsoLine` by the LINE cap-rim arm's
   own machinery (:683-722): `side_pick` for the fixed channel, the
   u-direction pick's `decide` door, the measured-foot fallback under
   that arm's rules. Every choice through named `decide` doors like
   the sibling arms; no sampled quantity chooses. The catch-all's
   sentence (:855-858) is updated to name the widened vocabulary.
2. **The recognition limb** (`recognize_curve.rs`): a line limb in
   `recognize`, ordered so the circle path's behavior is untouched.
   Certificate: `composite_sup` with
   `ImplicitSurface::Cylinder { point: chord start, axis: chord,
   radius: 0.0 }`; residual `√sup` (NaN routes to the file's existing
   tri-state discipline). Segment obligation (INV-C4): the projection
   onto the chord direction is affine, so the control values bound it;
   an excursion `o` outside `[0, ℓ]` folds as
   `residual ≤ hypot(δ_line, o)`. The minted `Curve3::Line` must trim
   forward through `geometry.rs::endpoint_params`' Line arm (:34-54).
3. **Report plumbing**: `PromotedCurveKind::Line` beside `::Circle`
   (`step-import/src/lib.rs:396-399`), carried through
   `CurvePromotion`.
4. **Adoption**: expected NO `adopt.rs` change — the promoted line
   flows through the existing `mapped_self_description` arms and the
   ladder; Phase 1 confirms. If an edit is needed, it is disclosed as
   such, not smuggled.
5. **Pins and censuses**: `r1_dm1_probe`'s coarse arm re-pins the new
   fact with measured values (`#389` offers a candidate and adopts,
   or the true post-change state); the dead `LADDER_NO_DESCRIPTION`
   fragment (tier_gate.rs:356-357) is deleted only if the gap is
   truly retired; tier_gate's three dm1 cells re-measured (expected
   unchanged — rational-flux refusals at all three ε); wild.rs
   `WILD_IMPORTS`/`WILD_REFUSALS` expected 9/4 unchanged.
6. **Export witness**: no writer change (`writer.rs:262-270` already
   emits `LINE(...)` for `Curve3::Line`); add the round-trip row —
   a fixture whose degree-1 carrier promoted re-exports `LINE(...)`
   where the merge base wrote `B_SPLINE_CURVE_WITH_KNOTS`.

## Constraints, binding

- **Everything that imports at the merge base imports bit-identically
  at the head, except the named flips** (self-descriptions, re-mints,
  `LINE` re-export, `#389`'s candidate) — each flip listed in the PR
  with its cause. Any unnamed moved bit is a finding, not a
  re-baseline.
- **No edit under `crates/geom-core/src/spline/`** (S-CERT's glob,
  Track N's fence). **`topo/src/pcurves.rs`: the rung arm, the
  catch-all sentence, and tests — nothing else** (TRIM's file, by
  announced seam).
- **No sampled normal or point decides anything**; samples may
  describe (payloads, fallback feet under the existing arm's rules).
- ε discipline: the certificate reads `eps_in` like the circle limb;
  no new ambient reads.
- The negative control stays green: `TAIL_TURBINE`'s freeform splines
  stay NURBS (wild.rs:102-107; `recognize_curve` pin C3).
- Comments state the invariant (discipline §4); the history of the
  gap is the PR body's.
- **No `Co-Authored-By` trailer in lane commits** (blinding overrides
  the harness convention); if one lands in a pushed commit, note it in
  the PR body and carry on — never rewrite history, never stop the
  unit over it.

## Acceptance

- The Phase 1 census table, from the merge base, in the PR body.
- The rung's rows: an `ExtrudedPoint` rim on a NURBS chart mints an
  exact `IsoLine` chart image; the mutant table shows side, direction,
  and segment-obligation mutants red on the new rows and green
  everywhere else.
- `#389` has a candidate at the coarse band; the re-pinned probe row
  states the measured numbers.
- The round-trip `LINE(...)` row green; the corpus differential clean
  except the named flips.
- tier_gate and wild counts unchanged, or every change named and
  argued in the PR.
- Hosted CI green at the drawn point; if the drawn point does not
  exercise the coarse-band gap, ask for the point that does
  (`CI-Config` trailer or workflow input) and say in the PR which
  gated, drawn or asked (discipline §2).

## Out of scope

Open arcs, ellipse, helix, and the derivative channel (unit 2,
`step-import-curve-recognition-named-exclusions`); the rational
patch-flux budget and the cylinder certificate (#390 — the #1315 dial
is S-CERT's fence, route 2 is unit 3); any `spline::compose` edit;
`nurbs_iso_derive` arms beyond the rung (the trimmed-NURBS lane is
TRIM's); the STEP/STL option surface (EXCH's D items).

## Re-scope at Phase 1 (2026-09-04, orchestrator)

Phase 1 executed as bound and its stop clause 2 fired; this section
records the orchestrator's ruling, so reviewers read it as settled —
they falsify the built limb and the rows, they do not re-open where
the rung lives.

**Measured corrections to §The claim** (each by execution on the
lane, differential against merge base):

1. `nurbs_iso_derive` needs **no new arm**: its Chart/Scaffold arms
   are carrier-keyed and the LINE cap-rim arm (:683) already covers
   line rims; the catch-all never fires in any measured
   configuration. Phase 2 item 1 is measured unnecessary and is NOT
   built.
2. The true #327-measured blocker is
   `geom-brep/src/pcurve_cache.rs::run_iso_checks` check 4: the SEAM
   class has no `Curve3::Line` carrier limb. Without it, the
   recognition limb plus the adoption candidate red exactly four
   rows with one cause — including a REGRESSION of a native rational
   arc-prism export→import round trip that is first-class at the
   merge base (`tcost_k3_import_certificate`), i.e. the class is
   every exported loft/prism with straight ruling seams.
3. The gap's mechanism: dm1's six matching slit carriers adopt via
   the bitwise IsoCurve rung today; `#389` alone is
   control-order-REVERSED against its wall column. The adopt.rs
   Line-column candidate (Phase 2 item 4's disclosed contingency,
   now in scope) hands it a candidate.
4. At the merge base the `#389` gap is MASKED at every band by the
   rational-flux refusal (the probe's Adoption arm is dead code), so
   Phase 1 step 1's expectation was stale; the masked state is the
   baseline truth.

**The ruling**: the announced TRIM seam extends to exactly one limb —
the seam-class `Curve3::Line` carrier limb in `run_iso_checks`
(`geom-brep/src/pcurve_cache.rs`), the shape the lane's reverted
prototype measured (~35 lines; Greville linear-precision hull
transposed from the cap class, unit-weight gate, rational columns
refuse typed like the cap class). Both keep_outs' rule — whichever
program dispatches first builds the pcurve-lane rung EXCH consumes —
covers this limb in spirit; TRIM has no live orchestrator until
CURVED's exit, and the announcement is this section plus the program
log plus the unit PR. Track Q's rows riding that file are not
absorbed and not edited. The binding constraint tightens accordingly:
**in `pcurve_cache.rs`, the one limb and its tests — nothing else.**

Pre-draw fields unchanged (M / NUMERIC), re-logged with this ruling
in `work/exch/log.md`. Acceptance gains, replacing the stale step-1
expectation: the four named rows green at head; dm1's refusal
bit-identical to the merge base at 1e-9 (same solid, same face, same
width); the arc-prism round trip stays first-class; `#389` holds a
candidate through its reversed column.

## Review

v6 dual on the frozen head, claims to falsify (reviewers get these
verbatim plus `docs/prompts/reviewer-style-lane.md` by path):

- **C1** The differential is complete: everything that imported at
  the merge base imports bit-identically except the flips the PR
  names, and the flip list is exhaustive (re-run it).
- **C2** The certificate is sound: `√sup` of the zero-radius
  composite bounds the true max distance to the minted line, and the
  segment obligation follows from control-value convexity — attack
  with a carrier lying on the line that doubles back, and with
  control excursions beyond the chord.
- **C3** The rung's decisions are real: the side/direction/segment
  mutants red exactly the new rows; no sampled quantity chooses.
- **C4** Negative controls hold: `TAIL_TURBINE` stays NURBS; a
  genuinely curved carrier never promotes to `Line` at any tested ε.
- **C5** The re-pinned dm1 rows state facts (re-measure `#389`'s
  adoption; the deleted gap fragment's retirement is justified by
  execution, not by the diff).
- **C6** The export round-trip is real — executed, and the merge-base
  half of the differential actually ran at the merge base.
