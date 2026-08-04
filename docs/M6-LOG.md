# M6 log

M6 = the main-path curved completions (renumbered 2026-08-03, Evan,
PR #169; plan assembled from #161 + #169 — see the orchestrator's
M6-PLAN). This log records units as they land, newest last.

## Unit 1 — the in-place edge-blend composition surgery (head of
## queue per Evan's #169 ruling)

**Scope**: close M5-walk row 8's carried item — THE COMPOSED DIE
(filleted blank + 21 pips + filleted pip rims in ONE body) — via the
surgery the PR 12 review sized at one reviewed unit: in-place face
split along STORED trimlines + ring carry-through + rim-edge →
torus-band replacement. Optional rider (taken): the circle-carrier
definite-miss bound retiring door A's unconditional arm.

**The mechanism** (`crates/sweep/src/fillet/surgery.rs`; routed from
`fillet_edges` whenever the whole-body door refuses, so the M5
whole-body path stays bit-preserved):

- *Open chains* (single convex plane–plane links ending at
  fully-requested trivalent corners): per support face, one strut
  `mev` per boundary vertex to the corner ball's foot + one trimline
  `mef` per edge carve the face into the SHRUNK face (same `FaceKey`,
  same surface/sense via S12 parent-sense inheritance, **rings
  kept**) plus per-edge strips; `kef` across each dying sharp edge
  merges strip pairs; per corner, three arc `mef`s + two `kef`s + one
  `kev` fuse the corner triangles into the sphere octant (the F2
  order-free chart pick, extracted to `octant_chart` and shared with
  the whole-body builder — code motion, arithmetic unchanged).
- *Closed chains* (circular plane–sphere rims): the plane side struts
  to the widened trim circle exactly as above (the ring loop's hole
  WIDENS — the fillet eats into the flat face); the sphere side
  SPLITS the revolve-seam meridian edges where the sphere trim circle
  crosses them (no struts into the cap), then one trim `mef` per
  half-cap; rim-edge `kef`s + strut `kef`s fuse the annulus. A curved
  face must be ring-free (`props`' closed-form inventory — the
  donut's own representation), so the closure vertex dies by a
  fan-merging `kev` that leaves one meridian remnant as the band's
  SLIT: a double-traversed minor-circle `Seam` edge, with the band's
  torus chart seamed at that azimuth (`u_ref` is conventional data,
  D2; Seam certification demands the seam lie in the chart's u_ref
  half-plane).
- *Exactness discipline*: the trim-circle carriers are the rim
  carrier's own frame SCALED (same axis, same u_ref, same parameter
  window; reversal by negating axis + window, never endpoint atan2 —
  π-arc safe); feet and split targets are evaluated ON those scaled
  carriers at the rim's own parameters, so azimuths are inherited,
  not reconstructed. Final pass: surfaces/senses first, then every
  new edge re-described `TangentIntersection` (trims/arcs) or `Seam`
  (slits), then a whole-body `mint_pcurves` re-mint (the input's
  caches are stale the moment the first strut lands).
- *The one new decision*: `fillet3_ring_clearance` — exact
  circle-vs-line / circle-vs-circle clearance between a support
  face's rings and every blend trimline, decided BEFORE mutation,
  typed refusal `FilletError::RingClearance`, two-tolerance on every
  arm (trio-pinned). Everything else is structural. In practice
  predicate 2's sampled screen fires first on the same length; the
  exact form is what the carry-through soundness rests on (sampling
  can overestimate a gap, the closed form cannot).

**The rider (door A)**: `geom_brep::circle_residual_extremes` — the
implicit residual of a circle carrier against a sphere (exact first
harmonic) or cylinder (degree-≤2 harmonic amplitude bounds) over the
whole circle, in meters — and a new `bool_circle_curved_clearance`
trilean in the boolean's curved-face arm: margin `max(lo, −hi)`
positive ⇒ the circle is definitely one-sided ⇒ no wall crossing;
zero/negative keeps the typed pierce frontier; in-band escalates.
Ellipse/NURBS carriers keep the M5 unconditional door. Judging the
full circle for an arc is conservative in the safe direction.

**The composed die** (`crates/sweep/tests/m6_surgery.rs`): cube ∖
21-ball group cut → 12 box edges filleted in place (r = 0.12, all 21
rims carried as rings) → all 21 rims filleted in one call (r = 0.02,
42 arcs → 21 closed chains → 21 torus bands). One body: 129 V / 195 E
/ 89 F, tiers 1–3 green, certified volume on the DERIVED closed form
— Steiner blank − 21·(cap + rim-torus term), the rim term derived by
Pappus over the removed cross-section (triangle minus two circular
segments, both disks tangent at the sphere-side trim point) — at
1e-9 relative with `volume_pad == 0`, watertight under `check_mesh`,
bit-replayed. Deviation 1 is FLIPPED at both doors
(`m5_pr12_die.rs::deviation_1_flipped_*`, S9 pattern — history kept):
door B composes via the surgery, door A composes via the rider.

**Deviations, numbered** — see the PR description for the final list.
