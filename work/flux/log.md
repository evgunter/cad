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
- 2026-10-09 — Seam note from NURBS. The span-locator question is decided: a NaN has no span, and every window reader takes a `ParamRange`. The final state is in `work/nurbs/span-locator-lands-a-nan-parameter-on-the-first-span.md` under `## Decided`. The build lane `nurbs/span-locator-refuses-poison` will touch your `props/quad.rs` window readers (`Collapse::Over`, `range_hull`, `raw_range_hull`, `raw_span`, `Dir::Const`). It supersedes your `props-collapse-over-lands-a-nan-window-on-the-first-span`, and its PR will close that row. Filed on your slate: `a-loop-area-nurbs-segment-integrates-an-inverted-window-as-zero`. (NURBS orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4443, merged at `070cec41da`): the "X is too close to call at this tolerance" sentences in `topo::validate`, `topo::census` and `geom_brep::props::PropsError::Escalated` now read "X is undecided" (one composer: `Indeterminate::undecided` / `geom_core::undecided!`). Tolerance advice lives in the ending, per D4 ¶1 (i). Not-yet endings join a note with `"; "` via `geom_core::noted`, and the `Recourse:` label is `geom_core::Recourse`. A new refusal on your ground should use these instead of hand-spelling. The remaining "too close to call" sites are listed in ENCL's `too-close-to-call-remainder`. (ENCL orchestrator)
- 2026-10-10 — Seam note: filed `net-state-reads-an-infinite-net-as-described` on this slate (P3/M, design). `NetState` reads a `±∞` net as `Described`; only tier-3 check 1 patches it, and `merge_faces` takes it as `Curved`. From the closing pass on PIPE's `described-net-two-state-reads-hand-a-poisoned-net-the-described-arm`. (PIPE routing lane)
- 2026-10-10 — Seam note from PIPE: filed `nurbs-curve-has-no-net-state-door` on this slate, the curve-side half of the described-net sweep that PIPE closed. (PIPE orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4475, merged at `61977302ae`): a poisoned margin's note now depends on the door that reads it. At a build it is "an unreadable margin may indicate a kernel bug worth reporting" ("or collapsed" is gone). At rest or at the import door it is "…a kernel or file defect worth reporting". `geom_brep::recourse::unreadable_margin_note(reading)` picks it. geom-core's `SizedWords` gained `unreadable: &str`, and `Indeterminate::ending_noted(levers, unreadable)` exists beside `ending`. An at-rest reader of a nested refusal composes its own ending (fork-log row 9): it does not add `ending(Reading)` to the type. `topo::props::SHELL_ROLE`, `ShellClassifyError::arm()`, `JoinUndecided::diag()` and `JOIN_SUBJECT`/`JOIN_LEVER` are public for that. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4474, merged): the material pairing is a decision. `geom_brep::MATERIAL_PAIRING` and `MATERIAL_PAIRING_CLAUSE` end it at tier 3 (`WedgeCheck::MaterialPairing`) and split finish (`SplitFinishError::DescribeSideEscalated`), with a true tolerance offer. Where the wedge would re-decide below m/K, `pairing_at_wedge` quotes the wedge and offers w/K, as `at_wedge` does. `MaterialStations`' `Break` is `MaterialStop { check, cause }`. The cusp-side Zero reads via `decide_nonzero`. New k_stats predicates appear on the refusal path only: "material_pairing_offer_wedge" and "material_pairing_wedge". (ENCL orchestrator)

## 2026-10-10 — a new orchestrator picks FLUX up and cuts it on its priority seam

FLUX measured **125.5 budget points against 30** (52 open rows, nothing
in flight, no `flux/` branch live). The cut is on the priority seam. FLUX
keeps the eight curved-arm rows, at exactly 30 points:
`rim-side-and-rim-dir-group…`, `sphere-wedge-arm…`, `stored-spans…`,
`an-ellipse-trimmed-ring…` (priced P0/H), `spiric-bounded-face-area…`,
and the sphere-arm trio (`sphere-face-with-a-hole…`,
`sphere-flux-arm-carries-two-closed-forms…`, `a-sphere-face-whose-boundary-encodes-no-side…`).
Its territory narrows to `props/*` and `topo/src/props.rs`.

The rest moved by `git mv`, ids and bodies unchanged:

- **FLUXTAIL** (12 rows, 22.5 points, P3, `ready`): the arms' numeric honesty.
- **KNOT** (10 rows, 24.5 points, P2, `ready`): the spline, net and fit doors.
- **SCALAR** (14 rows, 29 points, P1, `ready`): the scalar doors, their hand-spelled readers, and the box driver's readings.
- **FLUXHOLD** (4 parked rows and 1 closed row, P2, `blocked`): the D10-held rows, each parked on the INTENT unit that rebuilds its ground. Those are `an-assertion-relates-by-equality` (`assertion-verdict-derives-partialeq-alone`), `intent-stage3-is-built` (`the-box-driver-carries-no-part-resolver`), `transform-retires-into-a-placement` (`authored-and-derived-directions…`, datum normals and mate directions) and `declared-pairs-retire` (`indeterminate-display-offers-declare…`).
- **NURBS**: `the-convex-boehm-step-is-looser-than-lerp…` and `the-projective-applier-still-lerps…`. NURBS's charter is the certified Boehm step, and its blossom unit waits on the first.
- **STACK**: `stackup-measure-refused-carries-its-node-error-rendered`. Its D10 trigger, `measure-is-an-operation` (PR 4355), had already fired.
- **Closed**: `measureexpr-has-no-display-or-as-primitive`, superseded by PR 4355, because `MeasureExpr` no longer exists.

**The D10 check, row by row.** Several rows sat near held ground and were judged NOT held. They are recorded here so the next reader can disagree:

- `a-gate-rejection-of-a-decided-enclosure-bisects-to-budget` is the drive's leaf reading of a k_stats escalation. It is not Expr vocabulary.
- `analysis-boxes-keep-an-axis-order…` is post-stage-1 cleanup. No later stage rewrites `ParamBox`'s order.
- `the-race-rows-leaves…` is a test fixture.
- `invalid-margin-recourse…`: its question is `MarginDiag::Invalid`'s two causes. The declare lever in its text retires with declared pairs regardless.
- `a-planar-face-sums-its-area-about-a-far-carrier-origin` and `project-eps-point-is-absolute…`: stage 3 E (`an-operation-computes-in-a-frame-of-its-reads`) will shrink how far geometry sits from its origin, but neither fix depends on it.

**Pricing.** Unpriced rows were priced, and legacy `D` was re-priced (all to `M`). `patherror-display-renders-float-noise` went from E to M, on its own body's three-owner analysis. `an-ellipse-trimmed-ring…` was banded P0: a union of a tilted bar and a pipe refuses on 72 of 72 ops.

**Band collision fixed.** FLUX's `ab_band` 10300–10399 was PLACE's (EDIT's 2026-10-02 exit), and FLUX never appeared in the roster. FLUX now takes 12600–12699. The cut claims 12200–12599 for its four new programs (`docs/MODEL-AB-LOG.md`). No FLUX ordinal was ever drawn.

**What FLUX does next.** The sphere-arm trio goes to a designer pair first; whether Gauss–Bonnet subsumes the iso-rectangle form decides all three. The P0 lanes (rim-side with stored-spans, then the wedge fold) and the spiric arm go to implementers. All of them edit `props/curved.rs`, so they are sequenced or seamed. — (FLUX orchestrator)
- 2026-10-10 — **Wave 1 dispatched.**
  - A designer pair (A and B, concurrent, problem statement only) weighs the sphere arm as one question. It covers the two closed forms, the ring face, the side encoding, the wedge's lineage fold, and `props_rim_side`'s cycle-order reading. Design-fork byte 142, recorded under `analysis/design-fork/` if the question goes to Ev. The wedge and rim-side rows wait on the pair, because their fixes may be moot under its answer.
  - Implementer `flux/ellipse-ring` (P0 H) has its own cloud session.
  - Implementer `flux/spiric-area` (P2 H, the row with a demo behind it) has its own cloud session.
  - Both implementers edit `props/curved.rs` and are seamed to each other. Their review tier is set when each PR is up. Per plan §3, expect dual review on both, because each changes what the kernel certifies over a family. — (FLUX orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4517, merged): per Ev's ruling in PR 4498, the C2 foot-orthogonality limb is retired. `ssi_foot_orthogonality` and its check row are gone from `geom_brep::ssi::certify` `nurbs_limbs`, and `geom::SurfaceProjection.orthogonality_u/v` are deleted: a surface projection now carries only `distance`, which is the residual that gets banded. `SurfaceProjectionInconclusive::last_orthogonality_*` stays as refusal text. The curve half's orthogonality field is FLUX's row `curve-projection-orthogonality-field-has-no-reader`. (ENCL orchestrator)
- 2026-10-10 — **The sphere-arm designer pair converged (fork log row 109) after four rounds, two of them crossings.** Both now recommend one Gauss–Bonnet form over every loop, the iso arm retired from flux, and one `sphere_loop_side` reader (per-pole winding) called by the flux arm and check 6. Round 4 turned on a fact the orchestrator verified: check 4 reads material pairing only on `Smooth` edges, so a flipped one-loop face that meets its neighbours transversally is caught only by the pole-free side. That reading rests on Ev's provisional PR 2850 ruling, so the `[ev]` PR `flux/ev-sphere-pole-side` asks whether the ruling may bear a refusal. The rest of the design changes no ratified text. The build is dispatched now, because today's tilted-circle reader already rests on the same ruling; a "no" from Ev removes the one-loop reader and states the gap. Filed off-question, each with its owner: `cone-face-closed-form-runs-an-iso-premise…` (FLUX, P1), `a-torus-face-whose-chart-closes-in-both-directions…` (FLUXTAIL, a probe), `same-surface-faces-sharing-an-edge…` (RESTFRONT), `sphere-window-borrows-the-material-sign…` (BOXES). — (FLUX orchestrator)
- 2026-10-10 — **Ev answered PR 4549: "this works".** The PR 2850 ruling is load-bearing for the sphere side check, and its home is geom-brep README C12 (7). Fork row 109 is filled in (renumbered from 108 at merge), and `needs_ev` is cleared on `sphere-flux-arm-carries-two-closed-forms-for-one-face-kind`. `flux/sphere-arm` keeps the one-loop pole-free reader. — (FLUX orchestrator)
- 2026-10-10 — **Wave 1, spiric lane: PR 4554 merges.** The unit got dual review (DR-149), a concurrent pair on `74fd2a71`. Both reviewers' verdicts were NOT-MERGEABLE-AS-IS on the endpoint census alone; neither falsified any correctness claim by execution. The adjudicated fix pass at `d43a02f36a` did the following:
  - the census reads are now guarded by name;
  - committed rows kill all eight surviving mutants;
  - one span-window rate bound replaces three spellings, so a cut through the tube hole (|d| > R − r) now measures;
  - the 1024-piece fallback is deleted;
  - the elbow gets an e2e surface-area oracle, and a reference re-take script is committed.

  An independent verifier confirmed all seven items by execution. Its far-offset note is filed as FLUXTAIL's `a-spiric-elbow-a-hundred-kilometres-out-exhausts-the-quadrature-budget`. — (FLUX orchestrator)
- 2026-10-11 — **Wave 1, the ringed cylinder wall: PR 4557 merges.** The unit got dual review, a concurrent pair on `60597c6d`.
  - The real gap the review found (R2, demonstrated): the ring-winding gate ran only on a converged round, so an open window could carry a same-wound ring. It now runs on every round exit.
  - The per-PR gate could not go red on a ring dropped from o·A⃗ (R1). A fast row now catches that.
  - Corrupt loops now get typed defect refusals.
  - The verifier caught one regression the fix pass itself introduced: a `debug_assert` replacing a typed refusal. It was restored.
  - Filed on QUAD: the pre-existing thin-ring refinement gap. Filed on GERM: the plane×cone apex fuzz counterexample, which reproduces on main.
  - Extent-gate timing was argued and kept converged-only; the verifier could not break the argument. — (FLUX orchestrator)
- 2026-10-11 — Note from ENCL: main is red on step-import `nurbs_import::arc_loft_natively_computes_its_rational_volume`. ENCL bisected it to NURBS #4553. The cause is a face-order dependence in `fold_runs` that #4553 exposed; #4553 itself is not wrong. Filed `work/exch/arc-loft-round-trip-volume-differs-by-face-order.md` (P1) with the mechanism and two repair shapes. (ENCL orchestrator)
- 2026-10-11 — **Wave 1, the sphere arm: PR 4573 merges.** Every circle-bounded sphere face is now measured by one Gauss–Bonnet form, and its side comes from one reader, `sphere_loop_side`. The iso-rectangle arm is retired. The unit went through dual review (a concurrent pair on `0cf61fd2`), three fix passes and two independent verifiers.
  - R1's demonstrated MAJOR (a red row left un-re-baselined) is the tallied finding. R2 found the silent double flip of seam siblings, filed as `a-pair-of-seam-siblings-flipped-together-is-silent`, and the negative holed area, now refused.
  - Thin-region precision is restored by a Van Oosterom–Strackee fan, not by looser bounds.
  - The first verifier caught a regression the fix pass itself introduced: the interval lane's small-region enclosure floored at ±2e-12 sr, about 1.7e8× wider than main, and `certify_role` reads it. The root cause was the fan's apex. It is now the centroid, within 3.5× of main.
  - The widened `pi_seam` row is pinned per member order. Its 1e-12 asymmetry is filed on FLUXTAIL (`a-kept-rim-off-its-sphere-is-read-on-its-own-sphere-so-member-orders-diverge`).
  - The second verifier caught a sliver row whose volume check could not fail. It now asserts relative to the exact lens.
  - Main-side reds found on the way, both filed: the arc-loft thread-count digest (NURBS) and an SSI row at 1e-6 (ENCL).
  - Five rows close. — (FLUX orchestrator)
- 2026-10-11 — **Wave 2, stored spans: PR 4680 merges.** Review tier: single full review. The decides fire only on hand-built loops, and no certified value moves, only verdict populations. Two named decides now cover every rim and torus-meridian span reader in props: `props_rim_span_*` at the rim mints and in the cone's shared span readers, and `props_meridian_span_*` behind one door.
  - The review (APPROVE-WITH-FIXES, 0/3/1) found two cone readers still reading a 3π span raw, and two torus levers no row pinned. All are fixed and pinned by mutants.
  - On the way, main stopped compiling `sweep` (PRs 4591 and 4662 crossed). FLUX fixed it in PR 4700. The digest re-cut that follows is flagged to NURBS' PR 4684.
  - Main itself has since moved that digest's sym columns again (`sz` 48 → 28/24). — (FLUX orchestrator)
- 2026-10-11 — **Wave 2, cone faces: PR 4696 merges.** Every one-nappe cone face is now measured by its vector area over all of its loops. The S58 iso premise and the `du_of_rims` family are retired.
  - **Review.** Dual, holdout arm (byte 120), on `5df2f692`.
    - R2's demonstrated MAJOR is the tallied finding: the rewrite had dropped the on-surface premises, and straight chords measured as a cone wall. R1 raised no MAJOR, so this is an **M-tier miss**, and a short readout to Ev follows.
    - Bilateral: apex loops sweeping past 2π, and loops meeting themselves at a vertex.
  - **First fix pass:**
    - incidence restored;
    - apex loops bounded at 2π;
    - a vertex decide with a seam exemption;
    - typed `ConeLoop` and `HolesExceedFace` refusals.
  - **Verifier 1** found a regression the restoration introduced: the rim-fit lever was off by 1/cos α, so 316 legal near-flat faces escalated. **The second fix pass** levers it at true distance, gives the material sign lane parity with the flux lane, and adds incidence rows.
  - **Verifier 2** confirmed: the sweep's failures are identical to `5df2`'s. Its two fail-closed MINORs are filed (`cone-incidence-decides-over-lever-toward-the-needle-and-the-flat`).
  - **Main-side reds** met on the way and filed by others: `near_tangent_pierce_poses` at 1e-6 (FUSE), and two compile breaks (4700, 876d537). — (FLUX orchestrator)
- 2026-10-11 — **Seam note from NURBS: a design row on FLUX's slate.** PR 4684 filed `the-whole-body-property-fold-depends-on-face-order` (`design: true`).
  - `topo::props`' `fold_runs` sums face terms in arena order, which is the D9 addendum's idiom 2. So one solid stored in two face orders measures a volume a few ulp apart. The per-face terms are bit-identical, as step-import's arc-loft row now asserts.
  - An order-independent fold would change ratified text (the D9 addendum), so it is Ev's call, routed through FLUX as the owner of `props`.
  - ENCL's PR 4682 bound (`faces − 1` ulp) is the body-level claim until then.
  - The same mechanism explains NURBS' `quad-de-boor-through-the-blossom-breaks-the-import-bit-identity`, so routing `de_boor` through the blossom waits on, or is checked against, this row. — (NURBS orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4687, merged at `aaf011d01b`): `geom_core::interval::certified_hi` (the upper end if certified, else NaN) replaces quad's `hi_or_refuse`, and `geom_core::interval::{cross, dot}` replace quad's `rv_cross`/`rv_dot` (also read by `props/spiric.rs`). Same component and association order, so the results are bit-identical. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4745, merged at `512ef4b6c7`): `props.rs`'s shell-volume sign refusal is payload-only through `geom_core::undecided!`; its text is unchanged apart from the shared joint. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4754, merged at `40eefeb668`): `props/quad.rs`: `Loose::from_derived` is the one Derived→Loose mapping, read by `Level::of` and `PatchGrid::step`. `Loose::below` is deleted, and `Loose::of` serves only the raw-list step-down. Bit-identical. The FLUX row `second-derivative-sup-takes-a-control-net-beside-its-vector` gained evidence and a stale-premise line; the duplicate KNOT row was dropped. (ENCL orchestrator)
- 2026-10-12 — **Wave 3, cone tail: PR 4744 merges.** Every cone edge is now decided on the cone. Ring arc spans are decided rather than read off their endpoints. Ellipse edges carry a nappe decide and an incidence decide: a 128-piece trig-coefficient bound on the section's elevation, divided by a lower bound on the true distance to the cone. The incidence decides are levered at true distance through `implicit::cone_offset`.
  - **Review.** A single full review raised one MAJOR: the first spelling decided elevation at five span points, which passed sections up to 584ε off. The coefficient bound replaces it.
  - **Verifier.** It confirmed the bound's expansion and enclosure (7,482 off-cone verdicts, none answered Ok), that no legal section is refused, and that R2's 5,616-case sweep is identical to the merge-base. Its three MINORs are fixed:
    - escalation starts at about a tenth of the claimed reach and depends on where the pieces fall, and the audit row now says so;
    - a threshold row pins the `k_y·Y` term and the least-ρ and least-|h| levers;
    - an apex-plane row pins the nappe decide.
  - The second-order terms `k_x`, `k_xx` and `k_xy` remain known survivors, absorbed by the over-read.
  - Three rows close. — (FLUX orchestrator)
- 2026-10-12 — **Wave 3, seam siblings: PR 4746 merges.** A sphere face whose own boundary encodes no side is now read through its seam region (`topo::validate::seam_region`, `sphere_region_material_sign`). A pole on an interior edge lies inside the region, so it reads that side. A region with two or more boundary loops reads its side by area sign. Tier 3's check 6 refuses a flipped seam region with `CurvedSenseInverted`, so R2's six silent seam-sibling flips are now refused (568/0/0/0).
  - **Fork check.** There were two ways to do it: (a) read the region, or (b) refuse the configuration. (b) was dominated, so this was not a fork. The C12 (7) wording went to Ev as [ev] PR 4753, which Ev approved.
  - **Review.** Dual tier M, sequential arm (byte 56). Review 1 raised one MAJOR: an arm that read a pole lying on no member's loop assumed C12 (7), which validity does not enforce, so it refused a valid imported STEP ball. That arm is gone. Review 2, on the fix head, confirmed every fix. It found the whole-sphere doc sentence false for a double cover, and three reader decisions no row could catch going wrong. Both are fixed, with rows red under the `held` and `start_only` mutants.
  - **Filed:** `a-seam-region-covering-the-sphere-twice-is-unread` (P2) and `a-pinched-seam-region-has-no-witness` (P3). The both-poles residual is pinned at P2 (`a-seam-region-whose-boundary-carries-both-poles-is-unread`).
  - One row closes. — (FLUX orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4784, merged at `854efb839a`): the P1 you found in PR 4573's eps rows (`a_side_whose_hull_is_loose_reads_clear_over_the_grid` at 1e-6) is fixed. (ENCL orchestrator)
- 2026-10-12 — **Wave 4 opens on two design forks and two independent units.** The D10 hold overlaps none of it.
  - **Fork: cone ring containment** (`a-cone-face-whose-rings-leave-or-overlap-its-region-measures-silently`). Both designers agree: tier 3's check 9 owns "a face's loops bound one region" on every kind; the lanes trust it; the public measurement door takes `&AtRestBody`. `[ev]` PR 4822, design-fork row 121.
  - **Fork: whole-body fold determinism** (`the-whole-body-property-fold-depends-on-face-order`). One reconciliation round settled the fact: per-face terms move with a loop's anchor. So an exact sum at the fold alone is out. What remains is a stated split: storage-dependent and stated, or storage-invariant at every level. `[ev]` PR 4830, design-fork row 122.
  - **Held:** the seam residuals (both poles on the boundary; the double cover). The double cover is the region-multiplicity class PR 4822 asks about.
  - **Dispatched:** `flux/cylinder-band-quad` (the seamless band with an ellipse rim; single full review) and `flux/geom-tidy` (`Projection3`'s orthogonality field and `second_derivative_sup`'s net; orchestrator's read, two PRs).
  - **Main-side red:** the sweep thread-count digest's `validate_geometric` frozen column (1060 → 1065, 1177 → 1181) is red on main. Bisected to PR 4803 and filed on NURBS (PR 4821). — (FLUX orchestrator)
- 2026-10-12 — Seam note from SYM: SYM-17 Phase 2 (Ev's answer on PR 4902; `docs/SYM-17-SPEC.md` §Phase 2) replaces `OrthoFrame` with `RigidMotion<T>`, a proper rigid motion carried by construction. `topo::transform_rigid` will take it and ask nothing about rigidity; `check_rigid` becomes its one decided door, for raw maps. The type acts on `UnitVec3`, and pose inverses become transposes. Branch `sym/17-rotation-readout`. Touches linalg, `transform.rs`, `eval/wire`, placement, the mate solve's representatives and inverses (not `member_of` or `check_offsets`, which stage 3 retires), and the revolve, sweep and pattern rigid maps.
- 2026-10-13 — **Wave 4, geom tidy: PRs 4835 and 4838 merge** (orchestrator's read). `Projection3.orthogonality` is gone, because nothing read it. `second_derivative_sup` became `nonrational_second_derivative_sup(&NurbsCurve3)`, which takes the curve rather than a control net beside its vector, and refuses a rational curve with the typed `SecondDerivativeUnbounded::Rational`. Two rows close. — (FLUX orchestrator)
- 2026-10-13 — **Wave 4, cylinder band: PR 4839 merges.** The cylinder quadrature lane now reads −∮(v − v_a) du, with v_a an in-face anchor. This is the form the closed-form chart reads, so a seamless band measures with a circle or an ellipse rim, and a winding ring is passed over as a hole.
  - **Winding.** Integer turn checks replace the per-junction period-jump refusal: each loop winds |k| ≤ 1 and the face's turns total 0. A genuine defect is refused as `LoopsWindChart` or `ChartLoopOpen`.
  - **Junction closure** (`props_quad_junction_closed`/`_open`) is two-sided. A gap closes if its upper bound is within slack S plus a 3ε vertex term. It is open if its lower bound is beyond that. Otherwise it escalates. The end of an edge reads its own vertex's trig bracket (`TrimEdgeQ::trig1`), which brings real gap widths from 60ε down to 1.8ε.
  - **QUAD's row is folded in as forced scope.** `Schedule::converged` reads all six mints. A face in the convergence band refines while a round remains, and escalates at the last round and in the exact lane.
  - **Review.** Dual tier M, holdout arm (byte 60), on `56f5d96d`. Both reviewers returned APPROVE-WITH-FIXES with no MAJOR. Two fix passes followed:
    - The first verifier found two MINORs: the decide read only the lower bound, and the vertex term was unpinned.
    - The second pass made the decide two-sided, added trig1 and pinned the vertex term.
    - The re-verifier found no real junction escalates at any ε: 10,611 junctions at 1e-12, worst excess 1.78ε. Each mutant reds exactly its row.
  - **Filed on the way:** `the-escalation-tolerating-rows-can-assert-their-answers-now` (QUAD), the periodic-closure class, a pctail row on tier 3 passing a seamless band, and a QUAD budget row for a micron-thin wall. Two rows close. — (FLUX orchestrator)
- 2026-10-13 — **Wave 4, iso parse: PR 4932 merges** (orchestrator's read). The cone and cylinder iso parses now decide a rim circle's incidence at its true distance from the surface first: the cone through `require_on_cone`'s circle arm, now shared as `require_cone_circle`, and the cylinder through the new `props_cylinder_circle_tilt`/`_off_axis`, `r·(1 − cos θ)` and `o`. Then they ask whether it is the iso rim as inventory. So a circle tilted within band of the surface refuses `NotIsoRectangle`, as the sphere and torus already did, rather than `OffSurface`.
  - **Re-baselined:** `review_m2_pr7`'s off-surface rim names the new decide; the voided-rod census and the thread-count digest's `bulged_extrusion` each gained the cylinder's two decides on two rims (verdicts 22 → 26, values unchanged).
  - **Filed:** the line half (`cylinder-meridian-axial-refuses-a-line-within-band-as-off-surface`), and on LIB the main-side python red its CI met (`runline-is-curated-in-the-facade-and-unbound-in-python`, from PR 4922). One row closes. — (FLUX orchestrator)
- 2026-10-13 — **Wave 4, periodic closure: PR 4938 merges.**
  - **One decision for "a loop closes and winds as a face's boundary does" on a periodic chart** (`geom_brep::props::closure`), read by the quadrature, the cylinder and torus closed forms, and the spiric wall.
    - Per junction, the chart gap less whole periods is decided against a stated slack. Turns are counted as exact integers (`|k| ≤ 1`, `Σk = 0`), and a connected face has at most two winding loops.
    - Each edge's own turn about the surface axis is carried, so the whole-turn decide `props_chart_loop_whole` holds W to kτ. `props_chart_axis_inside` keeps those turns clear of the branch cut.
    - The interval junction walk has one home (`interval_loop_turns`), and the bracket helpers moved to `props/bracket.rs`.
    - Real defects fixed on the way: the cylinder closed form measured a ±2 pair as twice the band, and four rims wound `++−−` as 8π. Both now refuse `LoopsWindChart`.
  - **Review.** Dual tier M, holdout arm (byte 201). Both reviews APPROVE-WITH-FIXES, 0 MAJOR. The blinded coder (byte 35) tallied 0. It noted a possible under-grade of the `++−−` finding, recorded only.
  - **Verifier.** Across four fix passes it caught three things no review had raised:
    - a silent wrong area from a loop of many ε-slanted rulings, whose own turns were read as 0;
    - a regression of the first whole-turn fix on rims whose centre sits ε off the axis;
    - a 2ε escalation window in the axis decide.
    Each is fixed and pinned.
  - **Filed:** the f64 spiric ends near tangency (P3); closed-form loop_whole charging near-axis rounding at the largest radius (P3); the cone closure as a fifth spelling. One row closes. — (FLUX orchestrator)
- 2026-10-19 — **Wave 4, iso parse lines: PR 5038 merges** (orchestrator's read). The line half of PR 4932: the cylinder and cone line arms decide `props_meridian_on_surface` at an upper bound on the segment's true distance from the surface (the ends' residuals plus the new `radial_sag`, the cone split at the apex's plane), then ask ruling/generator-ness (`props_meridian_axial`; `_generator`, `_apex`) as inventory. A line tipped in the tangent plane within band now refuses `NotIsoRectangle`, not `OffSurface`.
  - **Fix pass:** the first head reddened 20 cone/revolve rows. One cause: a generator ending at the apex has an apex-side piece with both ends on the axis, and at Interval `radial_sag`'s quotient divided by an enclosure of 0. The sag is now `(ℓ−δ)·min(2, (ℓ+δ)/(√ρ0+√ρ1)²)`, the same bound (`ℓ+δ ≤ 2·max ρ`), and pinned by `iso_parse_incidence::a_cone_sector_through_its_apex_measures`. Green on `9328e3a0` (run 39188277128).
  - **Re-baselined:** `props_cone_incidence::PERTURBED`'s turned/off-apex generators, the needle generator, `props_cone_vector_area`'s chords and `review_m2_pr7`'s diagonal chord now name `props_meridian_on_surface`; the thread-count digests move `bulged_extrusion`'s verdict hash only.
  - **Left open, noted on the PR:** `props_meridian_generator` is now implied to within about two bands by incidence plus `props_meridian_apex`, so no row isolates it; and whether the cone's flux lane needs generator-ness at all (it would widen what measures).
- 2026-10-19 — Seam note from RESTREAD: filed `a-ring-on-a-cone-wall-escalates-props-chart-loop-whole-at-1e-12` (P1, a main red at 1e-12). `reach_cone_root_lane`'s seed-15 ring on a cone wall escalates `props_chart_loop_whole`. It was seen on PR 5363's CI and reproduced on main 20a9d7928f, and is unbisected. (RESTREAD orchestrator)
- 2026-10-19 — **Wave 4, cone closure and per-term whole: PR 5333 merges** (dual tier, M, SEQUENTIAL by byte 28; one review, no MAJOR).
  - **(a) The cone through `closure`.** Junctions are decided in space under `closure::require_gap_closed` (3ε, `ChartLoopOpen`); windings through `closed_loop_turns`, rims carried to the axis by `about_axis`; the apex is the chart's puncture (a virtual winding −Σ when the face sums to ±1); a loop through the apex keeps `props_cone_apex_loop_sweep` with k = 0 rings. Retired on the cone: `props_loop_closed`, `props_cone_loop_winding`, `props_cone_ring_contractible`. **Widening:** a band of two opposite rims now measures, pinned to its seamed rectangle and closed form on a skew, off-origin, tilted cone and with rims a period off; holes in a band are read as a cap's (wound against its rims' summed side, `HolesExceedFace`).
  - **(b) `props_chart_loop_whole` charged per term:** Σ_e lever_e·|miss_e| against 3ε, the miss read clear of the branch cut, each edge at its own least radius; the torus meridian's `about_core` reads the meridian's own plane. **Widening:** spindle and horn tori with a parallel within 1e-8 m of the axis measure at Interval, pinned to closed forms (and a metre spindle near its axis).
  - **Review** (`session_01A5XHA3`, 54 probe configurations at both doors, both scalars, three ε, head vs base): APPROVE-WITH-FIXES 0/3/3. Fixed: the band hole arm unpinned and its holes skipping `HolesExceedFace`; the filed axis-inside row false for the cylinder; the apex-ring refusal and the cone lever unpinned; M2 pinned only at 1e-12; style (one turn spelling, stale cone docs, the loop_whole audit row).
  - **Filed:** `cylinder-axis-inside-reads-the-surface-radius-not-the-rims` (P3; R = 9.6ε with 10.4ε rims escalates, pre-existing); `cone-holes-outside-their-face-are-subtracted-silently` (P2, pre-existing class now reaching bands); `cone-apex-loop-beside-a-winding-rim-refuses-a-region-it-bounds` (P4, conservative).
- 2026-10-19 — **Main reds: PR 5369 merges** (orchestrator's read).
  - **P0, the thread-count serial golden (`tilted_cut_upper`'s pads): not FLUX's, already fixed on main.** The pads moved with PCERT's #4952 (the pcurve envelope's radial pair read whole; pads shrank, value, area and verdicts unchanged), and NURBS's #5055 (`cd00022655`) re-cut the golden, its bisect naming the #4952 merge as first failing. #4938's merge `63740ca8` is an ancestor of the passing `b6ef0ab7` (checked). Green at three ε on current main; sweep's slow set 384/384. Row closed on its record.
  - **P1, the open split-rim probe** (`rim_dim_review_probes::small_body_rims_50eps_apart_refuse`, feature `probe`) re-pointed to `ChartLoopOpen`: a 50ε gap past the 3ε junction slack, refused on its closure since #4839.
  - **The probe pre-pass on main + this fix found one more, ours (#5333):** `rim_dim_scale_twins::the_cone_flux_lane_decides_lengths_at_both_scales` named `props_chart_loop_sense`, which none of its four faces decides; a fifth face (a band between two whole rims) winds and decides it. All `plain` probe suites at 1e-9 and geom-brep `--features probe` at three ε green.
- 2026-10-19 — **Lane honesty bundle: PR 5375 merges** (orchestrator's read; four E rows).
  - **Cylinder axis-inside at its rims:** `props_chart_axis_inside` decided once per face at the least rim radius, not the surface radius; a cylinder under Kε with admitted rims (R = 9.6ε/9.95ε, rims 10.4ε/10.5ε) now measures `2π·R` (red on main at three ε).
  - **Sphere pole levels carry their cosine** (`LinearBoundary`): measured first, the √(1−s²) completion refused or escalated legal windows at the iso shape door (`props_rim_level`) at 1e-9 and 1e-12 (every k ≤ 1e4 at 1e-12); the flux lane had measured them. **Widening at the iso door**, pinned by `s58_iso_rectangle::a_sphere_window_with_a_rim_near_a_pole_is_an_iso_rectangle`.
  - **Body pads round up:** `fold_runs` sums pads with `interval::add_up` (new) and the volume pad's third by `div_up`; `MassProperties::enclosure`'s doc now bounds the fold error by Σ|flux| and says no certified sign reads these ends. **Re-baselined:** five thread-count digest `vpad`s one ulp up (no `apad` moved).
  - **Torus lift as structure:** `props_torus_lift_turns` (new, `Period::decide`, levered at `minor`) after the closure decision bounds each rim within ≈Kε/minor of a whole turn (the row's premise re-derived: 4938 had retired the joint decide it cited). The Gauss–Bonnet fan keeps its floor, with the argument at the site.
  - **Tracker:** `cone-holes-outside-their-face-are-subtracted-silently` now points at Ev's PR 4822 decision (check 9 owns the region); RESTFRONT's containment row refs it back.
- 2026-10-19 — **Round-trip bound computed from the terms: PR 5402 merges** (orchestrator's read). The follow-up Ev's (A) on PR 4830 left: `topo::test_support::fold_order_bound` bounds two fold orders' volumes and pads by the face terms (γₙ₋₁·Σ|t|, not `faces − 1` ulps); a row over every order of six signed terms 1e5 m out (spread 6.4e-12, thousands of the volume's ulps, within the bound). Converted: nurbs_import's arc-loft round trip (per-face bit identity kept) and step-import roundtrip's `2·kernel_faces` ulps. Listed, not converted: freecad.rs (closed form), asm2a ×2 (a rigid move changes every term), review_probes, reach_continuation. Row `the-whole-body-property-fold-depends-on-face-order` closed.
- 2026-10-19 — **1e-12 cone main reds: PR 5400 merges** (orchestrator's read). Two FLUX regressions red on main only in the 1e-12 slow set, bisected to their own PRs; both units had run sweep at default ε only.
  - **#5038's `props_meridian_on_surface`** (steep frustum at its own gate): `radial_sag` formed ℓ−δ as a difference of norms, each ~1e-12 wide at Interval on a 410 m wall, so a generator's zero sag read ±1.9e-12. N = ℓ²−δ² is now 2(ρ0ρ1 − q0·q1), or 2‖q0×q1‖²/(ρ0ρ1+q0·q1) where that is tighter (q0·q1 ≥ 0); same bound, no cancellation.
  - **#5333's `props_chart_loop_whole`** (cone ring): each edge's miss was read back about the axis, charging Interval width as disagreement (5.1e-12 on the ring). `EdgeTurn::exact` for generators and ellipse thirds, `EdgeTurn::carried` for rims. Band and slack unchanged. **Cost:** #5333's M4 (drop the rim carry) no longer reds; stated in the row.
  - **Rows:** `periodic_chart_closure::a_steep_wide_frustum_wall_off_the_origin_measures_at_every_eps` (red on main; each half of the fix alone leaves it red). The duplicate WIRE row folded into the #5038 row.
  - **Filed:** `cylinder-and-torus-edge-misses-are-read-back-about-the-axis` (P3), the same reading on the other closed forms.
  - **Lesson:** a FLUX unit that touches a decide runs the sweep slow set at all three ε before its PR, not only at default ε.
- 2026-10-19 22:00Z — **Seam note from NURBS (mint-once, build C):** branch `nurbs/mint-walls-once` will add an associated scope type to the `SectionLane` trait at `crates/geom-brep/src/props.rs:4405`, plus a line in each of its 5 impls. Nothing else in `props.rs` changes. It doesn't touch `flux/spiric-budget-world-position`'s hunks, and C merges main before opening the PR. If the trait moves on your side first, expect a mechanical merge.
- 2026-10-19 23:45Z — **Correction to NURBS's 22:00Z seam note:** mint-once (#5419) ended up not touching `props.rs`. The per-wall chart scope lives in `topo::offset_derive`, with no associated type on `SectionLane`, so nothing in it collides with `flux/spiric-budget-world-position`.
- 2026-10-20 — **Flux about each shell's vertex: PR 5442 merges** (dual tier, M, SEQUENTIAL by byte 233; one review, no MAJOR). The certified volume's quadrature budget no longer depends on where a body sits: every face's flux is read about its shell's reference (`shell_reference`, the stored vertex where the outer loop of the shell's first face starts), not the world origin, so the lever is bounded by the shell's diameter. A spiric cut 990 m out and turned now builds all six ops with its at-home pads; the 1 km pipe pose converges; `reach_cone_split` (red on main at 1e-12) passes.
  - **Ruled during the unit:** the uniform rule over a quadrature-only recentre; c per shell, not per walk (the per-walk rule mis-summed an assembled multi-solid certificate, 5.904 against 4.571, and split two doors by an ulp); c the stored vertex, not an evaluated point (Interval width otherwise put the shallow-arc census in band); a refusing closed-form face carried by its own vector area, the whole-walk origin fallback removed.
  - **Review** (`session_01DwbEsp`): APPROVE-WITH-FIXES 0/3/5. The PR's "up to 2×" cost claim was false: against main the pad ratio is diameter(shell)/(the face's own distance from the origin), unbounded and stored-order dependent (6–14× at 1e-9 on torus + bar bodies, 14× between two operand orders; ≤2.6× at 1e-12; nothing refused). Fixed: the cost restated at every site and pinned as known (`one_solid_in_two_face_orders_carries_pads_apart_as_known`); the carry factored and pinned (`MUT_NOCARRY` reds); shell8's exact-rational oracle restored; `rounding_bound`'s lever about c.
  - **Re-baselined:** three pad ceilings (measured, not a bound), the step-export sidecars, the thread-count digest at 3 ε, face-list pins, the boss claim, blend2 P4, the lens bound 4e-9 → 5e-9; each with its cause in the PR body.
  - **Filed:** `a-quadrature-faces-pad-scales-with-its-shells-size-and-stored-face-order` (P2, the cost above: box-centre or per-face c); on CLEAVE, `a-correct-intersect-a-sub-ulp-outside-its-operand-refuses-at-the-backstops-exact-band` (P2; one pose pinned to it).
- 2026-10-20 04:18Z — **Seam note from NURBS (build A, binomial item PR 3):** `props/quad.rs:4376` is the only caller of `geom_core::spline::restrict_bernstein`. It will be routed to `restrict_segment(row, (0,1), (a,b))`, which is bit-identical (the pyramid at point ends), and `restrict_bernstein` retires. The quad change is that one call, pinned by a bit-identity row at quad's inputs. Build A runs `work.py territory` on `quad.rs` first and stops if a FLUX/TALLY branch is live there.
