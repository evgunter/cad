---
id: dihedral-arm-lever-says-curves-tightly-of-a-radius-below-zero
kind: issue
title: geom-brep: DIHEDRAL_ARM's lever names a short edge or a tight curve, and is already met by a stored radius below zero
status: closed
pr: 5349
opened: 2026-10-14
priority: P3
cost: E
closed: 2026-10-19
---

(Filed by the ENCL implementer of `param-span-lever-says-vanishingly-short-of-a-reversed-span`, from that unit's sweep of levers about length.)

## What

`DIHEDRAL_ARM` (`crates/geom-brep/src/dihedral.rs`, `DIHEDRAL_ARM`) ends in the lever "move the geometry so that edge is clearly longer and no face curves tightly there", with the note "an edge of no length or a cone's point leaves no angle to measure; a radius below zero is a defect". Its doc comment names the one way the wedge goes below zero: a stored radius below zero. `dihedral::tests::a_gates_rejection_ends_by_its_reading_on_either_arm` pins that a rejected Negative (`reject(Margin::of(-1e-3))`) takes the lever and the note at a build.

That is the shape `CertCheck::ParamSpan`'s lever had before `param-span-lever-says-vanishingly-short-of-a-reversed-span`: the lever names one failure mode, a short edge or a tight curve, and a refused reading can already meet it. A face whose stored radius is -5 m does not curve tightly, and lengthening the edge does not help: `dihedral::folded_lever_arm` is the pair's curvature arm (the stored radius) clamped by the extent, so a -5 m radius pins the arm at -5 m whatever the edge's length. The lever names a move that cannot reach the reading, while the note calls it a defect.

## Question

Should the lever name the goal so it is true of every reading the decision refuses, as ParamSpan's now does ("move the geometry so this edge runs forward over a clear length"), for example by asking for a clear positive radius? Or is a radius below zero, which no geometry the user moves can reach, a reading that wants no geometric lever? Any rewording must keep every render within the editor-core `refusal_concision*` 75-word budgets.

## Depends on `[ev]` PR 5134

PR 5134 (fork-log row 142) proposes making `DIHEDRAL_ARM` `StoredDefinite::Lever` and dropping "a radius below zero is a defect" from its note. Its reason is that a negative arm needs a datum a profile refuses, and that at-rest representability reports first. If Ev ratifies it, this row's premise goes stale: the negative reading is unreachable and its note no longer names it. The lever then needs only to be true of a zero arm (a cone's point, a short edge), which it already is. Settle this row after 5134.

## Resolution (PR 5349)

`DIHEDRAL_ARM` is `RefusedSide::Lever` and its note is "faces of no extent or a cone's point leave no angle to measure": a negative arm takes the lever and the note at every door, and the lever is true of the zero arm the note names. `editor-core`'s `refusal_concision_chains` now carries the negative `ArmCollapsed` row.
