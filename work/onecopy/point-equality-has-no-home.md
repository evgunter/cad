---
id: point-equality-has-no-home
kind: issue
title: Point equality has no home: (a - b).norm() == 0.0 (28, inexact), componentwise ==, to_array() == and to_bits() spell it beside a Point{2,3} reader that does not exist
status: open
opened: 2026-11-09
priority: P3
cost: E
---


From PR 6647's review (NOTE-3), with the sites counted at main
`97c90dd6` by `onecopy/chord-length-notes`.

## What

`Point3` (and `Point2`) carry no `PartialEq`, deliberately, and nothing
else answers "are these two points the same point?". Each site spells it:

- **`(a - b).norm() == 0.0`, 28 sites** (`grep -rnE "\)\.norm\(\) == 0\.0"`).
  **Not exact**: the squared norm underflows, so points about `1e-170`
  apart read as equal. Production sites among them:
  `topo` `splitting/rules.rs` (1356, 1398), `ring_path.rs` (2302),
  `boolean/reduce.rs` (8171, 8228), `census.rs` (13571), `stands.rs`
  (669); the rest are rows and tests (`face_normal.rs` 841/850,
  `census.rs` 20928, …).
- **Componentwise `a.x == b.x && a.y == b.y && …`, about 9 sites**,
  among them production `step-import` `assemble.rs` `coincide` (119)
  and `editor-core` `clearance.rs` `GeometryWitness::eq` (548); the
  rest in `topo/tests`.
- **`to_array() ==`, 4 to 6 sites**: `topo` `merge_faces.rs` (3523),
  `editor-core` `mate/coset.rs` (115, 119), `editor-core`
  `test_support.rs` (658, 660), one integration test.
- **Bitwise `x.to_bits() == …`**, a fourth spelling (it tells `-0.0` from
  `0.0` and matches NaNs): `topo` `sector_shape.rs` (326), `mesh`
  `planar.rs` (302, 1706), `face_normal.rs` (797), several tests.

## Owed

A `Point{2,3}` exact-equality reader in `geom-core` (decide whether
`-0.0 == 0.0` and NaN are equal, i.e. `==` or bits, and name it so), and
route the production sites to it. The `norm() == 0.0` sites are the
priority: they are not the question they ask.
