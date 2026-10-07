---
id: a-chord-across-a-curved-rim-cut-in-pieces-refuses-shared-rim
kind: issue
title: A union whose post crosses a curved rim a notch cut into pieces refuses SharedRim(Several)
status: open
opened: 2026-10-07
priority: P2
cost: M
---



## What

Found while building a test row for PR 4228 (a union reading a sense along a
curved member line). The document:
- a 270° arc prism: an arc of radius 1 from 0° to 270° about the origin,
  closed through the centre, extruded 1;
- minus a notch x ∈ [−1.5, −0.6], y ∈ [−0.38, −0.30], z ∈ [0.8, 1.8]. This
  leaves the top rim arc two pieces, so the top cap and the arc wall share two
  edges;
- united with a post x ∈ [0.6, 0.8], y ∈ [0.4, 1.2], z ∈ [0.5, 1.5] standing
  across the first piece.

The union refuses `NamingError::SharedRim { found: Several }` at the union
node, from `emit_topo::name_boolean_edges`'s chord derivation
(`Rim::NotOne(RimShare::Several)`, `emit_topo.rs` near line 1212). The same
union over a straight edge in two pieces evaluates
(`resolve_cited_line::a_union_reads_a_cited_member_line_by_its_rows`).

`origin/main` at 6d5bfb778e refuses it identically, so the naming of
crossings and pieces by lines did not cause it. It is the missing rule
`shared-rim-several-is-a-missing-rule-legal-declared-unions-reach` owned,
reached by an undeclared union over a curved rim. That row closed on PR 3167.

## Next

Read the chord's rim as the piece of the shared rim the chord's ends lie on,
as PR 3167 did for a fragmented merged face, or say why a curved rim differs.
