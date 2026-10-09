# FLUX log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/flux/plan.md`.

## Opened as PROPS' successor (2026-10-01)

Ev, in chat: PROPS' orchestrator should handle whatever fraction of the
remaining slate it wanted, move the rest to a successor, and close PROPS
on finishing. **PROPS kept the refusal-text and recourse family — eleven
rows about what a refusal SAYS — and everything else came here**: the
four hard curved-arm rows, the arms' smaller gaps, and the `geom-core`
scalar and spline doors under them.

Why the cut fell there. The refusal-text rows are one family with one
shape (D4's recourse grammar, mostly arriving from ENCL's and TOPO's
seam notes as their D4 work crossed PROPS' ground), they are cheap, and
clearing them leaves a boundary a reader can see. What is left is the
work PROPS was opened for and never reached, and it wants a program with
its slate in front of it rather than behind it.

Inherited: 22 rows at the cut, plus
`f64-refinement-inside-an-enclosure-has-five-more-sites`, which stayed on
PROPS' slate until PR #3524 merged because that unit closes its fifth
site and moving the file mid-flight would have collided.

Territory is PROPS' unchanged, and the three programs PROPS cut out on
2026-09-20 keep what they took. Shared ground with QUAD and FRAME is
expected rather than a conflict, per Ev's ruling of the same date.
- 2026-10-01 — From PROPS' close (`props/recourse-grammar`): two rows land here. `invalid-margin-recourse-cannot-tell-an-unimplemented-kind-from-bad-inputs` moved from `work/props/` **stopped and reported**, with the reach sweep it asked for run: two of the five `classify_dihedral` callers that can see an arbitrary body face establish its no-spline-kind premise and three do not (the missing gates are filed on `work/topo/`), and one of its two candidate fixes is now closed by ENCL's PR 3418 — `MarginDiag` is opaque, so nothing can ride on the margin. What is left is a typed refusal on a public predicate's error, rippling to twelve callers across `topo` and `sweep`, which is a typing unit rather than the refusal-text fence PROPS was cut to. Newly filed: `stackup-measure-refused-carries-its-node-error-rendered`, the sibling of PROPS' `measure-refused-reduces-the-typed-refusal-to-its-name` that needs `NodeErrorKind: Clone + PartialEq` first. (PROPS implementer)

## PROPS closed; twelve more rows arrive (2026-10-03)

PROPS landed its last unit (PR #3942, the refusal-text and recourse
family, ten rows) and closed. `work/props/` is deleted whole and its
done-state of record is
`docs/doc-ledger/props-leaves-the-tracker.md`. This program now carries
**39 rows**.

Arriving at the close, twelve:

- **`f64-refinement-inside-an-enclosure-has-five-more-sites`**, back to
  `open` with its four remaining sites. PROPS closed site 5 — the convex
  form in `insert_once_ring`, PR #3524, dual-review row DR-33. Sites 1
  and 2 are spelling 1 (`f64` refinement then lift) and the two
  `quad.rs` sites are spelling 2 (rounded ratio); they are different
  defects with different fixes, and two of them are QUAD's ground by the
  2026-09-20 cut, so the row travels with an announced seam rather than
  a claim. **Note what arrived with it**: SSI measured a THIRD consumer
  on main while that unit was in review — 98.5% of limb 2's certified
  `hull_sup_chart` on the m8_4 seam was ring widening from this site's
  lerp form, growing as N³ — and those numbers are the lerp form's, so
  they are SSI's to re-take now the convex form has landed.
- **`invalid-margin-recourse-cannot-tell-an-unimplemented-kind-from-bad-inputs`**,
  which PROPS' last unit **stopped and reported** rather than serving:
  one of its two candidate fixes is closed by ENCL's PR 3418
  (`MarginDiag` is opaque, so nothing can ride the margin), and the other
  is a typed refusal on a public predicate's error rippling to its
  non-test callers across `topo` and `sweep` — a typing unit, not a
  refusal-text one. Its sweep is live, not closed: tier 3 and `rim_wedge`
  gate on `spline_chart()`, while `census`, `boolean::ops` and
  `splitting::finish` do not.
- **Ten filed by other lanes crossing PROPS' ground** between the
  2026-10-01 cut and the close, and they are this program's subject
  rather than PROPS' leftovers: `sphere-flux-arm-carries-two-closed-forms-for-one-face-kind`,
  `a-sphere-face-whose-boundary-encodes-no-side-is-measured-under-its-bit-alone`,
  `an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane`,
  `a-cylinder-rims-level-is-recovered-by-a-dot-product-that-loses-a-short-faces-height`,
  `the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column`,
  `the-projective-applier-still-lerps-so-a-nurbs-refined-at-t-interval-pays-twice`,
  `nurbs-interval-ders-at-a-wide-parameter-grows-with-translation`,
  `interval-sin-theta-as-cross-over-norms-loses-the-shared-magnitude`,
  `authored-and-derived-directions-decide-a-unitless-norm-against-the-length-band`,
  `trim-walk-chord-lengths-are-rooted-to-nearest`,
  `the-race-rows-leaves-need-two-amounts-where-they-needed-six`.

**Two of those deserve reading together.**
`sphere-flux-arm-carries-two-closed-forms-for-one-face-kind` sits beside
`sphere-flux-arm-refuses-partial-bands` in §1 of the plan, and
`the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column` is a
direct answer-back to the unit PROPS just landed — if the convex step is
looser than lerp on a varying column, the width argument that justified
it is narrower than it was stated to be. Take that one early; it bears on
a bound now shipped on main.

**One reference repaired across six other programs' rows.** Deleting
`work/props/` left seven `refs:` entries naming rows that no longer
exist, in `nurbs`, `quad`, `restfront`, `verdict` (×2) and `wire`. They
are dropped from `refs:` (which names live items) with a note in each row
saying where the finding is still readable and that PROPS' ledger note is
its done-state of record. Breaking them was this close's doing, so
repairing them was too.

## 2026-10-03 — HOLD: a refactor of dependency, placement and intent is underway (Ev, `[ev]` PR #3990)

Ev has opened a redesign of how a document says that one thing depends
on another and that things are meant to coincide. The question and Ev's
direction are `work/recipe/one-way-to-say-dependency-and-intent.md`;
the design lands through `[ev]` PR #3990. The direction, in short: no
node consumes another; no raw numbers (every slot holds a variable);
nodes are operations on typed variables; no absolute coordinates
(spaces are what is related to what, placements are relations); tangency
and coaxiality by construction; checked assertions replace declared
contacts; contact and tangency complaints become lints where the
answer is already known.

**Do not start a new unit that meaningfully uses** any of: the node
vocabulary's edges and consumption (`Node::inputs`, product roots),
`Expr`/document parameters and literals, placement (`Datum`
coordinates, `Transform`, `Pattern`/`PlacedUnion` frames, gauges,
offsets, mates and their solve), declared pairs and declared contact
(`Boolean`/`Union` `declare`, `ContactClass`, continuations, seams),
the undeclared-coincidence and undeclared-contact refusals, axis
declarations, `ParamSource`, the parameter-coincidence lint, or
`Measure`/`Assertion`.

**A unit already started may be finished**, even where it collides with
the above — land it as planned. Park each row the hold covers
(`status: parked`, `blocked_on: [one-way-to-say-dependency-and-intent]`,
so the row fires when the ruling closes). If that leaves your program
with nothing it may start, set its `status` to `blocked` and stop.

## 2026-10-03 — the intent refactor's hold now waits on the build, not the ruling (Ev ratified #3990)

Ev ratified DESIGN.md D10 on PR #3990, and the ruling
`one-way-to-say-dependency-and-intent` is closed. The hold announced in
the entry before this one CONTINUES until D10 is built: it now waits on
`work/recipe/d10-one-way-to-say-intent-is-unbuilt.md`. Every row that
was parked on the ruling or on #3990 has been re-pointed there, so
nothing fires at this merge. Park any further held row with
`blocked_on: [d10-one-way-to-say-intent-is-unbuilt]`. Units already
started may still finish. Read D10 before resuming work on this ground:
coincidence is now a margined verdict (no declarations), checked by the
`unproven-coincidence` lint.
- 2026-10-04 — **SSI built the banded collocation solve on flux ground: PR 4023** (`ssi/banded-fit`), closing `the-interpolating-fit-solves-a-banded-collocation-system-densely`. It touches `crates/geom/src/curves/fit.rs` and `crates/geom-core/src/linalg/lsq.rs`. SSI did it because SSI's fit budget depends on the fit's cost: refinement by certificate refits every round, and the dense O(n³) solve was most of the 1e-12 suite. `geom::Collocation` (`interpolate_on`) is a new public door on this ground, over a crate-private banded LU (`curves/banded.rs`); the control points are bit-identical to the dense solve, which stays in `lsq` as their reference. Filed here: `interpolate-with-params-passes-a-non-finite-point-through-as-nan` (P3) `the-approximation-refit-solves-banded-normal-equations-densely` (P4) and `unordered-fit-parameters-refuse-as-a-count-mismatch` (P4). (SSI implementer)
- 2026-10-08 — **Seam note from ENCL (`encl/offset-cert-coefficient-norms`): `geom_core::spline::compose` is edited from outside.** `tensor::coefficient_norm_bound` is now `pub` and takes `[&[Interval]; 3]` (with a ragged-row refusal); `tensor::coefficient_norm_sup` is its polynomial (`D ≡ 1`) case, and `patch::PatchSpans::cell_norm_sup` reads one tensor cell's three channels through it. SSI's `cell_residual` answers bit-identically (only the signature it calls moved). These are the D4 ¶2 doors for a certified vector upper bound; `offset_fit`'s `Y`/`M̃` and `patch_bound`'s integral chart speeds read them. (ENCL implementer)
- 2026-10-08 — Seam note from ENCL (PR 4331, `encl/adoption-at-rest-eps-in`, merged): `Reading::Adopt` is gone, and adoption certification reads as at rest. At the import door, the file's ε_in picks the words, for error text only (`geom_core::FileCoincidence`; `MarginDiag::sized_recourse_in_file`; `FileCoincidence::miss_recourse_in_file`; `geom_brep::certify::recourse_in_file`; `SizedDecision::recourse_in_file`).
  - A size at or below ε_in reads "This {size} is below the file's declared coincidence distance ε_in = X m, so the file does not state it", or "may lie below … may not state it" where only the nearer end is. It ends in the lever plus "re-export … declared below {near} m and tighten below {near/K} m" where a value exists.
  - A miss within ε_in names re-exporting, plus the set-ε-to-ε_in stopgap.
  - `CertifyError::ResidualExceeded` now carries `margin: MarginDiag`.
  - The `reporting-margin-door.sh` gate pins the two new sentence functions; recourse.rs is at 5 sites.
  - DESIGN.md D4 commitment 3 is reworded (the magnitude carrier moved). (ENCL orchestrator)
- 2026-10-08 — Seam note from ENCL (PR 4348, `encl/offset-cert-coefficient-norms`, merged): the offset certificate's vector upper bounds (‖Y‖ via `Composite::y_sup`, `m_tilde_sup`, and the integral arm's chart speeds via `PatchCell::s_u_sup`/`s_v_sup`) now read coefficient norms (D4 ¶2), not per-coordinate boxes. geom-core gains public `tensor::coefficient_norm_bound` (`[&[Interval]; 3]`, refuses ragged rows), `coefficient_norm_sup` and `PatchSpans::cell_norm_sup`. The bounds are tighter or equal, and the stored offset numbers were re-baselined. The three `rigid_map_near_eps_approx` rows moved to the slow set, and their witness now mints on round 2 at `MARGIN = 1/4096`. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4367, merged): `offset_fit.rs`/`offset_meters.rs` folds that feed guards now use `geom_core::interval::max_bound`/`min_bound`, so a NaN reaches its guard. `PatchRegularity::sup` and `CellNormal::sup` are NaN on a refused cell (previously the last cell only). `Composite::cell_terms` is the one home of `cell_bound`'s guards. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4377, merged): `offset_meters::patch_collapse` reads an unbounded κ⁺ (unreadable curvature) as `reach = NaN`, so `offset_curvature_headroom` escalates on an invalid margin rather than refusing sign-certain at −5e-324 m, on every route. New: `MeterError::ending_with_lever`/`render_with_lever` and `OffsetFitError::render_with_lever` (`METER_UNFITTED`). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4395, merged): a limb refusing at the offset fit's mint is now `OffsetFitError::MintLimb`, ending in the kernel-defect ending; the at-rest `Limb` keeps "re-fit" (`geom_brep::offset_fit::LIMB_REFIT_RECOURSE`). The shell and the transform re-fit both reach `MintLimb`. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)

- 2026-10-09 — Seam note from NURBS. NURBS has picked up its track, and its wave 1 has lanes live on ground we share:
  - `nurbs/refine-dir-hairline` flips `GridSkip::BitEqual` to `WithinUlps(SLIVER_CLEARANCE_ULPS)` at `props::quad::refine_dir`, `ssi::certify::refined` and `edge_nurbs::localized::breaks`, and re-baselines the widths those sites produce.
  - `nurbs/knot-mirror-on-knotvector` moves `mirror_symmetric` from `geom/src/surfaces/nurbs.rs` into `geom_core::spline::knots`.
  - A designer pair is weighing `span-locator-lands-a-nan-parameter-on-the-first-span`, the class your `props-collapse-over-lands-a-nan-window-on-the-first-span` is an instance of. Its answer may make that row's per-reader fix unnecessary, so it may be worth holding until the pair reports.

  NURBS's `certified-blossom-primitive-in-geom-core-spline` waits on your `the-convex-boehm-step-is-looser-than-lerp-on-a-varying-column`. (NURBS orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4443, merged at `070cec41da`): the "X is too close to call at this tolerance" sentences in `topo::validate`, `topo::census` and `geom_brep::props::PropsError::Escalated` now read "X is undecided" (one composer: `Indeterminate::undecided` / `geom_core::undecided!`). Tolerance advice lives in the ending, per D4 ¶1 (i). Not-yet endings join a note with `"; "` via `geom_core::noted`, and the `Recourse:` label is `geom_core::Recourse`. A new refusal on your ground should use these instead of hand-spelling. The remaining "too close to call" sites are listed in ENCL's `too-close-to-call-remainder`. (ENCL orchestrator)
