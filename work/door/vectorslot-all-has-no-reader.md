---
id: vectorslot-all-has-no-reader
kind: issue
title: VectorSlot::ALL is a public hand-written list with no reader anywhere in the workspace
status: closed
opened: 2026-09-11
closed: 2026-09-12
branch: door/vectorslot-all
pr: 2446
refs: [all-census-idiom-forces-the-visit-not-the-update, boolean-op-has-a-third-hand-written-complete-list, vectorslot-slots-has-no-reader, seven-vector-families-is-a-prose-count-at-two-tag-sites]
---



Filed by the `BooleanOp::ALL` unit's sweep of the eight unswept
`const ALL` neighbours named in
`boolean-op-has-a-third-hand-written-complete-list`. It is not that
unit's class — it is the neighbouring one — and it is the only hit of
the eight that is dead rather than merely unforced.

## The finding

`crates/editor-core/src/node.rs:409` declares

    pub const ALL: [VectorSlot; 7] = [ … ];

on a `pub` type re-exported through `pncad::document`
(`crates/pncad/src/document.rs:78`). **Nothing in the workspace reads
it.** `rg 'VectorSlot::ALL'` over `crates/`, `demos/`, `tools/` and
`benches/` returns the declaration and nothing else; the type's other
seven uses (`crates/viewer/src/props.rs`, `crates/pncad-py/src/py/doc.rs`,
`crates/viewer/tests/panel_display.rs`) all name variants directly.
`Axis3::ALL` three hundred lines below (`node.rs:1941`) has readers in
production and in two suites, which is what the same shape looks like
when it is earning its place.

So it is a hand-written complete list, unforced, public, and read by
nobody: the class
`work/view/tool-kind-all-and-ordinal-have-no-production-reader` was
about, at a different site. Its doc says it is "for a consumer
enumerating families rather than reading one off a slot", and no such
consumer exists.

## What to decide

Delete it, or find the consumer that should have been reading it —
`SlotId::component` (`node.rs:565`) is the one exhaustive match over
the families, so a census over `ALL` is the natural row if one is
wanted. Whichever way it goes, the list should not stay public and
unread: an unread list is a ratification waiting to be inherited by
whatever is written at that name next, and nothing would notice it
going stale in the meantime.

## Closed (2026-09-12) — deleted

**The premise held, re-measured two ways.** `rg VectorSlot` (not
`VectorSlot::ALL` — the shape, so an alias or a re-export path would
show) over `crates/`, `demos/`, `tools/`, `benches/` and
`interval-transcendentals/` — every cargo root
`scripts/doc-gate.sh --print-roots` names — returns the declaration
and thirteen sites that all name variants directly. There is no
`use … VectorSlot as _`, no `Self::ALL` in the impl, and no glob
re-export in `crates/pncad/src/` or `crates/editor-core/src/`. Then by
compiler rather than by grep: with the constant deleted,
`cargo check --workspace --all-targets` is green, and so are all three
out-of-workspace roots that can see the façade (`demos/tour`,
`demos/wild`, `benches`; the other four roots do not depend on it).

**Nothing outside the workspace can depend on it either.** The root
`Cargo.toml` sets `publish = false` for every member — *"Nothing is
publishable until the project has its name (Q9)"* — so no version of
this crate has ever left the tree. And the type does not cross the
Python boundary: `crates/pncad-py/tests/test_binding_census.py` carries
`VectorSlot` in the not-bound set as `different-shape`, on the stated
reason that a Python caller addresses a slot through a door that names
it, so `pncad.pyi` never mentions the type. So the usual cost of
deleting public API — a consumer you cannot see — is measurably zero
here, not merely unlikely.

### What the list would have had to become, and why it did not

The doc's claimed consumer ("for a consumer enumerating families
rather than reading one off a slot") is not one that has not been
written yet: it is one the design rules out. The single site in the
tree that groups by family — `group_rows`
(`crates/viewer/src/props.rs`) — discovers families by walking the
slots a node actually lists through `SlotId::component`, and its own
doc gives that as the reason it needs no edit when the vocabulary
grows. A roster is the thing that site deliberately does not read.

Rejected: **keep it and give it a census**, the `BooleanOp::ALL` shape
this row's sibling landed. Two reasons, in order.

1. It would not force what this row asks for. Measured, on this tree:
   add an eighth variant to `VectorSlot` and the compiler names
   exactly two sites — `VectorSlot::slot` and `VectorSlot::label`,
   both `error[E0004]`. `SlotId::component` is NOT one of them (it
   matches on `SlotId`, so it reds when a vector SLOT is added, which
   is the other direction). Give those two arms and the crate compiles
   clean with `ALL` still declaring seven. A `len()`-against-a-match
   census does not close that: it is exactly the hole
   `all-census-idiom-forces-the-visit-not-the-update` measures at the
   three sites that already have one, whose stated repair is one
   instrument written once and adopted at all three. A fourth adopter
   before that instrument exists makes that row's job bigger.
2. Manufacturing a reader to justify a list inverts the order. The
   list was written for a consumer; the consumer never came; the
   answer is not to write the consumer.

### What is forced now

Deletion cannot be guarded mechanically — no test can see a `pub
const` that nobody wrote yet, and a source grep is not a guard — so
what stands in its place is stated at the site rather than claimed
here: `VectorSlot`'s doc now records that nothing enumerates the
families, that a consumer reaches one from a slot and its slots back
from the family, and that an array of every variant would not be a
census of the declaration anyway, with the measurement above as the
reason. An eighth family still reds in two places, which is what the
type's doc already promised and is unchanged by this.

### What the row got wrong

- *"`SlotId::component` (`node.rs:565`) is the one exhaustive match
  over the families"* — it is not a match over `VectorSlot` at all.
  The two matches over the families are `VectorSlot::slot` and
  `VectorSlot::label`, and they are what an eighth family reds.
- *"a census over `ALL` is the natural row if one is wanted"* —
  measured above, a census over `ALL` would have forced a visit and
  not the edit, and would have been the idiom's fourth holed copy.

Everything else in the finding is as stated.

### Filed while here

- `work/docm/vectorslot-slots-has-no-reader` — `VectorSlot::slots` is
  the same shape one member down, and deleting it also leaves the
  workspace green. Not taken: one PR, one row, and it is DOCM's file.
- `work/lib/seven-vector-families-is-a-prose-count-at-two-tag-sites` —
  two `pncad-py` doc comments count the families in prose.
