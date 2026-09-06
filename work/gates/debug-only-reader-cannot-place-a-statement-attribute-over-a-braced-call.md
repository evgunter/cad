---
id: debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call
kind: issue
title: The debug-only gate's reader cannot place a statement-position cfg(debug_assertions) attribute over a braced call
status: open
opened: 2026-09-06
refs: [debug-only-helpers-outside-the-subject-list, 2049]
---


## Finding

Disclosed as the residue of `debug-only-helpers-outside-the-subject-list`
(PR 2049) and recorded there as `scripts/gates/bit-identity-debug-only.sh`'s
KNOWN GAP 6.

The gate's reader places a use against the `cfg(debug_assertions)` item
that encloses it by tracking brace depth, and it decides an item has been
ENTERED at the first `{` it meets after the attribute. When the attribute
is in STATEMENT position over a multi-line call whose arguments carry a
brace, that first `{` is inside the call's still-open `(` — a struct
literal handed to the call, not the item's body — and brackets still open
at a body brace is the positive desync the reader refuses to tolerate, so
every such site reds the gate.

That made the gap a CEILING on the subject list rather than a nuisance: a
mechanism whose sites take that shape could not be a row at all.
`crates/topo/src/euler.rs`'s `ArenaDelta` is the live population — the
per-operator arena shift `assert_euler_postcondition` checks — with twelve
sites of the shape

```rust
#[cfg(debug_assertions)]
self.assert_euler_postcondition(
    before,
    ArenaDelta { solids: 1, ..ArenaDelta::ZERO },
    "mvfs",
);
```

in `euler.rs` (3), `euler_ring.rs` (3), `euler_kill.rs` (4), `null.rs` (1)
and `split.rs` (1), plus three one-line statement sites in
`boolean/voids.rs`, `euler_ring.rs` and `movefac.rs` that the reader
already placed.

## The rule

An item under a statement-position attribute has no body brace, and that
is the SAME bracket-depth test the reader already applies at a `;`, read
at the `{` instead. Which delimiter arrives at bracket depth ZERO first
says what the item is:

- a `{` at bracket depth zero is the item entering, and the item runs to
  the `}` that balances it;
- a `;` at bracket depth zero ends an item that was never entered;
- a `{` met at bracket depth ABOVE zero is neither — it is argument text,
  so it moves brace depth and nothing else.

## Closed by

`scripts/gates/bit-identity-debug-only.sh`: the rule above, the seven
`ArenaDelta` rows it unblocks, and three fixtures (the live shape, the
same with a use below it, and nested braces in two arguments).

## Residue

`assert_euler_postcondition` is a second spelling of the same mechanism
and is on no row: `debug-only-assert-euler-postcondition-is-on-no-row`.
