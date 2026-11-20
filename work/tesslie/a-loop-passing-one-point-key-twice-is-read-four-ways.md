---
id: a-loop-passing-one-point-key-twice-is-read-four-ways
kind: issue
title: A loop passing one point twice is read four ways: walk::pinches, topo's check-9 grouping, planar::Pinches and a sweep helper
status: open
opened: 2026-11-20
priority: P1
cost: M
---


Found by TESSLIE's walk-slit unit and its dual review (branch
`tesslie/walk-slit`).

## What

"This face's loop passes one point at two corners" is read in four
places, each with its own notion of "one point" and "two":

- `mesh::walk::pinches` (`crates/mesh/src/walk.rs`): the outer loop's
  junctions grouped by point key, two DISTINCT vertices on one key. The
  route's pinch fact and the curved lane's `PINCHED_FACE_ROUTED_HERE`
  refusal read it.
- `topo::validate::loop_point_groups` (`crates/topo/src/validate.rs`,
  `pub(crate)`, tier 1's check 9): every loop's corners grouped by point
  key, two or more CORNERS, so one vertex visited twice (a seam walked
  both ways) groups too.
- `mesh::planar::Pinches::of` (`crates/mesh/src/planar.rs`): distinct
  mesh ids at one CDT handle, after spade dedups equal positions, so it
  reads positions, not keys.
- `faces_through_two_vertices_on_one_point`
  (`crates/sweep/tests/pinch_faces_tessellate.rs`): every loop's start
  positions compared by `equals_exactly`, a vertex met twice included.

Related, and reading the same structure: `topo::census::shared_segments`
and `topo::sector_shape::SlitTips` (a slit as two edges on the same two
point keys).

They agree on every body measured so far, and nothing pins that they do.
A body whose two vertices at one place carry two point records (an
import, or a parting that mints a record per wedge) splits them:
`walk::pinches` and topo's grouping see no pinch, while the planar lane
and the sweep helper see one.

## Owed

Decide the one reading (point key, distinct vertices, is the body's own
statement), give it one home in `topo`, and have the three others read it
or state why they differ. `walk::pinches` was not moved onto topo's
grouping in the unit that wrote it: that grouping counts corners, not
vertices, and is crate-private in a contended file.
