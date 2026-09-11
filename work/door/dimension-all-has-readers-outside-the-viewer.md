---
id: dimension-all-has-readers-outside-the-viewer
kind: issue
title: Dimension::ALL has five hand-written mirrors outside crates/viewer/src that could read it
status: open
opened: 2026-09-11
---


Found by the sweep for
`dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum` (PR
PRNUM), which published `editor_core::Dimension::ALL` and retired the
one mirror that row named. The sweep was scoped to `crates/viewer/src`
by the unit's brief; re-run over `crates/` it returns five more
complete hand-written enumerations of the same four variants, none of
which the unit touched.

## The hits

Two are in ANOTHER crate, which makes them the same defect as the row
that is closed — a complete mirror of a vocabulary the mirroring crate
does not own:

- `crates/pncad-py/src/tests.rs:45` —
  `dimension_tags_match_the_kernel_prose` iterates a four-entry array.
  Its own doc calls the tag and the prose word "two spellings of one
  closed list"; a fifth dimension leaves the pin covering four of five
  and green.
- `crates/pncad-py/src/tests.rs:1362` —
  `literal_refusals_come_from_the_kernel_with_stable_tags`, whose
  comment says "the reachable set, exhaustively: every dimension". A
  fifth dimension makes that sentence false with nothing red.

Both are `for dim in [ …four… ]` and both become `for dim in
Dimension::ALL` unchanged.

Three are inside the declaring crate, so the compiler at least stands
over them; they are still second copies of a list the crate now
publishes:

- `crates/editor-core/tests/switch_display_units.rs:417` — a `dims`
  local of all four, under a doc that also carries the prose count
  "6 rows x 4 dimensions". Reads `Dimension::ALL` directly.
- `crates/editor-core/tests/u8a_parse.rs:482` and `:725` — proptest
  `prop_oneof![Just(Length), Just(Angle), Just(Scalar), Just(Count)]`.
  Not a drop-in: the projection is `proptest::sample::select(&
  Dimension::ALL[..])`, a strategy of a different type, so each call
  site wants reading rather than substituting.

A sixth site is deliberately NOT one of these and must not be
projected: `crates/editor-core/tests/u8a_parse.rs:506`'s `rt_params`
maps four arbitrary parameter NAMES onto the four dimensions. The names
are the fixture's own invention, so there is nothing to project from —
what it wants, if anything, is a growth alarm, which is the sibling
`mate-primitives-is-a-partial-mirror-with-no-growth-alarm`'s question
and not this row's.

Also not this row: `crates/viewer/README.md:1500` spells
`Dimension = Length | Angle | Count | Scalar` in prose. That is the
GQ5 design-question recap, where the variant identifiers are the
design vocabulary rather than words shown to a user, and no gate reads
it. A doc mirror is a different class from a code one.

## Why it is not fixed in the unit that found it

DOOR's posture is one PR is one row, with the mirror class's two rows
in one file as its single ruled exception (`work/door/plan.md`,
**Territory** and **Order**). `crates/pncad-py/src/*` is a third
crate's ground and the two `editor-core` suites are neither the file
the unit changed nor its own tests; widening into them would be a lane
minting a second exception for itself.

## What it is worth

Low, and it should be said plainly: every hit is test code, so the
consequence of a fifth dimension is a pin that silently narrows rather
than a user-visible row that silently shrinks. What makes it worth a
file rather than a sentence is that two of the five assert
exhaustiveness in their own prose, so the failure mode is a test whose
doc says "every dimension" while it covers four of five.
