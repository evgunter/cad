---
id: main-red-recharted-row-unstated-renders-its-relation-through-debug
kind: issue
title: Main red: BooleanError::RechartedRowUnstated renders its Relation through Debug, which the binding's prose census refuses (since PR 5795)
status: closed
opened: 2026-10-27
priority: P0
cost: E
closed: 2026-10-27
---


Filed by the 2026-10-27 nightly triage (run 40455870959, main
`fb476030a9`).

## What

`pncad-py` `prose_census::tests::no_display_impl_renders_a_brace_shaped_payload_through_debug`
(`crates/pncad-py/src/prose_census.rs:2274`) is red at every ε row of
the full suite, and on main `57e3f95a` at the default ε:

    a `Display` impl renders a struct-shaped payload through `Debug`, so the refusal PANICS the binding at `typed_err` instead of raising: [
        (("crates/topo/src/boolean/mod.rs", "BooleanError", "relation"), 1),
    ]

The site is `BooleanError::RechartedRowUnstated`'s arm of
`impl Display for BooleanError` (`crates/topo/src/boolean/mod.rs:4790`):
`"… sphere ({relation:?}) on that sphere's own faces: {what}"`.
`relation` is a `crate::Relation` (`crates/topo/src/coincidence.rs:77`),
which has no `Display` and holds a struct-shaped variant
(`Tangent { aligned }`), so its `Debug` carries braces into the
binding's message.

**Introduced by** `ab801e31` ("Merge origin/main (PR 5798's more); a
re-charted cell reads as the set of its input shell's faces"), which
reached main with the merge of **PR 5795**
(`wire/re-chart-rows-name-input-cells`, `423a53eddc`).

## Owed

Render the relation through a `Display` of its own (give `Relation`
one, or word the arm from its variant), so the census row is green
without a new allowance.

Reproduce: `cargo nextest run -p pncad-py -E 'test(=prose_census::tests::no_display_impl_renders_a_brace_shaped_payload_through_debug)'`.

Closed by this PR (wire/recharted-relation-words, 2026-10-27). `RechartedRowUnstated`'s Display now renders the relation through `Relation::noun` (words), not `Debug`. The same PR fixes the sibling census red from PR 5829: `UnsupportedTurnedCell`'s message renders its key through the variant's `cell` field, which the census can type, instead of locally bound keys. Both `prose_census` tests pass.
