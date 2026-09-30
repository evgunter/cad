---
id: need-count-spells-every-failure-as-a-pattern-count
kind: issue
title: need_count's one refusal says "pattern count" and it is never used for a pattern count
status: open
opened: 2026-09-21
priority: P1
cost: E
refs: [a-pattern-count-has-no-upper-bound-at-the-loop-that-uses-it]
---


Filed by the VGEOM orchestrator while re-deriving the fork on
`work/vgeom/a-count-slots-cast-still-saturates-for-a-finite-value-too-large`
against today's tree, per the VIEW register's rule that an item's own
menu of options is a claim like any other and is costed against the
tree of today rather than the tree it was written from. The
re-derivation moved that row's premise, and this is one of the two
findings that moved it.

## Finding

`crates/editor-core/src/eval/wire.rs`, `need_count`:

```
/// A structural (Count) slot, refused typed when absent or unusable.
fn need_count(vals: &SlotValues<impl Decide>, slot: SlotId) -> Result<usize, NodeErrorKind> {
    let n = slots::count(vals, slot).ok_or(NodeErrorKind::MissingSlot { slot })?;
    usize::try_from(n).map_err(|_| NodeErrorKind::NonPositiveCount { count: n })
}
```

`NodeErrorKind::NonPositiveCount`'s `Display`
(`crates/editor-core/src/eval/mod.rs`) is

```
write!(f, "pattern count {count} is not at least 1")
```

and its doc comment is *"A pattern count that is not at least 1."*

**`need_count` is never used for a pattern count.** Its only two call
sites are `wire_loft`'s `need_count(vals, SlotId::VDegree)` and
`wire_sweep`'s `need_count(vals, SlotId::Stations)` /
`need_count(vals, SlotId::VDegree)` — three calls, two slots, no
pattern among them. The pattern count is read directly in
`wire_pattern` and `wire_stepped`, which raise the same variant about
their own `SlotId::Count`, where the word is right.

So every failure of `need_count` reports a **loft's V-degree** or a
**sweep's station count** as *"pattern count N is not at least 1"* —
the wrong slot, and, for the too-large arm, the wrong fault.

## Two faults, one arm, and the word is right for one of them

`usize::try_from(n)` fails in two directions and the arm does not
distinguish them:

- **`n < 0`** — on any target. Here *not at least 1* is true, and only
  the noun is wrong: a V-degree of `-1` is reported as a pattern count.
- **`n > usize::MAX`** — on a 32-bit target only, so `wasm32`, which
  this workspace builds (`ci.yml` runs
  `cargo check -p viewer --features app --target wasm32-unknown-unknown`).
  There a V-degree of `5e9` is reported as *"pattern count 5000000000
  is not at least 1"*, which is false about the number in front of the
  reader. Both nouns and the predicate are wrong at once.

The doc comment says *"refused typed when absent or unusable"*, which
is the honest description of what the function does; what has no
spelling is **unusable**, so the nearest existing variant is borrowed
and it means something else.

## Reachability

`sure` that the word is wrong at the site — that is two lines of source
and needs no execution. `likely`, not `sure`, that a person reaches it:
`crates/viewer/src/props.rs`'s `SlotValue::of` turns any typed `f64`
into an `i64` for a `Count`-dimensioned slot and
`crates/viewer/src/pane/properties.rs`'s `slot_field` sets no
`egui::DragValue` range, so `-1` in a V-degree field is a value the
chrome accepts — but the route was read, not driven. That chain is
established by
`work/vgeom/a-count-slot-launders-a-typed-nan-into-zero` (closed, #3000).

Nothing asserts the message: `rg 'is not at least 1' crates --type rust`
returns the `write!` and its doc comment and no test.

## What a fix has to decide

Whether the repair is a second variant (*a count outside the usable
range*, with the slot named) or making `NonPositiveCount` carry its
`SlotId` the way `MissingSlot` already does. The second is cheaper and
fixes the noun everywhere at once; only the first fixes the predicate.
Note that `work/vgeom/a-count-slots-cast-still-saturates-for-a-finite-value-too-large`
wants exactly the second thing minted — a refusal for *this count does
not fit* — so whichever is chosen here is the word the viewer raises
there, and the two should be decided together rather than twice.

## Fence

`crates/editor-core/src/eval/wire.rs` and `eval/mod.rs` — WIRE's, by
`scripts/work.py territory --files -`. Filed, not fixed: a numeric door
the viewer consumes is a hand-off and never a diff from VGEOM
(`work/vgeom/program.md`'s `keep_out`).
