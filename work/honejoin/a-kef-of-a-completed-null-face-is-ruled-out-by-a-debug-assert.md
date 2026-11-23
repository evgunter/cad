---
id: a-kef-of-a-completed-null-face-is-ruled-out-by-a-debug-assert
kind: issue
title: The join's interior-null-edge kef is kept off a completed null face by an argued invariant and a debug_assert! alone
status: open
opened: 2026-10-11
priority: P3
cost: M
---


Found by FLUSH's E tail (`flush/e-tail`), from the third bullet of
`work/flush/the-turn-sense-divisor-is-nonzero-by-prose.md`, which PR
4396's review left unscheduled.

## What

`crates/topo/src/chord_join.rs`, the cut of an interior null edge in
`ChordJoiner` (the `kef` arm beside the `kemr` that completes a
polygon): the comment argues that neither side of the edge is a
completed null face ("the edge's sides are the slivers its own
polygon's chords walled off at its two ends, and a completed face is
bounded by another polygon's two copies"), and a `debug_assert!`
(`"{} would kef a completed null face"`) is all that holds it. A
release build runs no check: if the argument fails, the `kef` picks
its victim from `slivers` and can merge a sliver into a completed null
face, or kill one that also sits in `slivers`, silently, which the
repo's fail-loud rule forbids.

## Owed

- Make the guard a typed refusal that runs in every build (the arm
  already refuses `SplitJoinError::CutInvariant` when neither side is
  a sliver), so a violation is loud.
- Then either prove the invariant (the sliver/completed-face argument
  above, written against how `slivers` and `completed` are filled) or
  search for a witness: a pose where two section polygons share a face
  and one completes before the other's interior null edge is cut.

## Note (2026-10-26, easy-fixes lane)

The guard now runs in every build: `crates/topo/src/chord_join.rs` `ChordJoiner::cut_core`'s interior-null-edge arm checks `completed` against both sides and raises `unreachable!` with the argument as its message (DESIGN `[failure.taxonomy]` row 4: a kernel bug observable in a branch). It is not the typed refusal the first bullet asks for. `SplitJoinError::CutInvariant`'s text ("neither face … is a sliver") is false for this case, and a new variant reaches the refusal census rows. Which of the two the arm is depends on the second bullet, the proof or the witness, which is not done. Verified: `cargo nextest run -p topo --lib -E 'test(/chord_join|splitting|…/)'`, 367 passed.
