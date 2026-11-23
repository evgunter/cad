---
id: a-sphere-face-whose-extent-stops-ulps-short-of-its-pole-never-places-the-pole
kind: issue
title: A sphere face whose stored extent stops ulps short of its pole never places the pole, so a cell at the pole reads Local at every depth and six valid f64 cavities run out of pairs
status: dispatched
opened: 2026-11-18
priority: P0
cost: M
pr: 7115
branch: embed/pole
---

Traced by lane `embed/cavity-trace` (2026-11-18) for
`the-f64-cavity-gate-refuses-thirty-two-valid-shells`. Main `da49c63f`,
f64, the cavity asked through a local, uncommitted `CavityGate` switch,
with `topo::injective` and `topo::clearance` instrumented (uncommitted).

## Evidence

Six valid f64 cavity rows refuse `Budget(Pairs { max_cell_pairs: 65536 })`,
in two cavities.

**The two-arc sphere** (four sphere faces, radius 0.95, axis `y`):
- `shell7_seam_corner::a_two_arc_sphere_shells_to_its_closed_form` (the
  cheapest, about 20 s);
- `shell9_probe::sphere_grafted_body_is_tier_3_valid_before_and_after_the_closing_mint_which_shell_runs`;
- `shell9_r2_probes::r2_e2e_two_arc_sphere_hollows_and_tessellates`.

All three read the same receipt: 45,928 discharged, 19,608 refused
(the frontier left at overflow), 65,526 splits.

**The dome** (two sphere faces, radius 0.45, beside two cylinders and a
plane), in three `shell_curved_mouth` rows:
- `a_tangent_dome_shells_at_its_tangent_circle`;
- `opening_a_tangent_dome_refuses_two_equidistant_roots`;
- `a_dome_short_of_tangent_shells_at_its_upper_root`.

Each reads about 45,850 discharged and 19,700 refused.

## Trace

**Which pair spends the budget.** In both cavities nearly every split
is one touching pair: the two sphere faces `1v1 × 2v1`, which share both
meridian halves of a great circle and its pole vertex `1v1`. That pair
splits 84,624 times on the two-arc sphere and 85,074 on the dome
(`Split { Departure, Edge(1v1) }`), down to depth 34–35. Every other
pair certifies within a few dozen cells. The pair never certifies,
which is a different defect from one that certifies slowly.

**What each face presents there.** In 76,382 of the pair's cells, both
faces read `Err(Local)`. The cells sit at the pole: `v` up to
1.5707963267948963, each face's cell in the middle of its lune in `u`.
- **The `Local` is `zero_of`'s.** The shared vertex `1v1` is loose: no
  zero side holds it, and it is no single corner of two clips.
- **The pole is never placed.** `present_chart`'s sphere-pole reading
  finds `1v1` at the north pole (`vertex_at_singular_point` answers
  `Ok(true)`). But the rectangle has already been clipped to the face's
  sheet, whose `v` top is the stored extent's, 1.5707963267948963. That
  is 1 ulp below `π/2`'s enclosure, whose low end is
  1.5707963267948966, the `f64` nearest `π/2`.
- **So the clip is skipped.** `north && y.1 < line.lo()` takes the
  `continue`: the cell is read as not reaching the pole, though the pole
  vertex is in its reach, near the shared cell.
- The dome is the same: 161,301 `zero_of` `Local`s (the dome's run
  counts both faces' calls; the two-arc sphere's figure counts cells
  where both faces read `Local`). The same `v` top, `1v1` at the pole.

**The missing reading.** A face whose extent stops within the rounding
of its chart's singular latitude still has the pole in its closure. Its
pole vertex is placed by `vertex_at_singular_point`, and its meridians
end there, so the collapsed side `v = π/2` is the face's. Where the
extent's bound and the pole's latitude are within a few ulps, read them
as one, as the sheet's other few-ulps concessions do (`sheets`, "the
touch"), or place the pole off the vertex rather than off the clipped
rectangle. Either way the collapsed side becomes a zero side at the
shared vertex, and the pair has `planes_through`'s great-circle plane.

**Not traced by that lane:** where the stored lift's `v` top comes
from, and whether PR 7086's `face_extent` reads it; both are traced in
the section below. This is the same mechanism in both cavities, and
pre-dates PR 7086: these six rows were out of pairs on 2026-11-13 too.

## Traced and fixed on `embed/pole` (EMBED, lane pole, 2026-11-18)

Main `ff2319654`, f64, the cavity asked through the trace lane's
switch (`origin/embed/cavity-trace-measure`).

**Where the short top comes from.** Nothing rounds it down on the way
to the certificate. It is the meridian's stored pcurve, read exactly:
- the two-arc sphere's cavity stores each northern meridian as
  `Harmonic { u: Angle { phase: 2π or π or 0, winding: 0 }, v: Angle {
  phase: 1.5707963267948963, winding: -1 } }` over
  `t ∈ [±2.9e-18, 0.7853981633974484]`;
- the phase is the polar angle of the edge circle's own frame vector,
  an `atan2` (`geom_brep::pcurve_cache`, the `SphereMeridian` arm). The
  offset's circle starts `2.9e-18` of parameter off the pole, and the
  angle rounds to the `f64` under `π/2`;
- `boundary_of` (`topo::clearance`) lifts each row's two ends as point
  intervals and `extent_of` takes their hull, so the extent's `v` top
  is that phase. `face_extent` returned it unchanged, and
  `present_chart` clipped the cell to it before the pole was read.
- **The south pole was placed all along**, by the same rounding going
  the other way: the row's end there is `phase − π =
  -1.5707963267948968`, the low end of `-π/2`'s enclosure.

So no source has the pole's latitude to the ulp: a meridian's end at a
pole is a rounded value on either side of it. The fact that the face
reaches the pole is the body's, the vertex decided at the pole.

**What landed.** `face_extent`, the one reader of a face's extent,
returns a sphere face's extent reaching the latitude of each pole the
face holds a vertex at (`reaching_its_poles`, by
`vertex_at_singular_point`, the reading the pole's collapsed side
already used). That is the face's fact, read in every cell from the
face's own vertices, and the extent it returns is a superset of the
stored one. No ε concession is added: nothing is dropped, and no bound
is read as the pole for being near it.

**Measured, before and after** (222 tests of the cavity rows' modules,
302 cavities asked before, the same switch):
- **The six rows shell.** The two-arc sphere's cavity reads `Embedded`
  at 526 discharged, 0 refused, 516 splits (it was 45,928 / 19,608 /
  65,526). The domes read 182 / 0 / 169 (tangent, and opening it) and
  228 / 0 / 215 (short of tangent).
- **Nothing else moves:** every other cavity's verdict and receipt is
  identical, the other six valid rows refuse as they did and the two
  crossing pins fail as they did.
- **The tour walk** (`embed_walk`, release, all 85 bodies, main
  against the branch): identical line for line. 82 hold and the three
  bench assemblies cross at their recorded contacts; 6,929 candidates,
  479,866 discharged, 0 refused and 473,162 splits on both.
- **`self_intersection_proves_injectivity` and
  `shell_cavity_embedded`:** 57 of 57 on both, every receipt identical,
  at the default ε, 1e-6 and 1e-12.
- **The endpoint census:** `injective.rs` reads 179 / 79 (177 / 79 on
  main); the two added reads are `reaching_its_poles`' ends of the
  polar latitude.

## The fix pass after review (EMBED, lane pole, 2026-11-18)

The review of PR 7115 reached the cone's apex with a plain built body,
so the reading became one for both kinds, and its premise is checked:

- **One reading of a vertex at a singular point** (`singular_ends`):
  a sphere's poles and a cone's apex, by `vertex_at_singular_point`.
  The apex's own site, `self_intersection_apex`, is retired.
- **`face_extent` checks the stored extent, then moves it.** The lines
  and the nappe are read against the stored extent, as on main. Only
  then is it moved to each singular latitude the face holds a vertex
  at (`reaching_its_singular_points`).
- **It moves only across a rounding.** A stored end farther than
  `chart_slack` from the latitude contradicts the vertex and refuses
  `Structural`. That left a vertex in band of its pole with honest
  rows refused, and a built cone cavity a hundred units off the origin
  with it; the second fix pass below replaces this bound.
- **The extent's end moves for a vertex the band reads at the point**,
  which is one within the band's zero of it, `1e-9` m at the default
  ε. That is the incidence's decision reused, not a concession of this
  reader's own; the earlier section's "no bound is read as the pole
  for being near it" is true of the extent's bound and not of the
  vertex.

**Measured on the merged tree** (main `bb54d3e4d` merged in):
- the cavity rows at f64: the same 8 of 222 fail, and all 307
  cavities' verdicts and receipts are those of the first head;
- `self_intersection_proves_injectivity` and `shell_cavity_embedded`:
  59 of 59 at the default ε, 1e-6 and 1e-12, every earlier receipt
  unchanged; the cone pin holds at 157 discharged, 151 splits;
- a cone sweep (six shapes, 61 apex heights, apex down and up, each
  body and its axial-door cavity): 1,464 of 1,464 hold. On the first
  head 113 of the first 949 refused `Budget(Pairs)`;
- the endpoint census: `injective.rs` reads 177 / 75.

## The second fix pass (EMBED, lane pole, 2026-11-21)

The pass was interrupted from 12:37Z on 18 November until 21 November.

**One scale, the band's** (the orchestrator's ruling, replacing the
slack bound above):
- **The extent moves where its stored end is itself at the singular
  point by the band**: the surface's parallel at the stored end
  latitude, decided by `vertex_at_singular_point` (`parallel_at`). No
  `chart_slack` and no constant of this reader's own. A stored end that
  is not at the point contradicts the vertex, `Structural`.
- **A cone's end past its apex** is on the face's nappe by the same
  decision (`face_extent`'s nappe check).
- **An edge that ends at the collapsed side's vertex runs on to that
  side** (`ends_on_collapse`, read by `run_on`): by the vertex's key,
  and only where the edge's image there is at the singular point by the
  band. This closes the strip between a row's stored end and the
  latitude by topology; `covered`'s slack is not touched.

Two rows this lane filed in the first fix pass are withdrawn from the
branch, their subjects closed here before they reached main:
- the in-band vertex whose rows end at its own latitude (it moves now;
  pinned by `the_extent_moves_for_a_vertex_the_band_reads_at_the_pole`
  and `an_extent_moves_where_its_stored_end_is_at_the_singular_point_by_the_band`);
- a meridian ending an ulp of its far end off the apex, which did not
  cover the apex cell's side (it runs on now; pinned by
  `a_cone_face_whose_extent_starts_a_rounding_off_its_apex_places_the_apex`,
  `a_meridian_ending_at_the_poles_vertex_runs_on_to_the_poles_side` and
  `cones_far_from_the_origin_hold_across_their_apexes`).

**What still refuses**, on its own row: cones in turned frames,
`two-half-cones-sharing-both-rulings-refuse-departure-at-their-apex-in-a-turned-frame`
(P0).

**Measured on the merged tree.** Main moved twice during the pass; each
pair below is one main with and without PR 7115's `injective.rs`.

| | main `dc7059c445` | with PR 7115 | main `94b6b284e` | with PR 7115 |
|---|---|---|---|---|
| cavity rows at f64, failing of 222 | 14 | 8 | 15 | 9 |
| tour walk, holding of 85 | 82 | 82 | 78 | 78 |

- **Cavity rows:** on each main only the six pole rows differ; every
  other cavity's verdict and receipt is the same with and without the
  PR. The fifteenth and ninth failure is main's own
  (`shell8_r2_probes::r2_three_solids_on_one_sphere_chart_open_additively`,
  since the carrier frames landed).
- **Tour walk:** identical line for line on each main. The four bodies
  lost between the two mains (three die bodies and the lily lantern)
  refuse with main's own code.
- **The injectivity suite** at the default ε, 1e-6 and 1e-12: on
  `dc7059c445`, 62 of 62 with the PR and 58 without, the four new
  holding rows red and every other receipt identical.
- **The review's 120 cone tips and cavities** (plain frame, apex
  heights to 101.3, sizes 0.013 to 50): 120 of 120 hold, on both mains.
  Twelve refused on the first head and three on the second.
- **The review's turned frames** (20 cones, 12 ice-cream cones, each as
  a body and a cavity): eight cones refuse on `dc7059c445`, on
  `two-half-cones-sharing-both-rulings-refuse-departure-at-their-apex-in-a-turned-frame`;
  on `94b6b284e` the turned ice-creams' sphere caps refuse as well, on
  `a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval`.
- **This lane's sweep** (six shapes, 61 apex heights, apex up and down,
  body and cavity, plain frame, within six units of the origin): 1,464
  of 1,464, at the receipts of the first fix pass.
- **The endpoint census:** `injective.rs` reads 180 / 78.

**Re-measured on main `c3ffedeb2`** (EMBED, lane pole, 2026-11-22), with and
without PR 7115's `injective.rs`:
- f64 cavity rows failing: 15 of 221 without, 9 with; only the six pole
  rows differ, every other cavity's verdict and receipt the same;
- tour walk: 78 of 85 on both, identical line for line;
- with the PR: the injectivity suite 64 of 64 at the default ε, 1e-6
  and 1e-12; the review's 120 plain-frame cone cases 120 of 120; its
  turned-frame sweep 47 holding and 21 refusing (five turned cones and
  the sixteen turned ice-creams); this lane's sweep 1,464 of 1,464 at
  the first fix pass's receipts;
- 25 mutants, each over the 118 unit rows and the 64 body rows: all die.

**Final pass, on main `ae076e050`** (PR 7120's boundary boxes and the
carrier frames both in), with and without PR 7115's `injective.rs`:
- f64 cavity rows failing: 15 of 221 without, 9 with; only the six pole
  rows differ. PR 7120 itself moved 17 other cavities' receipts, the
  same with and without this PR.
- tour walk: 78 of 85 on both, identical line for line;
- with the PR: the injectivity suite 64 of 64 at the default ε, 1e-6
  and 1e-12; the review's 120 plain-frame cone cases 120 of 120; its
  turned-frame sweep 46 holding and 22 refusing (six turned cone cases
  on the P0 row, sixteen turned ice-creams on the seam-meridians row);
  its far-body probe 23 of 24 against main's 17.

**Re-measured on main `a3d5ce89c`** (2026-11-22, 577 commits on from
`ae076e050`, none to `injective.rs`): every number above is the same.
f64 cavity rows failing 15 of 221 with main's `injective.rs` and 9 with
this one, the six pole rows the only difference and every other
receipt unchanged from `ae076e050`; tour walk 78 of 85; the injectivity
suite 64 of 64 at the default ε, 1e-6 and 1e-12; 119 unit rows; the
review's 120 hold, its turned set 46 hold and 22 refuse, its far probe
23 of 24, each refusal at the receipt it had on `ae076e050`.

**On the landing train** (`origin/embed/wave3-land` at `f06547ccf`,
merged in on 2026-11-23, with `embed/apex-separator`'s plane through a
pair's one shared vertex): the turned balls and the sixteen turned
ice-creams hold, so the turned-ball refusal pin is a holding assertion
now; the review's turned set reads 62 holding and 6 refusing (the six
cone cases of the P0 row), its far probe 23 of 24. The mirrored cone
wedges that needed this PR's run-on and that plane together are
embedded in thirteen of fourteen poses at both scalars
(`work/embed/a-mirrored-cone-wedge-refuses-departure-at-its-apex-off-the-plain-frame.md`).

**On the final tree** (the landing train at `3e433e6786`, with lune's
poses and remeter's margin, merged in on 2026-11-23): 171 `injective::`
unit rows; 90 body rows at the default ε, 1e-6 and 1e-12; the endpoint
census green with `injective.rs` at 197 / 95; all 29 mutants die on the
unit rows (the body rows were not run under a mutant on this tree).
