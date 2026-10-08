---
id: a-chord-across-a-curved-rim-cut-in-pieces-refuses-shared-rim
kind: issue
title: A union whose post crosses a curved rim a notch cut into pieces refuses SharedRim(Several)
status: review
opened: 2026-10-07
priority: P2
cost: M
branch: emit/curved-rim-chord
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

## Fixed on `emit/curved-rim-chord`

The cause is the curved carrier. `emit_topo::rim_holding` picked the
piece a chord lies within by `chord_on_rim`, which tests the chord's
ends against the segment between a piece's ends. An arc chord lies on
no piece's segment, so neither piece held it and the pair refused
`Several`. The straight row passes only because its pieces are segments.

The chord reaches `rim_holding` because its split lineage runs through
the stretch the post killed, whose record died with it. That gap is
filed as
`a-split-piece-whose-lineage-runs-through-a-killed-piece-is-read-as-a-derived-chord`.

`rim_holding` now reads a curved chord against the curved pieces along
their common carrier (`Track::cover`), the reading `joined_cover`
already takes, and a straight one as before. Same rule as PR 3167, read
on the carrier. Rows: `emit_curved_rim_chord`, which has this document
in both member orders (red before the fix), the post across the second
piece, and a notched full circle.
