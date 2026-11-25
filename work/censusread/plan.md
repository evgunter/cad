# CENSUSREAD — the plan

The census's own reads at rest. `python3 scripts/work.py status --program censusread`
lists every row; this names the clusters that set the order.

## The slate

Opened 2026-11-25 with 33 rows from CONTACTHOLD, all P2 to P4. The
CONTACTHOLD cleanup checked every row against main at `6003877ad`, so the
rows here are live as of that date. Where a row was found narrowed or
needing re-measurement, its body has a dated note saying so.

## Order

Dispatch by cluster: the rows in a cluster read the same functions.

1. **The edge lying in a face** (P2, design):
   `an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`,
   which absorbed the conic edge-in-face row. Then
   `census-reads-a-line-edge-in-a-curved-face-only-between-two-planes`, and
   the parked `a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`,
   which waits on the first.
2. **Residuals read before the region** (P2): `the-census-confirm-pass-and-confinement-screen-escalate-a-residual-before-the-region`
   (it absorbed the confinement-screen and vertex-cut rows),
   `the-census-pierce-refuses-a-grazing-edge-within-its-placement-span-of-the-boundary`
   (it absorbed the stored-origin residual row), and
   `a-confirmed-face-pair-swallows-the-crossing-lanes-side-verdict`, with
   `ef-bound-backing-refinds-a-cuts-vertex-by-distance` on the same
   vertex-cut code. They all touch `pair_holds_point`, `pair_holds_edges`
   and `confirm_edge_face`.
3. **Material sides**: `the-edge-edge-and-point-record-confirms-read-no-material-sides`
   and `tier-3-prime-reads-no-side-between-coincident-curved-edges` share
   a fix for curved edges, which is reading the two arcs' wedges.
   `touch-sites-read-no-side-for-an-edge-on-a-ruled-face` is near them.
4. **The self-touch links**: `a-lens-candidate-dropped-in-band-refuses-unreadable`
   and `a-sheet-on-a-whole-tangent-plane-is-read-only-along-a-line-on-a-planar-face`
   are one unit, since both turn on `second_order`'s `pair` and
   `along_a_line`. Beside them are `a-lens-of-the-wrong-kind-for-its-region-refuses-instead-of-crossing`,
   `the-lune-reading-depends-on-the-corner-fans-quarter-turn-split`,
   `self-touch-cause-arms-no-row-reaches`,
   `one-material-leaves-its-in-band-arm-and-one-sector-reason-unpinned`
   and `a-same-solid-self-overlap-seen-only-as-touches-is-not-a-self-crossing`.
   The last one probably only needs its fixture built.
5. The rest in any order.

## Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` D1–D10 as ratified; `crates/topo/README.md` tier 3
  and tier 3′; the at-rest record rulings ATREST builds on (PRs #5555, #5494)
  and the finished-body records HELDRECORDS builds (PR 5967).

## Review posture

The review tiers of `memories/orchestration-model.md`. The lesson
CONTACT's reviews taught still holds, so briefs carry it: **nearly every
defect was a lever or a bound that is conservative for one verdict and
unsound for another**. A brief for any unit that levers a `decide` asks,
for each verdict, which bound makes that verdict sound. A lane measures
its row's repro on current main before it builds.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`.
