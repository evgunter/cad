---
id: a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face
kind: unit
title: A ray that passes within the band of an edge meets it, half a crossing for each face the edge bounds, and reads the role off its signed count (D10, Ev on PR 5119)
status: open
opened: 2026-10-15
priority: P2
cost: H
---


The ray half of Ev's ruling on the needle class (PR 5119, 2026-10-15,
fork-log row 173). The volume half landed in PR 5119 itself.

**Ratified text.** D10 "Booleans" says: "A ray that passes within the
band of an edge meets it, half a crossing for each face the edge bounds
(Q1), and reads the role as the sign of its count of crossings; it
refuses only on two faces that do not meet whose crossings lie in band
of each other, a pair, and a ray that meets a corner or a wedge reads
nothing." Q1 says an instrument does not represent a tie-break the band
forbids it to make.

**Today.**
- Each face decides for itself whether a crossing lies in it. The
  in-face walk refuses a crossing within Kε of a loop boundary:
  `point_in_loop` (`crates/topo/src/splitting/containment.rs:1883`),
  reached through `point_in_loop_projected`
  (`crates/topo/src/boolean/solid_contain.rs:3930`).
- So a ray aimed at a face narrower than 2Kε is set aside, and so is
  one landing near any edge. The reader
  (`crates/topo/src/side_reader.rs:439`, `side_at_infinity`) then
  falls back to the volume.
- The same walk serves `point_in_solid`
  (`crates/topo/src/boolean/solid_contain.rs:3967`), so needle
  operands lose rays at the boolean's containment fallback too.

**What is owed.**
1. **Planar faces first.** A crossing within the band of an edge meets
   that edge, and counts half a crossing for each face the edge bounds;
   a crossing within the band of a vertex reads nothing. The ray reads
   the role as the sign of its signed count (+1 entering material, −1
   leaving it); crossings in band of each other are summed, not
   ordered.
   - It refuses only on a pair: two faces that do not meet
     (`side_reader.rs:347`, `meet`).
   - The designers measured an edge-by-edge probe reading all 20
     needle cases (w ∈ {10.5, 12, 15, 21, 30}ε, ε 1e-9 and 1e-6, both
     orientations). It is on branch `flush/needle-design-x` (head `feb5b660e2`) (fork-log
     row 173).
2. **Curved faces.**
   - Two faces on one carrier (a seam split) share one comparison of
     the root against the seam parameter.
   - An edge between two carriers needs one number per edge and ray:
     the ray's signed offset from the edge near the crossing.
3. **`point_in_solid` reads through the same walk,** so its needle
   operands keep their rays. That is CLEAVE's
   `point-in-solid-sets-a-ray-aside-near-a-narrow-faces-edge`, which
   this unit closes when it lands; settle the split with CLEAVE before
   dispatch.
4. **Re-measure:**
   - the needle row's fixtures;
   - the `brick ∩ brick` needle poses;
   - the 1e-12 torus (`p3b_a_band_scale_major_radius_through_the_revolve_door`);
   - the five batteries.

   Re-pin whatever moves, and say what moved.

**Done when** a ray within the band of an edge meets it; set-asides
are left only at vertices, wedges and real pairs; and the fixtures
above read by ray rather than by the volume fallback.

**It also retires the aims' exact ties at symmetric nominals**
(`work/flush/the-side-readers-aims-decide-exact-ties-at-a-documents-nominal.md`):
a planar aim's `point_in_loop_arm` and a wall aim's `curve_foot_branch`
read exactly 0 at `die_chamfer`'s and `cut_cylinder`'s nominals, read
apart today as the reader's family.
