---
id: at-rest-check-rows-left-unwitnessed
kind: issue
title: Stage 5 C's at-rest check leaves one spec clause unwitnessed: a ⊔ whose gather refuses
status: open
opened: 2026-10-26
priority: P3
cost: M
refs: [the-at-rest-census-is-a-check]
---


Stage 5 C (`the-at-rest-census-is-a-check`, spec §6) built every row of
`crates/editor-core/tests/intent_s5_c_at_rest_check.rs` but one clause,
which has no witness the public API can author today:

- **Row 15, "a `⊔` whose gather refuses is one `NotGathered` finding".**
  `crates/editor-core/src/checks.rs` `at_rest` turns an
  `AssembleGather` whose `gather` is `Err` into one
  `AtRestEvidence::NotGathered` finding. No document reaches it: the
  `⊔`'s own wire (`eval/wire.rs`, `wire_assemble`) grafts its members
  first and fails the node on a graft refusal, and each member is gated
  at its own door, so the census's graft and tier-1–3 gate
  (`product::gather_sources`) do not refuse over a `⊔` that evaluated.
  The arm is reached only by a hand-built `AssembleGather`.

Owed: a witness, or the reason none can exist recorded where the arm
is.

Row 17's other clause, "a host `Gap` over picks of the instance's
pieces quiets nothing", was witnessed in stage 5 C's fix pass
(`a_hosts_gap_over_picks_quiets_no_carried_finding`): a pick of an
instance's piece is authorable, and its `Gap` holds and quiets nothing
of the part's.
