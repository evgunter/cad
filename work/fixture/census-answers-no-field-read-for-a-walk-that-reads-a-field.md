---
id: census-answers-no-field-read-for-a-walk-that-reads-a-field
kind: issue
title: the hand-written-impl census answers NoFieldRead for two shapes that do read a field, and its own doc says so
status: open
opened: 2026-09-16
priority: P4
cost: E
---



Filed by CENSUS-HAND-LISTED-SIBLINGS (2026-09-16), whose spec directs
that a wanted widening of `crates/test-utils/tests/hand_written_impl_census.rs`
be filed rather than taken — `crates/test-utils/*` is not CENSUS's.

## The finding

[`Verdict::NoFieldRead`] is documented as *"Reads no field by name — a
newtype's `self.0`, an enum's `match self`, a delegation. Nothing to
fall behind."* For two shapes that claim is false, and the file's own
blind-spot list says so four paragraphs above the enum:

- **a delegation to an inherent method of the same type.**
  `impl PartialEq for SketchPlane<f64>` is `self.bit_eq(other)`, and
  `bit_eq` reads twelve stored coordinates. The census answers
  `NoFieldRead`.
- **a read through a tuple index into an inner type.**
  `crates/editor-core/src/names/role.rs`'s five walks over `NameRef`
  all read `self.0.name` — a NAMED field of `Held`. `first_read_of`
  rejects `self.0` because the name after the dot starts with a digit,
  and stops there; the census answers `NoFieldRead`.

The second is the sharper one, because no type graph is needed to tell
that `self.0.name` reads a named field of SOMETHING. What the reader
cannot do is name the declaration — which is an argument for a verdict
that says so, not for one that says nothing is read.

## What was measured

Both sites were repaired in CENSUS-HAND-LISTED-SIBLINGS: `bit_eq` now
binds `Self`, `Affine3`, `Mat3` and `Vec3` by pattern, and each of the
five `NameRef` walks binds `Held { name, stamp }`. `cargo test -p
test-utils --test all` is green before and after, and
`every_hand_written_walk_is_held_to_its_declaration` names neither site
in either state — **the instrument's answer did not move when the
defect it exists to find was removed**, which is the thing this program
calls a row that cannot go red.

Executed both ways for the four sites the census CAN see, as the
contrast: re-adding a repaired entry to `KNOWN_HAND_LISTED` reds
`every_known_hand_listed_impl_is_still_found`, and reverting one
destructure with no entry reds
`every_hand_written_walk_is_held_to_its_declaration` with
`crates/editor-core/src/expr.rs:518 impl PartialEq for Lit — reads
self.value with no exhaustive pattern`. Neither row can be made to say
anything about the two invisible sites.

## What a fix has to weigh

The blind-spot list is **reasoned, not accidental**: *"following those
would make this a dataflow analysis over text"*, and the same
paragraph's one-`let` rule (`self_aliases`) is where the author drew
the line. So this is not "the walk is broken"; it is that two of its
verdicts are stated more strongly than they are true. Three shapes a
disposition could take, cheapest first:

1. **Rename nothing and narrow the doc** — say `NoFieldRead` means
   "reads no field THIS reader can attribute to a declaration". Costs
   one paragraph, buys honesty and no detection.
2. **A fourth verdict for `<tuple index>.<name>`** — the reader can
   see it without resolving anything. It would red both `role.rs` and
   any future newtype walk, so it needs the `KNOWN_HAND_LISTED` shape
   or a per-site marker; and a walk that HAS been repaired (the five in
   `role.rs`) is indistinguishable from one that has not, since the
   pattern is on the inner type. That last point is what makes this a
   design call rather than a patch.
3. **One hop into an inherent method of the same type in the same
   file** — bounded exactly as `self_aliases`' one `let` is bounded,
   and it is the only shape that would reach `SketchPlane::bit_eq`.

Sites: `crates/test-utils/tests/hand_written_impl_census.rs`
(`Verdict`, `first_read_of`, the module header's *What this cannot
see* list); the two subjects are `crates/profile/src/lib.rs` and
`crates/editor-core/src/names/role.rs`.
