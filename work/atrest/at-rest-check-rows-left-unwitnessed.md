---
id: at-rest-check-rows-left-unwitnessed
kind: issue
title: Stage 5 C's at-rest check leaves two of its spec rows unwitnessed: a ⊔ whose gather refuses, and a host Gap over picks of an instance's pieces
status: open
opened: 2026-10-26
priority: P3
cost: M
refs: [the-at-rest-census-is-a-check]
---


Stage 5 C (`the-at-rest-census-is-a-check`, spec §6) built every row of
`crates/editor-core/tests/intent_s5_c_at_rest_check.rs` but two clauses,
which have no witness the public API can author today:

- **Row 15, "a `⊔` whose gather refuses is one `NotGathered` finding".**
  `crates/editor-core/src/checks.rs` `at_rest` turns an
  `AssembleGather` whose `gather` is `Err` into one
  `AtRestEvidence::NotGathered` finding. No document reaches it: the
  `⊔`'s own wire (`eval/wire.rs`, `wire_assemble`) grafts its members
  first and fails the node on a graft refusal, and each member is gated
  at its own door, so the census's graft and tier-1–3 gate
  (`product::gather_sources`) do not refuse over a `⊔` that evaluated.
  The arm is reached only by a hand-built `AssembleGather`.
- **Row 17, "a host `Gap` over picks of the instance's pieces quiets
  nothing".** Not built: the quieting rules match an assertion's
  selections against the copies by their reads, and a pick of a piece
  is a read of its own (`VarDef::Pick`), so a host's `Gap` over picks
  cannot match a part's carried finding by construction (a carried
  finding keeps the part's assertion, `CarriedInterference`); the row
  wants a document that tries it.

Owed: a witness for each, or the reason none can exist recorded where
the arm is.
