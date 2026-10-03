---
id: merge-coplanar-faces-returns-the-kept-boundary-described-against-the-absorbed-key
kind: issue
title: merge_coplanar_faces is public and returns a body whose kept face's boundary still names the absorbed face's key; only the boolean's describe_minted_edges repairs it
status: review
branch: topo/merge-door-re-describes
pr: 3702
opened: 2026-10-01
priority: P2
cost: M
---


## What

Found independently by both designers weighing the kef/kfmrh-across-keys
fork (TOPO, 2026-10-01). It was read, not executed.

`Body::merge_coplanar_faces` / `merge_coplanar_faces_declared`
(`crates/topo/src/merge_faces.rs`, ~:1318 and ~:1442) are public. A
group of coplanar faces on **distinct** keys is merged by `kef_minting`
(`merge_group`). The absorbed face's boundary edges then lie between the
kept face and their other neighbour, but their descriptions still name
the absorbed face's key.

The door gates its result with `validate_closed`, which is tier 1 and
tier 2. Tier 3's `DescriptionNotAdjacent` therefore goes unseen at the
door, and the body is returned with the kept face's boundary stranded.

Every production caller is inside the boolean (`boolean/ops.rs` ×3 and
`boolean/rest.rs`). Each repairs the strand afterwards through
`describe_minted_edges`, whose worklist names "merge-KEPT faces'
boundaries". The docs of `DeclaredCarrierUnsupported` admit "a caller
that re-describes edges after this call". That obligation is held by
the caller, not the door: S93's class, one level up from the operator.

## Witness to build

A declared-planar merge of two coplanar faces on distinct keys with real
`Intersection` edges, then `validate_geometric`. It is expected to
report `DescriptionNotAdjacent` on the absorbed face's boundary. The
`merge_faces` tests' `declared_planar_cube` shape is a starting point.

## Shape

The answer depends on the kef/kfmrh fork, which is with designers now.
Under one reading, the merge door re-describes its kept faces'
boundaries itself, moving that half of `describe_minted_edges`'
worklist into the door so it has one home. Under the other, the door
becomes `pub(crate)` until the representation stops storing key copies.
Either way, a public door must not return a strand at rest.

## Delivered

The witness reds on the merge base through both doors: a declared
merge, and an undeclared one on two keys of one surface source, each
leave `DescriptionNotAdjacent` on the absorbed wall's three edges. After
its tier-2 gate, the door now re-describes every boundary edge of each
kept face once, through the boolean's describer
(`boolean::describe_edges`), so none comes back with
`DescriptionNotAdjacent`; an edge it cannot describe refuses in the
door's own words (`KeptBoundaryUndescribed`, `KeptBoundaryUndecided`),
which the boolean maps back to its own refusal. The boolean's worklist
keeps the seam edges and the skipped groups' faces only. Rows:
`merge_faces::kept_rows`, and `boolean::ops::tests`'
`the_description_worklist_carries_no_kept_boundary`,
`a_boundary_edge_between_two_listed_faces_is_listed_once` and
`a_kept_boundary_refusal_comes_back_as_the_booleans_own`.
