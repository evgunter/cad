---
id: a-material-crescent-tangent-line-builds-two-coincident-cusp-edges
kind: issue
title: A material-crescent pinch along a tangent line builds two coincident cusp edges, and the carry re-pairs a line whose crescent side flips
status: open
opened: 2026-10-20
priority: P2
cost: H
refs: [a-bore-tangent-to-a-plates-face-mints-a-face-face-record-the-census-contradicts, revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet, census-passes-a-face-inside-an-edges-material-wedge]
---


Ev's decision on the pinch fork (`[ev]` PR 5395, 2026-10-20): where
four sheets continue through a tangent line and the crescents beside
it are material, the line carries two coincident cusp edges, wedge 0,
one per cusp, and no face–face record (`crates/topo/README.md`, D1
tier 3 and C7). The void crescent and every point touch stay records.

## What

Nothing builds this form today. Plate ∖ a rod tangent to its face from
inside mints a face–face row at the glue decision
(`verify_declared_contacts`, `crates/topo/src/boolean/mod.rs:6265`,
into `ReductionContacts::ff`, `mod.rs:809`) and leaves the plate face
and the bore wall whole through the line
(`a-bore-tangent-to-a-plates-face-mints-a-face-face-record-the-census-contradicts`).

## Owed

1. **The op.** Where the glue decides a tangency through both faces'
   interiors and the crescent beside it reads material, ∖ (and any op
   whose result keeps the material on both sides) cuts the line: the
   bore wall split along its ruling, the plane face carrying the two
   cusp edges as a zero-area ring where the line ends inside it, or a
   slit of its loop where the line reaches its boundary. No face–face
   row. The two edges each classify at tier 3 as wedge 0 on their jet.
2. **The carry.** A carried face–face record whose crescent reads
   material on the result (the mould, `box ∖ bridged rod`, measured in
   round 3: v = 107.466, the record carried, 3′ refusing on the
   crescent) is cut into the two cusp edges at its locus; a pair of
   cusp edges whose crescent turns void (the reverse mould) is sewn
   back into faces running through, plus a record. This is the
   complement re-pairing D1 tier 3 owes at any line four sheets meet
   on; TOPO's `revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet`
   is the same re-pairing at `revert`.
3. **The through-bore slot.** A through-bore slotted underneath, far
   from the line, leaves two solids joined only along it; with the
   edges already present, the sort only re-sorts. Pin it.

The interim refusal is
`a-material-crescent-face-face-record-refuses-typed-at-the-op-and-the-carry`;
the side check over the two coincident edges is RESTREAD's
`census-passes-a-face-inside-an-edges-material-wedge`.

## 2026-10-21: tier 3′ refuses two coincident cusp edges undecided (RESTREAD, `restread/coincident-wedges`)

Tier 3′ now reads the wedges of two line edges overlapping on one line
(`census.rs` `ee_overlap_sides`, through `census::line_wedges`). A
cusp edge's two sheets meet at a zero angle, which `line_wedges` reads
as `WedgeRefusal::Unread(_, TouchUnread::Degenerate)` and the census refuses as
`CoincidentWedgesOverlapUndecided { why: TouchUnread::Degenerate }`.
So the body this row builds refuses at tier 3′ until the reading takes
a cusp's side at second order (sectors 0 and 2π, the restread row's
"Evidence: the tangent pinch"). No shipped body carries two coincident
cusp edges today (topo and sweep, slow set included, at the default ε
and 1e-12). The builder owes that reading in `ee_overlap_sides`, or a
second-order arm in `line_wedges`.
