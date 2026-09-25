---
id: ratification-claims-in-session-docs-name-no-ruling
kind: issue
title: Session and form docs call something a ruling that no ruling of Ev's decides
status: open
opened: 2026-09-24
priority: P3
cost: E
---

Found by VNEWS's census,
`work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`
(§The census, population 2), at merge base `f4b178189`. That section
holds the searches. This row is the list of members on VSEAM's ground.

**FALSE: the "ruling" these docs cite is not a ruling of Ev's.**

- `crates/viewer/src/session/op.rs` `SessionOp::NewDocument` (:423) and
  `crates/viewer/src/session.rs` `DocSession::new_document` (:1937)
  both cite *"the identity ruling (logged in `docs/GAUTH-LOG.md`)"*.
  The GAUTH log (readable at `a1425f92f^:work/gauth/log.md`) files it
  under *"Unilateral decisions at opening (Ev reviews
  retroactively)"* and records no review. The GAUTH plan calls it the
  orchestrator's and says *"nothing here ratifies an open design
  question"*. The same claim at `session/refuse.rs`
  `Refusal::EmptyName` (:221) is inside VNEWS's fence and stays with
  the census.
- `crates/viewer/src/forms.rs` `PatternKindChoice` (:52) and
  `crates/viewer/src/session/author.rs` `PatternRuleSpec` (:220) say
  `Explicit` is absent *"by the plan's ruling"*. The source is the
  GAUTH plan's unit-4 spec (`58500afb1`, agent-written): *"`Explicit`
  is not a form's job"*. Ev ruled that plan's scope, not this line.
- `crates/viewer/src/pickindex.rs`, the comment on its
  `editor_core::resolve` import (:81), calls "a direct edge, never a
  new re-export" *"the ruling `pncad`'s own crate docs state"*. Its
  only statement is `crates/pncad/src/lib.rs`'s crate docs
  (`079632988`, agent). A pickaxe search for `direct edge` over
  `work/` and `docs/` history returns nothing.

**PARTIAL: one half is ratified.**

- `crates/viewer/src/session/select.rs` `Standing` (:274) calls
  vanished-is-a-state plus *"the affordances that need a live entity
  switch off"* the whole of *"GQ7's recorded constraint (tools survive
  the referenced entity vanishing)"*. The parenthesis is GQ7's own
  text as first written (`5267a9193`), ratified through the GUI plan's
  GUI-2. The switch-off clause is in no design document.
- `crates/viewer/src/session/probe.rs` `evaluate_with` (:269) cites
  *"the seam's ruled cancel-and-restart policy"*. The GUI plan's ruling
  is *"busy indicator + the shipped `CancelToken`"*. Restart is the
  implementation (`a9a660b83`).

**The fix is a word, not a design.** Say whose decision each one is,
or cite the clause that holds. None of these needs Ev: the claims that
turn out FALSE only stop asserting a gate that never existed.
