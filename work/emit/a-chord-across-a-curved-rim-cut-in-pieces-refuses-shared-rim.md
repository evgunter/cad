---
id: a-chord-across-a-curved-rim-cut-in-pieces-refuses-shared-rim
kind: issue
title: A union whose post crosses a curved rim a notch cut into pieces refuses SharedRim(Several)
status: closed
opened: 2026-10-07
closed: 2026-10-12
priority: P2
cost: M
branch: emit/curved-rim-chord
pr: 4794
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

The cause is a broken lineage. The arc piece from 53° to 197° is a
split piece of the rim `8v1`. Its lineage runs through the stretch the
post killed, `11v5`, and the kill drops that edge's birth record. So
the chase stopped short of the operand table, and the piece went to
the derived-chord path. That path's `rim_holding` then tested the arc
against each piece's straight segment, found no piece holding it, and
refused `Several`.

Both are fixed:
- **The chase.** It now reads the kills' dropped hops from the discard
  rows, so the piece resolves to `8v1` in the table. The details are in
  `a-split-piece-whose-lineage-runs-through-a-killed-piece-is-read-as-a-derived-chord`,
  closed with this row.
- **The derivation.** It reads "within" along the carrier
  (`emit_topo::chord_within`): segment ends on a straight rim, and
  `Track::cover` on a curved one. It does so for several shared edges
  (`rim_holding`) and for a lone shared rim alike. Main had accepted
  any chord on a lone curved rim, and could hold a straight chord on
  an arc piece whose ends it shared.

**Rows** (`emit_curved_rim_chord`), each in both member orders where it
is a union:
- the post across the first piece, which is this document, red before
  the fix;
- the post across the second piece;
- a notched full circle, twice;
- the cavity, whose rim pieces arrive through the graft.

Every one names each edge for the exact row it lies within. Unit rows
(`emit_topo::rim_holding_rows`) cover `rim_holding` and `chord_within`
on a split 270° prism.

## Closed (PR 4794, 2026-10-12)

The cause was the lineage chase. A split piece whose lineage ran
through a stretch the boolean killed fell to the derived-chord path.
`KilledSplits` now restores those links, so the piece resolves through
its operand table, and the killed-lineage row is closed with it.

The curved within-reading (`chord_within`) stays as the one reading for
`rim_holding` and the `Rim::One` arm, with unit witnesses for: a chord
held by its piece, a chord across a split, a chord at a piece end, a
chord through 0°, and a straight chord against curved pieces.

Only `crossing_slots` renamed in the corpus. Two union refusals in
`wire_legal_union_refusals` rightly went, because their edges were split
pieces.

