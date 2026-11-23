# The designers' attribution items, and M-f's must-carry half

## Run

`measure.patch` at the middle site: ε = 1e-9, topo, step-import and sweep in
full (8,624 tests), M-b's harness with five additions:
- a fold counter in `fold_arms`;
- a rerun of every composite at the same bound;
- a poison-verdict log;
- face tags at every door;
- the must-carry key for M-f.

editor-core was not run. Detail lines are sampled at most 5 per key per test;
counts are not.

## 1. M-b's pairing when the arm gate escalates first

Written into `m-b.md`. Pairing stops at the first differing decision. With a
poison bound from above that decision is the arm gate, so `dihedral_wedge`,
the bend and the material reads are unpaired on those reads. M-b's zero
counts for them cover the reads whose bound from above was finite, and say
nothing about faces with no surface.

## 2. On agreeing pairs, did the reach set the fold?

Judged against nothing. For each composite call whose two bounds agreed (EQ),
f64: did `fold_arms` take the reach as its least term at any station, or a
curvature arm at every station?

| Composite | reach least at some station | curvature arm least everywhere | reach share |
|---|---|---|---|
| WV `edge_wedge_over_as` | 2,584,647 | 618,152 | 81% |
| `classify_seam` | 210,938 | 226,515 | 48% |
| `rules::edge_wedge` | 2,748 | 856 | 76% |
| MC `must_carry_over_edge` | 2,386 | 69,682 | **3%** |
| `joinable` | 1,334 | 33,047 | **4%** |

M-b's "no straddle" says something about the reach for WV and the splitting
rule. It says little for the must-carry rule and `joinable`, whose folds a
curvature arm set on 96 to 97% of calls whatever the reach.

## 3. The rim's decision count

`classify_shared_rim` calls `edge_wedge_over_as`, which the harness also
shadows. So the rim's primary run contains the inner wrapper's extra runs:
- at the bound from above;
- at the bound with the edges added;
- the rerun.

That makes 4 × 45 = 180 decisions; the detached rerun, at the same bound,
makes 45. M-b's 135 against 45 was the same nesting, before the rerun was
added. **No cache.** Rerun at the same bound, every composite gave the same
verdict and the same decisions:

| Composite | calls rerun |
|---|---|
| WV | 3,369,332 |
| turn | 2,062,587 |
| seam | 439,689 |
| MC | 76,105 |
| `joinable` | 34,456 |
| `rules::edge_wedge` | 3,636 |
| rider | 2,698 |
| `sheets_part` | 260 |
| `stands_on` | 128 |

The only mismatch is these 304 nested rim calls.

## 4. Must-carry verdicts with a poison bound from above: whose faces?

Each read records the surfaces the must-carry verdict reads against the
door's own faces' tags.

| Verdict reads | door's faces | count | what it is |
|---|---|---|---|
| Torus/Cylinder, Plane/Cylinder, Cylinder/Cylinder | a placeholder NURBS face, uncovered | 12 | `contact_edge_must_carry::every_contact_edge_on_the_corpus_is_jet_determinate`. The reach is read off the edge's own faces, but a face is still the seed while the description names the real surfaces: certification's contract (an edge certified before its faces are described). **Not a reach read off other faces.** |
| Cylinder/Sphere, Sphere/Torus, Plane/Torus | the same surfaces, covered | 80 | the Dual lane (`blend_dual_tangent::*`, `general_circle_octant_dual::*`, `pole_slit_window::a_slit_body_does_not_finish_at_a_dual`). The bound from above's value is finite. Its **derivative channel is NaN**, which this instrument reads as poison |

So `disagreement.md`'s "poisoned nets" are placeholder seeds as `m-a.md`
says, plus a Dual-lane artefact.

**The Dual finding is not an instrument artefact for the run itself.** M-b
counted the Dual turn pairs at the bound from above as escalations. So on
Dual, a cover's far distance carries a NaN derivative, and every-sign
consumers there would escalate on covered faces. Where the NaN comes from (a
`sqrt` at 0 in the distance read, or the cover's `max`) was not chased.

## 5. Reads with both faces covered and the bound from above poison

524 reads, by door:

| Door | reads |
|---|---|
| `certify_edge_spec` | 158 |
| `chart_window` | 84 |
| `pcurves::mint` | 80 |
| `blend::surgery` | 80 |
| tier 3 | 56 |
| other | 66 |

Of the 202 sampled:
- **188 are the Dual NaN derivative above.**
- **14 are f64 fixtures:**
  - a null edge between two spline faces, with no cover of its own and a
    poison bound from below (`boolean::ops::spline_seam_rows::a_null_edge_between_two_spline_faces_is_refused_unplaced`,
    `boolean::vtxfac::vtxfac_spline_rows::review6166_…`);
  - an edge whose carrier datum describes no curve
    (`tier3_tests::check_1_names_the_carrier_datum_that_describes_no_curve`).

## 6. Zero-acting verdicts read with a poison bound from above

Among all composite verdicts with a poison bound from above:
- **WV `Seam`, 4:** `tier3_tests::check_1_names_the_carrier_datum_that_describes_no_curve`,
  a fixture at rest.
- **The turn on its mark (`OnMark`), 14:**
  - `a_plane_through_a_pole_cuts_a_cap::a_plane_through_the_pole_cuts_each_hemisphere_at_its_cap`
    (12, mid-construction: a boolean's site mint);
  - `coherence_channel::the_half_cap_findings_arrive_in_the_doors_order` (2).

Every other poisoned read acts on a definite sign or escalates:

| Read | count |
|---|---|
| turns `Ok(k)` | ≈ 47,000, the site mint's leg |
| WV `Crease` | 64 |
| MC `JetDeterminate` | 92 |
| seam `Transverse` | 9 |
| WV `InBand` | 22 |

The turns' tags cannot be trusted per call. A turn's lever is the lesser of
two legs, so the tag found under its bound from below can be the other leg's.
The site mint's leg is the only one whose bound from above is poison by
construction.

## M-f's must-carry half

Tier 3 never calls `must_carry_over_edge`. Its must-carry demand reads the
wedge fold (`edge_wedge_at`, then `wedge_reading`). So a door's must-carry
verdict was compared with tier 3's fold on the same edge:
- **key:** order-free surfaces, carrier and span;
- **class map:** `Crease` ↔ `Transverse`; `Seam` or `End` with a determinate
  bend ↔ `JetDeterminate`; an under-determined bend or `Lamina` ↔
  `UnderDetermined`; `Mixed`; `InBand`.

| both, same class | both, **different** | door only | tier 3 only |
|---|---|---|---|
| 18,943 | **12** | 18,308 | 494,622 |

The 12: the door read `UnderDetermined` and tier 3 at rest read a determinate
bend, all on `the_football_revolves::*`:
- `bodies_with_an_apex_as_a_vertex_revolve`
- `the_wide_apples_and_the_lower_needle_mesh_and_measure`
- `a_sheet_faces_boundary_encodes_its_side_where_a_rim_closes_it`
- `a_horns_cusp_revolved_part_way_is_its_lune`
- `a_horn_reached_at_its_cusp_from_one_side_is_a_needle_tip`

This is the shape the reach predicts: the door reads the edge before its faces
are finished, so its lever is shorter, and the sagitta's Zero at that lever
becomes Positive at the finished body's. These tests pass because tier 3
demands a tangent certificate only inside `tangent_certificate_lane`, which a
spindle's apex joins are outside. Whether the door's conventional description
there is the one tier 3 would demand was not checked.
