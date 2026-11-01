---
id: a-fate-cut-refuses-a-slit-floor-flush-with-an-inner-slab
kind: issue
title: Since PR 5368 the fate cut refuses ClassificationInvariant on a slit or ring floor flush with an aligned slab inside it, in ∪ and ∩: main built both
status: closed
opened: 2026-10-20
priority: P1
cost: M
closed: 2026-10-31
---


Found by CLEAVE (`cleave/boolean-merges-a-glued-slit`) on merging main.

## What

The operand is PR 5380's slit form: the slab `[0,4]²×[0,1]` less the
trench of `crates/topo/tests/a_keel_on_the_floor_subtracts_as_a_slit.rs`.
Its floor carries the keel as a zero-area ring (closed, `x ∈ [1.5, 2.5]`)
or as a slit of its outline (open at one end, `[1.5, 4.5]`). The same
form comes from the split. The other operand is the slab
`[0,4]²×[0, 0.5]`, whose bottom face lies flush on the floor with the
same sense.

| op | `763ca322a0` | `origin/main` after PR 5368 (`985a3d37e0`) |
|---|---|---|
| `[0,4]²×[0,½] ∪ slit`, closed and one end | builds, v16 e24 f11 / f10, volume `16 − 0.225 L`, tiers 1–3 and 3′ | `ClassificationInvariant` "a fate cut's end lies in no corner of the dropped face" |
| `slit ∩ [0,4]²×[0,½]`, closed and one end | builds, v14 e22 f10 / v15 e23 f10, `8 − 0.075 L` | `ClassificationInvariant` "a fate cut leaves a slit in the dropped face" |

Five poses, both slit operands (the Boolean's and the split's), and the
same at ε = default, 1e-6 and 1e-12: 40 cells per ε. Wall to wall
still builds. Both refusals come from the coincident fate cut
(`b8043fe8d1`, "cut a dropped coincident face where its fate
changes"). Its fate cut ends at the slit's tip, whose corner in the
dropped face is the full turn the slit leaves (`sectors::SlitTips`).
That is a guess at the cause, not a reading of it.

## Owed

Build these cells again, or refuse them typed as a construction gap if
the cut cannot read a corner at a slit's tip. `ClassificationInvariant`
claims a kernel defect. The row "the slab inside ∪ slit" is left out of CLEAVE's pin
`a_face_glued_to_the_slit_floor_merges_its_two_edges` until this
lands; put it back there.

## More witnesses (CLEAVE, `cleave/glued-slit-intersection-regression`)

The plate less a pyramid lying under its top (`below(lying(0))`) holds
a zero-area ring in its top. The box `[1,2]×[0.5,1.5]×[lo,1]` lies
flush under that top, `lo ∈ {0, 0.3, 0.6}`, at rest and turned. Both
refusals predate PR 5505, at `df1ed8d476`:
- `y ∩ x` (the plate first) refuses "a fate cut leaves a slit in the
  dropped face";
- `x ∪ y` refuses "a fate cut's end lies in no corner of the dropped
  face".

## Evidence (CLEAVE, `cleave/fate-cut-ends-at-a-slit-tip`)

The cause, read: `fate::places` read the corner at a slit's or ring's
tip, between the slit's two distinct line edges, as a cusp holding no
direction ("…end lies in no corner…"), and `fate::border_fates` kept
both of a slit's edges as fate edges where the kept face lies on both
sides of the line ("…leaves a slit…"). The branch reads that corner as
the full turn (a sliver's excepted) and drops such a pair; the
findings are on `work/cleave/a-boolean-cuts-a-flush-face-kept-once-along-a-slits-line.md`.

Every cell below builds on the branch, at ε default, 1e-6 and 1e-12,
both slit operands (the Boolean's and the split's), five poses:

| op | branch |
|---|---|
| `[0,4]²×[0,½] ∪ slit` | v16 e24 f11 closed / f10 one end and wall to wall, `16 − 0.225 L`, tiers 1–3 and 3′, no records, the keel filled; back in `a_face_glued_to_the_slit_floor_merges_its_two_edges` |
| `slit ∩ [0,4]²×[0,½]` and `[0,4]²×[0,½] ∩ slit` | v14 e22 f10 closed, v15 e23 f10 one end, v16 e24 f12 wall to wall (two solids), `8 − 0.075 L`, tiers 1–3 and 3′, the keel two edges each between the floor and a wall; one end carries one vertex–vertex contact record where the keel meets the floor's boundary |

On main the other order, `[0,4]²×[0,½] ∩ slit`, refuses too:
`SlitFaceGlued` closed (PR 5538's guard), `ClassificationInvariant`
one end. Pinned by
`a_keel_on_the_floor_subtracts_as_a_slit::a_slab_flush_with_the_slit_floor_intersects_it_keeping_the_keel`.

The plate with the ring against the flush box `[1,2]×[0.5,1.5]×[lo,1]`,
`lo ∈ {0, 0.3, 0.6}`, at rest and turned: `y ∩ x` and `x ∪ y` build
(and `x ∩ y`, which refused `SlitFaceGlued`), tier 3 and 3′, at their
closed-form volumes; pinned by
`a_keel_on_the_floor_subtracts_as_a_slit::a_box_flush_under_a_ringed_top_builds_in_every_op`.
The row's status is FLUSH's to change.

**Since PR 5545's fix pass.** The cells above still build, and the
dropped floor's substituted rows match main row for row on every cell
main builds (the review's probe, 2043 cells per ε). The partners whose
outline crosses or meets the slit also build in both orders of ∩:
`[0,2] × [0,4] × [0,½]` against the closed and the one-end slit, and
`[1,5] × [1,3] × [0,½]` against the one-end slit
(`a_flush_partner_crossing_the_slit_intersects_it_in_either_order`).

## Resolution

Closed by CLEAVE's PR 5545 (merged), on the FLUSH orchestrator's read,
2026-10-31. Every cell this row owed builds again at ε default, 1e-6
and 1e-12 in both slit operands and five poses, and the row "the slab
inside ∪ slit" is back in CLEAVE's pin. The pins on main are in
`crates/topo/tests/a_keel_on_the_floor_subtracts_as_a_slit.rs`:
- `a_face_glued_to_the_slit_floor_merges_its_two_edges`;
- `a_slab_flush_with_the_slit_floor_intersects_it_keeping_the_keel`;
- `a_box_flush_under_a_ringed_top_builds_in_every_op`;
- `a_flush_partner_crossing_the_slit_intersects_it_in_either_order`.

The cause, a slit's or ring's tip corner read as a cusp and both slit
edges kept as fate edges, is on CLEAVE's
`work/cleave/a-boolean-cuts-a-flush-face-kept-once-along-a-slits-line.md`.
