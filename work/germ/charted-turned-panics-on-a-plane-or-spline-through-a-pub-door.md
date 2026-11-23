---
id: charted-turned-panics-on-a-plane-or-spline-through-a-pub-door
kind: issue
title: Charted::turned panics with unreachable! on a plane or spline handed in through a pub door
status: open
priority: P2
cost: E
opened: 2026-11-23
---


Found by designer B on PR 7645's fork, by reading `crates/geom-brep/src/curve_pair/on_surface.rs` (`Charted::turned`). It is an input-reachable panic of the same family as the raw-surface doors. Refuse typed, or exclude the kinds by type at the door.

Signed: (GERM orchestrator)
