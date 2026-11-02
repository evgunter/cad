---
id: a-smooth-joint-whose-walls-part-in-band-refuses-sliver-join
kind: issue
title: A smooth (wedge-pi) profile joint whose walls' relative curvature is in band refuses SliverJoin at extrude
status: open
opened: 2026-10-19
priority: P3
cost: M
design: true
---


Found by designer A on the cusp fork (#5354) ("a smooth joint with
in-band κ_rel refuses SliverJoin, the must-carry lane's choice");
filed by `paths/joint-reads-its-turn`, which pins it
(`crates/sweep/tests/a_tangent_joint_on_near_concentric_carriers.rs`,
`a_join_whose_walls_part_in_band_refuses_its_sliver`, the "smooth"
row).

**The witness.** An arc of the unit circle into the joint `(1, 0)` and
an arc of the circle about `(17ε, 0)` of radius `1 − 17.5ε` out of it
ON ROUND through 1.6 rad, closed by a line: a smooth tangent joint
(wedge π), validated at every ε row. Extruded to depth 1, the strut's
dihedral reads `Smooth` and extrude's step 4 descends through the
must-carry rule (`crates/sweep/src/extrude.rs`, the
`DihedralClass::Smooth` arm; the rule's home is
`geom_brep::must_carry_description`, `crates/geom-brep/src/dihedral.rs`).
The walls' relative curvature `17ε` levered at the unit arm is 8.5ε,
in band, so the rule's in-band arm refuses `ExtrudeError::SliverJoin`
on `tangent_second_order`, at `f64` and `Interval`, at 1e-6, 1e-9 and
1e-12. The cusp on the same carriers refuses the same way, which is
right for a cusp: its legality is the second-order arm.

**Why it is a question.** A π seam's legality does not hinge on its
walls' relative curvature: they lie on opposite sides of the strut, so
nothing about the body is undecided. What the in-band arm leaves
undecided is the edge's DESCRIPTION, `TangentIntersection` (the
surfaces determine the locus) or an image in the previous wall's chart
(they do not). The under-determined arm already stores the image when
`κ_rel` reads Zero; storing it in band as well would build the body,
and refusing is the conservative choice the rule makes today. Whose
call: CARVE's, as the owner of extrude's join arm and the must-carry
rule's sweep callers (SELECT's file holds the rule; PROPS is told of a
must-carry wrapper change, per CARVE's `keep_out`).
