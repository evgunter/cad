---
id: dihedral-arm-lever-says-curves-tightly-of-a-radius-below-zero
kind: issue
title: geom-brep: DIHEDRAL_ARM's lever names a short edge or a tight curve, and is already met by a stored radius below zero
status: open
opened: 2026-10-14
priority: P3
cost: E
---

(Filed by the ENCL implementer of `param-span-lever-says-vanishingly-short-of-a-reversed-span`, from that unit's sweep of levers about length.)

## What

`DIHEDRAL_ARM` (`crates/geom-brep/src/dihedral.rs`, `DIHEDRAL_ARM`) ends in the lever "move the geometry so that edge is clearly longer and no face curves tightly there", with the note "an edge of no length or a cone's point leaves no angle to measure; a radius below zero is a defect". Its doc comment names the one way the wedge goes below zero: a stored radius below zero. `dihedral::tests::a_gates_rejection_ends_by_its_reading_on_either_arm` pins that a rejected Negative (`reject(Margin::of(-1e-3))`) takes the lever and the note at a build.

That is the shape `CertCheck::ParamSpan`'s lever had before `param-span-lever-says-vanishingly-short-of-a-reversed-span`: the lever names one failure mode, a short edge or a tight curve, and a refused reading can already meet it. A face whose stored radius is -5 m does not curve tightly, and a longer edge makes its wedge more negative, so the lever points the wrong way while the note calls it a defect.

## Question

Should the lever name the goal so it is true of every reading the decision refuses, as ParamSpan's now does ("move the geometry so this edge runs forward over a clear length"), for example by asking for a clear positive radius? Or is a radius below zero, which no geometry the user moves can reach, a reading that wants no geometric lever? Any rewording must keep every render within the editor-core `refusal_concision*` 75-word budgets.
