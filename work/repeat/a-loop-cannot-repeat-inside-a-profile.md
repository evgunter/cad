---
id: a-loop-cannot-repeat-inside-a-profile
kind: issue
title: A loop or a run of pieces cannot repeat inside one profile: patterns exist for bodies only
status: parked
opened: 2026-11-20
priority: P1
cost: X
points: 36
blocked_on: [profile-readers-read-kernel-steps-through-the-written-address-map, a-profile-piece-name-carries-its-member, a-profiles-loop-list-repeats-a-loop-with-each, a-chain-repeats-a-run-of-its-steps-with-each, a-ring-closes-each-k-from-its-start-point, the-profile-editor-shows-and-writes-a-repeated-loop-or-run, python-writes-each-k-around-a-loop-or-a-run]
---


Asked by Ev in chat (2026-11-20), and banded P1 at Ev's request: "is
there any kind of pattern functionality for profiles?"

## What exists

- **Repetition of bodies.** `docs/DESIGN.md` `[repetition]`: an index
  `k = index(N)` and one placement whose spin or slide reads it; the
  viewer's Pattern tool writes exactly that. There is no pattern node
  (`Node::Pattern` and `PlacedUnion` retired with FORK-PAT;
  `docs/KERNEL-VERBS.md` still names them).
- **One repeated run.** `circle_split(c, r, n, phase)`: `n` equal arcs
  of one circle, its `n` a literal.
- **A profile in a family.** A profile's expressions can read the
  family's index, which gives one profile per index value, each swept
  as its own body.

## What is missing

Nothing repeats a loop, or a run of pieces within a loop, inside one
profile: a bolt circle of holes in a plate's profile, a row of slots, a
gear's teeth along one outline. Each copy is written out by hand.

A profile with several outer loops is also refused today
(`MultipleOuterLoops`,
`work/profile/a-word-sketched-as-one-profile-refuses-multiple-outer-loops.md`),
so a pattern of separate outlines in one profile waits on that row as
well as on this one.

## The design question (`design`)

What repetition inside a profile is, given that `[repetition]` says a
pattern is an index and not an operation: whether a loop (or a run of
pieces) is written once and read at each index value, how a member is
named and edited, how it lowers so the replay and validation see an
ordinary program, and how it relates to SELFREF's reads of a profile's
own earlier parts (`work/selfref/`), which is the neighbouring way one
part of a profile depends on another.

## Not checked

Whether several inner loops in one profile can already be written by
index through some existing form. None was found by reading.

## Weighed (2026-11-20)

Two designers, one round, agreed (fork log row 296). What can be
written today, one level up, by reading and not run: a bolt circle or a
row of slots as `subtract(plate, union(cutters))`, the cutters one body
placed by `k` or a hole profile whose centre reads `k` extruded as a
family; teeth as `union(hub, teeth)` (the `comb` fixture in
`crates/editor-core/tests/g_index_families.rs`). What no level up says:
one outline as one value for a loft, a revolve or a sweep along a path
to read; holes as the sweep's own walls, named by the profile's pieces;
flanks joined to the root by an authored tangency. The recommended
final state is in `docs/DESIGN.md` `[repetition]` and
`docs/PATHS-DESIGN.md` §2 *Composition*.

## Cut (2026-11-22)

Ev ruled (PR 7263, 2026-11-21): "(a) is good! this plan sounds good".
The row moved from PROFILE to REPEAT by `git mv`. It is now parked on
its seven units, and priced `X` at 36 points: the five REPEAT units
(8 + 3 + 8 + 8 + 3) plus the viewer half and the Python half (3 each).
Only the units count towards a track's load; this row is parked.

In order:
1. `profile-readers-read-kernel-steps-through-the-written-address-map`
   (H): one map from kernel steps to written ones, identity today.
2. `a-profile-piece-name-carries-its-member` (M): beside 1.
3. `a-profiles-loop-list-repeats-a-loop-with-each` (H): on 1 and 2.
4. `a-chain-repeats-a-run-of-its-steps-with-each` (H): on 3.
5. `a-ring-closes-each-k-from-its-start-point` (M): on 4.
- `work/vseam/the-profile-editor-shows-and-writes-a-repeated-loop-or-run.md`
  (M): on 4.
- `work/pydoor/python-writes-each-k-around-a-loop-or-a-run.md` (M): on 5.

The cut's answer to "the flat `ids`, the authored address and
`SlotId::Profile` against a nested program": all three keep addressing
what was written. A braced step has one written address however many
copies it draws. Only readers of what the kernel drew go through the
map (unit 1).

Filed beside it: `a-selection-cannot-name-a-repeated-piece-by-an-index-expression`
(`design`, here); `work/profile/validation-pair-pass-has-no-pruning.md`;
`work/doctail/circle-split-holds-its-count-as-a-literal.md`;
`work/selfref/a-profile-point-has-no-2d-spin-move.md`;
`work/doctail/kernel-verbs-names-a-pattern-node-no-crate-has.md`. That
last one corrects this row's "What exists" note on `docs/KERNEL-VERBS.md`.
