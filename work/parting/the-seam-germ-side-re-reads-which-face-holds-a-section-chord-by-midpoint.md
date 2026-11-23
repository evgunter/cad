---
id: the-seam-germ-side-re-reads-which-face-holds-a-section-chord-by-midpoint
kind: issue
title: The join's seam-germ side re-reads which face of the other operand holds a section chord by midpoint containment
status: open
opened: 2026-11-01
priority: P3
cost: M
---

Found by PARTING's `parting/sweep-records-edge-in-face` sweep for sites
that re-derive an edge-in-face verdict from values (2026-11-01). Read,
not measured.

**What.** `boolean::join::seam_germ_side` (`crates/topo/src/boolean/join.rs`)
reads the side a section loop's regions leave their seam on from each
line chord whose mate bounds a planar region: it asks
`holding_face_normal` which one face of the other operand holds the
chord's midpoint strictly inside (`contain::face_placement` against
every face), and reads that face's normal. Which face of the other
operand a section chord lies in is the question the reduction sweep now
answers and records for an operand's own line edges
(`reduce::settle_in_plane`, an edge-face record per piece), and the
section that minted the chord was built from a named face pair; here it
is re-derived from values by midpoint containment, outside
`[coincidence.door]`, and a midpoint no face holds strictly (a chord on
a face's boundary) reads nothing.

**What is owed.** Read the face the chord lies in from what built it (the
section's face pair, or a record), not by placing its midpoint; or
show why the midpoint reading is the one decision and record it.
`join.rs` is shared ground (CLEAVE's PR 6177 edits it); this row is on
PARTING's slate because it is the parting's question (does this edge
lie in this face), the sixth answer to it beside the five
`the-parting-reads-no-edge-lying-in-a-face-no-record-names` lists.
