# RESTFRONT — log

## 2026-09-27 — opened by ATREST's close

Eleven rows moved from `work/atrest/` by `git mv`, ids kept. ATREST's
thirteen residue rows priced at 32.5 against the 30-point ceiling, so the
two that fit a live program's charter went there instead:
`a-shell-role-is-decided-by-two-spellings` to S-DUP (one thing spelled
twice) and
`validate-classifiers-paraphrase-lower-recourses-with-no-row-comparing-them`
to S-DUP as well (a hand paraphrase of another crate's text; CENSUS fits but stood at 46.5 of 30). The rest
cohere as one track: what tier 3 does not yet examine.

## 2026-09-28 — from CONTACT-4: the reconciliation with ATREST-12, and the seam on the walk

Posted by the CONTACT-4 lane (branch `contact/4-contfp-carriers`), after
merging main with ATREST-12 (#3325), for RESTFRONT's `validate.rs`
ground. **`validate.rs` is untouched.** Check 9's calls keep their
signatures and their answers:
- `point_in_carrier_loop(body, loop, normal, q, band) -> Option<LoopContainment>`;
- `arc_trim(p, ends, apex, anti, lever, &ArcTrimRows, band) -> Sign`;
- `loop_shape`'s `Disc` class.

What CONTACT-4 changes under them in `splitting/containment.rs`:
- **One `arc_trim`, ATREST-12's.** Its step 2 is factored as
  `arc_trim_margin`, which an ellipse's boundary reading uses after
  deciding its ends as exact distances. CONTACT-4's own copy is
  deleted, and `validate-window-arc-arm-folds-onto-arc-trim` is closed.
- **The walk's boundary pre-pass is `LoopEdge::contact`.** An ellipse is
  bounded on both sides, and the end zone is read in metres.
  `boolean::contain::contfp` shares the same pass.
- **Rays keep ATREST-12's retry and distance-trim window.**
- **A spiric or spline edge is held as its own ball.** A ray that could
  meet any such ball is abandoned; `None` comes back only where every
  scheduled ray could meet one. The single whole-loop ball is gone.
  Check 9's `Ok(None)` arm reads the same.
- **`boolean::contain::disc_side` is deleted.** It had no caller left
  once `contfp` stopped dispatching on `LoopShape`.

`check-9-and-classify-contain-describe-contfps-retired-polygon-walk`
is re-homed here, with three sites still standing on main.

Signed: (CONTACT-4 lane)

## 2026-09-28 — from CONTACT-4: an over-wound ellipse window no longer reads as an arc

Posted by the CONTACT-4 lane, for check 9's walk (`Boundary::Verdict`),
which shares `splitting/containment.rs`. `ConicArc::of`'s span rule is
now two-sided for an ellipse:
- **Wound past a period** where the overlap's lower bound `(τ − w)·b`
  is definitely negative.
- **An arc** only where its upper bound `(τ − w)·a` is not definitely
  negative.
- **Otherwise it escalates.** The walk returns
  `PointInLoopError::Escalated`, which check 9 reads as an undecided
  placement, never a verdict.

A circle is unchanged.

**No check-9 row reaches it.** Certification pins both carrier ends
of an edge to its vertices within the zero band (`geom_brep`'s
`carrier_endpoint_{start,end}`). A window whose overlap is longer than
the band therefore cannot be certified onto a one-vertex edge, so no
validation door can carry one. The walk's own row is
`containment::tests::an_over_wound_ellipse_window_is_not_an_arc`.
Check 9's rows in `validate.rs` pass at all three ε rows. `validate.rs`
is untouched.

Signed: (CONTACT-4 lane)
## 2026-09-28 — note from CONTACT: ATREST-12 and CONTACT-4 decided the same four questions in `splitting/containment.rs`

ATREST-12 (#3325) landed after CONTACT-4's branch last took main. Both
units decided the same four questions in `splitting/containment.rs`:

1. where the arc distance trim lives (`arc_trim`);
2. how a ray crossing reads an arc's window (distance trim vs cosine);
3. what an in-band ray crossing does (retry vs escalate);
4. when an arc's span counts as a whole turn.

CONTACT's ruling: main's decisions stand by default. CONTACT-4 rebases
onto them and keeps one `arc_trim` in one home. It may move check 9
(`validate.rs`, RESTFRONT's ground) onto a richer result from that same
function. If it does, the move is minimal and announced here. The one
exception is question 4. If main's span rule reads an arc of 2π minus a
small gap on a very eccentric ellipse (a/b = 20, a 15ε gap) as closed,
CONTACT-4 fixes the rule in place with a two-sided bound and a row. A
delta review then checks the reconciliation.

CONTACT-4 also re-homes two rows it filed under the now-deleted
`work/atrest/` onto this slate:
- `validate-window-arc-arm-folds-onto-arc-trim`
- `check-9-and-classify-contain-describe-contfps-retired-polygon-walk`

Signed: (CONTACT orchestrator)

- 2026-09-28 — Seam note from ENCL: PR 3346 (merged `fb0ec473b8`) adds `geom_core::predicate::KERNEL_DEFECT_ENDING` and `KERNEL_OR_FILE_DEFECT_ENDING`, plus hidden `concat!` macros. A forwarded carrier now labels its repair `Recourse:`, and dead ends take the shared ending. It rewords refusal prose on your ground: `predicate.rs` and `geom/src/curves/fit.rs` (props), knots and spline texts (nurbs/props), validate DEFECT and census (restfront), Boolean `ResultVolumeImplausible` (contact), and editor-core concision rows (tcost/tint). No behaviour changed. Rows filed for the hand-spelled endings on your slates are listed in the PR. (ENCL orchestrator)

- 2026-09-28 — Seam note from ENCL: PR 3347 (merged `2e913b87d0`) gives the offset meters one escalation-routing home, `geom_brep::offset_meters::escalation_recourse`, plus the public lever constants `NORMAL_FLOOR_RECOURSE`/`CURVATURE_HEADROOM_RECOURSE`. `topo::validate::classify_offset_fit` now reads it (its own table is deleted; the checks-window text is unchanged), and unknown names render `MissingRecourse`. It also adds a `validate` pin and the editor-core concision rows. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3363 (merged `c28651d7c3`) adds `geom_core::KERNEL_LIMIT_LAST_RESORT` (a tail, with `kernel_limit_last_resort!` for `concat!`) and `KERNEL_LIMIT_RECOURSE` (the whole no-value sentence, "Recourse: loosen the tolerance, as a last resort; this refusal may indicate a kernel bug worth reporting"). This is the one home for D4 ¶1 (i)'s last-resort ending: a site with no other lever composes from it rather than spelling "loosen the tolerance".
  RESTFRONT: `crates/topo/src/validate.rs` `classify_offset_fit` now splits Improved/SampleCapReached (split the face) from the kernel-limit arms (`KERNEL_LIMIT_RECOURSE`). A new pin, `a_refinement_refusal_ends_in_its_routed_sentence`, holds it. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3351 (merged `e39a5c4cc4`) routes certification refusals per D4 ¶1 as Ev ruled on PR 3352. Recourse belongs to the decision (`CertCheck::ending()`, one table) and the reading belongs to the door: `geom_brep::certify::recourse(check, RefusedArm, Reading::{Build, AtRest, Adopt})`. `CertifyError`/`PlaneNurbsRefusal` `Display` is now payload-only. Each door appends `ending(reading)`. The `certification: ` prefix is gone. `validate.rs` `classify_certify` keeps its own lead and takes the ending from `CertifyError::ending(Reading::AtRest)`. Its own endings exist only where that is `None`. `validate-own-close-levers-follow-the-d4-recourse-ruling` (encl) covers the `SliverDihedral`/`own_close` "lower the tolerance" sites. (ENCL orchestrator)

- 2026-09-28 — Seam note from S-DUP: PR 3393 (`dup/b10-a`) gives the shell-role reading one home, `crate::props::ShellRole::decided_at(BracketEnd, Sign)`. On your ground in `validate.rs`, check 7's `plus_v_decide` now maps the new private `plus_v_read` (the same two decide calls, names, order and laziness). Check 10's `shell_role` calls `plus_v_read` directly, and its `Pass→Outer` re-map is gone. A recorded probe (verdict logs, certificates, results) is byte-identical before and after. No behaviour changed. (S-DUP lane, dup/b10-a)
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". `validate.rs` `classify_offset_fit`: the lead is keyed on the verdict ("may fold" / "may degenerate" for Zero), and the ending comes from `MeterError::ending(Reading::AtRest)`. The pin is `a_meter_refusal_renders_whole_at_rest`. `validate-own-close-levers-follow-the-d4-recourse-ruling` (encl) gained 7 more sites. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3392 (merged `277dcb052b`) splits certify's conflated Zero/Negative verdicts: `IntervalNotForward { verdict }`, `WindingExceeded` routed as sign-certain, the tangent tube as its own `CertifyError::TubeNotSeparated` (`CertCheck::TangentTube`, lever alone at every reading), and `TubeStraddles { verdict: Refused }`. `RefusedArm::ZeroOrNegative` is deleted; `geom_brep::recourse` now holds `Refused` and `Definite`. A zero span stays a defect at every reading (Ev, e1600790f9). `validate.rs` `classify_certify`: span leads back to MISMATCH (defect); the tube's lead is 'its faces are not certainly curving apart along it…', ending in the lever. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3398 (merged `3aac59af62`) moves `topo::validate`'s remaining 'lower the tolerance' endings onto their decisions (D4 ¶1). `geom_brep::recourse` gains `Unsized` and `defect_ending`, moved out of certify. `validate.rs`: `SliverDihedral { check: WedgeCheck }` with per-check leads and neutral wedge/separation levers; planar corner → defect, planar boundary → last resort; `DegenerateTorus { verdict }`; unrecoverable decisions end 'There is no way through yet'; `EDGE_CLOSE` gone; a pin fails on any 'lower the tolerance' outside the scoped coincidence-menu sites. Your row `ring-contact-and-sliver-split-endings-want-their-decisions` was filed from it. (ENCL orchestrator)
- 2026-10-08 — Seam note from CONTACT-10 (branch `contact/10-contain-endings`): `validate.rs` changes in three places. `classify_contain`, `classify_point_in_solid` and `classify_census_cause` now return their recourse as a `Cow`. `close_to_boundary` renders an escalation through `boolean::placement_ending(decision, escalation, diag, Reading::AtRest)`: a carried decision ends as it gives itself (the valued tighten where it is sized), and an unnamed one in its lever, with the unreadable-margin note on a poisoned margin. Before, a poisoned margin ended in the defect ending. The `OFF_BOUNDARY` constant is gone. Your row `census-undecidable-what-cannot-carry-a-valued-ending` was filed from it. (CONTACT-10 implementer)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/tier3_tests.rs`, `topo/src/validate.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: PR 3506 (branch `topo/torus-and-merge-one-story`, not yet merged) edits `validate.rs`: tier 3's `RING_TORUS` is `geom_brep::TorusConvention::Ring.sized()` and `ValidationError::DegenerateTorus`'s fact reads `TorusConvention::Ring.refused(..)`, the one table the Boolean's pierce reads; its rendered text is unchanged. (TOPO, PR 3506 fix pass)
- 2026-09-30 — Seam note from TOPO: In PR 3513 (branch `topo/every-escalation-names-its-decision`), `geom_brep::enters_material`, `enters_material_order2` and `classify_dihedral` return `LeverEscalation { rung: LeverRung, diag }` (the arm gate or the reading) instead of a bare `Indeterminate`, and a decided-zero arm carries its decided margin (`geom_core::k_stats::decide_positive_reported`) where it carried `INVALID`; `validate`'s rim screen reads `.diag` and behaves as before, a zero arm's payload now quoting its margin. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513's second fix pass (branch `topo/every-escalation-names-its-decision`), `topo::validate`'s rim screen reads `classify_dihedral`'s `LeverRung`: the arm rung is `WedgeCheck::Arm` (new), ending in `geom_brep::DIHEDRAL_ARM`, and the wedge stays `WedgeCheck::Dihedral`; `certify_undecided` names `CertCheck::TransversalityArm` (new). `own_close_endings_follow_their_decisions` gains the two arm rows. (TOPO implementer)

- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`, the last unit of that program): the D4 ¶1 (i) recourse GRAMMAR moved in `geom-core`, so refusal text changed across the tree. `COINCIDENCE_RECOURSE`, `NO_DECLARATION_RECOURSE` and `SPLIT_PLANE_RECOURSE` lost their unvalued `", or lower the tolerance"` tail and are now the LEVERS alone; `DEFINITE_COINCIDENCE_RECOURSE` retired into `COINCIDENCE_RECOURSE` (with the tail gone the two were one string). The valued conditional arm has one home, `geom_core::Indeterminate::ending(levers)`, composed through `MarginDiag::sized_recourse`: a site that holds an escalation gets "Recourse: {levers}, or, if this size is intended, tighten the tolerance below {m/K} m", and loses the offer exactly where the margin gives no value. `Indeterminate`'s own `Display` (and `under`) therefore renders a LABELLED recourse now, with each margin kind's first lever folded inside it, so `test_utils::refusal::recourse_markers` counts 1 where it counted 0. `MarginDiag`'s invalid rendering says "NaN or a refused enclosure", not "poisoned". Assertions written as `contains(COINCIDENCE_RECOURSE)` followed the constants; literal pins of "lower the tolerance" did not and were re-baselined. (PROPS implementer)
- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`): three edits in `crates/topo/src/validate.rs`, each named by a PROPS row. (1) `classify_mass_props` now reads `geom_brep::props::PropsCheck` off `PropsError::Escalated` for BOTH halves — its clause and its recourse — and `PropsError::OffSurface` is a new arm, classified `defect: true` with the kernel-or-file ending (an edge off its own face's surface is a premise no valid body violates). (2) `P::QuadratureBudget` composes `geom_brep::props::quadrature_budget_recourse(width_len)`, which is now the one home that sentence has, so the window and the refusal cannot disagree; it names loosening TO A VALUE and ends in `KERNEL_LIMIT_LAST_RESORT`. (3) `unnamed`'s words are held equal to `geom_brep::recourse::not_yet` by a new in-file row, and `too_close`'s two hand-spellings are now `concat!`ed from `geom_core`'s own arm macros. The residue PROPS did not take — `too_close` cannot carry the tolerance VALUE through its `&'static str` return — is filed as `work/restfront/at-rest-coincidence-endings-name-no-tolerance-value.md`. (PROPS implementer)
## 2026-10-02 — a row filed here by FUSE

`tier-three-accepts-two-coincident-duplicate-shells-in-one-solid`: found
by FUSE's review of PR 3897 (the whole-shell `On` verdict) under a
mutation that kept both copies of a coincident shell. `validate.rs` is
this program's, so the row is filed here. Signed (FUSE orchestrator).

## 2026-10-06 — note from CLEAVE: a row moved onto your slate

CLEAVE measured two lamina states (a two-edge self-slit ring in one plane face, and a zero-area
two-edge membrane face) that pass tiers 1–3 and 3′. It filed both rows on its own slate, but neither
is on CLEAVE's ground, so both now live with the program that owns the file:
`validators-accept-a-lamina-slit-or-zero-area-membrane-face` (validate.rs, check 4's wedge rule)
on RESTFRONT, and `tessellator-panics-on-a-self-slit-face-every-validator-passes` (mesh) on TESS.
The two refer to each other. JOIN's PR 4074 is reworking the planar lane's same-position dedup,
which the self-slit relies on, so read that PR before taking the TESS row.

Signed (CLEAVE orchestrator).
- 2026-10-06 — Seam note from SHELL: evidence added to `validate-classifiers-and-lower-displays-classify-refusals-differently`. A designer pair converged on putting the violates/cannot-decide class in the tier-3 verdict itself, which makes that row load-bearing for shell's curved rims. Short notes were also added to `ring-and-scaffold-refusals-read-three-ways` and `check-9-meeting-arms-silent-off-a-plane-…`. (SHELL orchestrator)
- 2026-10-06 — Note from TANG: filed `tier-3-passes-a-face-whose-loop-crosses-itself-at-a-repeated-vertex` at P1 (PR 4129). Tier 3 and 3′ pass a face whose corners overlap at a vertex its loops pass twice. On main a one-shot subtract of two leaning wedges reaches it, and five of JOIN's sweep rows build it. Whether tier 3 refuses it waits on JOIN's design question `two-representations-of-holes-meeting-at-a-point`. The planar check lives at `topo::test_support::meeting::corners_disjoint`. (TANG lane)
- 2026-10-07 — Note from TANG: since JOIN's `split_cones` (main 52cbed0c), `tier-3-passes-a-face-whose-loop-crosses-itself-at-a-repeated-vertex` is not known to be reachable on main: the one-shot subtract that built it now splits the point per cone. The ring-order mirror mutant still reaches it, so the item stays open at P1 (PR 4129). (TANG lane)

- 2026-10-06 — From FUSE: PR 4108 has landed (`crates/topo/src/stands.rs`,
  check 10 reads the vertex → edge midpoint → face interior ladder).
  `check-ten-falls-silent-on-a-shell-whose-every-vertex-touches-another`
  records it as the answer, so its owner may close that row.
- 2026-10-08 — Seam note from ENCL (PR 3431, `encl/collapsed-arm-gates`, merged): a definitely collapsed dihedral lever arm and a collapsed NURBS span meter now refuse as their own decisions instead of folding into a poisoned margin. `geom_brep::enters::LeverEscalation` carries a private gate verdict (`refused`, minted only by `LeverEscalation::arm(gate)`; re-quote with `with_diag`, read with `collapsed_arm()`), and struct literals of it no longer compile outside `enters`. New: `CertifyError::ArmCollapsed`, `ValidationError::NoDihedralArm` (pncad tag `no_dihedral_arm`), `CertCheck::ParamSpanMeter`/`SpanMeterCollapsed`, `recourse::Refused::rejected`; `DIHEDRAL_ARM` has an `at_zero` note (cone apex); the arm texts now read "long enough … to measure the angle between them". `LeverEscalation::of_rung` is gone; the boolean seam routes by rung through `BooleanDecision::of_lever`. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4423, `encl/second-order-walk-one-home`, merged at `c981d151cb`): the edge-level second-order walk now has one home. `geom_brep::interior_stations` owns the interior station schedule (`1..CERT_SAMPLES-1` via `sample_param`, as `ders1`). `geom_brep::second_order_walk` (with the `StationHook` trait and `SecondOrderWalk::{Determinate, UnderDetermined, InBand, Stopped}`) owns the per-station reading and the one `"tangent_second_order"` decision. Both are `#[doc(hidden)]`: they are kernel-internal and public only for `topo`. `must_carry_over_edge` and tier 3's check 4 both descend through it. A source scan (`tier3_tests::check_4_routes_its_second_order_reading_through_the_one_walk`) reds on a second spelling in `validate.rs`. A new edge-level second-order reader should take the walker, not inline the loop. No answer moved (the k-stream is byte-identical). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4433, `encl/rim-wedge-one-walk`, merged at `0bdb2baeae`): `geom_brep::second_order_walk` now takes its stations (`impl IntoIterator<Item = (Point3, Vec3)>`) instead of `carrier, t0, t1`. `topo::validate::MaterialStations` (`pub(crate)`, `new`, `outcome`) is the one home of the per-station material reads, shared by tier 3's check 4 and `boolean::rim_wedge::classify_shared_rim`. The rim's own station loop and its cusp-side Zero mint are gone; the Zero arm is now `MaterialStations::after_positive`. Tier 3 reads `interior_stations` once for checks 4 and 5. A source scan (`tier3_tests::check_4_and_the_rim_route_…`) reds on a second walk in either file's production code. No answer or text moved. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4443, merged at `070cec41da`): the "X is too close to call at this tolerance" sentences in `topo::validate`, `topo::census` and `geom_brep::props::PropsError::Escalated` now read "X is undecided" (one composer: `Indeterminate::undecided` / `geom_core::undecided!`). Tolerance advice lives in the ending, per D4 ¶1 (i). Not-yet endings join a note with `"; "` via `geom_core::noted`, and the `Recourse:` label is `geom_core::Recourse`. A new refusal on your ground should use these instead of hand-spelling. The remaining "too close to call" sites are listed in ENCL's `too-close-to-call-remainder`. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4457, merged at `dcff36e784`): the import-door twins `ending_in_file`, `render_in_file` and `recourse_in_file` are removed. The one body now takes `at: impl Into<ReadAt>`, so the import door is the same call without the suffix: `err.render(file)`, `err.ending(file)`, `decision.recourse(arm, file)`. An in-flight branch that calls a `*_in_file` will fail to compile at merge; drop the suffix. `Unsized::residual_in_file` and geom-core's `sized_recourse_in_file` / `miss_recourse_in_file` keep their names. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4461, merged at `a71723601f`): `topo::ShellRead::of` now returns `Result<Self, ShellClassifyError>`, with no `Option`: the unreadable selection was unreachable. `SolidFaces::of_shell` returns `Self`, and `ChartGroups::of_live` groups faces already read off the live arena (`of_body` uses it). `PieceSortError::RoleUnread` carries the role read's `ShellClassifyError` and renders it. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: your `validate-material-side-zero-mints-an-indeterminate` (the cusp-side Zero in `MaterialStations::after_positive`) is folded into ENCL's `material-pairing-gate-definite-zero-ends-as-unreadable`, together with the material-pairing gate's own decided Zero. ENCL will close your row with that unit. (ENCL orchestrator)
- 2026-10-10 — Seam note from PIPE (PR 4486, `pipe/census-arms-and-witness-caps`): `ChartRegionError::WitnessBudgetExhausted { segments, cells }` is split into `WitnessSegmentCapExceeded { segments }` and `WitnessCellCapExceeded { segments, cells }`, and `topo::WITNESS_BUDGET` / `WitnessBudget` into `topo::WITNESS_SEGMENT_CAP` / `WITNESS_CELL_CAP`. Every census and validate mapping keeps one answer for both faces. A match in your files names both. (PIPE implementer, witness-cap lane)
- 2026-10-10 — Seam note from ENCL (PR 4475, merged at `61977302ae`): a poisoned margin's note now depends on the door that reads it. At a build it is "an unreadable margin may indicate a kernel bug worth reporting" ("or collapsed" is gone). At rest or at the import door it is "…a kernel or file defect worth reporting". `geom_brep::recourse::unreadable_margin_note(reading)` picks it. geom-core's `SizedWords` gained `unreadable: &str`, and `Indeterminate::ending_noted(levers, unreadable)` exists beside `ending`. An at-rest reader of a nested refusal composes its own ending (fork-log row 9): it does not add `ending(Reading)` to the type. `topo::props::SHELL_ROLE`, `ShellClassifyError::arm()`, `JoinUndecided::diag()` and `JOIN_SUBJECT`/`JOIN_LEVER` are public for that. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4474, merged): the material pairing is a decision. `geom_brep::MATERIAL_PAIRING` and `MATERIAL_PAIRING_CLAUSE` end it at tier 3 (`WedgeCheck::MaterialPairing`) and split finish (`SplitFinishError::DescribeSideEscalated`), with a true tolerance offer. Where the wedge would re-decide below m/K, `pairing_at_wedge` quotes the wedge and offers w/K, as `at_wedge` does. `MaterialStations`' `Break` is `MaterialStop { check, cause }`. The cusp-side Zero reads via `decide_nonzero`. New k_stats predicates appear on the refusal path only: "material_pairing_offer_wedge" and "material_pairing_wedge". (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4497, merged at `4c7fb9b0fc`). A poisoned or contradicted margin now ends in a followable recourse:
  - at rest, the census coincidence menu (`validate::too_close` → `own_close`) gives the defect ending;
  - `ChartRegionError::Escalated`'s poisoned arm gives `defect_ending(Build)`;
  - `PointInSolidError::Escalated` gives the unnamed placement lever plus the reading's note, as `contfp`, `classify_point_in_solid` and the Boolean already did.

  `SliverSector` is unchanged, and its mints are on CLEAVE's split-escalations row. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4504, merged): `geom_brep::recourse::Unsized::LastResort` is renamed `Unsized::Fit`, with the same behaviour, and a new `Unsized::Bound` ends a refusal on a certified upper bound in the kernel-limit last resort at every reading. At the import door, `MissReading::Bound` names the bound. `CertCheck::bounds_a_miss()` answers whether a check is a bound. A table on your ground that refuses on a bound, not a measured miss, should take `Bound`. (ENCL orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4538, merged): C2 limb 2 (`ssi_hull_sup`, `ssi_hull_sup_chart`) now subdivides its Bernstein composite before refusing. It stops when the whole-carrier bound clears ε, when a break value is past the band, after two rounds that set no new lowest bound, or when the budget runs out (`SSI_HULL_WORK` 4096 spans recomputed, `SSI_HULL_CUTS` 1024). It reads the limb on the lowest round's hull. Two new refusals: on the plane × NURBS arm, `SsiLimb::HullValueChart` → `CertCheck::PlaneNurbsHullValue` (Bound ending: the carrier against its pcurve's surface point bounds the miss from above), and on the analytic arm, `SsiLimb::HullValue` → `CertCheck::AnalyticBreakResidual` (Fit). Refusals that used to read as a loose hull bound can now read as a measured miss, or certify. (ENCL orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4612, merged): seven geom-brep Displays now say "is undecided" where they said "is too close to call": edge_nurbs, newell, nurbs_iso, offset, offset_meters (now "whether this face can be offset is undecided"), and ssi's `Escalated` and `CertificateEscalated`. Tests that assert those sentences must use the new words. (ENCL orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4629, merged): `geom_core::Indeterminate::ending` now carries its margin kind's lever. A straddling enclosure that is not a terminal sliver gets "subdivide the parameter box for a tighter enclosure, or …". An unreadable margin with no reader's note (a build) gets "check the operation's inputs upstream, then …". `ending_noted(levers, note)` (at-rest and import readers) keeps the reader's note and adds no inputs lever. `under()` is byte-identical except on a terminal sliver, which no longer offers subdivision. Whole-escalation renders take the one undecided shape: `cause.undecided(subject, cause.ending(recourse))`. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4800, merged at `428057bb7c`): `topo/src/validate.rs`'s offset-fit recourse arm folds `SampleCapReached` into `WallReached` (both walls: split the face). (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4855, merged at `5903a8c5a2`): `geom_brep::recourse::SizedDecision::recourse`'s Undecided arm takes the decision's `at_zero` note when the cause carries a gate-rejected Zero, sharing the Zero arm's body. The note shows only where no smaller tolerance decides passing (a rejected-side zero, a proven zero, or an Interval straddle in the zero band). Readers of `DIHEDRAL_ARM` (merge_faces, validate, refusal_routes Seam, certify) and offset_meters' cell decisions gain the note on that route. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL: PR 4874 merged (`encl/fitted-escalated-keeps-decision`).
  - `pcurve_cache`'s `FittedEscalated` keeps its SSI decision (`SsiDecision {Trace, Limb}`) and ends through `SsiDecision::ending`, which takes `impl Into<ReadAt>`.
  - Its Display and `PcurveCertifyError::Escalated`'s now end in the decision's own labelled recourse at `Reading::Build`, so the doors that print the Display (`PcurveMintError::Certify`, step-import, `replace_face`, loft, blend) carry one ending each. Both texts are pinned literally.
  - `PlaneNurbsRefusal::ending` ends `Escalated` through `SsiDecision::Limb`.
  - Rows filed:
    - PCERT `a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect`;
    - CLEAVE `chart-pcurve-and-props-escalations-lose-their-check-at-split-join-and-point-in-solid`;
    - `work/issues/describe-maps-every-chart-pcurve-escalation-to-chart-image`;
    - ENCL `unsized-recourse-takes-a-gate-tagged-arm-and-renders-no-gate-word`.
  - Pointers were added on the PCTAIL coincidence-menu row, the CHROME escalations-forwarded-whole row and the EXCH step-import row. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4925 merged. ε now has a floor: geom-core `tolerance::SESSION_BOX_EXTENT` = 1 km and `EPS_MIN` = box · 2⁻⁵² ≈ 2.2e-13 m, per D4 ¶4 (Ev, PR 4836).
  - Every door that sets a run's ε refuses below it. `tolerance::EpsFloor` is the one wording of the refusal.
  - `Band::offer_deciding(m)` is the one offer rule: no tighten offer at or below ε_min. Tests that predict an offer should call it rather than restate `|m|/K`.
  - At CI's 1e-12 row, every zero-band offer now withholds.
  - A run below the floor (for example `CAD_TOLERANCE_EPS=1e-14`) is refused. Outside geom-core it silently falls back to the default; that is filed on SCALAR. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4943 merged. A sized decision's ending now depends on one classification of the reading, `MarginDiag::shows` (geom-core): Offer, NoSize, Floor, Unresolved, Past or Unreadable.
  - The decision's note (`SizedWords::no_size`, renamed from `at_zero`/`otherwise`; geom-brep `NoSize`, renamed from `AtZero`) appears only on NoSize: an exact zero, the refused side, or a theorem.
  - A positive reading below the floor gets the one `floor_note` sentence.
  - A reading that touches or straddles zero gets the lever alone.
  - The import door gives its sentence plus the lever, and never a decision note.
  - PR 4855's rejected-sign filter is gone.
  - New sized decisions take `no_size:`.
  - Tests that predict an ending should call `MarginDiag::sized_recourse`, not restate the rule. (ENCL orchestrator)
- 2026-10-13 — **Seam (ENCL):** PR 4976 merged. Every geom-brep Zero-arm payload (`CertifyError`, `TorusConvention::refused`, `MeterError` NormalFloor/CurvatureHeadroom) now reads "is not shown … (margin {:e} m)". Whether a reading shows no size is left to the ending (`MarginDiag::shows`). A row PR 4976 filed on this slate asks for the same idiom on this ground. (ENCL orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5023 merged. At a build, a refused reading past the band now ends like its in-band sibling (D4 ¶1 (iv)): the lever plus the decision's `no_size` note. geom-core `MarginDiag::shows` gains `Shows::Refused`. `validate.rs` pins follow `DIHEDRAL_ARM`'s new note ("…a radius below zero is a defect"). (ENCL orchestrator)
- 2026-10-14 — **Seam (PATHS):** PATHS took and closed this slate's P0 `circle-pair-edge-candidates-cancel-at-unequal-radii` on `paths/circle-pair-shared-home`: `validate.rs` `circle_pair` and arm 5's circle × circle candidates read `geom_core::Circles`. (PATHS)
- 2026-10-14 — **Seam (ENCL):** PR 5101 merged. At rest, one definite refused reading ends one way whichever arm routes it: `StoredDefinite::Lever` gives the lever plus the note; `Contradiction` gives `KERNEL_OR_FILE_DEFECT_ENDING`, at rest and at the door past ε_in. Filed `an-at-rest-certify-lead-calls-a-gates-definite-rejection-undecided` here: `classify_certify`'s "…is undecided" lead now sits over a defect ending. (ENCL orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5138 merged. `CertCheck::ParamSpan`'s lever now reads "move the geometry so this edge runs forward over a clear length", one word longer. Re-baselined pins in `validate.rs`. (ENCL orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5492 merged (`ef1b70f1`, NURBS build A, PR 2 of 4 for CPAIR `two-spans-projections-on-one-line-are-read-four-ways`). Check 9's `lines_meet` (`crates/topo/src/validate.rs`) reads the one-line gap first, on both axes (`gaps_apart`, row `ring_outer_meet_gap`), from `geom_core::one_line`, with `produced = 0` (stored vertex positions, traced). Its `overlap` closure now reads the same home's charged extent from both sides (Meet / Apart / straddle); the uncharged copy is gone. **Verdict change:** `review_cleave_farplane::booleans_beside_a_far_carrier_never_answer_wrong`'s six "through the top" ∪/∖ builds at k = 2, 5, 9 now BUILD (volumes equal the closed form within 3.6e-15 at default ε, 1e-6 and 1e-12; topology and validation pass). The old `RingContactEscalated` was a false escalation. No golden moved. (NURBS orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5504 merged (`cbbe917a`, NURBS build A, FULL). `geom_core::one_line::on_one_line`'s charge is now taken at the spans' own scale, `rounding_charge(produced + |a1−a0| + |b0−a0| + |b1−a0|)`, instead of their distance from the origin. The derivation is in the module docs (9u per projection, 10u over the extent, under the shared 16u). Both callers are covered: `curve_pair::pose::line_line` (with `produced` for evaluated ends) and check 9's `lines_meet`, `gaps_apart` and overlap closure. Noise for metre spans 10³ m out falls from ~7e-12 to ~5e-15. A row pins the scaling (red under the old charge). Every caller's fuzz is green with the charge and red without it, and an exact-arithmetic hunt over 120k draws found a worst share of 0.084. No golden moved. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5513 merged (`9866210b`, NURBS build A, FULL). This is PR 4 of 4 for CPAIR's `two-spans-projections-on-one-line-are-read-four-ways`. `topo::census` reads its collinear span pairs (`collinear_overlap`, `span_clear`, the `ee_overlap` midpoint) from `geom_core::one_line`. Below a threshold (the charge under (K−1)ε, divided by `rounding_charge(1)`) the end comparisons stay uncharged, as before. Past it they are charged straddles on m ± noise (`pm_census_span_order_charged` / `pm_census_ee_overlap_charged`), escalating on `pm_census_span_rounding` only when the two disagree; there is no blanket escalation. Ends that are one point read Zero without their projections: a shared point key, or two stored vertices that `pm_census_vv_gap` (the vertex pass's own decision) reads as one. A touch at a stretch's computed end can still escalate past about 1.3 km at 1e-12, as the doc says. The new user-facing story is `BooleanDecision::LineSpanPrecision` at the result gate (routed by `is_span_precision`) and `CensusEscalated` rendering `census::SPAN_PRECISION_RECOURSE` at validate, which names geometry levers only (D4 ¶1 (i)). The flip report gains `decision_words` rows. The 3 km bar union builds at every ε; the new rows are in `crates/topo/tests/long_edges_on_one_line.rs`. No golden moved. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5542 (CPAIR's one-line PR 3) reads `profile::seg::line_line` in check 9's order, with one change: a collinear pair is read before the other pair's in-band side escalates. Check 9's `lines_meet` keeps its order. Its hole reproduces at the function level, and reaches through validate only in a sliver past arms 1–3 (`lamina_with_ring`, at a gap of just over `Kε`), where it gives the same verdict, an escalation, under `ring_outer_meet_side` instead of `ring_outer_meet_overlap`. Filed as `check-9-lines-meet-escalates-an-in-band-side-before-its-collinear-arm` (P3) so that the two doors can be made one reading.
- 2026-10-21 — **Seam (NURBS):** NURBS build A's adjacent-pair PR (`nurbs/adjacent-shared-vertex`) closes `paths/an-adjacent-pair-through-a-joint-straight-within-the-band-escalates-on-chord-side`, retitled "An adjacent line pair escalates at its joint, not its pair" and re-rated P3. The item proposed the rule for check 9's `lines_meet` too. It has no subject there: check 9 pairs a ring's edges with its outer loop's, and a vertex the two loops share is arm 1's contact before any edge pair is read. Pinned in `crates/topo/src/validate.rs` by `check_9_refuses_a_ring_sharing_a_vertex_through_a_straight_joint_at_that_vertex`; no code changes in topo.

- 2026-10-23 — **Seam (QUAD):** PR 5589 (`quad/measure-a-lanes`, measure door unit A) re-words docs in `topo/src/validate.rs`: the recourse no longer names a round cap. No code change. (QUAD orchestrator's lane)

- 2026-10-23 — **Seam (QUAD):** PR 5628 (`quad/measure-b-door`, measure door unit B) edits `topo/src/validate.rs`: check 7 runs every solid's round 0, then one `role_walk_set` over the solids (`PLUS_V_SIGN`), each solid stopping where it stops alone. It refines past the reporting target while a sign straddles. (QUAD unit B lane)

- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
- 2026-10-25 — **Band rule (work/README.md, Priority):** split `material-arm-split-ends-as-a-hand-minted-invalid` (P1, `design: true`) out of the unbanded `ring-contact-and-sliver-split-endings-want-their-decisions`: the Split arm's hand-minted `INVALID` (`material_arm_error`) is the part CLEAVE's `the-indeterminate-seal` (P1) waits on, and the seal is re-parked on the new row. The original keeps its other parts and stays unbanded. (band-rule triage)

- 2026-10-26 — seam from ATREST (stage 5 C, `the-at-rest-census-is-a-check`, branch `atrest/the-at-rest-census-is-a-check`): `topo/src/validate.rs`: one comment naming `editor_core::assembly::attribute`/`AssemblyError` restated; no code.
- 2026-10-27 — **Seam (ATREST):** `atrest/a-crossing-touch-is-interference` touches this program's ground. A touch between two solids that the census's touch analysis decides is a crossing is now `ValidationError::TouchInterference { a, b }` (the touch's two `Cell`s), which `census_interference_solids` reads; `Undecided::MixedTouch` ("one passes into the other where they touch") retires. A pair that a standing finding already decides interferes (a pierce, a same-side crossing, two coincident edges' wedges overlapping) no longer also refuses `CensusUndecidable` beside it. New exhaustive-match arm: `boolean/ops.rs`, `checks/at_rest.rs` `finding_solids`, pncad-py `touch_interference`. Restated rows: `contact5_gate_and_beam` (the sunk beam and ridges), `contact7_touch_sweeps::refused`, `coincident_line_wedges` (the pinwheel), `a_vertex_read_by_two_sector_passes` (its allowlist). (ATREST crossing-touch lane)

- 2026-10-27 — **Seam (ATREST):** `atrest/curved-copies-filling-one-volume-interfere` rewords `ValidationError::SameSideInterference`'s `Display` in `crates/topo/src/validate.rs`: "a face of each lies on one surface facing the same way" instead of "one plane", since the census now decides the finding on cylinders and spheres too. No variant or field moved. (ATREST curved-copies lane)

- 2026-10-29 — **Seam (FOREIGN):** `validity-refuses-an-interior-chart-singularity` carries a re-aim note from FOREIGN's designer pair: the rule is "every chart singularity in a face's closure is a vertex of it", read off the region, not "the loop has no meridian". FOREIGN's import door (`import-normalizes-the-rim-only-cap`) is dispatched on that reading and mints today's two-half pole form, so the row can land as a plain refusal when it closes. (FOREIGN orchestrator)

- 2026-10-29 — **Seam (FOREIGN):** `[ev]` PR 6014 ruled the wrap tip at a chart singularity (one wrap edge, walked out and back; DESIGN `[topology]` seam clause and tiers 2/3) and, for import's C⁰-knot split, option (i): promotion first, the exact split, per-span quadrature schedules. Rows filed or updated on your slate from it are listed in that PR's body; the wrap-tip rows are parked on `work/issues/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`, which lands first (Ev: keep the complexity down). (FOREIGN orchestrator)

- 2026-10-29 — **Split (RESTFRONT orchestrator).** At 133 budget points against 30 (45 dispatchable rows), the track split along what tier 3 is reading. Rows moved by `git mv` with ids and bodies unchanged:
  - **RESTFRONT** keeps a planar face's region: check 9's ring nesting and line/arc arms, loop simplicity, the slit and membrane, check 6's remaining carriers, the corner rule (19 points).
  - **RESTCURVE**: the same reading off the plane — the P0 chart singularity, curved-boundary containment, check 9's meeting and corner arms on curved faces (28).
  - **RESTSHELL**: checks 7 and 10, two solids' overlapping material, duplicate shells, the sense bit (25).
  - **RESTGATE**: what `AtRestBody` and `gate_unverdicted` admit a finished body or operand on (15).
  - **RESTDATUM**: checks 1 and 4, a carrier against its datum and its faces (16).
  - **RESTWORD**: the ENCL-descended ending and classification rows, and k_stats past topo's funnel (30).
  - Re-priced on the move: the four legacy `D` rows to `M`; `tier3-never-checks-a-planar-loop-is-simple` P1 M, `check-4-gives-no-arm-verdict-…` and `transverse-not-intrinsic-…` P3 M. RESTWORD's six unbanded ENCL rows keep no band until its orchestrator reads them.
  - The D10 hold (`work/intent/d10-one-way-to-say-intent-is-unbuilt`) parks no row of this slate; stage 5 C (the census as a check) is closed, so the census-adjacent rows (RESTSHELL's overlap, RESTGATE's finished body) build on the census as it stands.
