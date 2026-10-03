---
id: long-turn-helix-has-no-demo
kind: unit
title: a long-turn helical sweep (a coil spring) has no demo
status: closed
opened: 2026-10-02
priority: P4
cost: M
closed: 2026-10-03
pr: 3918
---

## What

M8-14's long-turn sweeps (`crates/sweep/tests/m8_14_long_turn_sweep.rs`)
build a helix of many turns as one body, and no scene has ever shown
one. A compression spring is the natural part. Fold it into an
existing cell rather than adding one; the candidate is `projectbox`'s
section (a contact spring standing on a boss, visible because the
section opens the box). Take it after
`projectbox-section-cuts-through-bores` so the two do not collide in
one file, and only if it makes that cell a better picture; otherwise
close it with the reason.

## Closed

By #3918. A six-turn square-wire spring stands on a projectbox boss and
is split by the section. Round wire refuses `QuadratureBudget` (wall 1,
evidence on `work/quad/a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin.md`).
The spring is not sayable from Python, so audit row 40 is NO on G2.
Residue: `work/tess/a-long-helical-wall-meshes-far-under-its-delta.md`,
`work/carve/sweep-body-makes-every-caller-derive-its-start-frame.md`.
