---
id: a-knife-edge-plane-blend-divides-by-one-plus-the-normals-dot
kind: issue
title: band: plane_plane_blend's centre and setback divide by 1 + n_a.n_b, which cancels at a knife-edge wedge
status: open
opened: 2026-10-20
priority: P2
cost: M
---


## Finding

Found by the sweep of PATHS' `a-hairpin-line-fillets-half-tangent-cancels`
(PR #5441). That row's line×line fillet read tan(φ/2) as
`(v₁ × v₂) / (|v₁||v₂| + v₁·v₂)`. At a hairpin the sum cancels, and on main
it built and validated arcs 8.1e-3 m off their end vertex. This is the same
shape in 3D.

`sweep::blend::arms::plane_plane_blend` (`crates/sweep/src/blend/arms.rs`,
about lines 355–358) computes, with `d = n_a·n_b`:

- the centre `p − (n_a + n_b)·(signed / (1 + d))`
- the setback `radius·√((1 − d)/(1 + d))`

At a knife-edge wedge (the faces' normals nearly opposite, d → −1) both
`1 + d` and the vector `n_a + n_b` cancel. With the wedge's opening angle δ,
`1 + d ≈ δ²/2` while its rounding is about 2⁻⁵³, so the centre and the
setback carry a relative error of about 2⁻⁵²/δ². The hairpin fillet's
cancellation-free spelling is the select between the two half-angle
quotients (`profile`'s `sugar::split_half_tan`). The analogue here
reads tan of the half-angle from `|n_a × n_b|` and `1 − d` past a quarter
turn.

**Reachability is unprobed.** Whether a knife-edge wedge narrow enough to
lose ε reaches this arm past the blend's own gates (the dihedral and
convexity reads, the setback fit against the faces) has not been measured.
Band P2 until a probe says otherwise. If a probe builds a wedge whose blend
lands past ε and validates, it is P0 (a silent wrong answer).

**A lower candidate, same shape:** `topo/src/boolean/ops.rs` (about line
6399), the Rodrigues rotation's `kx(kv) / (one + c)`, cancels at `c → −1`.
Its comment says the alignment trilean above refuses the antiparallel arm
(`recut_lean`), so it is guarded by construction; confirm that the guard's
band is wide enough for the `1/(1 + c)` it protects.
