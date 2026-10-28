---
id: a-verb-door-hands-back-any-familys-record-and-the-reader-refuses-the-foreign-one
kind: issue
title: each verbs door hands back the whole VerbRecord channel, so every correspondence refuses a foreign family as NamingError::Emission; the door's own record type would make it unrepresentable
status: open
opened: 2026-10-20
priority: P3
cost: M
---


Filed by `wire-evalarms` (WIRE), alongside
`eval-typed-defect-arms-go-by-row-zero-or-panic`.

## Finding

`verbs`' run doors are methods of their payload types now (`Blend::run`,
`Boolean::run_pair`, `Sweep::run_profile`, `Split::run_split`,
`Shell::run_shell`), so no verb reaches a door that does not answer it.
Each door still hands back the whole closed channel (`VerbRecord`, in
`VerbOut::record`, `SplitOut::record`, or `run_profile`'s `Ok`), and
the document layer reads its family's arm back out:
`crates/editor-core/src/verbs/mod.rs`'s `read_record` refuses any other
arm as `NamingError::Emission { what: foreign_record }`, and each
correspondence (`verbs/{blend,boolean,sweep,split,shell}.rs`) carries a
`foreign_record` sentence for it — "a kernel bug surfaced typed,
unreachable while the doors and the correspondences agree".

D9 `[failure.taxonomy]` row 0 (PR 5411): the state is unrepresentable
if each door returns its own family's record type — the blends'
`Option<BlendNaming>`, the boolean's record fields, the sweeps'
`Extruded`/`Revolved`, `SplitNaming`, `ShellNaming` — which deletes
`read_record`'s refusal, the five `foreign_record` fields and their
census rows. The cost is local to `verbs/src/run.rs` and the
correspondences' `RecordReader`s.

Out of `eval-typed-defect-arms-go-by-row-zero-or-panic`'s scope because
its raise is a `NamingError::Emission`, whose sites are their own rows.

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).
