---
id: ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver
kind: issue
title: blend: a cap ring the ruled cut-off's sliver wholly contains refuses RingClearance; the bore vanishing with the sliver is buildable, and the meter is an enclosing annulus, not the sliver
status: open
opened: 2026-09-25
priority: P2
cost: H
---


## Finding

A convex ruled crease's cut-off removes a sliver from each transverse
cap. Every other edge of the cap is metered before the carve
(`ring_clearance_pass` arm (c), `crates/sweep/src/blend/surgery.rs`;
`CapSliver` in `crates/sweep/src/blend/open/ruled.rs`), each over its
own window, against a region ENCLOSING the sliver: the annulus
`r ≤ ‖p − c‖ ≤ reach` about the section centre `c`, cut down to the
half-plane `(p − c)·(V − c)/‖V − c‖ ≥ floor` the sliver lies in.
Anything not definitely clear refuses `RingClearance` at the cap.

Two things that meter leaves on the table:

1. **The bore wholly inside the sliver is buildable.** Its walls lie
   entirely in the material the band removes, so the right answer is
   the band with the bore gone: the bore's ring, its wall faces and
   their far-cap ring all die, and the sliver folds into the band as
   today. Rows that refuse it now:
   `band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_refuses_ring_clearance`
   and
   `review_band_ruled_ring_probes::a_bore_in_a_keyhole_creases_removed_sliver_refuses_ring_clearance`.
   Building it needs (a) an EXACT classification of "wholly inside the
   sliver" (not the annulus — see 2), and (b) surgery that kills a
   through-feature — its two rings and the walls between — which no
   blend carve does today; `kfmrh`/`kef` over the bore's walls is the
   likely shape. It was not attempted in the unit that added the meter
   because (a) has no closed form in the tree yet.
2. **The region over-reaches the sliver.** It is exact on the D-rod's
   line-and-wall corner up to the half-plane's tilt, but it is still
   an enclosure: an edge is cleared only if it lies wholly inside the
   ball's section, wholly beyond `reach`, or wholly short of `floor`,
   so an edge that leaves the region by different faces at different
   points — e.g. one running from inside the ball's section out across
   the annulus on the far side of `c` — refuses where it is clear.
   And the half-plane's direction is `V − c`, not the chord normal of
   the two feet, which leaves a thin wedge of kept material near each
   foot inside the region on corners where the two differ.

   **Witness, the cut cycle's own edge** (the tour's `rocker`,
   `demos/tour/src/rocker.rs`, `crease_narration`, wall 2): a keyhole
   through a plate (disc `R = 1/2`, slot half-width `w = 1/5`, slot
   end `0.8` from the disc's centre). Its two convex creases carve at
   the closed form below `r* = 0.3097` and refuse `RingClearance` on a
   cap from `r*` on, with margin exactly `‖E − c‖ − ‖V − c‖`, `E =
   (0.8, w)` the slot end edge's corner nearest the crease: `−1.5e-4`
   at `r = 0.31`, `−0.00884` at `r = 0.33`, `−0.03263` at `r = 0.4`,
   `−0.05264` at `r = 0.49`. `r*` is derived from that margin, not
   read off a sweep: `E` and `V` share the wall line and `c` sits `r`
   above it, so the margin changes sign where `cx = (x0 + 0.8)/2`, and
   `cx² = x0² + 2r(R − w)` gives `r* = (cx² − x0²)/(2(R − w))`
   (`ring_clearance_onset` in the scene, which pins it from both sides
   at ±1 %).
   The slot end edge is clear of the sliver at every one of those
   radii — the sliver ends at the wall foot
   `x = √((R + r)² − (w + r)²)` (`0.639` at `r = 0.33`, `0.671` at
   `r = 0.4`), short of the slot's end — and it is the case above: at
   `r = 0.33` its upper end is short of the floor but inside `reach`,
   its lower part past the floor but beyond `reach`, so no one face
   clears it whole. The enclosure refuses an edge the band does not
   touch, and the rendered sentence says that edge "lies in the part of
   a face the blend cuts away", which it does not.

## What the taker owes

An exact sliver-membership classification for a circle ring (a
Q1 trilean with its own margin), the refusal split into "straddles the
arc" (a frontier: the band would have to be trimmed by the bore) and
"wholly inside" (built: the feature dies with the sliver), and the two
rows above flipped to carves at the closed form `ΔV = −2·A·L + V_bore`.

## Witness 2 resolved (PR 4173's fix pass, 2026-10-06)

`CapSliver` now encloses the sliver by the disc to its reach less the
inside of the band's section, within the half-plane towards the old
vertex and the box the sliver spans in the section's own axes
(`crates/sweep/src/blend/open/end_face.rs`). The keyhole's slot end
edge is clear of that box, so the rocker's creases carve at their
closed forms past `r* = 0.3097` (asserted at 0.31 and 0.49 in
`demos/tour/src/rocker.rs`), up to the headroom wall at `R_BLEND`;
wall 2 is retired. `R_CREASE` stays at `R_EYE` until a `[render]`
pass re-baselines the scene at a larger radius. Finding 1, the bore
wholly inside the sliver, stands.

## Findings (2026-10-07)

Measured on `origin/main` at `f857d98de` (the branch point of
`band/a-bore-inside-the-cut-off-sliver-vanishes-with-it`).

**Claim 1 holds.** Both named rows still exist and still refuse
`RingClearance` definitely:
`band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_refuses_ring_clearance`
and
`review_band_ruled_ring_probes::a_bore_in_a_keyhole_creases_removed_sliver_refuses_ring_clearance`.

**Claim 2, in its two parts.**

- *The `V − c` half-plane leaves a wedge near each foot*: it does not
  bite on main. Bores straddling the ball's boundary just past each
  foot of the D-rod (past the flat's foot along the flat, and 0.5° to
  20° past the wall's foot along the wall, radii down to 5e-6) all
  carve. The floors on the section's own axes (PR 4173) close the wedge
  there. No witness was found, so nothing was built for it.
- *An edge that leaves the enclosure through different faces refuses*:
  this held for straight edges. Witnesses on main:
  - three rectangular holes through the D-rod, each with an edge that
    runs from inside the ball's section out past the sliver's box or
    half-plane. The first refuses with margin `−0.0047` while it is
    about `0.0099` clear of the arc;
  - the keyhole at `r = BR` and `1.1·BR`, which refused on the
    plate's side `x = 1`. That side runs through the ball's section and
    out of it below, clear of the sliver. The row
    `a_keyhole_crease_at_the_discs_radius_meets_the_cap_meter_not_the_headroom`
    pinned this as "the cap meter's over-reach".

  On a grid of 326 rectangular holes clear of the sliver by ≥ 0.003, 2
  refused on main. Of 1371 such bores (circles), none refused
  `RingClearance`, and one refused the reach meter's `FaceClearance`
  (filed: `blend-reach-refuses-a-bore-clear-of-a-ruled-cut-offs-sliver`).

  Built: `CapSliver::line_clearance` reads a straight edge point by
  point, exactly against the enclosure (with an elliptic section read
  through its minor disc). All the witnesses carve at their closed
  forms, and the grid's straight-edge refusals go to 0. Curved edges
  remain term by term (filed:
  `cap-sliver-meter-reads-a-curved-edge-term-by-term`).

**Claim 1 is a design fork, so it was not built.** "The bore dies with
the sliver" is a blend deleting an authored through-feature: its two
rings, its walls, and their names. No blend does that today, and four
things turn on it:

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
   which is a change to what predicate 2 certifies.
4. **The README clause.** `crates/sweep/README.md`'s ruled-band clause
   ("every other edge of the cap stays where it was … an edge not
   definitely clear of the region refuses `RingClearance`") came from
   a BAND fix pass, not from Ev (`git log -S`), so it binds nothing
   here. It would still be re-worded.

**Recommendation:** keep the refusal. An author who wants the bore gone
can delete it in one edit, and the refusal's sentence already says the
edge lies in the material the blend removes. If Ev wants the carve, it
should come with the finding in (1) and with predicate 2 exempting the
dying faces. It also needs an exact membership test for the sliver
itself. The enclosure `Ω` is not one: it contains the sliver, so a
ring inside `Ω` may still cross kept material.
