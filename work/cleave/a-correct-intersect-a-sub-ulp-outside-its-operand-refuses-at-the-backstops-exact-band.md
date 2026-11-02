---
id: a-correct-intersect-a-sub-ulp-outside-its-operand-refuses-at-the-backstops-exact-band
kind: issue
title: A correct A ∩ B whose stored geometry sits a sub-ulp outside A refuses ResultVolumeImplausible at the backstop's exact band
status: closed
opened: 2026-10-20
priority: P2
cost: M
refs: [a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop, 5442]
pr: 5842
closed: 2026-10-27
---

Found by FLUX's spiric-budget unit (branch
`flux/spiric-budget-world-position`), which now reads each face's flux
about a vertex of its shell instead of the world origin.

## What

`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`,
`a_partner_tipped_off_the_face_builds_sound_or_refuses_typed_at_every_pose`,
at ε = 1e-12, in the pose "inside the lying pyramid, the pyramid lying
100ε off the top, turned". `x ∩ y` should be `x` itself. It refuses:

```
ResultVolumeImplausible { which: "vol(A ∩ B) ≤ vol(A)", got: "0.0002614672282429761", bound: "0.0002614672282429758" }
```

Measured inside `bound_holds` (`crates/topo/src/boolean/ops.rs`, arm 1),
with the exact interval re-derivation (both bodies are planar and take
the exact polygon route):

| body | interval volume (m³) |
|---|---|
| the stored result | [2.6146722824297510, 2.6146722824297534]e-4 |
| A | [2.6146722824297470, 2.6146722824297480]e-4 |

The interval margin is [−6.5e-19, −3.3e-19] over an area of 0.0837 m².
So the stored result really is about 3e-19 m³ larger than A: a boundary
displacement of about 4e-18 m. That is the vertex rounding of the
boolean's output, below one ulp of the coordinates.

## Why it refuses

Arm 1 refuses a violation certified at the `exact` band "whatever its
size" (`Posture::Bound`'s docs). So a correct result whose stored
geometry sits a sub-ulp outside its operand is refused. Nothing in the
result is wrong at the model's resolution; the claim refused is one
the stored floats cannot meet.

On main the same pose builds, but only because the origin-levered `f64`
sums happened to round the other way. The walk's reading did not say
violated, so arm 1's re-derivation never ran. With each face read about
its shell's vertex, the `f64` walk now reads 761 against 758, the same
order as the exact values, and arm 1 confirms it.

## Relation

This is the same shape as
`a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop`
(P0): the arm has no slack for the output's own rounding. Here the
excess is below a coordinate ulp, not a Zero-band gap.

## Pinned

The pose is pinned as this typed refusal in the row above, so the pin
goes red when this is fixed. Remove the pin then.

## Done when

The pose builds `x` (or the arm's slack is ruled), and the pin is
removed.

## Closed (2026-10-27)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::volume_relations`), so nothing refuses or passes on it. The pinned cell (ε 1e-12, `turned`, inside the lying pyramid 100ε off the top) now builds; `a_partner_tipped_off_the_face_builds_sound_or_refuses_typed_at_every_pose` takes it through `builds_or_refuses`, which holds tier 3, the material at its probes, the edge classes, the volume identities and the volume relations, and its pin is gone.
