---
id: a-tangent-ruling-clear-of-the-region-keeps-its-tangency
kind: issue
title: A tangent ruling the region does not reach lists no component but keeps its tangency, so the guard reads R-tan for a pair that cannot meet
status: open
opened: 2026-11-22
priority: P2
cost: E
---

Filed by PR 7354's review (SECTCONE U1).

## What

Where a pair's section is a tangent ruling and the operation's region does not reach that ruling, the value lists no component, but it still records the tangency among its verdicts:

- plane × cylinder: `crates/geom-brep/src/pair_section/conic_pairs.rs`, `plane_cylinder` (~323: `TangentLine` → `ruling(..)` is `None` past the region; ~326: `decided` keeps `pc_parallel_gap`);
- two parallel cylinders: `crates/geom-brep/src/pair_section/cylinders.rs`, `cylinder_cylinder` (~899: `ParallelSection::Tangent` → `ruling(p)` is `None`; ~1059: `decided.extend(pose.tangency())`).
- a cone against a cylinder or a cone (PR 7357's review): `crates/geom-brep/src/pair_section/cones.rs`, `cone_ruled`, the `Sheets` arm. Every double root the sweep decides a tangency (`ruling::Touch`) pushes `Tangency(ruling::TANGENCY)` into `decided`, including a touch at a point the region does not hold. Only the `Point` component is filtered (by `placement_of`), and a crossing node by the sweep's `crossings`. Step-import no longer reads this flag for cone pairs, since it refuses them earlier by `pair_section::uncarried`. The section certificate will read it once it reads cone pairs on the value (`the-section-certificate-reads-the-cone-pairs-on-the-value.md`).

The guard (`section_cert::of_pair_section`) reads a section with a tangency and no crossed or unbounded component as R-tan. So a pair whose surfaces touch only beyond every face the operation holds refuses as tangent, when it cannot meet inside the region at all.

## Done when

A tangency the region does not reach is either not among the section's verdicts, or the guard reads an empty component list beside a tangency as apart. Both arms take the same reading, and a row pins each.
