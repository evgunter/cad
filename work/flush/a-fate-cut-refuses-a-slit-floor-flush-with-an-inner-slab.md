---
id: a-fate-cut-refuses-a-slit-floor-flush-with-an-inner-slab
kind: issue
title: Since PR 5368 the fate cut refuses ClassificationInvariant on a slit or ring floor flush with an aligned slab inside it, in ∪ and ∩: main built both
status: open
opened: 2026-10-20
priority: P1
cost: M
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
