---
id: the-shipped-parting-sites-decide-on-the-leading-term-with-no-witness
kind: issue
title: The shipped parting sites decide a tie's side on the leading term at a length, with no witness point
status: open
opened: 2026-11-02
---


Found by `a-leaves-verdict-carries-no-witness-point`'s lane
(`tangrim/leaves-witness`), its §5 sweep. The design's item 3
(`the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways`,
`## Design`) ends: "the shipped leading-term sites adopt the witness".
That lane built the witness for `topo::bound_side::Tie::descend` only
(`Tie::witnessed`: a bound's own point at the parting's arm, its offset
the carrier's residual). Every other site that reads a tie's side off
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

**`crescent` decides topology once PR 6375 lands** (`tangrim/tangent-crescent`,
TANGRIM orchestrator's note, 2026-11-02): its overlap verdict then mints
a coplanar `(On, On)` record in every op (`sectors::crescent_records`)
instead of gating a refusal, so its leading term, not a point of either
rim, decides which records survive. Of the sites above it is the first
whose witness is owed; its pin is a crescent pose whose two rims' parting
at the shorter reach is decided while a point of one rim stands within
the band of the other.
