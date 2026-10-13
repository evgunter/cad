---
id: mint-has-no-route-to-the-spiric-and-no-fitted-classes
kind: issue
title: the mint has no route to two minor uncovered classes, so their faces stay uncached under C4's exemption
status: closed
opened: 2026-10-02
priority: P2
closed: 2026-10-12
branch: pcert/mint-routes-spiric-nofitted
---

Filed by PCERT's `pcert/at-rest-rows-mandatory` (PR 3759), which makes
pcurve rows mandatory at rest (C4). C4 says every class of carrier a
chart can hold has a route into a certified row, and a face no route
covers refuses at the producer. Until each class's route lands, the
mint and tier 3 excuse it through one predicate
(`crates/topo/src/pcurves.rs`, `not_owed`), on
`PcurveCertifyError::UnsupportedCarrier` with its `UncoveredClass`.

This row is the schedule for `UncoveredClass::MirrorTorusSpiric, NoFittedClass`: two: a spiric on its torus's mirror through the cutting plane; and a line, ellipse or spiric offered a fitted-grade image, for which no fitted class exists. (A third, the zero-offset spiric, left the roster when `Curve3::spiric` began refusing a plane through the axis: that oval is a `Circle` carrier, so no spiric is one.)

When the route lands, the class leaves `not_owed`'s first arm in the
same change, so a face of that class is minted or refused at the
producer.

## Closed (branch `pcert/mint-routes-spiric-nofitted`, 2026-10-12): split by class

**`MirrorTorusSpiric` has its route, and the class is deleted.** About
the mirror torus (centre `c + 2d·n`), the oval's radial part is
`−n·d + m·f`, so the wall image is the own-torus one with the azimuth
channel's sign opposed: `u = u₀′ − σ·atan2(f, d)`, `v = σ·t`, where
`u₀′` is the azimuth of `−n`. `SpiricImage::Wall` keeps its one
`sense` and gains a `WallTorus` tag (`Own` or `Mirror`), which fixes
the azimuth's sign relative to `sense` and names which torus check 1's
chart gates measure against. `SpiricIdentity` holds by the same
algebra. A whole oval on its mirror torus now mints through public
`mvfs`, `set_face_surface` and `mef`, and tier 3 reads it clean
(`crates/topo/tests/a_whole_spiric_oval_bounds_its_mirror_torus.rs`);
on main (1f7ef8844e), that `mef` refused at the edge's description
residual ("definitely exceeds the tolerance band … this is a kernel
defect; report it"). Main's derivation handed the mirror chart the
OWN-torus wall image, which the residual refuses at sample 0. No producer
reaches the class: torus×torus is unimplemented in C5, and mirror is
unbuilt. `Pcurve::mirror_v` still answers `None` for a wall image on
either torus: `(u, v) ↦ (u, −v)` reverses a torus chart's orientation,
and no torus holds the reflected locus as a wall image.

**`NoFittedClass` is the fitted-row lane's.** The orchestrator ruled
that the fitted-grade row becomes mate-free, certifying any carrier on a
spline chart against carrier and chart alone, and deletes the class;
that lane's PR closes `no-fitted-class-misuse-is-excused-by-not-owed`.
This branch's reach measurement for it is in its PR body.
The spiric-wall quadrature lane's refusal
of a mirror wall is `the-spiric-wall-quadrature-lane-reads-only-the-own-torus`.

**The NoFittedClass half closed with the class** on
`pcert/fitted-row-reads-carrier-and-chart` (PR 4900). The fitted-grade
row reads its carrier and chart alone, so a line, ellipse or spiric on
a spline chart certifies its `General` image at the hull, or refuses
there typed. The mint's `Intersection` arm gives such a carrier its
`General` image after the exact candidates. With both classes routed,
`UncoveredClass`, `PcurveCertifyError::UnsupportedCarrier` and
`not_owed`'s class arm are deleted in that PR.
