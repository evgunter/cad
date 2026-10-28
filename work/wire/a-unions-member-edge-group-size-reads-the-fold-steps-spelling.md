---
id: a-unions-member-edge-group-size-reads-the-fold-steps-spelling
kind: issue
title: A union's member-edge group size reads the fold steps' spelling and differs across member orders
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Found by `wire-unionvertex`, measured with the same probe as
`a-unions-vertex-group-size-reads-the-fold-steps-spelling`. The probe
was an env-gated hook after `FragmentGroups::folded` in `eval/wire.rs`'s
union arm. It logged `sizes()` for every published name of every union
over the editor-core suite, minus the interval, range and solve drives.

A union answers the group-size rung for a non-seam edge base (a member
edge's line, `From(m, e)` with an `Ends` piece tail) from its fold
steps' records. The end pass re-derives every member-edge group over
the finished body (`emit_union`'s `group_member_edges`, `PieceGroup`),
but `groups.rs`'s `rederived` sends only face, seam and vertex bases to
that record. So the answer for an edge piece's base depends on which
fold step spelled it, and the steps run in member order.

## Measured (main 181d651b93, 2026-10-28)

- **582** (test, published edge piece) pairs whose base (`sizes` of
  the name with its last `Fragment` removed) read more than one value
  across the member orders of one test. All of them are `From`-headed
  member-edge lines. The values mixed are `Some([1])`, `Some([])`,
  `Some([2])`, `Some([0])` and `Some([3])`, so some orders read a
  different count, not only no group.
- **265** pairs did the same for `sizes` of the published edge name
  itself.
- Largest: `emit_union_flush_names::a_union_cites_only_what_the_finished_body_holds`
  (100) and `emit_union_rim_piece_ranks::*` (96 each).
- `wire-unionvertex` moved none of these. Its dump shows only vertex
  sizes moving.

## Repair direction

Record each `PieceGroup` that `group_member_edges` forms as the line's
group, through `record_on_line`, in the union's published record, and
send member-edge bases to it in `rederived`. Then retire the fold path
of `FragmentGroups::sizes` for every base except the vertex bases that
`Rederived::by_fold` leaves to it. Before and after, dump every union's
`sizes()`: nothing but edge sizes may move.
