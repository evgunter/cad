---
id: the-shipped-parting-sites-decide-on-the-leading-term-with-no-witness
kind: issue
title: The shipped parting sites decide a tie's side on the leading term at a length, with no witness point
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by `a-leaves-verdict-carries-no-witness-point`'s lane
(`tangrim/leaves-witness`), its §5 sweep. The design's item 3
(`the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways`,
`## Design`) ends: "the shipped leading-term sites adopt the witness".
That lane built the witness for `topo::bound_side::Tie::descend` only
(`Tie::witnessed`: a tied line or conic edge's own point at the
parting's arm, its offset that point's signed distance from a sphere's
or a cylinder's carrier, `link::Round`, the only kinds it reads). Every other site that reads a tie's side off
`geom_brep::parting` still decides on the leading term at a length,
whose third-order remainder over that length is unbounded:

- `boolean/sectors.rs` `runs_into` (`bool_sector_runs_into_bend`): two
  bounds along one direction, their bends' parting at the shorter reach
  (`geom_brep::parting(k - kb, u, l.min(lb))`);
- `boolean/sectors.rs` `crescent` (the in-plane bound-against-bound
  opening, `geom_brep::parting(kb - ka, u, l)`);
- `boolean/sectors.rs` `tangent_relative_side` (`parting_side`, the
  declared tangency's surface-against-surface reading; also on
  `second-order-side-readings-the-stage-1-sweep-left-beside-the-reading`);
- `boolean/fate.rs` `corner_holds` (`geom_brep::parting(k - kb, u, l)`);
- `boolean/slit.rs` `walls_inside` (`geom_brep::parting(floor_bend -
  wall_bend, floor, arm)`);
- `geom_brep::tied_germ_turn`, called by `splitting/insert.rs` and
  `boolean/vtxfac.rs`.

The bound-against-bound sites need a witness of a different shape from
`descend`'s: a point of one bound against the other bound (a curve, not
a carrier), so the curve-pair distance the crossing layer reads, not a
surface residual. `Reach::witness` gives the bound's point; the
partner's offset is what is owed.

**`crescent` decides topology since PR 6375** (`sectors::sector_overlap`,
recorded by `pair_search` as a coplanar overlap in every op). The
witness row's `## crescent now mints topology` holds the locality
argument for why its decided sign is safe without a witness (the
claim is local to the vertex, and a sign decided off zero dominates the
remainder near it); what a witness would add there is the crescent's
extent at a finite reach. Of the sites above it is the one whose
verdict reaches topology most directly.
