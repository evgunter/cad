---
id: a-cone-rim-chord-joined-to-a-higher-row-certifies-above-delta-s
kind: issue
title: a cone face's rim chord, sized on the small rim's radius, joined to the first grid row certifies above δ_s, and above δ when the radii differ enough
status: open
opened: 2026-10-25
priority: P1
cost: M
---

Found by a designer weighing PRED's `S29` (the sizing policy), on
2026-10-25. Read, NOT executed: the first act owed is a row that
builds it.

`mesh::chords` sizes a rim's chords on the rim's own radius
(`sagitta_step(δ_s, circle_radius(carrier))`). It tightens a rim to the
face's grid step only for the torus (`torus_boundary_step`) and NURBS
(`nurbs_tighten`). `mesh::cert`'s cone certificate (`cert_cone`)
reads the triangle's own `v_max` and Δu. Consider a truncated cone
whose small rim ρ_small is chorded coarse in u, and whose first grid
row sits at a larger radius ρ_row. The rim-adjacent triangle then
certifies at about δ_s · (ρ_row / ρ_small) · cos α. That is above δ_s,
and above δ = 2δ_s once ρ_row / ρ_small ≳ 2 at a coarse δ. The halving
in `sizing::sizing_target` hides it most of the time, and nothing pins
the margin.

Possible outcomes are a typed refusal (`CertificateExceeded`, sound) or
a mis-mesh (unsound). Which one is not established; that decides the
band (P0 if it mis-meshes). The sphere has the same shape at a smaller
size. A latitude rim, chorded on r·cos φ, joined to a grid row comes to
about 0.9δ today against 0.8δ for an interior cell. The repair the
designer proposed is to extend the chord pass's tightening door to the
sphere and the cone (rim count ≥ ⌈Δt/h⌉ at the face's grid step). It is
part of PRED's `S29` ruling, which goes to Ev. Signed (PRED orchestrator).
