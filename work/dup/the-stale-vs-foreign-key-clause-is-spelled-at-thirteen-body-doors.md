---
id: the-stale-vs-foreign-key-clause-is-spelled-at-thirteen-body-doors
kind: issue
title: The stale-vs-foreign key clause is spelled at thirteen Body doors, two of them near-identically
status: open
opened: 2026-09-20
refs: [the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times]
priority: P4
cost: D
---

## Finding

- **Where**: `crates/topo/src/body.rs`, thirteen door rustdocs.
- **Importance**: low
- **Confidence**: sure. Counted at `cd9fdfd6b`, one file.
- **Raised by**: the `Body::shells_of_solid` fold, 2026-09-20 — as an
  **X4 the fold caught in its own diff**.

`body.rs`'s module docs carry a `# Key validity: stale vs. foreign`
section (~:36). Thirteen door rustdocs below it restate the section's
consequence rather than pointing at it, in two shapes:

- **Seven parentheticals**, near-identical: *"or `None` if the key is
  stale (a foreign key is not caught — see the module docs)"* —
  `get_solid`, `get_shell`, `get_face`, `get_loop`, `get_half_edge`,
  `get_edge`, `get_vertex`.
- **Four long forms** that add a door-specific consequence:
  `solid_of_face` and `face_of_half_edge` (composing two lookups, so a
  foreign key does not stop at the first hop), `faces_of_solid` and
  `shells_of_solid` (a foreign `SolidKey` on a live slot hands back
  another solid's entities).

The seven parentheticals earn their keep: one clause, at the door, is
cheaper for a reader than a jump. The **four long forms are the class**,
and two of them were byte-parallel.

## A SECOND family in the same prose, which this row's first pass had no bucket for

Three door paragraphs in `body.rs` open with the byte-identical
sentence **"`None` is the only refusal this door can make"** —
`solid_of_face` (~:876), `face_of_half_edge` (~:999) and
`shells_of_solid` — and each then restates the consequence in its own
words: *"so three shapes of caller keep a hand-written walk"*,
*"so a caller whose own refusal distinguishes the hops keeps its own
walk"*, and (before the fix) *"that is why it can stand under callers
that refuse in different vocabularies"*. One argument, three
paragraphs.

**The third copy was minted by the unit that filed this row**, in the
same diff, and this row as first written sorted `body.rs` door prose
into the two shapes above and could not see it: the census was of the
stale-vs-foreign clause, and the fence was drawn at that clause rather
than at *arguments the module already makes that a door restates*.
`shells_of_solid` now states its refusal by reference; the other two
are untouched. **A census whose bucket is a SENTENCE cannot find the
next sentence**, which is the same shape as the code census this
program keeps re-learning — the fence is drawn at what the author was
already looking at.

## The X4, and how it was caught

`Body::shells_of_solid`'s first draft wrote
*"a foreign `SolidKey` landing on a live slot passes the resolution and
this door hands back **another solid's shell list** as though it were
the caller's"* — which is `Body::faces_of_solid`'s sentence with
"face list" swapped for "shell list", ten doc lines above it in the
same file, in the diff of a unit whose subject is one thing spelled *n*
times. It was caught by re-reading the diff for the class being closed,
before the branch was pushed, and the door now points at
`faces_of_solid` for the `SolidKey` hazard instead of restating it.

**The pair `solid_of_face` / `face_of_half_edge` is the other half and
is untouched**: both spell out the composes-two-lookups consequence in
full. Left as found — this unit fixed the instance it minted, not the
class, and a prose fold across four doors is its own decision about
where the argument's home is (the module section, or the first door).

## Why this is filed on dup

Its subject is one argument spelled four times, which is this
program's charter; `orient-module-prose-accumulation` on the same
slate is the same class in another module. `crates/topo/src/body.rs`
carries no `territory` owner.
