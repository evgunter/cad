---
id: main-red-the-brace-payload-census-flags-recharted-row-unstated
kind: issue
title: Main red (slow set): the prose census flags RechartedRowUnstated's Display rendering its relation payload through Debug, since PR 5795
status: open
opened: 2026-10-27
priority: P1
cost: E
---


Found by CLEAVE's lane on PR 5830, which confirmed it fails on unmodified main.

## What

`pncad-py` `prose_census::tests::no_display_impl_renders_a_brace_shaped_payload_through_debug`
(slow set) fails on main. It flags
`("crates/topo/src/boolean/mod.rs", "BooleanError", "relation")`: the
`({relation:?})` in `BooleanError::RechartedRowUnstated`'s `Display`
arm, which renders a brace-shaped payload through `Debug` into a
user-facing message. It came in with the merge of PR 5795
(`wire/re-chart-rows-name-input-cells`, `423a53eddc`).

## Owed

Render the relation in words, as the other arms render their payloads,
or give the census its reason if a `Debug` rendering is right here. The
test then goes green.
