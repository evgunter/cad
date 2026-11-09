---
id: point-equality-has-no-home
kind: issue
title: Point equality has no home: (a - b).norm() == 0.0 (28, inexact), componentwise ==, to_array() == and to_bits() spell it beside a Point{2,3} reader that does not exist
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6664
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

## Closed (`onecopy/point-equality`)

**The census, corrected.** The seven lines listed above as production
`norm() == 0.0` readers are all inside `#[cfg(test)]` modules:
- `splitting/rules.rs` 1356 and 1398;
- `ring_path.rs` 2302;
- `boolean/reduce.rs` 8171 and 8228;
- `census.rs` 13571;
- `stands.rs` 669.

All 32 `norm() == 0.0` sites were tests, so the inexactness below about
`1e-154` affected tests only. Seven more `v.norm() == 0.0` sites read a
single vector zero, a different question, and stay. The production
equalities were four:
- `step-import` `assemble::coincide` and `editor-core`
  `GeometryWitness::eq`, componentwise `==`;
- `editor-core` `mate::coset`'s `point_eq`/`vec_eq`, `to_array() ==`;
- `mesh` `planar::same_position`, bitwise.

**Two questions, two readers** (ruled A). On `Point2`, `Point3`,
`Vec2` and `Vec3<f64>` in `geom-core` linalg:
- **`equals_exactly`** is the value question: `==` per coordinate, `+0.0`
  one with `-0.0`, a NaN equal to nothing. It serves a degenerate chord
  and a record's structural equality.
- **`same_bits`** is the representation question: a dedupe key, a
  reproducibility check.

Each reader's doc says when to use the other, and why neither is
`bit_identity`'s generic scalar channel. Each binds its type's fields by
pattern, so a new field is a compile error rather than a coordinate the
equality skips. Every production and test spelling is routed, and the
local helpers are gone:
- `coincide`, `same`, `point_eq`, `vec_eq` and `same_position`;
- `bits_eq` ×2 and the export census's `same_bits`.

`scripts/gates/point-equality-reads.sh` refuses the four hand spellings
in every crate source, tests included. It has a planted fixture per
spelling and controls for the readers, a single vector read zero, and a
point against two scalars.
