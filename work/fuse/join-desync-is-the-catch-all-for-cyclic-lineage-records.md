---
id: join-desync-is-the-catch-all-for-cyclic-lineage-records
kind: issue
title: The split-hop naming tables (SplitNaming::joined_lineage, DiscardRow::boundary_chains) are chased by readers that bound or refuse a cycle, not checked where they are written; JoinDesync no longer carries any lineage cycle
status: open
opened: 2026-10-02
priority: P3
cost: M
refs: [descendant-chase-spends-its-budget-into-a-dropped-contact-record, 3874]
---


## Finding (review of PR 3874, 2026-10-02, style lane, likely)

`BooleanError::JoinDesync` (`crates/topo/src/boolean/mod.rs`) is named
and displayed as an A/B lockstep invariant ("A/B lockstep invariant
violated: …"). It now carries three corrupt-lineage cycles that have
nothing A/B in them:

- `discard.rs`: "a face's fragment lineage is cyclic";
- the split-lineage arm;
- `Descendants::live_face` (PR 3874): "a face's absorption rows are cyclic".

The same concept has dedicated variants elsewhere:
`Body::split_root`'s `SplitLineageCycle`, and editor-core's
`FragmentLineage` and `SplitLineage`. So a corrupt-lineage cycle is
spelled two ways, and a user reading the boolean's message is told about
a lockstep break that did not happen.

What is owed: one typed corrupt-lineage refusal in `BooleanError`,
naming which lineage cycled. Every `JoinDesync { what: "...cyclic" }`
site moves to it, and `crates/pncad-py`'s `boolean_error_tag` and stubs
follow. Sweep every `JoinDesync` site for others that are not lockstep
breaks.

## Narrowed by `fuse/face-lineage-at-append` (2026-10-12)

That PR moved the boolean's face lineage onto lists that refuse a row
that could cycle where it is added, with a typed refusal,
`BooleanError::LineageRow { rows, defect, key }`. Retired here:

- `discard.rs`'s "a face's fragment lineage is cyclic" (the face
  division rows are a `FaceDivisions`, so the chase is total);
- `Descendants::chained`'s "a cell's substitution rows are cyclic" (the
  `Descendants::live_face` site above, since folded into it; the rows
  are a `Substitutions`);
- `Fusions::push`'s two `JoinDesync` arms, now `LineageRow`;
- editor-core's `NamingError::FragmentLineage`, retired whole.

Still open, and what this row now holds: the lineage walks that chase
split hops, and their refusals.

- **`SplitEdge` provenance alone cannot cycle.** `Body::split_edge` is
  the one door that writes a `SplitEdge` record, and it records the
  parent on the child it has just minted, so each record points at an
  older key; a graft forwards records injectively, and provenance is
  never persisted or read back. So `Body::split_root`'s
  `SplitLineageCycle` is unreachable from provenance alone, and the two
  boolean sites that map it into `JoinDesync` (`discard.rs`, "a
  discarded edge's split lineage is cyclic"; `join.rs`, "an OnEdge
  germ's edge has a cyclic split lineage") guard an invariant the
  writer already holds.
- **What a split walk can still meet unchecked are the naming tables it
  steps through beside the provenance**, each a plain kernel-minted
  table that nothing checks where it is written:
  - `SplitNaming::joined_lineage` (`crates/topo/src/splitting/finish.rs`,
    written in `splitting/mod.rs` from the `SplitEdge` records taken
    before the kills), chased with a budget in editor-core's
    `emit_topo::chase_split_lineage`, which refuses
    `SplitLineageCycle`;
  - the kills' split hops, `DiscardRow::boundary_chains`, read as
    `KilledSplits` by `emit_topo::chase_edge_to_table` (bounded by the
    hop count, refusing `SplitLineageCycle`) and as `dead_split` by
    `names/borders.rs`'s `obstacle`, an inline bounded walk that ends in
    `bug("a piece edge's split lineage is cyclic")`.

  Each writes a split's minted piece onto the edge it was split from,
  so the fresh-key check `crate::lineage::FaceDivisions` holds for
  faces would hold for these edge rows where they are written, and the
  walks' refusals would go.

What is owed: the typed refusal for whatever of these stays
reader-checked, or, better, an edge division list in the
`FaceDivisions` shape for `joined_lineage` and the boundary chains, so
the walks are total; and a ruling on the two provenance-only sites
above. The null-edge copy chase (`uncopied`) is filed apart:
`null-edge-copy-rows-are-checked-for-a-cycle-by-their-reader`.

## Narrowed by `fuse/lineage-rows-refuse-at-append` (2026-10-13)

With PRs 4868 and 4981 and this one, `BooleanError::JoinDesync` carries
no lineage cycle:

- the null-edge copy chase (`ops::uncopied`, PR 4981) reads a
  `VertexCopies` list checked at append, and is total;
- the reduction's edge splits and the zip's copies per cone are
  checked at append (`EdgeSplits` over `crate::lineage::EdgeDivisions`,
  `crate::lineage::ConeCopies`), so `split_root`, `ops::input_cell`'s
  root and `Settler::settle_cones` are total and refuse nothing;
- the two provenance-only arms above (`discard.rs`'s "a discarded
  edge's split lineage is cyclic", `join.rs`'s "an OnEdge germ's edge
  has a cyclic split lineage") are not A/B lockstep breaks, so they no
  longer say they are: they refuse `BooleanError::SplitLineage`, which
  carries the same `SplitLineageCycle` editor-core's
  `NamingError::SplitLineage` carries, so a provenance cycle is spelled
  one way in both crates. They stay reader-checked: whether provenance
  can be checked where `Body::split_edge` writes it, and the walk given
  one home, is `origin/the-split-edge-lineage-walk-has-four-homes`.

The `JoinDesync` arms that remain and are not A/B lockstep breaks
either are torn-body reads (`topotail/torn-body-refusal-families-beyond-the-six-doors`)
and the graft doors' caller errors
(`issues/graft-door-caller-errors-are-join-desync`); neither is a
lineage cycle.

**What this row now holds** is the second bullet above: the
split-hop naming tables. `SplitNaming::joined_lineage`
(`crates/topo/src/splitting/finish.rs`, written in
`splitting/mod.rs`) is chased by `emit_topo::chase_split_lineage`,
bounded and refusing `SplitLineageCycle`; `DiscardRow::boundary_chains`
is read as `KilledSplits` by `emit_topo::chase_edge_to_table` (bounded,
refusing `SplitLineageCycle`) and inline by `names/borders.rs`'s
`reached`, which ends in `bug("a piece edge's split lineage is
cyclic")`. Each row maps a split's minted piece to the edge it was
split from, so an `EdgeDivisions`-shaped list written where the rows
are written would make the three walks total and their refusals go.
