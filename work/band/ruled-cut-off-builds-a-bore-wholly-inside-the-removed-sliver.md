---
id: ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver
kind: issue
title: blend: a feature wholly inside the material a band removes or fills goes with it (Ev, PR 4285); today a bore inside a ruled cut-off's sliver refuses RingClearance
status: closed
opened: 2026-09-25
priority: P2
cost: H
closed: 2026-10-19
---


## Ruled (Ev, PR 4285, 2026-10-08)

Build it. A feature wholly inside the material a fillet or chamfer band
removes (convex) or fills (concave) goes with that material, as a feature
wholly inside a Boolean's tool goes with the tool; a feature crossing it
still refuses. Predicate 2 reads each face three ways on one exact region
(clear / wholly inside / crossing), and the verdict is logged so a later
reader of a consumed name resolves `Vanished` with the blend's verdict as
the cause. No `Consumed` record in the blend's naming "at least until
something needs it". C8 says face clearance reads three ways. Silent at
the verb; a consumption lint over the blend and the Boolean as one rule is
INTENT's (`work/intent/a-feature-an-operation-consumes-has-a-lint.md`).

Build order: (1) the three-way classifier, "inside" still refusing; (2)
the verdict log; (3) the deletion, from the ruled cap (flips the two bore
rows); (4) the planar band, the concave fill, the chamfer.

## The question

A convex ruled crease's cut-off removes a sliver `S` from each
transverse cap. A bore whose walls lie wholly inside `S` refuses
`RingClearance` at the cap today (`ring_clearance_pass` arm (c),
`crates/sweep/src/blend/surgery.rs`; `CapSliver`,
`crates/sweep/src/blend/open/end_face.rs`). Rows:
`band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_refuses_ring_clearance`
and
`review_band_ruled_ring_probes::a_bore_in_a_keyhole_creases_removed_sliver_refuses_ring_clearance`.

The alternative is to build it: the bore dies with the sliver, its two
rings, the walls between them and their names gone, at
`ΔV = −2·A·L + V_bore`. That is a blend deleting an authored
through-feature, which no blend does today. Whether one may is the
open question, and four things turn on it:

1. **Intent (D10).** The author placed the bore. A fillet that erases
   it without a word is the silent outcome fail-loud exists to
   prevent, unless something reports it. D10 routes known-answer
   complaints to lints, so a third answer is to carve and report a
   finding ("the blend consumed the bore").
2. **Naming.** N5 already resolves a gone name to `Vanished`, so
   nothing dangles silently. But it is a new way for a name to
   vanish: a later step reading the bore's wall would break on an
   upstream radius edit, not on an edit to the bore.
3. **Predicate 2.** The reach meter meters the band's volume against
   every non-support face, and the bore's walls lie inside that
   volume. It would have to learn which faces die with the sliver,
   which changes what predicate 2 certifies.
4. **Exact membership.** Deleting the bore needs an exact "wholly
   inside `S`" test, a Q1 trilean with its own margin, and the
   refusal split into "straddles the arc" (a frontier: the band would
   be trimmed by the bore) and "wholly inside". The enclosure `Ω` the
   meter reads cannot serve: it contains `S`, so a ring inside `Ω` may
   still cross kept material. Its surgery is likely `kfmrh`/`kef` over
   the bore's walls.

`crates/sweep/README.md`'s ruled-band clause ("every other edge of
the cap stays where it was … an edge not definitely clear of the
region refuses `RingClearance`") came from a BAND fix pass, not from
Ev (`git log -S`), so it binds neither answer; a carve would re-word
it.

**Recommendation (PR 4271's lane, both reviewers concurring):** keep
the refusal. An author who wants the bore gone deletes it in one edit,
and the refusal's sentence already says the edge lies in the material
the blend removes. If the carve is wanted, it comes with the finding
in (1), predicate 2 exempting the dying faces (3), and the exact
membership test (4).

## Done

- **An edge that leaves `Ω` through different faces** (PR 4271). A
  straight cap edge is read point by point against `Ω`
  (`CapSliver::line_clearance`), at `f64` and at `Interval`. Three
  rectangular holes in the D-rod and the keyhole at `r = BR` and
  `1.1·BR`, all refused before, carve at their closed forms. Curved
  edges are still read term by term:
  `cap-sliver-meter-reads-a-curved-edge-term-by-term`.
- **The `V − c` half-plane's wedge near each foot** does not bite: the
  floors on the section's own axes (PR 4173) close it.
  `band_ruled_cap_ring::a_bore_on_the_arc_just_past_a_foot_carves_at_the_closed_form`
  pins it.
- **The keyhole's slot end edge** (the rocker's wall 2) carves past
  `r* = 0.3097` since PR 4173.
- A D-rod bore and five straight-edged holes clear of `S` still refuse
  the reach meter's `FaceClearance`:
  `blend-reach-refuses-a-bore-clear-of-a-ruled-cut-offs-sliver`.

## Findings (step 1: the three-way classifier)

- Built: predicate 2's reach (`band_reach`) and the cap meter
  (`ring_clearance_pass` arm (c)) read three ways; "wholly inside"
  refuses the new `BlendError::WhollyInside`. The cap's enclosure `Ω`
  answers only "clear"; inside and crossing are read against the
  sliver's own boundary (`CapSliver`'s `SliverBoundary`, round ends
  only — a chord end answers clear or a bound).
- The two witness rows are renamed for what they now pin:
  `band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_refuses_as_a_bound_at_its_wall`
  and
  `review_band_ruled_ring_probes::a_bore_in_a_keyhole_creases_removed_sliver_refuses_as_a_bound_at_its_wall`.
  Step 3 flips these two once the reach reads the cap plane non-strictly
  (below); without that, step 3 flips no bore row.
- **The verdict is the request's, not the first face's** (PR 4797's
  fix pass). Both reviews found the first version first-hit: the
  reach returned at its first face that was not clear, arm (c) at its
  first cycle, and arm (c) ran before the reach. So a feature crossing
  the band was refused `WhollyInside` when the first face or ring read
  happened to lie inside: an L-shaped void, a tilted through-slot, 18
  box voids of a scan, a D-rod cap carrying an inside bore and a
  crossing bore. The reach and arm (c) now feed one verdict
  (`reach::Unclear`). A crossing anywhere refuses as a crossing, then
  a bound, and `WhollyInside` comes only when every face and cycle
  that is not clear reads inside, in any order. "The feature" is the
  request's whole set of readings, not a connected feature. The two
  agree on every refusal. A connected feature with an inside face and
  a clear face has a face between them that reads crossing or a bound,
  since the shared edge lies in both. A separate crossing or bound
  refuses the request whichever way it is grouped. Grouping by
  connected feature matters only when something is deleted, in step 3.
- **A wall that meets a window's end plane reads as a bound, and so
  outranks its rings.** That covers a through-bore's wall, or any wall
  of a cap ring, which runs to the cap. For an exact core, the wall
  touches the core's cap half-space, and the strict cell walk cannot
  certify it inside. The ruled crease has no exact core, so its reach
  reads only clear or bound. Every bore wholly inside a sliver
  therefore refuses `FaceClearance` as a bound at its wall today. That
  includes both witness rows, renamed
  `…_refuses_as_a_bound_at_its_wall`, the planar band's square-wall
  and slanted-wall holes, and the through-slot along a block's edge.
  `WhollyInside` names a face (an interior void, an island) and never,
  so far, a cap ring. The cap's inside reading and its depth are
  pinned at unit level (`blend::open::end_face` tests). **Step 3's
  consequence:** the cap (`CapSliver`'s sliver `S`) and the reach (the
  core's slice at the cap plane) spell the region twice, and nothing
  ties them together. Before anything is deleted, both have to agree
  on what dies. A wall that ends on a cap reads inside only if the
  reach reads the cap plane non-strictly, as the closed form of the
  face's extent past it rather than cells. The ruled crease's reach
  also needs a prism reading (or an exact core) for a wall parallel to
  the spine. Without that, step 3 flips no bore row.
- **The README** (`crates/sweep/README.md`, predicate 2 and the ruled
  cut-off) now reads true of today's code. The ruling's content stays
  in the present tense as what the consuming carve does, and today's
  refusal and the aggregate verdict are stated beside it (Ev on PR
  4285: "probably it should be rephrased if the current state would be
  misleading").
- The chamfer refuses ruled creases (`ChamferArmUnsupported`), so the
  ruled cap is fillet-only; the second verb is pinned on the planar
  band's square wall (inside under the fillet, a bound under the
  chamfer's chord end).
- `band_planar_oblique_fillet`'s "cusp at the top foot" hole was
  listed as in the sliver; it crosses the elliptic arc and now reads
  as a measured crossing.
- Sibling meters still two-valued:
  `blend-support-ring-meters-read-two-ways`.

## Open for steps 2–4 (PR 4797's confirming review, 2026-10-12)

- **Which face is named on a margin tie depends on edge order.**
  `reach::Unclear` keeps the first reading on equal `hi`: one body in two
  request orders names `Face(7v1)` or `Face(13v1)`. The kind and margin
  are order-independent. Step 2 logs the verdict as the cause of
  `Vanished`, so it decides whether the named entity must be canonical.
- **"Wholly inside" is read per band, not over the union of bands.** A
  void wholly inside the convex union of a mitre's two bands, crossing one
  band's region, reads a crossing. That is conservative today; the ruling
  ("wholly inside the material a fillet or chamfer band removes") does not
  say whether a feature inside the union goes with them. Steps 3–4 meet
  it at mitres and corners.
- **Unprobed:** a face not clear for one (non-exact) reach but wholly
  inside another exact reach records inside and drops the first reach's
  bound (`reach.rs` `band_reach`, the `find_map … continue`). Sound by
  union for one convexity; a mixed-convexity overlap at a shared vertex
  was not built.
- **The cap's inside path cannot decide an end-to-end verdict today**
  (every cap ring's wall reads a bound first). It is pinned by unit rows
  only, and step 3's non-strict cap-plane reading may reshape it.

## Findings (step 2: the verdict log) — stopped at a fork

- **The verdict is already logged.** Predicate 2's readings, the three-way
  ones included, are `k_stats` decisions under `fillet3_face_clearance`
  (`blend::classify` → `k_stats::decide`), and the cap's under
  `fillet3_ring_clearance`. They land in the blend node's verdict log
  (`NodeValue::verdicts`), which is what N5's `PredicateFlip` is computed
  from (`resolve/vdiff.rs`). A face the blend carries through is named
  `From { read, of }` at the blend node (`names/emit_blend.rs`), so the
  blend is on its derivation path. Once step 3 builds a consuming carve, a
  name that resolved last run and is consumed now resolves `Vanished {
  PredicateFlip { fillet3_face_clearance, Positive → Negative } }` with no
  new code. That is the Boolean's path: a feature inside its tool vanishes
  on its classification's flip, as a split piece does (N1, "N5 diagnoses
  `PredicateFlip` from the split's recorded classification").
- **What the brief asked for beyond that is a design choice.** A cause
  naming the feature, the band and the depth needs a new `Diagnosis` arm.
  N5's list of arms is ratified (#74, `crates/editor-core/src/names/README.md`),
  and the ruling doesn't make that change. It also needs a record (on
  `Blended`, per face or per feature) whose difference from "a `Consumed`
  record in the blend's naming" the ruling doesn't draw.
- **There is no per-refusal N5 mapping to fill.** A refused blend leaves
  its names `Indeterminate` (`resolve`, step 4, the minting node's
  standing), whatever the `BlendError` variant. `WhollyInside` is carried
  through `NodeErrorKind::Blend` like every other refusal. Nothing
  `ReadGone`-shaped exists, and no test-support seam injects a verdict.
- **The canonical face on a tie** (NOTE 1 above) bears only on
  `WhollyInside { at }` under the verdict-log reading, since
  `PredicateFlip` names no face. It is still open, and small
  (`reach::Unclear`: break an equal-`hi` tie by `EntityId` order).
- **What the verdict-log reading cannot say:** with no prior run (a
  reference written after the feature was already consumed) the ladder has
  no flip to cite and falls to `cause_not_in_evidence`. The population
  diff's blind spot (a consumed face and a face freed in one edit cancel)
  applies here as it does to the Boolean's.
- **Adjudicated (orchestrator, 2026-10-12): reading (A), no new record.**
  The ruling says "the verdict is logged so a later reader of a consumed
  name resolves `Vanished` with the blend's verdict as the cause", and "no
  `Consumed` record … at least until something needs it". The blend
  node's verdict log is that log, and `Vanished { PredicateFlip }` is
  that resolution, the same path a feature inside a Boolean's tool takes.
  So step 2 adds no `Diagnosis` arm and no record on `Blended`; a cause
  naming the feature, band and depth waits until something needs it (the
  INTENT consumption lint is the likely first reader). What remains of
  step 2 rides step 3, where a consumed name first exists: (i) a row that
  a name resolved before the carve resolves `Vanished { PredicateFlip {
  fillet3_face_clearance, … } }` after it; (ii) `reach::Unclear` breaks an
  equal-`hi` tie by `EntityId` so `WhollyInside { at }` names one face
  whatever the edge order. The no-prior-run blind spot is the Boolean's
  too, and is recorded, not built.

## Findings (step 3: the deletion, from the ruled cap)

- **Built.** A feature wholly inside the material a convex ruled band's
  cut-off removes goes with it: the carve takes its faces, edges and
  vertices, and its cycles leave the caps. The two witness rows build at
  their closed forms, `ΔV = −2·A·L + π·a²·L`, at tiers 3 and 3′, with
  naming totality and the bore's walls in `BlendNaming::dead`:
  `band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_the_closed_form`
  (and `…_at_interval`, and the two inside bores of
  `…_a_hair_across_the_slivers_boundary_crosses_and_one_inside_it_goes_with_it`),
  and
  `review_band_ruled_ring_probes::a_bore_in_a_keyhole_creases_removed_sliver_goes_with_it_at_the_closed_form`.
  Every crossing row still refuses as a crossing.
- **Prerequisite 1, the wall at the cap plane.** The ruled crease's reach
  has no exact core, and no cell walk certifies a face that touches the
  window's end plane. So the reach reads a convex ruled band's faces
  through its caps instead (`reach::prism_inside`). The band's material
  is the prism over its cross-section along the spine, between the cap
  planes. A face on a surface ruled along `a` (a cylinder's axis, or a
  plane's direction nearest the spine `τ`), whose every edge is an edge
  of a cap cycle read inside or a line between two of their vertices,
  has each point on an `a`-line between two boundary points. So it
  projects into the cross-section when the cycles' least depth, times
  `|n_cap·τ|`, exceeds the lines' `|w − u|·|d × τ|` plus the face box's
  diagonal times `|a × τ|`. It lies between the cap planes as a convex
  combination of boundary points that do. That is the cap plane read
  non-strictly, in closed form rather than cells.
- **Prerequisite 2, `S` and the core's slice.** The prism's slice in a
  cap's plane is exactly `CapSliver`'s `S`: the end curve is the band's
  surface in that plane, and the rim pieces are the supports'. The
  reach's inside reading is computed from the cap meter's inside depths,
  so the two agree on what dies by construction; there is no second
  region to tie.
- **Prerequisite 3, the feature.** The faces read inside are grouped by
  the edges they share (`consume::plan`). A feature the carve takes is a
  through-feature between the band's two caps: every edge of its faces
  joins another face of it, or a cap cycle read inside, and it is
  bounded by exactly one such cycle on each of two caps. Its faces are
  each bounded by one cycle and together form an annulus
  (`V − E + F = 0`), so with the plug they merge to a disk whose inner
  edges are a tree. Anything else refuses `WhollyInside` before any
  mutation. That includes a seamless wall (one face, two cycles), which
  `kef` could not merge. No door builds such a wall today: an extrude or
  a Boolean imprints a seam. A one-edge bore (one full-turn arc per ring,
  one seam line) is an annulus of one face, and it goes at any seam
  azimuth
  (`band_ruled_cap_ring::a_one_edge_bore_inside_the_sliver_goes_with_it_at_any_seam_azimuth`).
  A blind bore refuses as a bound
  (`band_ruled_cap_ring::a_blind_bore_inside_the_sliver_refuses_as_a_bound`). Grouping is sound because
  the verdict is inside only when every reading not clear is inside, so
  an inside face shares no edge with a clear one. Only inside readings
  from a convex ruled cap (`reach::CapRing`) or the prism reading are
  consumable. Any other (an exact core's cells, a planar cut-off's cap,
  a concave ruled cap) keeps the request refusing `WhollyInside`.
- **The carve** (`blend/consume.rs`), on the clone before the band:
  `mfkrh` promotes one cap's cycle to a plug (ending the hole); the plug
  and the feature's faces `kef` into one; its spurs `kev` leaf first;
  that face `kef`s into the other cap, whose cycle's edges `kev` down to
  a lone vertex; a strut `mekr`s it to the cap's outer cycle and `kev`
  retires both. The face-destroying door `SourceFaces::kef_minted`
  admits the consumed faces while the features are carved, and no
  source face after that (`SourceFaces::after_consuming`). `Retired`
  gains a `faces` channel. The
  surgery postcondition, `assert_naming_totality`, and editor-core's
  emitter guard all read it.
- **Step 2's remainder.** `reach::Unclear` breaks an equal-`hi` tie by
  the entity named (kind, then key), for bounds and inside readings
  alike (`reach::tests::a_margin_tie_names_the_least_entity_in_either_order`).
  The document row:
  `edit_ruled_carve::a_bore_a_radius_edit_puts_inside_the_sliver_vanishes_on_the_blends_verdict`.
  On the bored D-rod, a fillet of radius `0.01` carries the bore's wall
  through. The edit to `ROD_FILLET` puts the bore wholly inside the
  sliver, and the wall's name resolves `Vanished { PredicateFlip {
  fillet3_face_clearance, Positive → Negative } }`.
- **The prism reading's soundness terms are pinned.** The line slack and
  the `diagonal·|a × τ|` term are held by two tilted bores, inside the
  sliver at both caps but through the kept ball between them:
  `band_ruled_cap_ring::a_tilted_bore_inside_at_both_caps_through_the_ball_mid_length_refuses`
  and `…::a_thin_steep_bore_inside_at_both_caps_through_the_ball_between_refuses`.
  With both terms deleted, each of them builds, and every other blend row
  stays green. For a plane, `|a × τ|` is `|n·τ|`, which is defined for a
  plane normal to `τ` too.
- **Still open, for step 4:**
  - The planar band (its exact core reads the wall at the cap as a
    bound), the concave fill, the chamfer, and inside-the-union-of-bands
    at mitres and corners. C8's sentence on where the carve takes a
    feature (`crates/geom-brep/README.md`) names only the convex ruled
    cut-off, and is widened as each of these lands.
  - The prism reading takes the least depth over all of a crease's
    inside cycles, not just the face's own. That is conservative: a
    tight face beside a shallow ring of another bore reads a bound.
  - **Any tilted bore refuses as a bound, however small the tilt.** The
    line slack `L·sinθ` dominates depths near `1e-3`: a lateral run of
    `0.0098` over `L` (about 0.56°) already refuses. This is sound but
    coarse. A per-station reading along the line would take it.
  - **Tilted caps are mostly refused.** On a rod cut by a tilted plane
    (`|n·τ| < 1`), 0.5 rad about `(1, 1, 0)` is consumed at the unbored
    rod's carved volume. 0.3 rad about `x` refuses as a bound at the
    wall, and −0.35 rad about `y` refuses `RingClearance` at the elliptic
    ring.
  - **The oblique-cap factor and the off-spine plane term have no row of
    their own.** Every committed fixture has caps normal to `τ` and
    planar walls along `τ`. The plane normals are now normalised where
    they are read, so a non-unit stored normal no longer scales the
    depth.
  - **In the escalate zone the refusal reports the cell walk's margin.**
    A bore within `1e-9` of the arc refuses `FaceClearance { bounded:
    true }` at its wall, with `margin ≈ −4.7e-3`. That margin comes from
    the reach's cell walk. The prism excess that failed to be sure is
    about `1e-9`. Reporting the deciding depth means choosing which of
    the two readings a bound names.
  - **The tie-break is canonical only across entities.** `displaces`
    keeps the first reading when two readings have equal `hi` and name
    the same entity (one face against two reaches, for example), so the
    refusal's `chain` and `margin.lo` can depend on read order. `at` does
    not. `entity_order` hand-rolls an order on `topo::EntityId`, which
    topo could derive.
  - **The prism reading reads the cap cycles found so far**
    (`Unclear::cap_rings`). That depends on `ring_clearance_pass`
    running before `band_reach` in `blend_surgery`, and nothing but call
    order holds it.

## Findings (step 4: the planar band, the chamfer; the concave fill split out)

- **(a) The planar band, built.** The plane–plane band's convex cut-off
  now carries its crease into the cap meter, as the ruled one does
  (`surgery.rs`, the `CapMeter` list), so a cycle read inside its sliver
  is a `CapRing` and the prism reading (`reach::prism_inside`) reads the
  feature's walls through it. Nothing else was needed: the band's
  material is the prism over its section along the spine between the
  cap planes (both supports and the band are ruled along it), the
  prism's slice in a cap's plane is the cap meter's sliver, and the
  carve (`consume.rs`) is the ruled one's. The exact core's boundary
  reading still runs first, so a wall with a sampled point in kept
  material refuses as a crossing before the prism reading is asked.
  Rows (`crates/sweep/tests/band_planar_consume.rs`): a bore at scales
  `1e-2`, `1`, `1e2`; a square slot (plane walls, lines between ring
  vertices); leans `s = 0, 0.3, ±1` (elliptic rings on oblique caps); the
  certified scalar; all at `ΔV = −(A − A_feature)·L`, tiers 3 and 3′,
  naming totality, walls in `BlendNaming::dead`, genus 0. Crossings: a
  hair short of the arc and on it refuse `RingClearance` measured, a
  hair past it goes; a blind bore refuses as a bound; a tilted bore
  inside at both caps whose wall dips `a` into the ball mid-length, its
  seams kept in the sliver so the exact core samples no crossing,
  refuses as a bound, and builds with the line slack and the
  `diagonal·|a × τ|` term deleted (checked by deleting them).
- **(c) The chamfer, built.** A chord end's sliver is the triangle of
  the old vertex and the two feet (the rims are lines; a chord end whose
  rims are not reads as before). `CapSliver` reads it through its three
  sides: a point's depth is its least distance inside them, a point past
  one is an outside witness, and a cycle is inside when each piece's
  least reach along each side's inward normal (`piece_along`) is
  positive, the least of those its depth — exact for a convex region,
  where the sliver meter's round end needs the first-point and
  rim-clearance argument (unit row
  `end_face::tests::a_chord_end_reads_a_cycle_through_its_triangle`).
  The chamfer's material is a triangular prism, convex, so the prism
  reading owes no slack there (`prism_inside`'s `convex`, true when
  every ring was read through its cap's exact triangle, `CapRing::convex`;
  the prism's slice in a cap's plane is an affine image of its
  section, so the triangle certifies the section convex): every point
  of a qualifying wall is a convex combination of boundary points whose
  projections lie at least the least depth inside the triangle, and the
  distance from a convex region's boundary is concave inside it. That
  takes the tilt conservatism for the chamfer: the fillet row's tilted
  bore, inside the chamfer's triangle at both caps with a lateral run
  twenty times its depth, goes at `ΔV = −(2A − π·a²·ℓ)`
  (`band_planar_consume::a_tilted_bore_inside_the_chamfers_prism_goes_with_it`).
  No crossing can be built for a chamfer from rings inside at both caps,
  by the same convexity. The rows above run under both verbs; the hair
  rows have a chord twin. One existing row moved:
  `band_planar_cut_off_meters::a_hole_in_the_end_face_sliver_refuses_on_either_side`'s
  chamfer reads each hole's ring inside the triangle now (the convex
  hole is blind, the concave one runs into the die), so both refuse as
  a bound at the hole's wall (`FaceClearance { bounded: true }`) rather
  than `RingClearance`; still refusals. The review rows taken in
  (`band_planar_consume`): a bore inside the sliver at 60° and 120°
  under both verbs at `ΔV = −(A(θ) − π·a²)`; hairs either side of the
  chord at 45°, 60°, 120°, 135° and of the arc at 60°, 120°; hairs
  either side of the chord at oblique caps, which alone pin the
  triangle's outside witness (there `Ω`'s floor runs past the chord);
  a C-section whose bore lies in kept material past a rim inside `Ω`,
  which alone pins the triangle's two rim sides; a tilted bore leaving
  the chamfer's prism in three poses; and the chord's hairs at
  `Interval`. A turn's overrun sliver is read through the same triangle
  and keeps no crease (that triangle is a bound larger than the
  overrun): `band_turn_overrun_chamfer::a_slot_through_the_overrun_is_never_consumed`
  goes red if it is given one.
- **Naming.** `editor-core` `edit_planar_carve`: a bore clear of a
  `0.005` blend is carried through, and after a size edit to `0.1` its
  wall's name resolves `Vanished { PredicateFlip { …, Positive →
  Negative } }` on one of the two clearance verdicts the consumption is
  logged under. The one reported is the first net flip in predicate-name
  order (N5's population diff): `fillet3_face_clearance` for the fillet;
  for the chamfer `fillet3_ring_clearance`, because the chamfer's
  smaller reach box at `0.005` never meters the bore's walls, so face
  clearance only gains verdicts and has no net flip — a population
  artefact, not a property of the consumption. The row holds the cause
  to the two.
- **(b) The concave fill: a question, now its own item.** What the
  fill does with a hole whose mouth it seals, with the blended solid's
  own protrusions standing in it, and with an island wholly inside it
  is weighed in `a-concave-fill-and-a-whole-shell-inside-a-bands-region`
  (`needs_ev`, PR 4963). Today a sealed mouth refuses as a bound at the
  hole's wall
  (`band_planar_consume::a_hole_whose_mouth_lies_inside_a_concave_sliver_refuses_as_a_bound`,
  and `band_planar_cut_off_meters::a_hole_in_the_end_face_sliver_refuses_on_either_side`'s
  concave chamfer), and an island refuses `WhollyInside`
  (`band_reads_three_ways::an_island_wholly_inside_a_concave_bands_fill_refuses_wholly_inside_under_both_verbs`).
- **Owed on the convex side: a void wholly inside the material a convex
  band removes.** That is the ruling's convex half, not a question; it
  refuses `WhollyInside` today under any straight band
  (`band_reads_three_ways::a_void_wholly_inside_a_convex_bands_material_refuses_wholly_inside_under_both_verbs`)
  because no carve kills a whole shell. So (a) and (c) are complete for
  features attached by cycles on the caps, and voids still refuse. The
  carve: merge the void's faces across their edges (`kef`) to one face
  whose edges are a tree, retire the tree (`kev`) to a lone vertex, and
  `kfmrh` it into a face of the outer shell as a lone-vertex ring that
  goes as `consume` ends (`mekr` strut, `kev`); `plan` admits a feature
  that is a whole shell with every face read inside, and `Retired`
  needs a shell channel. The door's shape is shared with the island's
  question in the item above.
- **The step-4 owed list, as taken.**
  - *Tilt conservatism*: taken for the chamfer (above). Not for the
    fillet: its section is not convex, and a per-station reading along
    each line would be the refinement; sound as it stands.
  - *Tilted caps / the oblique-cap factor*: the lean rows exercise the
    `|n·τ|` factor at `s = 0.3, ±1` under both verbs. No crossing row can
    pin it: a wall that dips into the kept ball between rings at
    projected depth `δ⊥` needs a lateral run of at least
    `2·√(2rδ⊥)`, which the line slack already charges, and of a chord's
    two ends one lies towards each support, so the least ring depth over
    the crease has one end whose nearest boundary the cap's tilt does
    not stretch. The factor is the derivation's soundness term, and is
    kept without a red row; `prism_inside`'s doc says why (it binds only
    for caps leaned past about 88°).
  - *The off-spine plane term*: the square slot's walls lie along the
    spine (`|n·τ| = 0`); an off-spine plane wall is a tilted slot, whose
    line slack dominates as the bore's does. Not separately pinned.
  - *The escalate-zone margin* and *the tie-break's non-entity fields*:
    untouched by these readings, still open as written above.
  - *The prism reading reads the cap cycles found so far*: unchanged —
    the planar cut-off's cap meter runs in the same `ring_clearance_pass`
    before `band_reach`.
- **Still open:** the void above (owed), the concave fill (its own
  item), and inside-the-union-of-bands at mitres and corners. The concave twin of the mitre void and island rows in
  `band_planar_mitre` is unchanged.

## Closed

All four steps of Ev's PR 4285 build order are built: the three-way
classifier (PR 4797), the verdict log, the deletion from the ruled cap,
and the planar band and chamfer (PR 4957). The concave fill went to
`a-concave-fill-and-a-whole-shell-inside-a-bands-region`, whose ruling
(Ev, PR 4963) supersedes this item's case table and whose steps 2–3
(PRs 5099, 5161) built the bar, the blind bore and the pin. The one
residue this item names, a void wholly inside the material a convex
band removes, is that item's step 4 (whole shells and the seal), so
nothing is left here.
