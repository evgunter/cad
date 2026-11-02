---
id: closed-form-limb-2-costs-milliseconds-per-curved-edge
kind: issue
title: Limb 2 through the Taylor enclosing net costs 1-6 ms per circle and 8-73 ms per spiric edge in release; curved boolean rows run about ten times slower
status: open
opened: 2026-10-25
priority: P2
cost: M
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
---

Measured by the unit's implementer lane (branch
`offset/limb2-for-every-carrier`, 2026-10-25, release, workspace profile,
ε = 1e-9).

Per edge, through `EdgeCurve::certify_via` with the lane (both operands'
limb 2, `pcurve_cache::closed_form_offset_sup`):

| edge | before (samples only) | after |
|---|---|---|
| plane × cylinder circle, full turn | < 1 µs of limb work | 5.7 ms |
| plane × cylinder circle, quarter | < 1 µs | 1.3 ms |
| plane × torus spiric, `d = 0.1` | < 1 µs | 8–17 ms |

That is after cutting each Taylor span into one composite part rather than
`projected::INCIDENCE_CUTS` (a first cut that read every span eight ways
cost 76 ms per full circle and 370 ms per spiric). Rows of
`sweep::all` that build curved booleans (`a_closed_conic_in_the_partners_face`,
`a_circle_of_one_radius_elsewhere_is_no_rim`,
`a_body_built_turned_is_the_body_turned`, 13 rows) took 0.24 s in all on
`main` and 2.95 s on the branch.

Where the time goes (callgrind, one full circle): the canonical composite
~50% (`geom_core::spline::blossom::span_restrict` ~35% of the total — the
net is already in Bézier form, full multiplicity at every break, so
restricting a span to its own ends reproduces its row), the enclosing net
itself (`carrier_net::span_bernstein`) ~27%, and
`projected::piece_controls` ~19%, again a span restricted to its own
ends. Candidate cuts: an identity path for a Bézier span read at its own
ends in `CoeffWindow::restrict`/`SpanRow::restrict` and in
`piece_controls` (`Certification::between` already returns the source at
an exact end ratio, but caps its decoration at the weakest of the three,
so the path is not bit-identical in the decoration); a first grid graded
to the band (fewer spans where ε is coarse); or an exact sup for a
circle or ellipse against a quadric, whose residual is a degree-2
trigonometric polynomial (`harmonic::harmonic_sup`), with the net kept
for the spiric and the torus.

## Measured again on the door split (2026-10-26)

Conic unit 1's PR (branch `offset/limb2-for-every-carrier`, the door
split on Ev's ruling on PR 5689) measured the same 13 rows, release with
the workspace profile (`debug-assertions = true`), `--test-threads 1`,
three runs each, on the same box back to back. Before is `main` at
`bc80647eef`; after is the branch.

| row | before (s) | after (s) |
|---|---|---|
| `a_frustum_through_a_plate_with_a_hole_of_its_top_radius_builds` | 0.108 | 2.899 |
| `a_boss_on_a_plate` | 0.127 | 1.673 |
| `a_rim_inside_a_ball_face` | 0.108 | 1.339 |
| `a_boss_beside_a_ring_and_over_a_hole` | 0.090 | 1.332 |
| `a_plug_in_an_annulus_bore` | 0.092 | 1.133 |
| `a_ball_union_built_on_a_turned_plane_is_the_union_turned` | 0.064 | 0.985 |
| `crossing_washers_with_radii_in_band_are_no_rim` | 0.040 | 0.922 |
| `coaxial_equal_cylinders_unite` | 0.052 | 0.583 |
| `every_construction_built_on_a_turned_plane_is_the_construction_turned` | 0.022 | 0.533 |
| `a_boss_on_a_plate_at_interval` | 0.074 | 0.486 |
| `a_boss_over_a_hole_tangent_inside_its_rim` | 0.046 | 0.483 |
| `a_puck_and_its_rounding_ring_unite` | 0.041 | 0.302 |
| `a_closed_edge_in_a_cylinder_or_cone_face_refuses_typed` | 0.021 | 0.191 |
| **all 13** | **0.89** | **12.9** |

About 14.5× in all. A dual pass pays none of it (its door only mints);
an `f64` pass pays it at the attach door and again at rest, where every
at-rest door's check 2 now runs the scalar's edge door. Nothing in the
door split cut it; the candidate cuts above stand.

## Measured on the one reader (2026-10-26)

Same 13 rows and method, after fork row 203's fix (one limb-2 reader
framed in certification arithmetic, PR 5728). Twelve of the rows are in
the tree under the same names, and they total **10.7 s**, against
12.4 s on the door split. The thirteenth,
`a_boss_over_a_hole_tangent_inside_its_rim`, is now main's
`a_boss_over_a_hole_by_pose` (23 s, a pose sweep), and is not comparable.
The candidate cuts above stand.

**PR 5728's cost pass (2026-10-27).** Over a parameter box the rounds stop
at the net's rounding; at `f64` every read on the profiled rows clears in
its first round, so the cost is the number of composites times one
composite's cost. Two cuts landed:

- a bounded thread-local memo of limb 2's rounds
  (`crates/geom-brep/src/pcurve_cache.rs`, `limb2_memo`), keyed by every
  input a round reads. Hit rate: 82% on
  `split_through_a_ruling::a_frustum_split_through_a_ruling_answers`, 24%
  on `review_r1_turn_probes::r1_certify_turn_is_sound_against_the_true_defect_length`;
- a cheaper first composite: restricting a Bézier segment to its own span
  is its window (`crates/geom-core/src/spline/blossom.rs`,
  `whole_bezier_span`), and the enclosing net's frame-free half is formed
  once per span and memoised (`crates/geom-brep/src/carrier_net.rs`,
  `SpanBasis`), every net bit unchanged.

Fast-set rows over the bar at default ε on CI went 299 → 170 → 9; the
default-ε suite went 395 s → 155 s. A factored expansion
(`p₀ + u·A + v·B`) was cheaper still and was backed out: it encloses the
same reals but moved limb 2's bounds by rounding at ε = 1e-12, which
flipped four rows.

**Still over the bar, moved to the slow set with this item cited**
(`.config/nextest.toml`; CI default ε on the branch, all under 1 s on
`main`):

| row | time |
|---|---|
| `geom-brep::all review_r1_turn_probes::r1_certify_turn_is_sound_against_the_true_defect_length` | 17.7 s |
| `topo boolean::boxes::sphere_rect_rows::a_zone_reach_charge_covers_random_poses_near_a_box_row` | 8.8 s |
| `topo boolean::join::frame_dispatch_tests::the_agreement_gate_never_serves_on_the_span_alone` | 2.4 s |
| `editor-core::all m10_4_stackup_interval::a_foreign_verdict_or_box_refuses_typed` | 2.1 s |
| `sweep::all copied_carriers_at_interval::random_copied_carriers_extrude_at_interval_without_contradiction` | 1.7 s |
| `sweep::all split_through_a_ruling::a_frustum_split_through_a_ruling_answers` | 1.5 s |
| `step-import::all wild::wild_bodies_are_a_fixed_point_of_our_own_dialect` | 1.3 s |
| `topo chord_join::torus_ring_rows::a_ring_on_a_torus_face_is_re_homed_by_a_path` | 1.2 s |
| `sweep::all split_tangent_edge_curved::a_concave_graze_of_a_revolved_hole_refuses` | 1.04 s |
| `step-import::all wild::wild_files_import_and_agree_with_the_oracle` (next run, at the bar) | 1.18 s |
| `sweep::all split_section_rings::an_axis_parallel_cut_of_a_cylinder_along_any_axis_pairs_each_ruling` (next run, at the bar) | 1.01 s |

**The turn row's breakdown.** About 8,000 full-turn ellipse
certifications, almost all with distinct inputs (each case has its own
window), each a 32-span degree-12 composite against a cylinder; sampled,
the time is about half composite multiplication and half building the
net. The options: a cheaper composite (fewer spans per turn, or a lower
Taylor degree); running limb 2 less often in that loop; or changing the
test.

**A possible cut at the parameter-family scalar** (PR 5728's dual
review, not taken there): `Sym::free_identity` builds limb 2's free-`t`
residual (`crates/geom-brep/src/edge_nurbs.rs` `free_residual`) eagerly,
on every `Sym<Interval>` read, before the margin's numeric channel is
asked. Built only where the numeric channel does not already decide
the margin, it would cost nothing on the reads that clear on their
enclosure.

**PR 5885's cost pass (2026-10-28).** Profiled (callgrind) on the turn
row, the zone-reach row, the agreement-gate row and the stackup row.
Before, the turn row was 8,100 certifications and 6,167 first-round
composites at 36.6 spans each, 57% building the net and 41% composing,
with interval multiplication at 45% of the total. Four cuts landed:

- **A first reading on dyadic spans** (`pcurve_cache::limb2_quick`,
  C2 in `crates/geom-brep/README.md`). A curved closed form is read on
  the dyadic spans of its angle parameter, each a single degree-12
  Bézier segment enclosed alone (`carrier_net::framed_span_net`, its
  net factored through the span's `α`/`β` Bernstein rows) and composed
  alone. Uncleared spans are halved locally. A span may reach past the
  window; one past it whose halving stops halving its bound is read
  again clipped to it. The reading passes only on cleared parts that
  cover the window's outer ends with no gap (`limb2_quick::covering`),
  and a span it cannot halve further ends it. Everything else goes to
  the rounds unchanged, so every refusal is still the rounds'. A
  span's reading is keyed on the carrier, the operand, the chart (a
  cone's nappe included) and the span, so edges sharing a carrier
  share its spans. On the turn row, 6,912 reads compute 170 spans.
- **A Bernstein row squared once per mirror pair**
  (`compose::bern_square_row`), and no weight square for the plane's
  form. Bit for bit the same.
- **A one-signed interval product read on two corners**
  (`DInterval::signed_corners`). Bit for bit the four-corner fold.
- **Two quadratic scans out of the rounds**: `Knots::value_at`'s linear
  search, and the per-part scan for an uncertified control. Bit for bit
  the same.

**What remains slow.**
`editor-core::all m10_4_stackup_interval::a_foreign_verdict_or_box_refuses_typed`
took 1.43 s on CI (2.1 s before) and stays in the slow set. Its reads are at
`Sym<Interval>`, where the box's spread keeps the first reading from
clearing, so limb 2's rounds decide, as before: 48 reads, 80 rounds,
the net (47%) and the composite (50%) rebuilt for every round. The
cut not taken: the rounds re-expand every span every round, where a
span the round did not cut could keep its rows (bit for bit, since a
span's rows read only its own ends; its joint hull is re-formed).

The `Sym::free_identity` eager build above is not where this row's
time goes.

## 2026-11-01 — a renamed test (SPLITDOOR, `splitdoor/sided-followups`)

`coaxial_equal_cylinders_unite`, in the timing table above, is now
`a_closed_conic_in_the_partners_face::coaxial_equal_cylinders_build_every_op`.
It runs every op in both orders, not only the unions, so its time is no
longer comparable with the row above (0.30 s at ε 1e-9 locally).
