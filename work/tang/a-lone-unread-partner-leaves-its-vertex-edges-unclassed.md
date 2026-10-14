---
id: a-lone-unread-partner-leaves-its-vertex-edges-unclassed
kind: issue
title: A vertex in one pair whose partner reads nothing publishes no edge class, silently
status: open
opened: 2026-10-12
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
