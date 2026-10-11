---
id: at-rest-records-are-unordered-cell-pairs
kind: unit
title: Build the ruled at-rest record: ContactRecords is one list of unordered pairs of the body's cells plus citations; no operand label, stored granularity or witness
status: closed
pr: 4598
opened: 2026-10-10
priority: P1
cost: H
closed: 2026-10-11
---


## What

Ev ruled on `[ev]` PR 4547 (fork log row 108). D1 and topo README C3
now state it. A body's contact record is an **unordered pair of its own
cells (vertex, edge or face) whose interiors meet, plus the decisions it
cites**, kept in one list that `Backing::Carried { record }` indexes by
position. It stores no operand label (`a_on_b`/`b_on_a` merge;
`VvContact { a, b }` is unordered), no granularity, and no witness
(`CurveContact`'s third field goes). Every pair whose interiors meet is
recorded, a straight edge in a plane included; how the census confirms a
record (bound reconstruction, curve touch, patch) is read from the two
cells' carriers.

## Shape (both designers' final state; the PR body carries their reports)

- `Contact { cells: [Cell; 2] }`, canonical order (`Cell` is `Ord`:
  vertex < edge < face, then key). The constructor refuses a pair of
  one cell, or an incident pair. Kind-filtered views are sugar.
- `rows()` is the list. `cites`, `cites_mut`, `carried_from`, `carried`,
  `cell_pairs` and `rekeyed` each become one pass over one list. Today
  each restates a seven-list concatenation order by hand.
- The boolean's reduction keeps its operand-tagged working set
  (`(Operand, Cell)`, the `RowCell::Input` shape) inside `ContactAcc`;
  `ops.rs::carry` maps it to result cells and drops the tags.
- The census: its face rung generalises to "both cells of the event lie
  in the closures of the record's two cells". A face–face pair on one
  carrier, opposed, is a patch (the rest doors, unchanged). On two
  carriers, the jet schedule runs along the closed-form locus, refusing
  typed where no lane exists. An edge–face pair is a curve touch along
  the edge's carrier, or bound reconstruction when straight in a plane.
  `CurvedRestUnrecorded` retires into a stored record, and
  `StaleDeclaration::CurveLocus` becomes stale-by-cells.
- Producers: boolean (carry), product (re-key + concatenate, re-cite by
  position), import (vv pair), mate (face–face pair), split (none).

## Scope notes

- The B2 sites (~99 files, `work/intent/contact-records-cite-their-decision.md`
  lists them) are the footprint. Census goldens that print records move.
- An operand's own records crossing a boolean (only via declarations
  today, and INTENT F deletes `CarriedContacts`) is INTENT F's and WIRE's
  (`a-boolean-drops-its-operands-own-contact-records`). Under one list,
  that carry becomes one function. Coordinate with F if it is live.
- Review tier: **dual** (an architectural change, broad and hard to
  change later), under `docs/DUAL-REVIEW-PROTOCOL.md`.

## Built as a stack (2026-10-10)

PR 4598 (1/2): the one-list type (`Contact`, `ContactKind`,
`ContactRecords`), the reduction's `ReductionContacts`, and every
producer and consumer, with the census reading what it read before.
PR 4604 (2/2), stacked on it, is the census's carrier-read
generalisation and the new edge–face and face–face rows.

## Closed 2026-10-11

PR 4598 (the type, DR row file `PR-4598`) and PR 4604 (the census by carrier, `PR-4604`) merged. Filed residue: `restread/a-two-carrier-curve-touch-has-no-bounded-locus`, `restread/an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`, `msolve/a-tangent-mate-can-mint-a-two-carrier-face-pair`.
