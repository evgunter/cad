---
id: the-ruled-section-tables-carry-their-rulings-as-any-curve
kind: issue
title: The plane×cylinder and plane×cone tables carry their rulings as Curve3, so every reader of a two-ruling section re-checks that it is a line
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## What

`geom_brep::PlaneCylinderSection::ParallelLines { l1, l2 }`,
`PlaneConeSection::ApexLinePair { l1, l2 }`, `EqualCylinderSection::ParallelLines` and
`RuledSection::ParallelLines` (`geom-brep/src/intersect.rs`) type their rulings as `Curve3<T>`.
Every constructor builds a `Curve3::Line` there. The type does not say so, so a reader that needs
the line's origin and direction re-checks the kind at run time, under an arm that nothing can
reach (D9 row 0).

The split's `ruling_pairs` (`topo/src/splitting/join.rs`) is one such reader. It pairs a curved
face's crossings along each ruling of `chord_join::SectionCase::Straight`, and refuses a non-line
as `SplitJoinError::SectionInvariant` ("a ruling section carried a non-line"). Carrying the
rulings line-typed inside `topo` alone would only move that check into `chord_join::section_case`.

## Owed

Type the rulings as lines where the tables build them, for example as a `(Point3, Vec3)` or a
line struct. Readers then destructure instead of matching. The readers are `chord_join::section_case`,
`boolean/join.rs`'s frame arms (they ignore the payload), `geom-brep`'s `locus.rs`, and the
`intersect_table.rs` / `germ_pose_gate.rs` tests. Once the tables are typed, `SectionCase::Straight`
can carry the line type and `ruling_pairs` loses its invariant arm.

## Found by

The review of PR 4181 (`cleave/frustum-apex`), finding S1.
