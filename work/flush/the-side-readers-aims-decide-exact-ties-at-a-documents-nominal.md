---
id: the-side-readers-aims-decide-exact-ties-at-a-documents-nominal
kind: issue
title: The side reader's aims decide exact ties at a document's nominal, so revalue_corpus rows lose their apart move
status: open
opened: 2026-10-19
---

Since PR 5119, check 7 reads each solid with the side reader's aimed
rays (`crates/topo/src/side_reader.rs:257`, `aim_at`), so every at-rest
gate runs the aims' own decisions. Two aims sit on symmetric points of
the stored geometry:

- a planar face aims at its certified interior point
  (`crates/topo/src/stands.rs:258`, `planar_interior_point`), whose
  in-face walk decides `point_in_loop_arm` against an axis-aligned
  schedule direction;
- a cylinder, cone or torus face aims at the middle of its chart window
  (or the point (π, π)), where the ray's foot on a circular edge decides
  `curve_foot_branch`.

At a document's nominal those margins can be exactly zero, and any move
of the variables flips them. The reader's answer does not turn on them
(`[verdicts]`), but `editor-core`'s `revalue_corpus` compares the two
builds decision for decision, so:

- `die_chamfer` parts under the apart move at decision 1617 of 4547
  (`point_in_loop_arm`, 0.0 → 1.43e-7) and is pinned in step;
- `cut_cylinder` flips `curve_foot_branch` twice under the in-step
  move (decisions 1211 and 1214, 0.0 → ∓2.2e-16), owed here.

On main, check 7 reads a volume sign and casts no ray, so neither moves.

**The fix belongs with the edge-meeting ray**
(`work/flush/a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face.md`):
aim off every symmetric point of the face (a golden-ratio share of the
window, a planar aim off its loop's axes), or record the aims'
decisions under the reader's family on the symbolic recorder too, so
the instrument reads them apart. Either returns `die_chamfer` to its
apart move (floor 1690) and clears `cut_cylinder`'s owed flips.
