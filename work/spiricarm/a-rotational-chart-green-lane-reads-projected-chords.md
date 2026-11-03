---
id: a-rotational-chart-green-lane-reads-projected-chords
kind: unit
title: One rotational-chart Green lane for faces trimmed by a projected chord, stage 1: the cylinder chart
status: dispatched
opened: 2026-11-03
priority: P0
cost: H
branch: spiricarm/rotational-lane-cylinder
refs: [a-projected-boundary-on-an-analytic-chart-has-no-volume-lane, a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane, a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses]
---


Stage 1 of 3 of the decide on `spiricarm/chord-volume-decide` (2026-11-03). Stage 2 is the sphere chart, which closes `a-projected-boundary-on-an-analytic-chart-has-no-volume-lane`. Stage 3 is the torus chart, which closes `a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane` and the P2 spiric-beside-chord row. This stage closes neither P0 row by itself: every cylinder × sphere and donut × rod result the decide probed holds chorded faces of both charts. Its value is the lane's shape, plus the cylinder × cylinder rows if premise (h) holds.

## What is true today (measured by the decide; re-check, do not trust)

- `topo::props::quad_lane::cut_face_lane` routes a trimmed face as follows:
  - a spline chart goes to `nurbs_face`/`trimmed_face`, which take `Pcurve::General` only and refuse `Fitted`/`Projected` by name;
  - a plane, a cone, or a torus carrying a spiric goes to `rim_face`;
  - any other non-cylinder chart refuses "conic trim on a cone/sphere/torus chart". That message is a misnomer: every face measured there had only `Nurbs`/`Projected` chord edges;
  - a cylinder passes two gates, `Pcurve::harmonic_form` on every edge and then `harmonic_carrier` (which refuses every `Curve3::Nurbs`), before reaching `quad::cylinder_cut_face_rounds`.
- The cylinder lane (`quad/cylinder.rs`, `A_s = −Σ∮(v − v_a)du`, `flux = r²A_s + o·A⃗`) and the spiric wall (`props/spiric.rs` `spiric_wall`) are two spellings of one form. On a rotational chart the flux and area densities depend on `v` alone, so a face's integrals are `−Σ∮F(v)du` and `−Σ∮G(v)du` plus a chart term. Each spelling has its own edge loop, closure and winding reading, and pad.
- A projected chord's image (`ProjectedImage`: `carrier()` a `FramedCarrier::Net` or `Circle`, `chart()`, `deck()`, `breaks()`, `azimuth()`) carries what an integrand needs. On the cylinder, `v = v_off + v_sign·z` and `u′ = (xy′ − yx′)/(x² + y²)`, all rational in the carrier. The machinery that integrates along a 3-D carrier is the curved-rim Taylor rule (`props/taylor.rs` `Jet`, `composite_by`, `recip`; `props/rational_rim.rs`'s Bézier span forms), not the spline-chart lanes.

## The final state this stage builds

**One rotational-chart Green lane:**
- a per-chart `F`/`G` table, with the cylinder as this stage's only entry;
- an edge-term enum covering a harmonic edge, a projected `Net`, and a projected `Circle`, with rims and meridians as exact fast paths;
- one closure and winding reading;
- the existing harmonic cylinder arithmetic moved onto it, so that one cylinder lane remains (`[simple.uniform]`: no second copy beside `quad/cylinder.rs`).

The spiric wall's migration onto the lane is stage 3's; shape the edge-term enum so it can take it.

- Both cylinder gates open for projected chords. A projected edge's end values are read through `image.eval` at the lane scalar, as `spiric_wall`'s `ends` already do. Its `Δu` comes by quadrature, with its envelope pad.
- The lane's pieces refine the image's `breaks`, so that the sector bounds (ρ, the pole distance and the tube distance kept off zero per piece) hold. This is premise (e): check it, don't assume it.
- Off-question drive-bys that fall in the fence:
  - rename the "conic trim …" refusal to say what it is, a trimmed face with no lane on its chart, and re-pin the `starts_with` sites (`torus_doors_join.rs`, `cylinder_sphere_frame.rs` `door`, `spiric_roundtrip.rs`, `demos/tour/src/teapot.rs`);
  - fix `harmonic_carrier`'s stale `Nurbs` message;
  - check `cut_face_lane`'s comment on a sphere's general circle, which may be stale.

## Phase 0 (report before building the lane)

- **(h)** Do the cylinder × cylinder results hold only cylinder and plane faces? The rows to check are `verbs_germarms2::a_rod_through_a_drum_wall_reaches_the_volume_lane`, `the_fenced_poses_keep_their_own_outcomes`, `tang_circle_cylinder::a_tilted_rod_through_a_rim_reaches_the_volume_lane` and `verbs_cylcyl_r1_review_probes::every_reachable_crossing_pose_refuses_typed_or_answers_correctly`. Use the decide's `SPIRICARM_PROBE` dump, cherry-picked from `spiricarm/chord-volume-decide` at `1da971d5f4` and not merged as-is.
- **(f)** The chord's certified deviation from the true section, against `VOLUME_CLAIM = 1e-7`. This decides whether a true-geometry slicing oracle can witness containment, or whether only the carrier-swap fixtures can.
- If (h) fails, the stage still lands on the carrier-swap fixtures. Say so.

## Oracles (containment, never narrowness)

- **Carrier-swap fixtures.** Build a cylinder face with one boundary carrier rewritten as a NURBS that represents it exactly: a ruling as a degree-3 line, a rim circle as a rational quadratic. Each then mints `Projected`, and its enclosure must contain the harmonic lane's exact answer. Cases:
  - a chord loop that winds the cylinder;
  - `v_sign = −1`;
  - a chord across the seam;
  - a chord longer than a quarter turn;
  - mixed harmonic and projected loops;
  - the body translated by 1e3.
- **An absolute z-slicing oracle** for every cylinder × cylinder row that goes SOUND.
- **Inclusion–exclusion is NOT admissible.** A chord bounds complementary faces with opposite traversal, so a consistent chord-term error cancels out of it.

## Rows that move

- **Go red, re-pinned to their next stop** (Sphere, the renamed trim stop, or Ring), never to SOUND:
  - `cylinder_sphere_frame`'s 23 `Door::Projected` runs;
  - the three `torus_doors_join` rows at `Projected`;
  - 10 seam-region door stops;
  - `conic_edge_curved_face::a_rim_crossing_reaches_the_join`;
  - `germ_tilted_rod`'s `"projected volume"` cells.
- **Pin by face order is a known defect.** A `Door` or `Stop` pin records whichever chorded face the validator reads first. Where a pin moves without the body changing, re-pin it to a reading that does not depend on face order, or say why not.
- **Go SOUND, if (h) holds:** the cylinder × cylinder rows above, against the slicing oracle. The P1 march-cost row then loses its refusal half. Update that row's body: it becomes a pure cost row, and its "decide before marching" fix retires.

## Coupling

- PR 6460 (`onecopy/face-loops`) touches `topo/src/props.rs` mechanically. Merge main after it lands.
- QUAD shares `quad/cylinder.rs`, `quad_lane.rs` and `props/spiric.rs`. Announce the seam in the PR body (`python3 scripts/work.py territory --base origin/main`).

Review tier: **dual** (`docs/DUAL-REVIEW-PROTOCOL.md`). The lane's shape is an architectural decision that two further stages build on, and a wrong edge term is the error that reads as an improvement.
