# OFFSET — the log

## 2026-09-20 — opened

Cut out of SHELL, which was carrying 65.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed SHELL's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

5 rows arrived by `git mv` with their ids, bodies and history
unchanged. SHELL keeps its band 2300-2399; band 8100-8199 is claimed
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

**Your ground:** `crates/geom-brep/src/offset_meters.rs`, which
territory names as ENCL'"'"'s, OFFSET'"'"'s and SHELL'"'"'s together.
`MeterError::NormalFloor` and `CurvatureHeadroom` gained repairs — the
floor-of-exactly-zero case now says to split the face clear of the
degeneracy, and points at `OFFSET_METER_LADDER` for the regular-patch
case, on that constant'"'"'s own sentence that the refusal names the
numbers "so that consumer will know". `Escalated` is unchanged in shape
and asserted transitively at all three `MarginDiag` arms.

Signed (FIX orchestrator).

- 2026-09-28 — Seam note from ENCL: PR 3332 (`encl/rigid-map-approx-headroom`) edits `crates/topo/src/transform.rs`'s `map_approx`. When a rotated Approx face's limb check refuses and the original face certifies in its own frame, `map_approx` now re-fits the mapped description through `OffsetFitLane::mint` at the same ε instead of refusing. Every map that succeeded before still ships the image bit for bit. Residues filed on encl: the 1e-12 re-fit stall, and the edge/meter refusals a re-fit cannot answer. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3351 (merged `e39a5c4cc4`) routes certification refusals per D4 ¶1 as Ev ruled on PR 3352. Recourse belongs to the decision (`CertCheck::ending()`, one table) and the reading belongs to the door: `geom_brep::certify::recourse(check, RefusedArm, Reading::{Build, AtRest, Adopt})`. `CertifyError`/`PlaneNurbsRefusal` `Display` is now payload-only. Each door appends `ending(reading)`. The `certification: ` prefix is gone. `transform.rs`: `TransformError::Certify` appends the Build-reading ending. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". `offset_meters.rs`: `Meter::{NormalFloor, CurvatureHeadroom}`; `MeterError` carries `Refused::{Zero(Classified), Negative{margin}}` and renders its ending per reading; "or lower the tolerance" and the name routing are gone. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3392 (merged `277dcb052b`) splits certify's conflated Zero/Negative verdicts: `IntervalNotForward { verdict }`, `WindingExceeded` routed as sign-certain, the tangent tube as its own `CertifyError::TubeNotSeparated` (`CertCheck::TangentTube`, lever alone at every reading), and `TubeStraddles { verdict: Refused }`. `RefusedArm::ZeroOrNegative` is deleted; `geom_brep::recourse` now holds `Refused` and `Definite`. A zero span stays a defect at every reading (Ev, e1600790f9). `offset_meters.rs`: `Refused` moved to `geom_brep::recourse` (import path only). (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/offset_axial.rs`. `offset_axial.rs` states each re-charted face's own bit (the door kept it before). (TOPO implementer)
- 2026-10-01: Seam note from SSI. PR 3653 routed `offset_meters` `cell_normal`'s `dn` through `geom_core::interval::norm_sup` (and thinness and the regularity margin through `div_down`), so `offset-meters-cell-normal-midpoint-direction-is-an-f64-fold` looks closed in substance; there is a note on the row. (SSI orchestrator)
- 2026-10-06 — Note from SHELL's triage: `offset-lane-has-no-conic-carrier` carries no `design` flag, but its fix moves C5 R1's ratified "outside the conic inventory by decision" (`geom-brep/src/intersect.rs`), so it is a fork for Ev and reads as dispatchable when it is not. SHELF's parked `partial-cone-frustum-three-quarter-turn-refuses-edge-disagreement` waits on it. Also: SHELF now holds `offset-doors-take-solids-and-a-per-chart-rule`, which edits the same doors as your `doors-still-read-the-whole-body-for-tier1` — whichever dispatches second coordinates. (SHELL orchestrator)
- 2026-10-06 — Seam announcement from SHELL: unit 6 (`shell/axial-closed`, in flight) widens `offset_axial`'s gate to admit every plane parallel to the axis at any stand-off, so the together door is closed under its own output. `classify` takes one surface, `Constraint::Meridian` is renamed, and rim carriers derive from the moved pair. Bodies such as a cylinder void in a box, or keyed and flatted shafts, will newly take the together door. The decision and its spec basis are in `work/shell/shell-open-lift-takes-the-per-chart-door-on-the-klein-elbow.md` ("Decided"). Rows of yours that assert a door choice may re-baseline. (SHELL orchestrator)
- 2026-10-08 — Seam note from ENCL (`encl/offset-cert-coefficient-norms`): `crates/topo/src/transform.rs`'s module docs and `map_approx`'s doc now say that an `Approx` face's `hull_sup` reads its vector upper bounds from coefficient norms and that what still moves under a rotation is its box-assembled LOWER bounds (the regularity floor and the floors on `‖E‖`); docs only, no code. The `offset_meters` `PatchRegularity` speeds now read `PatchCell::s_u_sup`/`s_v_sup`, coefficient norms on the integral arm. `offset-meters-cell-normal-midpoint-direction-is-an-f64-fold`'s citation of the retired in-module counter row is updated. (ENCL implementer)
- 2026-10-08 — Seam note from ENCL (PR 4348, `encl/offset-cert-coefficient-norms`, merged): the offset certificate's vector upper bounds (‖Y‖ via `Composite::y_sup`, `m_tilde_sup`, and the integral arm's chart speeds via `PatchCell::s_u_sup`/`s_v_sup`) now read coefficient norms (D4 ¶2), not per-coordinate boxes. geom-core gains public `tensor::coefficient_norm_bound` (`[&[Interval]; 3]`, refuses ragged rows), `coefficient_norm_sup` and `PatchSpans::cell_norm_sup`. The bounds are tighter or equal, and the stored offset numbers were re-baselined. The three `rigid_map_near_eps_approx` rows moved to the slow set, and their witness now mints on round 2 at `MARGIN = 1/4096`. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4367, merged): `offset_fit.rs`/`offset_meters.rs` folds that feed guards now use `geom_core::interval::max_bound`/`min_bound`, so a NaN reaches its guard. `PatchRegularity::sup` and `CellNormal::sup` are NaN on a refused cell (previously the last cell only). `Composite::cell_terms` is the one home of `cell_bound`'s guards. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4377, merged): `offset_meters::patch_collapse` reads an unbounded κ⁺ (unreadable curvature) as `reach = NaN`, so `offset_curvature_headroom` escalates on an invalid margin rather than refusing sign-certain at −5e-324 m, on every route. New: `MeterError::ending_with_lever`/`render_with_lever` and `OffsetFitError::render_with_lever` (`METER_UNFITTED`). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4395, merged): a limb refusing at the offset fit's mint is now `OffsetFitError::MintLimb`, ending in the kernel-defect ending; the at-rest `Limb` keeps "re-fit" (`geom_brep::offset_fit::LIMB_REFIT_RECOURSE`). The shell and the transform re-fit both reach `MintLimb`. (ENCL orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4687, merged at `aaf011d01b`): `offset_meters::CellNormal::sup` is gone; readers take `PatchCell::m_sup`, a certified coefficient-norm bound on ‖S_u×S_v‖ (with the rational quotient terms) that is rigid-invariant to rounding. The rational chart speeds are coefficient-vector norms too. At rest the bounds are up to ~1.8% looser than the box in a frame that suits it. A NaN `m_sup` now refuses where the box/Gram fallback used to answer. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4800, merged at `428057bb7c`): `OffsetFitError::SampleCapReached` is now `WallReached { wall: OffsetFitWall::{SampleCap{cap} | Work{budget, spent}} }`, with "split the face" as the recourse for both. A new cumulative `OFFSET_FIT_WORK` = 16 384 samples is measured against the corpus's largest run (8851) and guarded by `a_bound_found_then_lost_carries_the_bound_it_lost`. `RefinementStalled` fires on two idle rounds against the lowest bound; a both-directions fallback marking is not an idle round, so one sequence that refused on main now certifies (pinned). No corpus row flipped. (ENCL orchestrator)
- 2026-10-12 — Seam note from SYM: SYM-17 Phase 2 (Ev's answer on PR 4902; `docs/SYM-17-SPEC.md` §Phase 2) replaces `OrthoFrame` with `RigidMotion<T>`, a proper rigid motion carried by construction. `topo::transform_rigid` will take it and ask nothing about rigidity; `check_rigid` becomes its one decided door, for raw maps. The type acts on `UnitVec3`, and pose inverses become transposes. Branch `sym/17-rotation-readout`. Touches linalg, `transform.rs`, `eval/wire`, placement, the mate solve's representatives and inverses (not `member_of` or `check_offsets`, which stage 3 retires), and the revolve, sweep and pattern rigid maps.
- 2026-10-13 — **Seam (ENCL):** PR 4976 merged. Every geom-brep Zero-arm payload (`CertifyError`, `TorusConvention::refused`, `MeterError` NormalFloor/CurvatureHeadroom) now reads "is not shown … (margin {:e} m)". Whether a reading shows no size is left to the ending (`MarginDiag::shows`). A row PR 4976 filed on this slate asks for the same idiom on this ground. (ENCL orchestrator)

- 2026-10-13 — **Seam note from NURBS: PR 4984 merged** (`2639b944`), unit 2 of the quotient ruling.
  - **Second order on the quotient form.** `C″`, `S_uu`, `S_uv` and `S_vv` now read the one Bernstein quotient bound over `W³`. Entry points: `geom_core::spline::{accel_pieces, cell_second_quotients, polynomial_area_pieces}` and `NurbsCurve3::accel_upper_bound`.
  - **Moved onto it:**
    - `geom-brep` `patch_bound` `rational_cells` (first- and second-order boxes, `m_sup`). The refinement and `coordinate_hull` are gone.
    - `mesh` chords' rational `sup|C″|`.
    - `offset_meters` `cell_normal` / `patch_regularity`, which now reads the area floor `PatchCell::m_inf`.
  - **Integral arm:** the area floor reads `A_u×A_v` at `2p−1` and certifies to degree 28. Past that it uses a conservative floor of 0.0.
  - **Refusals:**
    - a rational degree above 18 gives `PatchBoundError::DegreeAboveCeiling`, a typed recourse;
    - chords past the ceiling give `UnsupportedCurve`;
    - `QuotientRefused` is new (kernel defect);
    - `RefinedWeightLostPositivity` is gone;
    - `RefinementFailed` now means the integral insertion chain only.
  - **Mesh consequence:** a rational face above degree 18 now refuses `UnsupportedNurbsFace` in nurbs_cert, and offset refuses through `MeterError`.
  - **Goldens re-baselined, with causes in the PR body:** `m_sup`, chord counts, offset goldens, the budget and endpoint censuses, and meter ceilings.
  - **Perf:** the per-face bound is about 2× base on small rows and up to 11× on wide rational faces. Filed at `work/nurbs/second-order-quotient-pieces-cost-the-face-bound-eleven-times-main.md`. — (NURBS orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5023 merged. At a build, a refused reading past the band now ends like its in-band sibling (D4 ¶1 (iv)): the lever plus the decision's `no_size` note. geom-core `MarginDiag::shows` gains `Shows::Refused`. Touches `offset_meters.rs`: NormalFloor Negative at a build now ends in the lever plus the report note. (ENCL orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5101 merged. At rest, one definite refused reading ends one way whichever arm routes it: `StoredDefinite::Lever` gives the lever plus the note; `Contradiction` gives `KERNEL_OR_FILE_DEFECT_ENDING`, at rest and at the door past ε_in. `Meter::NormalFloor`'s sign-certain arm at rest now ends in the lever plus the note. (ENCL orchestrator)

## 2026-10-24 — the `rewritten` set (OFFSET orchestrator)

PR 5651 (`offset/transform-rewritten-secondary-map`) closes
`topo-transform-rewritten-is-a-hashset-the-crate-doc-argues-against` and
the riding `transform-corrupt-renders-an-orphan-entry-as-a-missing-reference`.
Tier: orchestrator's read (mechanical). `rewritten` is a `SecondaryMap`;
the sweep found a second production `HashMap` the crate doc never named
(`wedge_reach.rs`'s `WedgeReach::points`) plus three in tests, all
converted, so `lib.rs`'s D9 paragraph now states the rule with no
exception. `TransformError::Corrupt` carries a whole clause (`found`), so
the orphan entry reads as unreferenced; a test pins it. The first CI run
missed an `E::Corrupt` alias in `editor-core`'s concision test; the lane's
sweep grepped the qualified name only, which is the blind spot it now
reports. The charter's sentences on the closed rows are rewritten.
## 2026-10-24 — picked up (OFFSET orchestrator)

Set `active`. Triage at `c4318c38`:
- `offset-meters-cell-normal-midpoint-direction-is-an-f64-fold` closed
  as overtaken: assembly B no longer exists (NURBS PR 4984, ENCL PR
  4687), as SSI's 2026-10-01 note foresaw.
- Priced the four unpriced rows: `cell-curvature-…` P2 E,
  `cone-offset-displacement-…` P2 M, `offset-axial-re-authored-arc-sweep-…`
  P2 E (a turn past a half is re-authored as its complement),
  `offset-torus-ring-refusal-…` P3 E.
- `doors-still-read-the-whole-body-for-tier1` re-priced from legacy D
  to H with `design: true`; `offset-lane-has-no-conic-carrier` gets the
  `design` flag SHELL's 2026-10-06 note said it lacked.
- Overlap with INTENT checked (Ev asked for the sitting to favour rows
  INTENT will not make irrelevant): none of this slate is on D10's
  ground; `plan.md` says why.
- Plan: designer pairs on the P0 and on the doors row, concurrently;
  an implementer on the `rewritten` set with the orphan-render row.

## 2026-10-24 — two forks weighed (OFFSET orchestrator)

**Conic carrier (P0).** Both designers recommend exact
`Curve3::Hyperbola` and `Curve3::Parabola` kinds, plane×cone total at
rung 2, the axial door taking the moved pair's own section, and C1's R1
sentence ("outside the inventory by decision") replaced by the
closed-form principle. Both traced R1 to M5's scheduling fence, hardened
by later agent text; Ev's OQ1 and spiric rulings point the other way,
#908 the one note for fitted NURBS. Both corrected the framing: the gap
is the conic set's, not the offset lane's (booleans' `GermSectionOutsideInventory`,
the general offset door, draft, STEP import). They agreed on the first
round; `[ev]` PR 5652 carries it (fork-log row 193). Off-question from
both: STEP import has no `HYPERBOLA`/`PARABOLA` arm; `offset_general` and
`offset_axial` describe the same edge differently. Both ride the build if
Ev says yes.

**Doors' whole-body tier 1 (P1).** Both recommend against a scoped tier
1. Agreement on the final state: one tier-1 derivation per staging door
end (`closed_by_tier`, both contracts from one walk), and scope-driven
walks with SHELF's solids unit. One designer additionally measured a
large single-solid offset as quadratic in its scope (237 ms at a 1024-gon,
tier 1 under 1%), from the attach setter's per-key orphan scans, and
`shell_open`'s per-solid staging as quadratic in solids. Not a fork for
Ev (no ratified decision moves), so decided here: the row closes, re-cut
into `a-staging-doors-end-derives-tier1-once-and-its-orphan-sweep-is-batched`
(P1 M, this slate), a note on SHELF's solids unit, and SHELL's
`shell-open-stages-a-door-per-solid` (P2 M). The other designer would have
banded the remainder P3; the quadratic is why the unit stays P1. Not a
design-fork row: the population is forks that go to Ev.

## 2026-10-24 — one tier-1 walk per offset door end (OFFSET orchestrator)

PR 5661 (`offset/door-end-tier1-once`) closes
`a-staging-doors-end-derives-tier1-once-and-its-orphan-sweep-is-batched`.
Tier: single FULL review (the panic/typed split of a gate merged into one
walk). Verdict MERGEABLE-WITH-FIXES, no MAJOR. The reviewer's executed
differentials: `close_gated` panics at depth 1 and returns typed at depth
2; the batched reaper against the per-key sequence over the whole topo lib
suite (zero diffs, free-list order included) and a 3000-trial fuzz (one
divergence, the claimed dangling-surface fix); the orphan counter goes red
with the per-key calls restored.

- `Surgery::close_gated` derives tier 1 once via `closed_by_tier`, through
  one postcondition helper shared with the sweeping closes (the first draft
  spelled the assertion twice; the reviewer caught it). "Once per door end"
  holds for the three simultaneous offset doors; merge, the boolean,
  `shell`, the split and STEP import still derive twice
  (`staging-doors-gated-at-tier-3-still-derive-tier-1-twice`, P3).
- The orphan sweep is one batched pass; the setter's edge walks follow the
  moved faces. Two deterministic counters pin both halves (each shown to
  bite). Every face of an n-gon prism offset: 224 → 49 ms at n = 1024
  (release, debug assertions on). `WedgeReach` keeps it superlinear
  (`wedge-reach-folds-a-faces-whole-boundary-per-edge`).
- The merge door's run is one scope now; a tear in one group followed by a
  typed refusal in a later group comes back typed, stated at the loop as
  the accepted cost.
- Filed by the lane: `an-offset-doors-trailing-join-sweeps-tier-1-per-kill`
  (P2 E), the two above, SHELF's
  `offset-charts-staged-corner-solve-walks-the-edge-arena-per-group`, and
  TOPO's `set-edge-curve-scans-the-edge-arena-per-call`. Twin corner-solve
  setup in `offset_together.rs`/`offset_axial.rs` noted on SHELF's
  one-door row.
- Main stopped compiling mid-unit (CHART #5656 + #5657, a missing
  `walk_refusal` arm); CHART's #5662 one-line fix was ported onto the
  branch.
- Process: the lane's worktree and the main checkout both had the branch
  checked out, so the lane's commit moved the main checkout's HEAD under a
  stale index that would have reverted the fix pass. Do not check out a
  lane's branch in the main checkout while the lane works on it.

## 2026-10-24 — a join sweeps tier 1 once (OFFSET orchestrator)

PR 5666 (`offset/trailing-join-one-sweep`) closes
`an-offset-doors-trailing-join-sweeps-tier-1-per-kill`, the follow-up the
tier-1-once unit's own trace found. Tier: orchestrator's read (a scope
around an existing loop). The fix sits in the join, not in each door:
`Body::join_edges_within` and the public `Body::join_edges` run their kills
under one surgery scope closed by `sweep_and_close`, which covers the four
offset doors, `shell`, the blend, the merge, the split's halves and STEP
import. Pinned by `validate::TIER1_DERIVATIONS` (`cfg(test)`): constant in
k, where the reverted code paid about 2k. Filed by the lane:
`offset-surfaces-together-leaves-joinable-vertices-at-rest` (P2 E,
pre-existing: `offset_charts_staged` offers the join only the rewritten
edges' ends). Correction to the 2026-10-24 tier-1-once entry: the lane
banded `staging-doors-gated-at-tier-3-still-derive-tier-1-twice` P2, not P3.
Friction: the box's disk was nearly full from finished lanes' target dirs
(about 25 GB of OFFSET's own), now removed.

## 2026-10-24 — the conic fork, rounds 2–4 (OFFSET orchestrator)

Ev asked on PR 5652 why exact conics are not NURBS, then: "we're going to
have to make nurbs usable at some point, and so i don't know what we gain
from having a zoo of rarely encountered closed-form curves". Round 2,
each designer shown that verbatim: both dropped the kinds. Rounds 3 and 4
crossed twice on one point, whether the exact arc's certificate follows
the carrier's kind (the full NURBS certificate, waiting on the cone's
metres enclosure; [one-way.readings]) or its class read from its data (the
sampled check with a written bound; today's closed-form check rests on
provenance inferred from the variant). Converged on everything else: no
kinds, an exact rational-quadratic arc in the NURBS container minted in
closed form, edge identity by description, the axial door reading the
moved pair's section and refusing at its gate meanwhile. The PR now states
that in C1 and puts the one split to Ev (row 193 updated). The hex-nut
boolean waits on a NURBS crossing layer either way.

Filed: CHARTTAIL's `the-cones-rung3-certificate-has-no-metres-enclosure`
(P1 M, the shared cone blocker, no item named it) and SELECT's
`closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them`
(P2 M, both designers' finding; its fix follows Ev's answer).

## 2026-10-25 — the conic fork closes; the build is cut (OFFSET orchestrator)

Ev approved PR 5652 (merged `6e9df01639`): *"i'm still not totally sure
about having special cases for the parabola and hyperbola, but sure, we
can run with it"*. Round 5 had converged after Ev asked what the class
rule would do: B withdrew it and stated a pair-based rule; A adopted it
with an amendment for soundness (transversality over the span, clearance
from near misses); B accepted. On Ev's last question both conceded the
tube exemption is the part resting on the classification and is
severable. Row 193 filled (A = Opus, B = Fable; match: neither first
report). Built as three units (plan.md), the exemption last and alone so
Ev's reservation can drop it cheaply. SELECT's
`closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them`
is claimed as unit 1 (moved here).

## 2026-10-25 — conic unit 1 stops on DL1; unit 2 un-parks (OFFSET orchestrator)

Unit 1's lane (`offset/limb2-for-every-carrier`, head `452e7d8757`, no PR)
stopped before opening a PR, rightly. Limb 2 for every closed-form carrier
needs the `NurbsLane`. `AtRestPolicy::nurbs_lane` is `None` at `Dual64`,
per DUAL-DESIGN DL1 ("a Dual is tangent transport and never certifies",
ratified #1146), so 39 of 63 dual rows refuse. It also measured the cost:
a full circle's limb 2 takes 5.7 ms, a spiric 8–17 ms, and 13 curved-boolean
rows run 12× slower. Ev's new C2 ("limbs 1–2 owed by every carrier") and
DL1 conflict at the dual. That is a design fork, so unit 1 parks on the
lane's fork row and the row goes to designers. The lane also filed the
cost row and the spiric-net row (the net is tight enough; long ovals halve
to clear).

My error: unit 2 was parked on unit 1, but the arc is a NURBS carrier and
already owes limb 2 through the existing path. Unit 2 is un-parked and
dispatched.

## 2026-10-25 — conic unit 2 merges (OFFSET orchestrator)

PR 5692 closes `the-plane-cone-arc-is-minted-exactly-and-the-axial-door-reads-the-moved-section`.
The plane×cone parabola and hyperbola are exact rational-quadratic arcs.
The old "by decision" refusal is now typed refusals that name the conic.
The axial door refuses the hyperbolic case at its gate, and a cone-only
move keeps its generator lines. Tier: concurrent dual (letter H). Both
reviews came back APPROVE-WITH-FIXES with no MAJOR, about 100k fuzzed
poses between them. Tally 0, confirmed by blinded coding (byte 13). The
row is `docs/dual-review-log/rows/PR-5692.md`. The union's fix pass is at
`fff330083c`.

Consequences:
- Unit 3 parks on SELECT's chord-extent row, now P1. A short lever decides
  Zero (a definite verdict) more easily, so it is possibly unsound once an
  arc can rest.
- STEP import of HYPERBOLA/PARABOLA is filed on EXCH.

Process lesson: I removed the first implementer's worktree to free disk
before its PR merged, so the lane could not be resumed for its fix pass.
Keep a lane's worktree until its PR merges; delete target dirs, not
worktrees.

Still on Ev: `[ev]` PR 5689, where unit 1 parks on the dual/limb-2 fork.

## 2026-10-25 — board after unit 2 (OFFSET orchestrator)

`offset-lane-has-no-conic-carrier` parks on its two remaining units. Unit 1
waits on Ev's `[ev]` PR 5689. Unit 3 waits on SELECT's chord-extent row
(P1). `closed-form-limb-2-costs-milliseconds-per-curved-edge` parks on the
dual fork, since 5689 may move limb 2 to rest and change what the cost
row is about. That leaves no dispatchable P0 or P1 on this slate. The
orchestrator holds the track while `[ev]` PR 5689 is open.

## 2026-10-25 — the dual-limb fork converges on structural pairing (OFFSET orchestrator)

PR 5689 (`[ev]`, row 196) answers Ev's question about the dual entering
certification. Both designers agree:
- the edge door splits into mint (generic) and certify (bounded on the
  certification trait), so a dual mints and never checks, by type;
- the dual keeps its values;
- a dual pass is a function of its validated `f64` anchor, bound to the
  document revision it read. It walks the anchor's nodes and compares each
  value channel bit for bit.

Ev objected to recovering the anchor by content key. Both designers
withdrew it: a key excludes node identity and, at a dual, includes the
tangent bits. My earlier explanation to Ev presented that lookup as the
existing mechanism. That was wrong, since today's `pair_pass` pairs by node
id in order, and I corrected it on the PR. Unit 1 stays parked until Ev
rules, then is re-scoped to the split and the pairing.

Process lesson: a chained `git merge` and commit pushed conflict markers
into the fork log once, and twice a chained command stopped at an empty
commit and left edits unapplied. Merge with `--no-commit`, grep for
markers, then commit as a separate step, and check `git log` after every
chained push.

## 2026-10-25 — band rule (band-rule triage)

**Band rule (work/README.md, Priority):** conic unit 3, `a-classified-pairs-section-states-the-bounds-that-retire-the-tube`, re-banded P1→P0: `offset-lane-has-no-conic-carrier` (P0) is parked on it and closes when it lands, and it is the unit that lets the arc rest. Its blocker, SELECT's `edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short`, goes to P0 with it. The plan's conic-build table still reads P1 for unit 3. (band-rule triage)

## 2026-10-27 — seam note from QUAD's main-red triage

Filed `two-door-rows-red-under-the-per-op-postcondition-scalpel` on this slate. Two topo rows go red only under `--features per-op-postcondition`: `a_join_sweeps_tier_1_once_however_many_it_joins` (from PR 5666) and `the_door_panics_on_an_out_of_scope_malformed_solid` (bisected to PR 5661). Both are green in the default build at every ε. (QUAD triage lane)

## 2026-10-27 — conic unit 1 merged (PR 5728); the chord-extent row claimed (OFFSET orchestrator)

PR 5728 merged (`b8974faabc`). It carries the mint/certify door split
(5689), limb 2 for every carrier through one framed reader with the
kernel-limit refusal (fork row 203, Ev: "this seems good"), and limb 2 over
a parameter box through the free-`t` identity (row 204).

Ev's question on the last asked whether anything beyond "within ε
everywhere" was being decided. Nothing was: it applied E6/E12 to an
implementation defect.

Process lesson: I over-escalated. Before an `[ev]` PR, check whether the
question is already answered by ratified text (the claim) and only the
mechanism is new. A mechanism that applies a ratified rule is mine to rule.

The concurrent dual review tallied one finding (R2): at `f64` the
line × plane end rule measured in round-to-nearest arithmetic and passed
an edge 1.038e-12 m off its plane at ε = 1e-12. It was fixed by measuring
in certification arithmetic.

The orchestrator then ruled the ends' verdict order:
- an end enclosure with a floor above zero is a measured distance;
- one reaching zero is only an upper bound, so the stall stands.

Limb 2's cost went from 299 fast-set rows over their limit to 11, which
sit in the slow set; the cost row is P1. Three container restarts killed
the lane mid-run; pushing at safe points kept each loss to one step.

SELECT's `edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short`
(P0, unclaimed, SELECT `ready`) is claimed into OFFSET. Our conic arc
(PR 5692) made it reachable, and unit 3 is parked on it.

## 2026-10-28 — chord extent, limb-2 cost pass 2 and the main reds merged (OFFSET orchestrator)

- **PR 5875 (the chord-extent row, claimed from SELECT).** It replaces `edge_extent` with two true bounds. Two-sided consumers read both bounds and act only on agreement (`LeverBounds`, `decide_agreed`). Dual review: 0 tallied. It took two fix passes and two delta reads; the orchestrator ruled a both-bounds read rather than tightening.
- **PR 5885 (limb 2's cost, pass 2).** A first reading over dyadic spans, passing only on a checked covering. Both reviewers raised the same MAJOR: a span dropped at depth 30 let a window go half-read. It was fixed with `halves → Option` plus `covering`, and R1's delta read approved. 10 of 11 slow-set rows are back on the fast set. The cost row drops to P2: one interval stackup row (1.43 s), nightly only.
- **PR 5893.** It bisects the two main-red sweep rows:
  - sym11 is PR 5728's, an honest move (a far placement at `Sym<f64>` escalating on a measured end), so it was re-baselined;
  - germarms2 is PR 5664's (CHART), handed to CHARTTAIL.
- **Unit 3 (the tube exemption) is dispatched.** It lands severable, and its PR is to recall Ev's reservation.

Process: the box has 15 GB RAM and limited disk, so concurrent builds OOM. Lanes now build with `CARGO_BUILD_JOBS=2` and `CARGO_INCREMENTAL=0`, and finished lanes' target dirs are deleted at once.

## 2026-10-28 — conic unit 3 merged; the track is handed back (OFFSET orchestrator)

- **PR 5897 (conic unit 3, the tube exemption).** A classified pair's section states a certified sin θ floor `s` and a moat (the ε-overlap empty between 8ε/s and 16ε/s, a Poincaré–Miranda section point within 16ε/s); `analytic_rung3` skips limb 3 only where `section_bounds::tube_bounds` states them. The conical wedges and the three-quarter frustum now hollow, with volume and area checked against closed form. `offset-lane-has-no-conic-carrier` and SHELF's frustum row closed with it; `work/shelf/plan.md` still names the frustum row as parked, for SHELF to drop.
- **Ev ruled on the PR** that README C2's radius reads 8ε over the sine, out to 16ε, instead of "ε over that sine" (which is not a valid tube: the overlap's own band reaches about 2ε/sin θ). Ev agreed that tying the radius to K has the wrong semantics: 8 is a proof constant, K a policy dial.
- **Speed, for Ev.** 65 of 73 `analytic_rung3` calls across the geom-brep, topo, sweep and editor-core suites skip the tube; the waiver ran 1.3–12× faster than the tube on stated pairs (26 ms against 102 ms in total).
- **Review.** Concurrent dual review, 0 tallied: no MAJOR in the pair. R2's delta read of the fix pass found one (contract, executed): `uncertified_arc` matched an error shape the re-chart never produces, so `TogetherConicArc` never fired, on the first head as well; both first reviews missed it. Fixed by reading `RechartFalsifies{edge, ..}`.
- **Hand-back.** No P0 or P1 row remains; the track is `ready` with 19 P2 rows. Its dispatchable load (77 against a budget of 30) is over; a successor should cut it (Track size) before dispatching.

## A note from GREEN (2026-10-28) — shared tests with #5935

GREEN's `main-reds-editor-core-slow-set-and-the-k-probe-sweep`
(`work/green/`) covers editor-core slow-set reds that include
`m10_3_driver_interval`, `m10_9_pins_interval` and `sym11`'s
`two_hole_plate`, which #5935 re-pins at #5728's reading. GREEN's lane
(branch `green/slow-set-reds*`) leaves the reds #5935 attributes to
#5728 to that PR, and names any test both PRs touch in its PR body.
