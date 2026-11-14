---
id: levers-read-a-whole-carriers-cover-where-a-span-is-consumed
kind: issue
title: Levers read a whole carrier's cover where an edge's span or a face is what the verdict consumes
status: open
priority: P2
cost: M
opened: 2026-11-14
---


`crates/geom-core/README.md` "Float rounding" builds a lever "only from
the geometry consumed", and `cover-levers` narrowed `geom_brep::Reach`
to the span. These levers still read the whole carrier's cover
(`Curve3::cover`/`Surface::cover`, then `ExtentBall::of_covered(..).lever()`),
each pinned as `whole` by `scripts/gates/cover-kind-match.sh`:

- `geom-brep` `props::curved` (ten sites: `require_one_chart_branch`,
  `cylinder_boundary`, `cone_boundary`, `sphere_boundary` and their
  neighbours): each levers a props edge's class margin at its whole
  circle's ball, though the edge (`e.t0..e.t1`) is what is classified.
- `topo` `chord_join::SectionCurve::lever` (and the lone ring's
  `bool_lone_ring_region_side` lever in `RimSide`'s arm): a closed
  section's whole conic. The lone ring IS its whole circle, so that site
  consumes the whole carrier; whether a section curve's departure
  consumes the whole curve or the chosen arc is the question for the
  first.
- `topo` `boolean::join`, `boolean::solid_contain` (two),
  `boolean::trace` (`plane_misses`, the whole carrier by C5's design),
  `separating` (a full section circle, whole by construction);
  `geom-brep` `curve_pair::pose`, `intersect`, `pair_section` (and
  `cylinders`, `torus`), `pcurve_cache`, `ssi`: carrier-pair readings,
  where the carrier is the operand and the whole cover may be what is
  consumed.

Each site that narrows moves its verdict (a shorter lever decides more
readily) and needs its own differential; each that stays says why the
whole carrier is consumed. `boolean::arcs::arcs_along` narrowed to its
edge's cover in `cover-kind-match-gate`. (boxes-cover7 lane)
