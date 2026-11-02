---
id: a-rest-mates-flip-chain-walk-has-no-cycle-guard
kind: issue
title: mate::rested_faces walks a Flip chain of document vars with an unbounded loop and no cycle guard
status: closed
opened: 2026-10-19
priority: P3
cost: E
closed: 2026-11-01
---


Found by the STYLE review of PR 5405 (`wire-chase-rev`), outside its
scope.

`crates/editor-core/src/mate.rs` `rested_faces` (~line 490) follows
`PoseDef::Flip { pose: RoleRead { read, role: None } }` from var to
var in a bare `loop`, with no bound and no visited set. Whether
document validation rules out a Flip cycle among a document's vars was
not checked. If it does not, an authored document can hang evaluation
(D9 row 1: a typed refusal at the door). If it does, the walk should
cite that check.

Mates are D10 ground, so this is parked until the mates rebuild settles
what a pose chain is.

## Opened (2026-10-21)

`a-mate-relates-two-poses` closed and nothing else gates this row.
`mate::rested_faces` is gone; the walk that follows a `PoseDef::Flip`
from var to var now lives in `eval/wire/pose.rs` `own_direction`, which
recurses through each `RoleRead<VarId>` with no depth bound and no
visited set. The question stands there: whether document validation
rules out a pose cycle among a document's vars, and if it does, the
walk should cite that check.

## Re-checked after D10 (2026-11-01, WIRE orchestrator)

From the read-only `wirehold-triage` against origin/main 8509df2510. INTENT closed on 2026-10-30, and every trigger this row waited on has fired.

**Moot: a cycle cannot be represented.**
- `definition_edges` includes `VarDef::Pose` (`doc.rs:1338-1345`), and `definition_reads` takes a pose's reads (`doc.rs:1316-1319`). So `Doc::expansion_fault` refuses any definition cycle (`doc.rs:1789-1799`), at the edit door (`edit.rs:6124`) and at load (`persist/check.rs:858`).
- The walks are now at `mate.rs:177` (`poses_equated`'s `unwound` loop), `mate/value.rs:117` (`flipped`) and `eval/wire/pose.rs:408`.
- Neither `while` loop cites the check that makes it finite. That one-line citation rides `missing-slot-stays-typed-on-the-held-placement-datum-and-pattern-readers`.
