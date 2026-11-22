---
id: the-departure-is-levered-at-the-reach-alone-where-check-4-folds-the-faces-curvature-arms
kind: issue
title: The embedding question levers how two faces leave their shared edge at the faces' reach alone, where tier 3's wedge check folds their curvature arms in, so a pinch between stations that is curvature-limited passes both
status: open
opened: 2026-11-22
priority: P1
cost: M
design: true
---



## Finding

From EMBED's lane `embed/remeter` (2026-11-22), which re-metered the
departure reading under Ev's ruling on PR 7095.

`sign_on` (`crates/topo/src/injective.rs`) decides how a face leaves a
separator as a sine levered at `arm_of`'s arm: the reach of the two
faces from the middle of the edge they share
(`crates/topo/src/wedge_reach.rs`, `reach_over`, the recipe
`edge_wedge_reach` is built by). That is the lever `edge_wedge`
(`crates/topo/src/edge_wedge.rs`) hands tier 3's check 4.

Check 4 does not lever at that reach alone. At each station
`geom_brep::folded_lever_arm` (`crates/geom-brep/src/dihedral.rs`)
takes the least of the reach and the two faces' curvature arms
(`curvature_lever_arm`: a cylinder's or a sphere's radius, a torus's
tube radius, a cone's radial distance at the point).

So where a face's curvature radius is shorter than the faces' reach,
the embedding question levers the same sine longer than check 4 does at
the same point. Check 4 reads only at its stations
(`work/restfront/check-4-reads-an-edges-wedge-at-stations-where-the-embedding-question-reads-the-same-margin-over-the-whole-edge.md`),
so a wedge that dips into the band between two stations, by the
folded arm and not by the reach, passes both readings.

**The fold is the more faithful tolerance.** A cylinder whose axis is
ε off turns its normal by ε over its radius, not over its length; a
cone whose apex is ε off turns its normal at radial distance ρ by ε/ρ.

**The fold cannot be read over a cell that holds an apex.** Measured
(prototype on `embed/remeter-measure` at `d1ed935d9`, default ε, the
arm folded with each face's curvature arm over the cell's image):
`self_intersection_proves_injectivity::a_revolved_cone_holds` refuses,
3,573 leaves refused, where it holds at the reach alone. A cell holding
the apex has an arm enclosure `[8.8e-17, 0.35]`: the cone's curvature
arm is its radial distance, which is 0 at the apex, so the margin
reaches 0 at every depth. The suite's other 55 holding rows hold under
either arm.

**Tier 3 passes the same edge only because its stations are interior**:
the certification schedule's interior never samples the apex end of an
edge, so its folded arm is positive at every station.

## What is needed

A design answer, not a fix a lane picks: whether the embedding
question's arm folds the curvature arms in.
- If it does, every cone that owns its apex refuses unless something is
  conceded within the band of the apex. That is a new ε concession, and
  Ev's to rule.
- If it does not, this row's gap stands: the two readings agree on the
  sine and the lever's source, and differ in the arm wherever a face
  curves tighter than it reaches.

Until then the arm is the reach alone, which is the ruling's word
("metered at the reach of the faces it governs").
