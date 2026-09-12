---
id: name-placed-union-spells-the-narrowing-rule-itself
kind: issue
title: name_placed_union spells the tie-narrowing rule itself instead of calling narrow_into
status: open
opened: 2026-09-12
---


Found by the sweep for
`product-gather-refuses-a-split-root-whose-tie-spans-both-halves`,
whose subject was that `product::carry_names` was a THIRD
hand-written copy of `names::defer::narrow_into` — "the one narrowing
rule for a tie's survivors", whose own doc says `flush` and
`NameTable::project` both write through it *"so the two doors cannot
narrow differently."* That unit routed the gather through the door.
`name_placed_union` (`crates/editor-core/src/names/emit.rs`, the
group-boolean fuse's emitter, ~line 337) is the copy the sweep turned
up, and it is outside that unit's fence.

The shape, verbatim, after filtering the prototype's candidates
through the instance's `GraftKeys`:

```rust
match moved.len() {
    0 => {}
    1 => t.insert(wrapped, moved[0])?,
    _ => t.insert_tied(wrapped, moved)?,
}
```

against `defer.rs`'s

```rust
match ents.as_slice() {
    [one] => t.insert_ref(name, *one),
    _ => t.insert_tied_ref(name, ents),
}
```

The two agree TODAY — `insert` is `insert_ref(NameRef::new(name))` —
so this is a duplication finding, not a behaviour one, and nothing
observable is wrong at this commit. What it costs is the guarantee the
door exists for: a change to what narrowing MEANS (say, refusing a
zero-survivor tie instead of dropping it, or recording the drop) lands
at `narrow_into` and leaves this site behind, silently, exactly as the
gather was left behind.

Two neighbours in the same file are NOT instances and should not be
swept with it: the `Instance`-wrapping loop (~line 254) and the
placed-body loop (~line 390) preserve the entry's shape with no filter
in front of them — no candidate can be dropped, so neither narrows;
a row at an unexpected body index is surfaced as
`NamingError::Emission` rather than filtered away.

Fix: call `super::defer::narrow_into` with the non-empty `moved`,
keeping the `0 => {}` arm (a tie whose every candidate the fuse
consumed has no row to write). One site, no behaviour change, and the
CI matrix already covers the fuse.
