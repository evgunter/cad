---
id: componentwise-equality-of-the-linear-types-is-hand-listed
kind: issue
title: componentwise equality of Vec3/Point3 is hand-listed at eleven sites in seven crates and a fourth component would be silently outside every one
status: open
opened: 2026-09-16
priority: P3
cost: E
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

**And one more the list missed, which its own sibling repair is
about**: a `Mat3` COLUMN hand-list — `linear.c0`/`c1`/`c2` read by
name. That is exactly what the `Mat3 { c0, c1, c2 }` patterns in
CENSUS-HAND-LISTED-SIBLINGS repair at two sites, and `rg 'linear\.c0'`
finds roughly twelve more (`topo/src/separation.rs`,
`geom-core/src/linalg/{frame.rs,affine.rs}`,
`pncad-py/src/py/doc.rs` twice, and several test suites). This row
carries that arm; a lane taking it may split the arm out, but may not
close the row without dispositioning it. Found by the style review of
the unit that filed this row — a blind-spot list short in the
direction of its own author's other change, which is standing finding
2's shape.

## The disposition this row owes

Not "destructure eleven sites". `Vec3` gaining a component is not a
live prospect, and the honest question is whether the shared helpers
(`vec_eq`, `point_eq`, and the `same` closure that is a third copy of
them) should be one door on the linear types themselves — which is this
program's charter question, not a sweep — with the test and demo
assertions following whatever that door decides. The three kernel
copies are the row; the rest ride it.

**The split, re-measured** (the first writing of this sentence said
"six assertion sites" and sorted `mesh`'s two and `step-import`'s one
onto the wrong side): of the eleven, **six are library sites** —
`editor-core/src/mate/coset.rs:91` and `:95`,
`editor-core/src/clearance.rs:548`, `mesh/src/planar.rs:1099` and
`:1546`, `step-import/src/assemble.rs:115` — and **five are assertions**
— `geom-brep/tests/pcurve_p1b_r2_probes.rs:215` and `:219`,
`sweep/tests/verbs_offc_consumer.rs:262`,
`sweep/tests/m7_skin_integral.rs:162`,
`demos/tour/src/skinned.rs:1186`. The count of eleven was right; the
sorting was not.
