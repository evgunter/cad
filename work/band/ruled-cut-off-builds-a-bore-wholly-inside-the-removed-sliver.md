---
id: ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver
kind: issue
title: blend: a feature wholly inside the material a band removes or fills goes with it (Ev, PR 4285); today a bore inside a ruled cut-off's sliver refuses RingClearance
status: open
opened: 2026-09-25
priority: P2
cost: H
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
