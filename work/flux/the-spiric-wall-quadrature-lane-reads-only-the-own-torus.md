---
id: the-spiric-wall-quadrature-lane-reads-only-the-own-torus
kind: issue
title: the spiric-bounded torus-wall quadrature refuses a wall on the carrier's mirror torus
status: open
opened: 2026-10-12
priority: P3
cost: M
---


Filed by PCERT's `pcert/mint-routes-spiric-nofitted`. That branch gives
a spiric on its torus's MIRROR through the cutting plane a wall image:
`SpiricImage::Wall` with `u_sense = −v_sense`
(`crates/geom-brep/src/pcurve_cache.rs`, the variant's docs). The
spiric-bounded wall quadrature (`crates/topo/src/props/quad_lane.rs`,
`spiric_wall`, feeding `crates/geom-brep/src/props/spiric.rs`'s
`WallImageQ`) reads one `sense`, and its wall terms read the face's
torus as the carrier's own. So a mirror wall refuses typed there:
`QuadratureUnsupported`, "a spiric wall image on the carrier's mirror
torus". That is from reading the code: no solid bounded by a mirror
wall can be built through the public doors today, so the refusal has no
fixture.

The fix is to carry both signs into `WallImageQ`, and the mirror
centre `c + 2d·n` into the wall's flux terms. Then pin the area and
volume of the whole-oval face in
`crates/topo/tests/a_whole_spiric_oval_bounds_its_mirror_torus.rs`
against an own-torus twin, which the reflection carries over exactly.

No producer reaches the class today: torus×torus is unimplemented in
C5, and mirror is unbuilt. So this is latent, P3.
