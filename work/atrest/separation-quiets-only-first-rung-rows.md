---
id: separation-quiets-only-first-rung-rows
kind: issue
title: The Separation check quiets two copies only on a row the door's first rung proves, a subset of the spec's Structural rows
status: closed
opened: 2026-10-23
priority: P3
cost: E
rides_with: the-at-rest-census-is-a-check
closed: 2026-10-26
---


Found by stage 4 I (`mates-declare-no-contact`); a deviation the PR
states.

`docs/INTENT-STAGE4-SPEC.md` §9 re-keys the `Separation` check's
suppression "to the door's `Structural` at-rest rows". Stage 4 I keys it
on `crate::coincide::first_rung_proves`
(`crates/editor-core/src/coincide.rs:669`): a record quiets two copies
only where it cites a `CensusAtRest` row whose cells are one
construction read twice or the two faces a mate rests on each other
(`stated_by_a_mate`), read by `mated_pairs`
(`crates/editor-core/src/checks.rs:1887`). The door's replay rungs also
prove rows `Structural` (`coincide.rs`, `Proof::Structural(rung)` from
a replay), and those rows do not quiet `Separation`.

The direction is the strict one: a row the replay alone proves leaves
its separation finding loud. The replay runs off the draw path
(`checks::run_checks` reads no replay), so keying on it would make the
check's verdict depend on whether a replay had run. Stage 5 C
(`the-at-rest-census-is-a-check`) dissolves this suppression into
`CheckId::AtRest`'s quieting rule; that unit decides whether replay-proven
rows quiet, and this row closes with it.

## Closed

2026-10-26, absorbed by stage 5 C (`the-at-rest-census-is-a-check`):
`Separation`, `mated_pairs` and their first-rung suppression retire. The
at-rest check reports no contact at all, so no suppression is left to key
on a rung; a contact is the `unproven-coincidence` lint's row, which the
door proves by every rung it has, the replay's included.
