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
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/tier3_tests.rs`, `topo/src/validate.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: PR 3506 (branch `topo/torus-and-merge-one-story`, not yet merged) edits `validate.rs`: tier 3's `RING_TORUS` is `geom_brep::TorusConvention::Ring.sized()` and `ValidationError::DegenerateTorus`'s fact reads `TorusConvention::Ring.refused(..)`, the one table the Boolean's pierce reads; its rendered text is unchanged. (TOPO, PR 3506 fix pass)
- 2026-09-30 — Seam note from TOPO: In PR 3513 (branch `topo/every-escalation-names-its-decision`), `geom_brep::enters_material`, `enters_material_order2` and `classify_dihedral` return `LeverEscalation { rung: LeverRung, diag }` (the arm gate or the reading) instead of a bare `Indeterminate`, and a decided-zero arm carries its decided margin (`geom_core::k_stats::decide_positive_reported`) where it carried `INVALID`; `validate`'s rim screen reads `.diag` and behaves as before, a zero arm's payload now quoting its margin. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513's second fix pass (branch `topo/every-escalation-names-its-decision`), `topo::validate`'s rim screen reads `classify_dihedral`'s `LeverRung`: the arm rung is `WedgeCheck::Arm` (new), ending in `geom_brep::DIHEDRAL_ARM`, and the wedge stays `WedgeCheck::Dihedral`; `certify_undecided` names `CertCheck::TransversalityArm` (new). `own_close_endings_follow_their_decisions` gains the two arm rows. (TOPO implementer)

## 2026-10-02 — a row filed here by FUSE

`tier-three-accepts-two-coincident-duplicate-shells-in-one-solid`: found
by FUSE's review of PR 3897 (the whole-shell `On` verdict) under a
mutation that kept both copies of a coincident shell. `validate.rs` is
this program's, so the row is filed here. Signed (FUSE orchestrator).
