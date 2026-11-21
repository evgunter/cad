---
id: a-loop-cannot-repeat-inside-a-profile
kind: issue
title: A loop or a run of pieces cannot repeat inside one profile: patterns exist for bodies only
status: open
opened: 2026-11-20
priority: P1
cost: M
design: true
---


Asked by Ev in chat (2026-11-20), and banded P1 at Ev's request: "is
there any kind of pattern functionality for profiles?"

## What exists

- **Patterns of bodies.** `Node::Pattern` (linear, circular, explicit)
  with `PlacedUnion`; a pattern is an index over a family
  (`docs/DESIGN.md` `[repetition]`).
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
