---
id: name-placed-union-spells-the-narrowing-rule-itself
kind: issue
title: The 1-vs-many tie decision is hand-written at eight sites outside the door that owns it
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

## The class, and the sweep that finds it

`name_placed_union` is one instance. The `insert_tied` sweep that
found it is structurally blind to the rest, because the same decision
is written a second way — straight into the deferral lane, with no
`insert_tied` anywhere near it (found by the T-review lane of PR
2442):

```rust
if members.len() == 1 {
    put(&mut t, &mut tie, from_tie, base, ent(0, ...))?;
} else {
    for &f in &faces { tie.push(name.clone(), ent(0, ...)); }
}
```

**The sweep that works is `rg 'if .*\.len\(\) == 1'`**, which finds
them in seconds. Six in `crates/editor-core/src/names/emit_topo.rs` —
`:601` (merge groups), `:726-733` (`SideOf` fragment groups), `:952`
and `:1009` (edge groups), `:1236` (vertices), `:1400` (members) —
plus two near-misses worth a reader's eye,
`crates/editor-core/src/names/emit_blend.rs:377,427`
(`Entry::Tied(es) => Some(es.len())`) and
`crates/editor-core/src/names/flush.rs:328`
(`if ca.len() > 1`).

**Nothing is wrong today at any of the six.** They are MINTING
decisions — this op is deciding how many entities it just made deserve
one name — rather than narrowings of an upstream tie, and they all
land in the same `TieRows` lane, so the flush still writes them
through `narrow_into`. What they are is eight hand-written spellings
of one sentence ("one ⇒ strict, several ⇒ tied"), which is the
condition under which a change to that sentence lands in one place and
leaves seven behind.

The unit's PR disclosed exactly this as its own sweep's blind spot —
*"a narrowing written without `insert_tied` at all … I did not find a
way to search for that shape mechanically"* — and the pattern above is
the answer to it. A lane taking this row should decide whether the
minting shape deserves a door of its own beside `put`/`narrow_into`
(something like "defer this whole candidate list"), or whether the six
are better left as the local decisions they read as, with `defer.rs`
saying so. That is the judgement; the hit list is not.
