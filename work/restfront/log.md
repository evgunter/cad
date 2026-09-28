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
