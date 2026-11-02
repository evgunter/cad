---
id: unsourced-levers-await-a-source
kind: issue
title: 74 lever arms fit none of the three sources and are pinned unsourced, awaiting Ev's ruling or a re-posing
status: open
opened: 2026-11-02
priority: P1
cost: M
design: true
---


Found by the `onehome/one-extent` unit (2026-11-02), which typed every
lever arm (`geom_core::Lever`) and built each from one of the three
sources Ev ruled on (PR 6329, `crates/geom-core/README.md` "Float
rounding"): a read of the consumed extent, a curvature arm, the session
box; rates moved to the metric doors. These arms fit none. Each carries
a `// Unsourced lever: …` line at its call and is pinned per crate by
`scripts/gates/lever-sources.sh` (`lever-sources.counts`, pinned per
file: 63 references at the unit's merge). A fourth source is Ev's
call; otherwise each family is re-posed.

**1. A source exists, but reading it would move verdicts by rounding.**
The arm is the norm of a vector scaled to an extent, or a sum of
reads, where the honest read is the extent itself or their union:
- the sector and germ arms, read through one home,
  `topo::boolean::sectors::metering_arm` (and `departure_arm_lever`,
  which `sector_shape`, `splitting::insert`, `splitting::neighborhood`
  and `splitting::rules` call), and `leaned_lever` (an arm times a
  lean's sine); `census` wedge arm. The vector is a unit tangent times
  `edge_extent_inf`; its norm differs from the extent in the last bits;
- `boolean::vtxfac` `held_reads` (a ball about the runs' shortest arm,
  which can under-state the region) and `unglued_flat_sector`'s ball;
- sums of radii or reaches, whose union read is their `max`:
  `boolean::rim_wedge` `same_circle` (2), editor-core
  `checks::at_rest` `constant_pairing`, `eval::measure` `parallel` and
  `plane_round`, `mate::coset` and `mate::value` (2) (`Arm`, an f64
  sum), `geom_brep::curve_pair::pose` `route` (`length + e + e`);
- arc lengths, where the extent reads are diameters:
  `offset_axial` `corner_arms` circle case, `ring_path`
  `spline_crossings` arc piece, `curve_pair::pose` `line_line` and
  `line_conic`; `chord_join` `SectionOval.extent` (the larger half-width,
  short of the oval's ball); `chart_region` `hyp` (2,
  `√(reach² + r_max²)`).

- lengths read off a line's parameter span where the edge extent reads
  the end points' distance: `offset_axial` `corner_arms`' line,
  `boolean::reduce` `line_cone_roots` (`dir.norm()·|Δt|`).

**2. A tolerance or a rounding standing in for an arm** (re-pose, not
a lever): `validate` `resolved`, `boolean::join` `off_conic_slack`,
`dihedral` `wedge_reads_zero_at_the_arm` / `_offer` (`band.escalate()`),
`pcurves` joint room (`d − 2ε`), `intersect` `pn_apex_tangent_reach`,
`pn_apex_point_reach` and `pair_bound` far/near/offset (the plane's
apex distance plus its rounding; `levered_inv` differs in the last bit),
`fate` `sign` (a unit arm on a unit vector's length or a turn count).

**3. A length a caller hands in with no geometry behind it**:
`geom_brep::ssi` `SsiDomain::extent` (a `pub f64` field; its producers
in `pair_section` are geometric, its ~80 test literals are not):
`ssi::boundary`, `ssi::certify::tube_transversality` and its test twin,
`ssi::march::decide_transversality`, `ssi::section::decide_crossing`;
`ssi::march` `ssi_closure_tangent` (the marched polyline's length);
`boolean::trace` `ray_crossings` and its line piece (a ray length) and
`plane_misses` (distance from a plane's stored origin);
`census` `parallel_reading` (a line's parameter interval);
`ring_path` lead `Piece::Segment` (zero, a placeholder); editor-core
`trace` (2: a coordinate gap; a carrier size plus an offset);
`splitting::containment` `extent_from`'s edge balls (topo's own
`step_ball`/`carrier_ball`/`conic_arc_ball`), `boolean::solid_contain`
`carrier_roots` (`RuledFaceRegion::reach`, a topo boundary read),
`boolean::recl` corner ball, sweep `blend::open::planar` mitre ball
(sampled chord diameters).

**4. A carrier's stored term or a coefficient**: `pcurve_cache` focal
`beta` (a map coefficient), Villarceau `tilt` (its honest arm, the
Villarceau circle's radius, equals `R` only to the band), `unit_sign`
(2, a speed with no span), `spiric_chart_axis` (the chart's `R`);
`props::curved` `require_cone_circle` `off_axis` (a residual).

**5. `profile` (6)**: `seg` `sagitta`, `junction_reverses`,
`junction_turn`; `path` seam side and `resolve_fillet`; `sugar`
`arc_fillet_trims`. Each is a length of the consumed path (a chord, a
leg), an extent read in kind, but `profile` does not depend on
`geom_brep`, so it has no extent home to read through: whether it gets
one is a design call.
