---
id: all-census-idiom-forces-the-visit-not-the-update
kind: issue
title: The ALL-census idiom forces a visit to the row, not an update to the list, at all three of its sites
status: open
opened: 2026-09-11
---



Filed by the `BooleanOp::ALL` unit, whose review measured the hole in
the very instrument that unit had just copied. The unit's own census
row carries the measurement at its site and points here rather than
patching one of three copies.

## The idiom

Three rows in the tree pin a hand-written `ALL` against its enum the
same way:

- `crates/verbs/src/verb.rs:436` `all_is_the_whole_vocabulary`
- `crates/topo/src/param_source.rs:318` `all_is_the_whole_field_declaration`
- `crates/topo/src/boolean/mod.rs` `all_is_every_operation` (added by
  the unit that filed this)

Each writes an exhaustive match over the enum whose every arm names the
same total, then asserts `ALL.len()` equals it and that `ALL` holds no
repeats. Each doc claims some version of *"every arm names the same
total, so visiting it means writing the new count, which then reds
until `ALL` has grown too."*

## The claim is false, measured

The match forces the VISIT — a new variant fails to compile until an
arm is written for it. Nothing forces the arm's NUMBER, and the arm an
author writes is the arm they copied. Compiled and run standalone:

    enum Op { Union, Intersect, Subtract, Xor }
    impl Op { const ALL: &'static [Op] = &[Op::Union, Op::Intersect, Op::Subtract]; }
    let ops = match Op::Union {
        Op::Union => 3, Op::Intersect => 3, Op::Subtract => 3, Op::Xor => 3,
    };

`ALL.len() == ops == 3` with `Xor` absent from `ALL`: **GREEN**. The
only murmur is a `variant is never constructed` warning, which is an
artifact of the fixture — a real variant is constructed somewhere and
that warning does not fire.

So what the row buys is a forced visit and a forced decision, which is
not nothing and is not what the docs say. The no-repeats half is sound
and does what it claims.

## The shape an answer has

The number has to come from somewhere the author cannot copy. Options,
none of them decided here:

- **Sum the arms instead of naming a total.** `ALL.iter().map(|v| match
  v { … => 1 }).sum()` counts what the match VISITS, not what an author
  typed — but it visits `ALL`'s entries, so an absent variant is absent
  from the sum too. It moves the hole rather than closing it.
- **`std::mem::variant_count::<T>()`** is exactly this number and is
  nightly-only (`feature(variant_count)`); this workspace is stable.
- **A discriminant walk**: give the enum a `const fn index(self) ->
  usize` (three of the sites have one already for other reasons), then
  assert every index in `0..ALL.len()` is hit by `ALL`. The index match
  is exhaustive, so a new variant must be given an index, and an index
  the list never yields reds. This closes it without a macro and is the
  most promising of the three.
- **A proc macro** projects the list from the declaration and ends the
  class outright. The workspace has none and
  `crates/viewer/src/vocab.rs`'s `vocabulary!` is the `macro_rules!`
  precedent for refusing to add one.

Whichever is taken, it is one instrument written once and adopted at
all three sites, plus the three docs corrected to claim what holds.
`crates/viewer/src/vocab.rs`'s projected `ALL` is NOT in this class:
there the list and the enum are one declaration and the question does
not arise.
