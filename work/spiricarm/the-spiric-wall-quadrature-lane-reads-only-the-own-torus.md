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
`SpiricImage::Wall` tagged `WallTorus::Mirror`, whose azimuth sign is
`−sense` (`crates/geom-brep/src/pcurve_cache.rs`, the variant's docs).
The spiric-bounded wall quadrature (`crates/topo/src/props/quad_lane.rs`,
`spiric_wall`, feeding `crates/geom-brep/src/props/spiric.rs`'s
`WallImageQ`) reads one `sense`, and its wall terms read the face's
torus as the carrier's own. So a mirror wall refuses typed there:
`QuadratureUnsupported`, "a spiric wall image on the carrier's mirror
torus".

The refusal has a fixture, through the public doors:
`topo::mass_properties` on the whole-oval mirror face of
`crates/topo/tests/a_whole_spiric_oval_bounds_its_mirror_torus.rs`
reaches it, and that file's
`the_quadrature_lane_refuses_a_mirror_wall_typed` pins it (a review
mutant that read the mirror wall as an own wall survived until that row
landed). What no public door builds is a closed SOLID bounded by a
mirror wall.

The fix is to carry the tag into `WallImageQ` (the azimuth sign `−σ`),
and the mirror centre `WallTorus::center` into the wall's flux terms.
Then turn the pinned refusal into the area and volume of the whole-oval
face, against an own-torus twin, which the reflection carries over
exactly.

No producer of a solid reaches the class today: torus×torus is
unimplemented in C5, and mirror is unbuilt. A caller of
`mass_properties` on a face built through the Euler doors does reach
the refusal, which is typed rather than wrong. So it stays P3.
