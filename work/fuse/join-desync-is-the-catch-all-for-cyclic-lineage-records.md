---
id: join-desync-is-the-catch-all-for-cyclic-lineage-records
kind: issue
title: BooleanError::JoinDesync, named and displayed as an A/B lockstep break, has become the catch-all for cyclic lineage records (fragment, split, absorption), beside dedicated cycle variants elsewhere
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
