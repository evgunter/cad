---
id: a-lone-unread-partner-leaves-its-vertex-edges-unclassed
kind: issue
title: A vertex in one pair whose partner reads nothing publishes no edge class, silently
status: closed
opened: 2026-10-12
closed: 2026-10-15
priority: P3
cost: E
refs: [pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided]
---

Filed by the sweep of `pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided`.

## What

`crates/topo/src/boolean/vtxfac.rs` `pair_classes`: a vertex in one
pair whose partner reads nothing (`sectors::wedge_classes`' `None`)
returns no rows (`unwrap_or_default`). The boolean builds on, and
`BooleanNaming::edge_classes` holds no class for the vertex's edges.
The only reader is editor-core's `names::emit_topo::senses_at`. It
refuses `NamingError::Emission` (`UNCLASSIFIED_SIDE`) only where it
reads a crossed edge at that vertex, and that error names a
missing-row bug rather than the unread partner. Elsewhere the absence
goes unreported. The vtxfac unit row "one partner that reads nothing"
pins the empty answer.

## The shape to give

Decide whether a lone unread partner should refuse at the boolean, as
two partners do (`VertexReadTwice`, pair and pair). The alternative is
for naming to treat a vertex with no rows as a typed refusal of its
own. Measure first which suite cells reach it.

## Closed

The alternative: the boolean keeps building, and the absence is a
typed fact naming refuses by. A refusal at the boolean would have
turned 586 sound results into refusals. Measured on the 3 ε rows, 602
boolean calls reach the lone arm: 407 bodies, 179 `Empty`, 16 typed
refusals, identical at every ε.

- `topo::BooleanNaming::unread_partners` lists each vertex
  `vtxfac::pair_classes` read no partner of (`topo::UnreadPartner`),
  with every null-edge copy, and why (`topo::UnreadNeighbourhood`:
  one face, a wedge with no bisector, an undecided corner). A debug
  assertion holds that no listed vertex has an `edge_classes` row.
- `names::emit_topo::senses_at` refuses
  `NamingError::PartnerUnread` (category 4, `UNRULED_FRAMING`) where a
  crossing's keys hold a listed vertex, whose edges would otherwise
  drop out of its senses unseen. `names::emit_union`'s
  `Flush::sense_against` refuses the same where a member edge has no
  classified side at a vertex its judgement listed;
  `UNCLASSIFIED_SIDE` stays the missing-row bug for every other vertex.
- Rows: `sweep`'s `full_turn_bore_mate::the_mate_records_its_unread_partners`
  (every span, both orders: the shaft's vertices at the rim's wedge,
  none holding a class); constructed rows on `senses_at` and
  `sense_against`, since no editor-core model found reaches a crossing
  at a listed vertex (stacked coaxial cylinders list none).
