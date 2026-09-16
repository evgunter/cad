---
id: componentwise-equality-of-the-linear-types-is-hand-listed
kind: issue
title: componentwise equality of Vec3/Point3 is hand-listed at eleven sites in seven crates and a fourth component would be silently outside every one
status: open
opened: 2026-09-16
---



Found by CENSUS-HAND-LISTED-SIBLINGS (2026-09-16) while repairing
`PartialEq for Coset`. The same class as
`hand-listed-partialeq-siblings-outside-the-census-debug-fence`, one
level further down: a comparison that spells `a.x == b.x && a.y == b.y
&& a.z == b.z` reads three NAMED fields of a declaration it has no tie
to, so a fourth component on `Vec3` or `Point3` lands outside it with
no error anywhere.

## It is executed, not reasoned

A fourth field `probe_zzz: T` added to `geom_core::Vec3` (with
`Vec3::new` patched to fill it) compiles `editor-core` **clean** —
`cargo check -p editor-core`, no error, no warning. The two comparisons
in `mate/coset.rs` and the one in `clearance.rs` each kept answering
over three of four components. The same probe against the repaired
`SketchPlane::bit_eq`, which now binds `Vec3 { x, y, z }`, is an E0027
at `crates/profile/src/lib.rs`'s `bits` closure — so the difference is
the pattern and nothing else.

## The hit list

Swept with `\.x == .*\.x && .*\.y == .*\.y` over every `.rs` file in
the tree; eleven hits, none of them tied to a declaration.

| site | what it is |
| --- | --- |
| `crates/editor-core/src/mate/coset.rs` (`vec_eq`, `point_eq`) | the two shared helpers `PartialEq for Coset` and `PartialEq for Subgroup` both route through — msolve's |
| `crates/editor-core/src/clearance.rs` (`same`, in `PartialEq for GeometryWitness`) | shell's, inside the impl their own row already covers |
| `crates/mesh/src/planar.rs` (two sites) | mesh's |
| `crates/step-import/src/assemble.rs` | exch's |
| `crates/sweep/tests/m7_skin_integral.rs`, `crates/sweep/tests/verbs_offc_consumer.rs`, `crates/geom-brep/tests/pcurve_p1b_r2_probes.rs` | assertions in test suites — tcost's and tint's territory |
| `demos/tour/src/skinned.rs` | a demo assertion; no open program claims the path |

**What the pattern cannot match**: a comparison written over two
components only (`Vec2`, `Point2`), one that compares through a method
(`a.to_array() == b.to_array()`), one spelled with `!=` and `||`, and
one whose operands are indexed rather than named. None was looked for.

## The disposition this row owes

Not "destructure eleven sites". `Vec3` gaining a component is not a
live prospect, and the honest question is whether the shared helpers
(`vec_eq`, `point_eq`, and the `same` closure that is a third copy of
them) should be one door on the linear types themselves — which is this
program's charter question, not a sweep — with the test and demo
assertions following whatever that door decides. The three kernel
copies are the row; the six assertion sites ride it.
