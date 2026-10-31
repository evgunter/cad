---
id: a-witness-that-stops-at-check-mesh-cannot-see-a-wrong-region
kind: issue
title: check_mesh is combinatorial only, so a witness that stops at it passes a consistently wound mesh of the wrong region
status: open
opened: 2026-10-28
priority: P3
cost: M
---


## What

`mesh::validate::check_mesh` checks combinatorics: closed, 2-manifold and consistently oriented. A mesh of the wrong region of a surface passes it, as long as that mesh is closed and consistently wound. Examples are a trimmed face's complement, or a face meshed inside-out together with its neighbour. The mesh's volume then disagrees with the body's.

Most witness rows across the tree that "mesh and pass `check_mesh`" stop there. Such a row cannot go red when the mesh covers the wrong place (reviewer-style Q3).

## Witness

The PR 5920 review, at frozen head `832e7cd` (`tess/sphere-tilted-circle-lane`, the sphere arm), found 47 of 566 meshed witness rows more than 5% off the closed-form volume. Every one was `Ok` and passed `check_mesh`.
- `tilted_sphere_pair` union: `signed_volume` −0.497, against `mass_properties` 7.8686.
- snowman `ball ∪ bar`: 0.372 against 4.560.

PR 5920's fix pass adds a volume check to its own witnesses. This row is for the rest of the tree.

## Owed

- **A shared test helper** that holds a mesh to the body. Mesh `signed_volume` must match `topo::mass_properties` within the mesh's chordal deficit, and each face must wind outward.
- **A sweep** of the witness helpers that stop at `check_mesh` (`rg 'check_mesh' crates/*/tests demos`), recording each one's disposition. Possibly `check_mesh` should take the body and do the volume check itself.

Filed by the TESS orchestrator at the adjudication of PR 5920's dual review.
