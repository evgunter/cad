---
id: a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex
kind: issue
title: A near-graze conic point is summed from its far centre, so its rounding is ~u·ρ; evaluated from the arc's vertex it is ~u·ρ·θ² and certifies the near-gap poses PR 6349 now refuses
status: open
opened: 2026-11-02
priority: P1
cost: M
refs: [6349, 6329]
---

From the delta review of PR 6349 (the conic frame unit), 2026-11-02.

## What

PR 6349 made every reader evaluate a circle or ellipse point the carrier's own way: `c + (u·(a cos) + v·(b sin))` (circle `c + (u cos + v sin)·r`). The residual mirrors' running bound now follows that chain faithfully, so the walk and the meter read one number. At a near-graze pose (θ ≈ 0, |c| ≈ ρ, the point near the origin) the in-plane offset is rounded at |E| ≈ ρ before it cancels against `c`, giving a per-coordinate bound of about 5u·ρ, against main's left-associated (off-carrier) chain at about 4u·ρ. Near-gap certified counts fell, measured against the merge base:
- ellipse k = 1.25, 0.999, 1.001: 17→7;
- ellipse k = 0.8: 7→1;
- circles: 17→7.

Every lost answer refuses typed (`Meter`) and every certified root is true. `the_near_gap_families_keep_their_certified_answers` floors the counts.

## The fix (well-conditioned, then treat as reals)

Evaluate relative to the graze vertex:

`P − o = (c + u·a − o) + u·a·(cos θ − 1) + v·b·sin θ`, with `cos θ − 1 = −2 sin²(θ/2)`.

The θ-dependent error drops from ~u·ρ to ~u·ρ·θ². The vertex term is one constant per carrier, not per-sample noise. Floor ~1–2 u·ρ. The walk and the meter both read this chain; the certificate is about the true curve, so `point_at`'s D9 bits are not what the near-gap reading needs.

This is the principle Ev is ruling on in PR 6329 ("make operations well-conditioned, then treat them as reals"). Settle it there first: whether the near-gap reading takes a re-posed evaluation, and where it lives (a `Conic` trait method beside `running_point`). Raise the floors to the new counts.

## Ev on PR 6329 (2026-11-02)

The margin-scale designers split on the far-centre ellipse: (A) mint a fitted spline once the conic's terms leave the box, or (B) re-mint it in a well-conditioned form relative to the arc. Ev: "seems worth measuring if B is necessary. it's not that bad to add another form of under-the-hood storage since our convention of having a shared api over a family of representations should make it simple". So this row measures first. Over the section tables' far-centre conics and the near-gap families above, does the vertex-relative evaluation alone (no new stored form) recover what is lost? If not, add the vertex-anchored stored form behind the `Conic` trait (Ev accepts another representation under the shared API). Report both measurements.

- 2026-11-02 — PR 6380 (`onehome/run-owns-rounding`), a sweep of the
  production readings that evaluate a conic point or margin from its
  stored centre and semi-axes, classified by reading the code (not yet
  probed one by one; `ring_path`'s
  `a_graze_against_a_far_centred_sections_plane_is_read_wrong_nowhere`
  pose is the probe to reuse). Charged, through `rounding_charge` on
  `|offset| + speed_hi`, `carrier_resolution`, `Rounded` or Interval:
  `implicit::sample_rounding`, `ring_path` (`path_resolution`),
  `chord_join` (section pins), `curve_pair::pose` (`one_curve_gap`,
  `line_conic`, the window terms), `splitting::containment::conic_arc_ball`,
  `circle_roots::conic_off_plane` (so `ruled_region`'s leg roots),
  `ellipse_torus`, `section_loops::conics_clear` (a relative pad, lean),
  `curve_face::spline_face`, `carrier_net::harmonic`. Bounds only (an
  extent or a lever, which rounding loosens): `extent::conic_arc_reach`,
  `Reach::lever_from`, `ruled_region::reach_bound`, `splitting::section`'s
  reported chart. **Decided margins reading the far terms uncharged:**
  - `topo::splitting::section_loops::line_clears_conic` (`m·(c − p)`
    less the reach, `split_nest_line_conic`);
  - `topo::census::depth_gap` (`conic_ring`'s minor `speed_hi − speed_lo`,
    and its depth origin at the centre; the band-only `sweep_pad`);
  - `topo::splitting::join` (the conic excess of `split_section_area`);
  - `geom_brep::pcurve_cache::carrier_harmonic`'s plane arm (no `terms`;
    lean, its difference mostly widens the envelope);
  - `geom_brep::pair_section::cylinders::conic_ball` (a listing filter);
  - `topo::boolean::conic_quadric::off_the_apex` (`CONE_APEX`);
  - `geom_brep::curve_pair::pose::harmonic_roots` (`ROOT_GAP`, a point
    gap);
  - `geom_brep::curve_pair::pose::window` (`Window::read`'s clearance,
    `ring`);
  - `topo::census` (`axial_carrier`, `edge_reach_of`,
    `pm_census_arc_piece`), `topo::boolean::factored_crossing`
    (`bool_factored_crossings_apart`);
  - fail-closed only (a misread refuses): `topo::pcurves::row_interval`,
    `geom_brep::certify`'s endpoint residuals.
  A reading on a conic whose stored terms stay in the box is unaffected;
  each of these misreads only past the box's terms, which this row's
  vertex evaluation removes at the source.

