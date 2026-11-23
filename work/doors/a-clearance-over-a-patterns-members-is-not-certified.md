---
id: a-clearance-over-a-patterns-members-is-not-certified
kind: issue
title: A clearance requirement over a pattern's members is refused, not certified per member
status: open
opened: 2026-11-02
priority: P2
cost: M
---

Found by the correctness review of PR 6357 (M1).

A clearance seat that reads a pattern's members — a tree pick of the
pattern node, as a body seat — lifts the assertion over the family:
its value is `ValuePayload::Family`, at `f64` and at `Interval` alike.
Until PR 6357 the certify door (`pncad::analysis::clearances_at_nominal`)
reached `unreachable!` on that value, and the viewer's certify worker
took the process down (`Coalescing::crashed`).

Now the door answers `ClearanceOutcome::OverMembers` — "… reads the
members of a pattern, so it is one requirement per member, and a
clearance over a pattern's members is not certified yet. Recourse: seat
one member — project it with the projection tool and pick the
projection" — the viewer's row says the same at `f64`
(`viewer::tree`'s `readout_of`), and `DocSession::land` no longer
submits a family (`holds_a_clearance` asks for a clearance verdict).
A face of one drawn member, picked in the picture, already seats that
member alone and is certified
(`viewer/tests/clearance_consumer.rs`'s
`a_requirement_over_a_patterns_members_refuses_in_words`).

## What is missing

Certifying the lifted requirement: one fused sweep per member (or one
over the family's bodies), a verdict that names the member a witness
pair is on, and a witness the viewer can draw on that member's copy.
