---
id: sorted-clouds-are-zipped-under-a-tolerance-across-a-sort-tie
kind: issue
title: two suites sort two point clouds independently and zip them under a tolerance, which pairs the wrong points when a pair straddles a sort tie
status: open
opened: 2026-10-01
priority: P3
---

Found by `linalg/doors` (PR 3710) while folding in the ruling that a
point set has no canonical order in this kernel
(`crates/geom-core/src/linalg.rs`, "Deliberate omissions"): two clouds
are compared by matching under a tolerance (0 for exact), never by
sorting and zipping.

## Where

- `crates/sweep/tests/sf2a_r1_head.rs`, its first half:
  `sorted_points` (around :61) sorts each body's vertex points
  lexicographically, and `h1_valence_four_concurring_corners_must_solve`
  zips `before` against `after` (around :120 and :140) and compares
  each pair under a tolerance after the λ-scale map.
  `h2_…` (around :183) compares two sorted lists with `assert_eq!`,
  which is exact and so is not this defect.
- `crates/step-import/tests/inst_review_probes.rs`, `assert_cloud_eq`
  (around :121): zips `got` against `want` per axis under `1e-9`;
  `mapped` (around :133) sorts the mapped cloud on its own and the
  imported cloud is sorted on its own.
- `crates/sweep/tests/verbs_chamfer.rs` (around :189): the fillet's
  feet are `sorted_points` against a hand-sorted `want`, compared with
  `assert_eq!`. That is exact, so no tie can mis-pair it today, but it
  is the sort-and-zip shape the rule retires, and the chamfer half
  of the same row already matches by proximity. It belongs in the
  same repair.

## Why it is wrong

Each list is sorted by its own coordinates, then the i-th of one is
paired with the i-th of the other. A map that moves two points whose
leading coordinates are within the tolerance of each other can swap
their sort order on one side only, and the zip then compares each with
the other's image: a correct cloud reads as wrong, or two wrong points
pass as each other's match. The rows are correct only while no pair
straddles a sort tie through the map. Today's fixtures do not, so this
is latent.

## The repair

Match nearest under a tolerance: for each wanted point, the nearest
got point by `(g - w).norm_inf()` (`Vec3::norm_inf`), with the count
checked and, where a bijection is the claim, each got point used once.
`demos/tour/src/diechamfer.rs` `feet_agreement` is the shape.
