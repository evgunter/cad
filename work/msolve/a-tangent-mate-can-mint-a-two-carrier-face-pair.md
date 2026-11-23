---
id: a-tangent-mate-can-mint-a-two-carrier-face-pair
kind: issue
title: A Tangent mate refuses NoAtRestRecord though the at-rest census now confirms a face-face record on two carriers along its tangent line
status: open
opened: 2026-10-10
---


Found by CONTACTHOLD building `at-rest-records-are-unordered-cell-pairs`
(PR 2 of its stack).

`mate::class_admission` (`crates/editor-core/src/mate.rs`) answers
`ContactClass::Tangent` with `ClassAdmission::NoAtRestRecord`: "a tangency
between placed faces is a face-face record on two carriers, and the census
at rest decides nothing along their locus for the record to cite". The
record now exists and the census confirms it: `census::confirm_face_pair`
reads a face–face record on two carriers through
`boolean::tangent_pair_at_rest` (the closed-form locus of
`geom_brep::tangent_locus`, plane × cylinder and parallel cylinders) and,
where the line runs through both faces, `boolean::tangent_table_at_rest`
(the C4 `Tangent` table along it). What the mint door lacks is the
decision it cites: `AtRestRows::decide` (`crates/editor-core/src/assembly.rs`)
cites `topo::census_rest_decision`, which reads one carrier only. A
`census_tangent_decision` beside it, recording the gap margin
`tangent_locus` decided, would let a Tangent mate mint like a Rest one.
