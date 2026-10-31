---
id: a-split-in-the-document-drops-its-operands-contact-records
kind: issue
title: editor-core's wire_split hands each side an empty Carry.contacts, so a split of a body with contact records loses them in the document
status: open
opened: 2026-10-28
priority: P1
cost: M
---


Filed by SPLITDOOR (2026-10-28). Both designers on SPLITDOOR's split-door-records question found it independently while reading the code. Neither built anything to confirm it.

## What

`crates/editor-core/src/eval/wire.rs` `wire_split` builds each side's value through `OpOut::per_output` / `plain`, with the default empty `Carry.contacts`. It never calls `topo::SplitResult::carry_contacts`; only tests do. So when the operand carries contact records (a `⊔` product, or a Boolean that recorded a pinch), its sides lose them. Any later at-rest check or lint reads the sides as recordless.

## Direction

There are two ways forward:
- **Interim:** carry the operand's records through `SplitResult::carry_contacts` in `wire_split`.
- **Pending design:** SPLITDOOR's design fork on the split door's contract may move records into the finished body (`AtRestBody`), which would make this drop unrepresentable.

Check that fork's outcome (`work/splitdoor/the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime.md`) before building more than the interim carry.

First, build a fixture: a Boolean pinch union, then split it in the document, then read its sides' records.
