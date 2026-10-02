---
id: long-turn-helix-has-no-demo
kind: unit
title: a long-turn helical sweep (a coil spring) has no demo
status: open
opened: 2026-10-02
priority: P4
cost: M
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
