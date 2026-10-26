---
id: a-pair-booleans-tied-corner-is-split-as-one-corner
kind: issue
title: The pair boolean groups an operand corner's copies by name, so a tied corner's candidates are split as one corner
status: closed
opened: 2026-10-25
priority: P2
cost: M
closed: 2026-10-26
---


Filed by WIRE's `wire-bridges` lane (PR 5535) from reading the code. It
has not been measured: no fixture with a tied operand corner held as
several vertices has been built yet.

## What

`emit_topo::name_boolean_vertices` keys `corners` by the corner's
NAME (`corners.entry(name)`). A tied operand vertex's candidates
share one name, so every result vertex at either candidate lands in
one entry, and the corner pass hands them all to
`Obstacles::split_corner` as copies of one corner. Two cases follow:

- **Both candidates survive whole.** No obstacle borders two of the
  copies, so `Obstacles::walls` refuses ("a copy of a corner held as
  several meets no recorded discard between them"). N4 wants the bare
  tied name instead.
- **One candidate is divided.** The undivided candidate's copy has
  empty walls, and the same refusal fires. N4 says: "the divided
  candidate's pieces are named by their `Borders`, the undivided
  candidates stay under the bare name".

The union's twin, `emit_union::name_corner_copies`, keys a corner by the
member corners it sits at, so each candidate is its own corner.

## What it needs

- A fixture: an operand whose table ties a vertex, met by a boolean
  that holds one candidate as several vertices, and one that leaves
  both whole.
- Then key the pair's corner pass by the operand vertex (the candidate)
  rather than the name, as the union does, sharing the body with
  `name_corner_copies` where that is possible.
- The union side has no witness either: the same fixture, met through a
  union, would be one.

## Correction from PR 5535's re-review (2026-10-26)

"No fixture has been built yet" is wrong. `union_member_orders_decide_alike::a_vertex_row_is_the_unions_once_in_every_member_order` (the prism-and-cube pinch) reaches the pair corner pass with a tied name held as 2 copies, twice, and it does not refuse. There the two candidates are copies of one point, split by real obstacles. The refusal claim narrows to tied candidates at *distinct points*, where no discard borders two copies (`borders.rs:691` refuses). That refusal is labelled `NamingError::Emission` although the input is legal.

## Closed (2026-10-26, sweep follow-up)

Fixed by PR 5732 (merge `94f46f49`, commit `5b3c7c96`: "its corner pass
keys a corner by candidate"). On main `name_boolean_vertices` keys
`corners` by `(StableName, Candidate)`
(`crates/editor-core/src/names/emit_topo.rs:2816`, the entry at
`:2849`), so a tied name's candidates are corners apart, and the pass
shares its body with the union's end pass (`name_corner_copies`). The
fixture the row asked for is
`a_tied_corners_candidates_are_named_apart_wherever_they_lie`
(`emit_topo.rs:8604`), in `x − y` over a void, both of the row's cases:
two whole corners tied in `x`'s table name one tied row under the bare
name, and with the apex (held as two vertices) tied to a whole corner,
the apex's copies are `Fragment(Borders)` and the whole corner keeps the
bare name. The union-side witness the row also asked for was not built; the
union already keyed by member corner before this row was filed.
