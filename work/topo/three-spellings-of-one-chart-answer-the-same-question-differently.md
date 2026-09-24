---
id: three-spellings-of-one-chart-answer-the-same-question-differently
kind: issue
title: three predicates in topo answer 'are these one chart' with different rungs and different bounds, and the pcurve doors use the weakest
status: open
opened: 2026-09-14
refs: [two-provenance-free-keys-holding-one-surface-read-as-two-charts, 2594]
priority: P2
cost: D
---


Filed by the fix pass of
`set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left`
(PR 2594), on R2's style finding (S5), as a CLASS row rather than an
instance: PR 2594 added the fourth caller of the first spelling below
without the other two being visible from it.

Three predicates in `crates/topo/src` answer "do these name one
chart / one described surface", with three different rung sets:

- `Body::same_chart` (`euler_ring.rs`) — one surface key, one
  `GeomSource`, or one shared `Arc` payload. `Decide` only: it TRUSTS
  the provenance channel and reads no scalar. Its callers are the three
  loop-re-parenting doors and, since PR 2594, the surface setter.
- `chart_region::same_chart<Decide + Bounds>` — key or source, and
  where the source is shared it VERIFIES the two surfaces' bits
  structurally; it is cross-body and carries the `Bounds` the first
  does not.
- `merge_faces::planes_declared_equal` — key or source, PLUS the
  face's `sense`, because a merge asks whether two faces are one
  REGION rather than which chart a row is stated in.

Two live consequences. The `Decide`-only spelling's known bound —
`two-provenance-free-keys-holding-one-surface-read-as-two-charts` —
says a structural compare "needs `geom_core::Bounds`", and a
`Bounds`-bearing chart compare is already in this crate; whether that
row closes by routing the pcurve doors to it (where their callers can
carry the bound) or by leaving them conservative is the question this
row wants answered once. And the `sense` rung's absence from the first
is deliberate and correct (a sense bit does not move a chart) but is
stated only in `same_chart`'s own docs, so a reader of the merge door
cannot see which of the three differences are decisions.

What would close it: one page that names the three, says which
question each answers and which bound it needs, and either unifies the
two that ask the same question or records why they cannot be one.
