---
id: a-merged-sphere-faces-outline-is-chosen-by-elimination-so-the-answer-depends-on-operand-order
kind: issue
title: The output merge chooses a glued sphere face's outline by elimination, so a ∪ b builds where b ∪ a refuses
status: open
opened: 2026-11-23
priority: P1
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote, a-section-loop-inside-a-one-band-curved-face-refuses-the-interior-loop-guard]
cost: M
---


Found by the LATHE interior-loop lane (2026-11-23, PR 7440). Both
re-reviews of that PR raised it, and one instrumented its cause.

## What

When the output merge glues a sphere face's pieces back into one face,
its role pass (`merge_faces::curved_outline_ring`) has to choose which of
the merged face's loops is the outline. It asks each loop in the outline's
role through the face's lift (`Body::face_lift`):

- it takes the one slot whose lift places every ring;
- failing that, it strikes out every slot whose lift places a ring
  decidedly nowhere or decidedly twice, and takes **the one slot left
  standing, whether or not its own reading settles**.

Where neither lift places the ring, two slots are left standing,
`[None, Some(0)]`. Which of them survives the elimination then depends on
the arena order the merge met the pieces in, and that is the operand
order. In one order the run merges. In the other it refuses
`PeriodClosure`, the cut-in's meridian survives, and the op refuses at
`boolean::ops::surviving_cut`. DESIGN calls an operand order that refuses
where the other builds a kernel defect.

So the defect is the choice by elimination. The short edge beside the
pole in the first witness below is a witness of it, not its cause.

## Witnesses (PR 7440's branch)

- `sweep/tests/spheres_crossing_off_every_edge.rs`,
  `a_trimmed_face_cut_beside_its_pole_builds`: `b ∪ a` refuses at every
  `δ` (`10⁻³`, `10⁻⁵`, `10⁻⁷`; one reviewer also saw it at `3·10⁻³`) with
  `FallbackExtentUnsupported { operand: B, what: "the meridian the sphere
  face was cut in along survives the output merge, …" }`, while `a ∪ b`
  builds. The row pins that cell as the refusal (`cut_survives`).
  - Instrumented at `δ 10⁻³`: the merge skipped the cut ball's two pieces
    (`5v1`, `6v1`) with `PeriodClosure { edge: 4v3 }`. The pieces share
    four edges, the cut meridian's pieces. `4v3` is about `10⁻³` long,
    from `(0, 0.9999995, 0.001)` to the pole `(0, 1, 0)`, where the small
    ball's face meets the cut.
  - The reviewer's instrumentation: `curved_outline_ring` left `[None,
    Some(0)]` standing, with neither lift placing the ring.
- `sweep/tests/snowman.rs`:
  - `a_slab_cutting_a_cap_off_a_pole_strut_carve_builds` (∪): the face
    merges, but its ring reads `Undecided`, and the mesh refuses
    `DomainUnread { cause: Undecided { ring, cause: None } }`.
  - `a_cut_ends_at_the_nearest_of_several_boundary_hits` (∪): the same
    `DomainUnread … Undecided` ring.

  Both are the same undecided ring the elimination accepted (TESSLANE's
  ring-reader row,
  `work/lathe/an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader.md`,
  carries the reader's side).

## Against main

All three are green on `origin/main` (`43295779ee`, two π-bands), but in
each case main's answer is a sound body that is not maximal. Measured with
throwaway probes (unions, both orders):

| row | faces | edges dividing two faces of one record | merge skipped |
|---|---|---|---|
| trimmed `δ 10⁻³` | 6 | 7 | two groups, `PeriodClosure` |
| pole strut | 10 | 2 | one pair, `PeriodClosure` |
| nearest boundary hits (lat ±60) | 10 | 5 | one group of three faces, `PeriodClosure` |

So the stack trades a non-maximal body for a refusal, or for an
unmeshable face, and in one operand order only.

## Fix direction

Make the outline choice independent of operand order: decide it, or
refuse loudly, rather than pick by elimination. Then both orders give one
answer. Where a slot stands only because its reading did not settle, the
choice is not decided. Once it is, `b ∪ a`'s pin flips back to its mesh
wants (`Draws`, `Folds`, `Budget` by `δ`), and the two snowman rows close
if this is their cause.

## A witness no operand order rescues (2026-11-23, LATHE one-band lane)

A sphere ZONE's pair meets the same elimination with nothing placed in
either order, so the symmetry rules the operand order out of it. A unit
sphere's zone `|y| < 0.5`, swept as one band and halved along the
meridian opposite its seam, glues back by the bridge (one `kef`, then
`kemr` on the meridian left twice in one loop, its ends on the rims
rather than free). The role pass then asks which of the two rims is the
outline, and on a zone neither lift places the other: `placed` is empty
and `[None, Some(0)]` stand, so the run is left unmerged with a
`PeriodClosure` skip naming the pair — whichever order the pieces arrive
in. Pinned as it stands by
`sweep/tests/curved_mergedoor.rs::a_sphere_zone_halved_by_two_meridians_records_its_undecided_outline`,
which flips to the merge when the choice is decided.

A pair whose union is the WHOLE sphere is a different refusal, not this
one: the last meridian the absorption leaves is then a strut with both
ends free (the two poles), and pruning a strut off a curved face is
refused before any role pass
(`curved_mergedoor::sphere_torus_and_cone_pairs_record_their_period_closure`,
the ball arm).
