---
id: declared-joint-kind-zero-margin-reads-smooth
kind: issue
title: profile: a declared joint whose heading margin is Zero at validation is silently recorded smooth, and its guided escalation is not the typed indeterminate its siblings raise
status: open
opened: 2026-09-26
priority: P3
cost: E
---


Found by PR #3257's delta review (MINOR-1, MINOR-2, and a NOTE).

- **Zero reads smooth.** `profile::seg::junction_reverses`, as
  `validate.rs::judge_joints` uses it, maps a `Zero` margin to "does not
  reverse", so the joint is recorded smooth: `ValidatedLoop::cusp_joints`
  omits it, the sweep carries no `Tangent` record for it, and
  `blend_arcs` lists the arc as a blend. The sweep predicate this
  replaced escalated on `Zero` (`decide_nonzero`). The doc's "cannot
  reach" argument is incomplete: the arm is bounded below by the leg's
  length, not by the band, so a major arc of radius about half the band
  gives an arm under the band. Sub-tolerance geometry that other gates
  may refuse first; no fixture built. At the path door the same `Zero`
  mapping only chooses between two refusals, so it is harmless there.
  Fix: at validation, `Zero` escalates.
- **Guided shape.** Under guidance an escalation of the heading surfaces
  as a bare `ProfileError::Escalated { SegmentPair }`, not as
  `Structure(StructureRefusal::indeterminate(Decision::CuspJoints { .. }))`
  the way the containment forest's does (the typed indeterminate is the
  bisection cue, `structure.rs`'s module doc). The tangency verification
  in the same loop has the same shape.
- **"One home" overclaims.** `path.rs`'s `seam_arrival_check`
  (`path_seam_arrival_side`) asks the same levered-dot reversal question
  inline, while `junction_reverses`' doc calls itself the one home.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: judge_joints/junction_reverses classify declared tangent_joints (validate.rs) into cusp and Tangent records; stored tangent-joint flags retire at stage 4. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by stage 4 G (`tangent-joints-are-derived`)

The stored tangent-joint flags are gone: `ProfileLoop` holds none, and
`validate.rs::judge_joints` derives the tangent-joint set from the
constructed joints a `ConstructedLoop` carries and the junction
verdicts, recording each joint no constructor made
(`ValidatedLoop::decided_joints`). `junction_reverses` still maps a
`Zero` margin to "does not reverse", now for every tangent joint between
two carriers, constructed or decided (a same-carrier joint is not asked:
its retrace is refused `NonSimple` first), so all three bullets above
stand as written and the row is dispatchable.
