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
  `line_conic`, the window terms), `splitting::containment::conic_arc_ball`
  (since retired on main by the edge-cover refactor),
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


## Slate triage (2026-11-08, ONEHOME split)

- Moved to ONEROUND (`work/oneround/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows on f64 rounding, levers and the consumed extent that PR 6380 and PR 6391 filed went to ONEROUND.

## Measured (2026-11-19, phase 1 of `oneround/conic-vertex`)

Taken locally on merge base `b80352cd9`; evidence, not committed
measurements. The tables, the measurement patch, the probe sources and
the commands that re-take each number are on the branch
`analysis/oneround/conic-vertex-probes` (never merged), under
`analysis/oneround/conic-vertex/`.

**(a) The near-gap families.** Reading the point from the seam vertex,
`(c + u·a) + u·a·(cos θ − 1) + v·b·sin θ`, with no stored form,
certifies 40 of 40 in all five families (main: 7, 7, 7, 7, 1), every
root within ε of the exact oracle's. The meter's bound falls from
6.4e-13 (3.8 u·ρ) to 1.28e-13 with the vertex summed in f64, and to
6.7e-16 with it summed exactly.

**(b) The section tables' far-centre conics.** Evaluation recovers
nothing. Ellipses minted by `plane_cylinder_section` and
`plane_cone_section` with centres past the box miss their own two
surfaces by 1–3.6 `u·a` at their in-box points (7e-10 m at `a ≈ 1e6`),
the same whether the point is read from the centre, from the vertex,
or from the vertex summed exactly. The loss is in the minted stored
form. Ev decided that form on PR 4812 (a minted section conic stored
anchored on its arc, the centre derived); the unit is
`work/honepair/the-section-table-admits-a-curve-on-its-resolution.md`,
which now carries these numbers.

**(c) The uncharged margins.** Thirteen sites probed; eleven misread
on main once the conic's terms are past the box. One reads wrong at
the default ε (the conic excess of `split_section_area`, from far
1e4); every other wrong verdict is at ε 1e-12 from far about 1e5, and
at ε 1e-9 they are in-gap only, from far 1e6–1e7. Reading the point
from the vertex fixes only the chords of one carrier. The rows:
- `the-conic-excess-of-a-section-area-reads-wrong-on-a-far-centred-conic-at-the-default-eps`
- `a-line-or-a-conic-nested-against-a-far-centred-conic-is-cleared-on-terms-that-cancel-at-the-far-scale`
- `the-arc-cover-misses-a-far-centred-arc-by-its-centres-rounding-and-the-depth-gap-decides-on-it`
- `a-chord-of-one-far-centred-conic-is-decided-on-two-points-each-summed-from-the-centre`
- `a-far-centred-conics-point-is-decided-against-an-in-box-point-at-the-centres-rounding`
- `a-far-circle-has-no-re-seaming-door`
- `work/select/an-endpoint-pin-passes-on-a-far-centred-conics-own-rounded-point.md`
- `work/charttail/row-interval-accepts-a-far-centred-conics-end-inside-the-bands-gap.md`
- `work/pcert/the-plane-arms-harmonic-envelope-reads-zero-between-two-far-centres.md`

**Corrections to this row.**
- "which this row's vertex evaluation removes at the source" holds
  only for the three chord sites. The others read `frame.origin` and
  the semi-axes directly, or need the vertex exact (a plain `c + u·a`
  rounds at `u·far` as the centre form does), or need neither centre
  nor vertex.
- "Bounds only … which rounding loosens": the arc cover's greatest
  depth undershoots the true arc and its least overshoots, each by
  about `u·far`, and `depth_gap` decides on them.
- "Fail-closed only" for `row_interval` and `certify`'s endpoint
  residuals: both can accept.
- `carrier_harmonic`'s plane arm, "mostly widens": in the probe its
  centre term reads 0 and the sampled schedule is the guard.
- "Floor ~1–2 u·ρ" for the vertex term: 0.77 u·ρ summed in f64,
  0.004 u·ρ summed exactly.
- Sites that have moved: `conic_ring` and `edge_reach_of` (retired by
  commit `f67931961`) and `axial_carrier` (`f097a63b8`) are now
  `geom::cover::Cover::arc`; `census::depth_gap` is a module and no
  longer reads `speed_hi − speed_lo`;
  `pair_section::cylinders::conic_ball` (retired by `c35ec02aa`) is
  now `placement(ExtentBall::of_covered(curve.cover()…), …)`, which
  under-read only (fail-closed) in the one axis-aligned pose tried;
  `Window::read`'s clearance is `Polish::closest`'s
  `curve_pair_pose_clear`; `topo::pcurves::row_interval` is in
  `pcurves/mint.rs`.
- `the_near_gap_families_keep_their_certified_answers` fixes its own
  ε at 1e-9, so its floors have no 1e-6 or 1e-12 row.

**Not built.** The vertex chain was not landed: the stored form is
what evaluates it, and it is a design question that leans on PR 7131.
