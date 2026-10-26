---
id: VERBS-C5ARMS
kind: unit
title: the two C5 section arms — cone×cylinder remains
status: closed
opened: 2026-09-01
branch: curved/c5arms-2
closed: 2026-09-05
refs: [1057]
pr: 1864
---

Issue #1057's two arms of the C5 table. PR-1 (plane×torus, `verbs/c5arms-1`)
was dispatched 2026-09-01, stopped at its opening measurement, was held behind
VERBS-TORAX (#1494, merged 2026-09-02), re-dispatched, reviewed at ordinal 111
and merged as #1577 (2026-09-02). PR-2 — `cone_cylinder_section` with the
coaxial-circles variant and the `(Cylinder, Cone)` table arm — is the
remaining half. Spec: `docs/VERBS-C5ARMS-SPEC.md` (its PR-2 section). The
log's tail (ordinal 111 claimed) predates the PR-1 merge; the merge is
evidenced by git history. PR-2 not yet dispatched.

**VERBS closed** (exit walk ratified, PR #1793); re-homed to
`work/issues/` awaiting an owner.

**Adopted by CURVED** at its opening for dispatch (2026-09-04, Ev's
in-chat direction): the plan's lane that carries this item is in
`work/curved/plan.md`.

**Remaining scope at re-home:** PR-2 only — the cone×cylinder section
arm, specced and small in `docs/VERBS-C5ARMS-SPEC.md` (its consumers,
rows 5/6/7/7b, verified unmoved by PR-1). A candidate seed for a
successor program.

## Closed (2026-09-05)

PR-2 merged as #1864 (ordinal 2200, sample #143): the coaxial
cone×cylinder section arm, station-guarded after the dual. Issue #1057's
two arms are both delivered (PR-1 #1577, PR-2 #1864). The spec's
"tier-3 volume pin" acceptance was a category error — `shell(coned_tube)`
succeeds flag-independently (a TORAX row) — recorded in the PR body and
the A/B row. The arm has no production consumer yet; the schedule is
`VERBS-CONE`'s dated note. Residues filed by the orchestrator at
adjudication: `teapot-walls-have-no-suite-row`,
`c5-gate-admits-every-pose-of-an-implemented-pair`.
