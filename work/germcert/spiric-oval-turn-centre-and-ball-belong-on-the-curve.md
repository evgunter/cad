---
id: spiric-oval-turn-centre-and-ball-belong-on-the-curve
kind: issue
title: geom: a spiric oval's turn centre k_σ and its closed-form ball are derived in topo's join from Curve3::Spiric's fields
status: open
opened: 2026-10-11
---


(GERM U-T1 fix pass, PR 4614.)

## What

`boolean::join::pt_germ_frame` (`crates/topo/src/boolean/join.rs`,
the `SpiricOvals` arm) unpacks `Curve3::Spiric { center, axis, u_ref,
major_radius, minor_radius, offset }` and derives two facts about the
oval from them:

- **its turn centre** `k_σ = c + n̂d + σŵ√(R² − d²)`, where the core
  circle crosses the oval's plane inside it: the point the oval turns
  about strictly monotonically (the proof is that function's doc);
- **its closed-form ball**: `t` between `√((R − r)² − d²)` and
  `√((R + r)² − d²)` along the side of the axis's trace, `|y| ≤ r`.

Both are properties of the curve, not of the join. They sit in topo
because the frame is their first consumer. The chord unit for the ovals
(GERM U-T4, `docs/GERM-TORUS-PLANE-FRAME-SPEC.md` §3) needs the same
oval's own parameterisation and turn, and `geom_brep::Reach::lever_from`
already reads a `Spiric` span by the whole torus's support (`extent.rs`),
which the closed-form ball would tighten.

## Shape

Methods on `Curve3::Spiric` (or a `Spiric` view of it) in `geom`
returning the turn centre and the ball; `pt_germ_frame` and the chord
arm read them, so that one derivation serves both. Not urgent: nothing
is wrong today, and the one consumer of the ball (`bool_connect`'s
antipode for a one-site loop) is unreachable for the torus until the
germ-pair dispatch has a torus arm (U-T2).

## Moved (2026-10-26)

From CURVED, which closed (Ev, in chat: "curved can close without a walk, and rows to germcert is good"; `docs/doc-ledger/curved-leaves-the-tracker.md`). Both rows came from GERM lanes and are about the spiric carrier's own facts.
