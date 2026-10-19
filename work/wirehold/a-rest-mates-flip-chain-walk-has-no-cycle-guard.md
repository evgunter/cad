---
id: a-rest-mates-flip-chain-walk-has-no-cycle-guard
kind: issue
title: mate::rested_faces walks a Flip chain of document vars with an unbounded loop and no cycle guard
status: parked
opened: 2026-10-19
blocked_on: [a-mate-relates-two-poses]
priority: P3
cost: E
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
