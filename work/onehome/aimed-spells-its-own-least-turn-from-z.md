---
id: aimed-spells-its-own-least-turn-from-z
kind: issue
title: RigidMotion::aimed spells its own least turn from ±z (1 − a²/(1 + c)) beside LeastTurn and turned_between
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Found by PR 6962's FULL review (ONEHOME least-turn-axis). PR 6962 gives the mate/snap least turn its home, `geom_core::LeastTurn`, beside its sibling `RigidMotion::turned_between` (the same turn as a quaternion matrix). A third spelling of a least turn sits beside them:

- `RigidMotion::aimed` (`crates/geom-core/src/linalg/rigid_motion.rs:1229`) is the shortest turn carrying `ẑ` onto `d = (a, b, c)`, rational in `d` with the one divisor `1 + c` (entries such as `1 − a²/(1 + c)`), or, where `c` reads negative, the turn from `−ẑ` after a half turn about `x̂` (divisor `1 − c`).
- It is read by the mate solver's pose frames (`crates/editor-core/src/mate/solve.rs:551`, `frame_of`) and the path sweep's start frame (`crates/sweep/src/skin.rs:1948`).

Over the reals `aimed(o, d)`'s linear part is `LeastTurn::between(ẑ, d)`'s where `c` is non-negative; the negative branch is a different turn (the half turn first), chosen so its divisor stays at least one.

**Owed.** Decide whether `aimed` reads `LeastTurn` (its non-negative branch, and the negative branch as `LeastTurn::between(−ẑ, d)` after the half turn), or says at the site why its closed form is its own. Either moves bits on its readers (the mate pose frames, the sweep's start frame); measure which first. Fork 270 (PR 6964), which decides whether `LeastTurn` and `turned_between` become one, is the place to settle it beside them.
