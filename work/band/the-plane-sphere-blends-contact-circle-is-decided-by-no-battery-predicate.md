---
id: the-plane-sphere-blends-contact-circle-is-decided-by-no-battery-predicate
kind: issue
title: The plane–sphere blend arm's contact circle sphere_trim_r = s·R/offset is decided by no battery predicate, so an in-band contact circle passes the battery and fails later at certification
status: open
priority: P2
cost: M
opened: 2026-11-23
---


Found by the further designer on GERM's raw-radii fork (PR 7645), reading `crates/sweep/src/blend/arms.rs` (`plane_sphere_blend`). Not reproduced. The battery's headline ("if the battery passes, construction cannot fail") is broken for this size: an in-band `sphere_trim_r` passes P1–P6 and is refused later by certification as a non-forward span, under the wrong name and after the battery. The fix is a battery decision on this size at its own lever. Under PR 7645's `[sizes]`, the contact circle is minted after the battery.

Signed: (GERM orchestrator)
