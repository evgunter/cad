---
id: the-cone-ruled-side-on-the-section-value
kind: unit
title: The section value takes a cone ruled side: cone × oblique cylinder and cone × cone poses, classified on pair_section
status: closed
opened: 2026-11-21
priority: P1
cost: H
parent: cone-germ-pairs-against-a-curved-face-have-no-section-frame
branch: sectcone/cone-ruled-side
closed: 2026-11-22
pr: 7357
---

## Closed (2026-11-22, PR 7357)

Cone × oblique cylinder and cone × cone are classified on the section value (`pair_section/ruling.rs`, `pair_section/cones.rs`). A ruling sweep finds both nappes' components. A crossing tangency is a C5 node, whose arcs form one `NodeSet`, and an apex meeting is a node, a touch or a refusal. The cone pair is `Arm::ConeRuled`, which is classified but has no march: `uncarried` refuses `SECTION_LANE` with "no march carries it" until the cone pair doors land.

- Review: dual Opus on `ed50ae6f45`, both APPROVE-WITH-FIXES with one MAJOR (a crossing tangency was listed as two components), fixed as a node. A fix-verification review on `94e2110b42` found a second, demonstrated MAJOR: an arc across the cone's seam took the other arc's witnesses. It was fixed in the walk and pinned by a row over 48 seam angles. A re-verification on `3335b164d8` returned APPROVE-WITH-FIXES with no MAJOR. Its two MINORs are filed as `checked-is-blind-to-a-wrong-arc-witness-by-a-node`.


The first of three units on `cone-germ-pairs-against-a-curved-face-have-no-section-frame`:

- **U1, this unit:** the pose and classification on `geom_brep::pair_section`. It is geom-brep only.
- **U2:** topo's section certificate and section table read the value, and the duplicate ruling sweep in `topo::boolean::section_cert::ruling` retires.
- **U3:** the join reads the value. U3 waits on the march, so it waits on `the-cone-has-no-implicit-enclosure-or-metres-conversion`.

**Review tier: dual** (two Opus reviewers; Ev 2026-11-21, Fable unavailable). The logic is especially tricky: the apex node and touch, a double root decided as a tangency with an upper clearance bound, and unbounded branches on both nappes. It also sets the cone's pose shape on the shared section object. Difficulty letter: H.

## Spec

Build the cone × oblique cylinder and cone × cone poses on the section value. They are instances of the ratified section object: C5's pose, ruling-sweep and node rules in `crates/geom-brep/README.md`, decided on `[ev]` PRs 6127 and 6432, and recorded in this unit's parent's `## Decided` section. They must never become a per-pair frame or lane beside it. The template is the skew cylinder pair: `pair_section/cylinders.rs` (`CylinderCylinderPose`), `PairPose::CylinderCylinder`, and `pair_section/ruling.rs` (`Ruled`, `sweep`, `coverage`).

1. **`pair_section/ruling.rs` takes a cone ruled side.** Add a `Ruled` implementation (or implementations) for a cone's rulings against a cylinder and a cone partner, and a cone side in `coverage`. The sweep must handle each of these:
   - a ruling whose quadratic's leading coefficient vanishes (an asymptotic direction): use projective roots, and let a sheet leave the region ball as an open component with cut ends;
   - the apex root: where the apex lies on the partner, divide it out;
   - both nappes.
2. **The pose** (a new `pair_section/cones.rs`, or wherever reads best) decides its margins in closed form over the region:
   - **The apex margin:** the partner's residual at the apex. Its Zero reads the cone's quadratic form on the partner's tangent plane: indefinite gives a node whose lines are the two rulings in that plane, definite gives a touch, and degenerate refuses. A Zero is never read as R-tan.
   - **A double root of the discriminant:** decided as a tangency. Zero needs an upper clearance bound at the root, because the sweep's residual is only a lower bound.
   - **The axis and aperture margins** that pick the pose.
   
   A shared-apex cone pair stays where it is today (`fan::cones_share_apex`); the C5 node criterion does not apply to it. The coaxial and parallel poses that `topo`'s `section_cert::cone_pair` reads in closed form are not this unit's scope unless they fall out naturally. Say what you did with them.
3. **`pair_section.rs`:**
   - add the `PairPose` variants and the `pair_section` arms;
   - list the apex node in `PairSection::nodes`, with arcs through the apex forming one `NodeSet`;
   - represent a touch as C5 says.
   
   The region: `traced_in_r3` and `region_for` currently tie "read over the operation's region" to "residual in metres". The cone pose needs the region before its metres conversion exists. Separate the two concepts where the code conflates them, and do not fake a metres residual. Components carry `ClassCarrier::General`. Their march (`PairSection::carry`) will refuse until the cone's metres conversion lands; that is expected, so make the refusal typed and honest.
4. **Algebra to port:** the cone chart and quadric algebra, and the per-metre clearance bound, currently live in `topo`'s `boolean/section_cert/ruling.rs`. Port what the value needs into geom-brep (`geom_brep::half_angle` is already there), and leave `topo`'s copy for U2 to retire. Say in the PR which functions are now duplicated, so U2 has its list.
5. **Tests:** geom-brep rows, pinned against a traced oracle. Port the relevant fixed rows and poses from `crates/topo/src/boolean/section_cert/cone_pair_search.rs` and `section_cert_cone_pair_rows.rs`, e.g. `a_rod_biting_the_side_cuts_one_null_loop`. Rows must cover:
   - the apex node;
   - the apex touch;
   - a tangency;
   - an unbounded branch cut by the region;
   - both nappes;
   - a clear pose.
   
   Each row must be able to go red when the pose misclassifies.

Out of scope: `topo` (U2/U3), the cone's implicit enclosure and metres conversion (the P0, with designers now), and cone × sphere and cone × torus.

Territory: the ruling sweep is SECTARMS-built and shared by design. PR 7203 (`oneround/anchored-conic`) makes small edits to `pair_section.rs`, `cylinders.rs` and `gap.rs`. Run `python3 scripts/work.py territory --base origin/main` and announce any seam in the PR.
