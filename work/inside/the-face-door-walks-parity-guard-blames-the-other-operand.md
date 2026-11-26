---
id: the-face-door-walks-parity-guard-blames-the-other-operand
kind: issue
title: The ring composite walk's parity guard refuses Unread, which reduce::composite_count reports as the other operand's InvalidInput, so a walk self-contradiction blames the user's input
status: closed
opened: 2026-11-26
priority: P3
cost: E
refs: [7801]
closed: 2026-11-26
pr: 7813
branch: inside/face-door-spline-arm
---

Found by PR 7801's verifier (2026-11-26). It was not fixed in that PR, because the PR's slow set costs about 3 h a run.

`crates/geom-brep/src/curve_face/composite/walk.rs` `read` (~:276) has a parity guard. When both span ends are past the band, the number of crossings must have the parity of the ends' sides. If it does not, the walk refuses `CompositeRefusal::Unread`, which becomes `MeetFault::Unsupported`. `boolean::reduce::composite_count` maps `Unsupported` to `BooleanError::InvalidInput { operand: x_is.other() }`. So a walk that contradicts itself (a kernel invariant) is reported as the other operand's invalid input. The region door blocks on the refusal, which is fine.

No row reaches the guard. It is defensive, and a fuzz-oracle mutation showed that it masks paired losses as a refusal.

Done when the guard's refusal carries a kernel-invariant class (for example a `ClassificationInvariant` ending) that no door attributes to an operand, and a row pins it, if a fault seam already exists.

**2026-11-26, PR 7801 at `83118f42d`.** The guard now refuses `CompositeRefusal::Contradiction`, which maps to `MeetFault::Contradiction`, a kernel fault. `reduce::composite_count` reads it through `CrossWhy::fault()` as `BooleanError::ClassificationInvariant` before its `Unsupported` → `InvalidInput` arm. The split reads it as `CrossingReading::Fault`, and the region door fails loud on it. No door names an operand. No row pins it: no fault seam reaches the walk. The site (`walk.rs` `read`) says why no input reaches the guard while the walk holds its invariants.
