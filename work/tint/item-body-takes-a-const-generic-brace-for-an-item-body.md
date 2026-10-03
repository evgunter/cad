---
id: item-body-takes-a-const-generic-brace-for-an-item-body
kind: issue
title: item_body reads a const-generic argument's brace as the item's body, so an impl with one silently stops being a scope in two censuses
status: open
opened: 2026-09-15
priority: P3
cost: E
---


Found by the style review of CENSUS-ARRIVAL-RESIDUE (2026-09-15) and
executed by that unit's fix pass, which could not take the repair:
`test_utils::source` is TCOST's and TINT's, and the unit's spec ruled
a widening of that module's grammar a filed row rather than a taken
one.

## What it does

`test_utils::source::item_body` answers where an item's body starts by
taking the first `{` or `;` after the keyword. A const-generic
argument spells a `{` inside the item's HEAD, so

```rust
impl Holds<{ N }> for Subject {
    const W: &str = "w";
}
```

gives the argument's braces for the body. Every item of the real body
then falls outside every scope the caller tracked.

Executed 2026-09-15 against `crates/pncad-py/src/tests.rs`'s
`read_minting_items` at `census/arrival-residue`: the source above
answers `{"W": ["w"]}` — the `impl` has vanished as a scope and its
`const` is keyed bare, under a name that does not exist. The same
input with `Holds<{ 1 > 0 }>` answers identically, which matters
because the comparison is not what does it:
`crates/pncad-py/src/tests.rs`'s blind-spot list had written this
down as an instance of `angle_end`'s disclosed
generic-closed-early residue, and it is not one — the braced form
answers the same with no comparison in it, and the unbraced form that
residue describes is not valid Rust.

## Why it is a class and not one caller

`crates/test-utils/tests/hand_written_impl_census.rs`'s `sites_in`
reads the same helper the same way: `item_body(&code, at)`, then
`impl_head(&code, at, body_start)` over the range it answers. A
`Debug` or `PartialEq` impl for a type with a const-generic argument
would meet it there. **Not executed against that census** — this was
filed by a lane outside that territory, and the shared reading is the
evidence, not a run.

`crates/pncad-py/src/tests.rs`'s
`the_errors_mint_reader_loses_an_impl_whose_generic_argument_holds_a_brace`
pins the wrong answer as the answer on that one caller, and its
blind-spot entry cites this row.

## What the repair looks like

The head ends where the generic list and the self type end, not at the
first `{`: `item_body` would have to skip a balanced `{…}` inside an
angle-bracket group before it accepts a brace as a body. That is a
widening of `source`'s grammar and belongs to whoever owns it.

Related but not the same: `source-scanning-censuses-are-a-tripwire-on-ordinary-rust`
is about these walks' FAILURE MODE being indistinguishable from a
defect in the scanned code. This one does not fail at all.
