---
id: the-boolean-records-no-line-touch-and-the-census-does-not-demand-one
kind: issue
title: No boolean result records a touch along a line (collinear edges, or a glued tangency), and the census backs a collinear overlap on its bounds alone instead of demanding the record D1 requires
status: closed
opened: 2026-10-11
priority: P2
cost: M
branch: contacthold/ee-line-touch-record
refs: [a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only, a-touch-along-a-glued-tangency-cites-no-tangency-decision, two-coincident-curved-edges-touching-record-no-edge-edge-row, 4723]
pr: 4740
closed: 2026-10-12
---


Found by the designer pair on `a-same-side-tangent-touch-refuses-at-every-op`
(PR 4704). Both designers agreed on it after a reconciliation round.

## What

D1's record paragraph, as Ev approved it on PR 4547 (`02d468376e`), says:
"Every pair whose interiors meet is recorded, an edge lying in a face or
along an edge included; only how the census confirms a record depends on
its geometry." Commit `02d468376e` replaced the earlier sentence ("…
coincident-edge segments are certified by reconstruction from their
bounding vertex records"). Reconstruction is now how a record is
confirmed, not a stand-in for having the record.

Two parts of the code still follow the pre-4547 reading:

- **The boolean writes no line-touch record.** The reduction's working
  set (`ReductionContacts`) holds `vv`, `a_on_b` and `b_on_a` only;
  `carry_rows` says "no reduction mints one". So two solids touching
  along a line carry only the vertex–vertex records at the line's ends.
  That covers collinear edges (`crates/topo/tests/union_flush_onto_edge_contact.rs`,
  two bricks kissing along an edge) and a glued tangency along a ruling.
  Edge–edge rows today come only from `ee_lineage` splits and join-made
  pairs.
- **The census never asks for the row.** `census.rs` `ee_collinear_lane`
  → `ee_bound_backed` accepts a collinear overlap once both bounds carry
  vertex records. The `collinear_overlap` doc still describes the
  pre-4547 model.

## Owed

- Add `ee` to the reduction's working set. Write it where the reduction
  knows two coincident real edges' interiors meet and both flanks read
  Out (designer B pointed to `recl::resolve_edge_edge`'s non-germ exit;
  measure that it is reached for the planar pinch too).
- Cite it per C3's shortest-chain rule, and for a tangent flank pair
  also the glue's tangency decision.
- Make `ee_collinear_lane` demand the stored row.
- Re-baseline the planar pins' vertex–vertex-only counts.

The same-side fork's build (removing the short-circuit) does not land
with this row's `ee` writer: it waits on Ev's ruling in `[ev]` PR 4723
(`a-same-side-tangent-touch-refuses-at-every-op`). Once the
short-circuit goes, a same-side tangent touch reaches the writer with no
further plumbing. FUSE's
`a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only` is
the edge–face slice of the same gap.
