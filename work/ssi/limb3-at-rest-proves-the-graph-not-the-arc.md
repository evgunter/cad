---
id: limb3-at-rest-proves-the-graph-not-the-arc
kind: issue
title: at rest, limb 3 proves its chain a graph, short of the ratified one arc; a declared plane x NURBS carrier joining two separate arcs certifies
status: open
opened: 2026-10-03
priority: P1
cost: M
design: true
refs: [limb-3-tube-banks-a-second-arc-as-accounted]
---

## The ratified text

- **CURVED-DESIGN OQ2** (commit `c0c74ea9b3`): "**DECIDED (Evan, #85,
  2026-07-24)** … the C2.3 uniqueness tube is required for every
  fitted `Intersection` at rest."
- **C2.3 as first written** (commit `58655b09ab`): over the chain of
  boxes covering the cache, "the system (f₁ = 0, f₂ = 0) has its
  solution set connected and transversal".
- **C2 today** (`crates/geom-brep/README.md`): "the solution set in the
  chain is one arc".

## Where the code falls short (found by the limb-3 lane, `ssi/limb3-one-arc`, 2026-10-03)

`limb-3-tube-banks-a-second-arc-as-accounted` made limb 3 prove the one
arc wherever a search banks the tube. `certify_rung3` (`ssi.rs`) is the
at-rest door. It serves the plane × NURBS edge lane (`edge_nurbs.rs`,
`plane_nurbs_limbs`), the pcurve cache (`pcurve_cache.rs`) and the
adversarial suite, and it passes `certify::Banked::No`. There limb 3
still proves only the graph: at most one solution on each slice of
each box. That is short of C2.3's "connected".

**Evidence that the at-rest class needs the stronger proof to be
built, not just switched on:**
- Running the one-arc proof at rest (`Banked::Wall` in `certify_rung3`)
  makes 9 certifying rows refuse: the four certifying
  `m7_8_plane_nurbs_edge` rows, `r1_pxn_probes`'s
  `a_drifted_subsegment…`, `displacement_scan…`, `near_tangential_scan…`
  and `the_certified_sup_bounds…`, and
  `d290_r2_e2e::plane_nurbs_limbs_image_lands_on_the_carriers_own_domain`.
- Their class is a planar cap meeting a wall along the wall's boundary
  iso-curve. The carrier runs along a domain side that lies on the
  plane within the band, so `φ` along that side is zero up to rounding.
- No boundary walk can count simple zeros there. And at ε scale such a
  locus need not be one arc: an edge within ε of the plane can cross it
  many times.

## Priority: P1 — a user can build a silently wrong certified edge

An at-rest cache does select a non-component silently, on input a user
can build. Measured on `ssi/limb3-one-arc` with a scratch probe (not
committed); the fixture is `ssi_limb3_one_arc.rs`'s fold:

- **Wall:** `z = c·x + a·x² + 4β·y(L − y)/L²` with c = 800ε,
  a = 0.28·c²/β, w = β/c, L = 1.2w, `x ∈ [−1.5w, 1.8w]`.
- **Locus:** two arcs, from `(0, 0)` and from `(0, L)` to the low `u`
  side, with `φ > 0` between them.
- **Declared carrier:** the segment `(0, 0, 0) → (0, L, 0)` against the
  plane `z = 0`, run through the public `geom_brep::plane_nurbs_limbs`
  (extent 1, the run band).
- **Result:** it certifies Ok at β = ε, ½ε and ¼ε. `on_locus_max` and
  `hull_sup` come out ≈ β. The tube certifies at rung 0.125, with one
  window over the whole wall.

The certified edge joins two separate arcs of the intersection. Its
witness, `carrier(mid) = (0, L/2, 0)`, lies where `φ = β > 0` and
there is no zero. Nothing refuses.

Any construction that declares an `Intersection` edge's carrier
reaches this door, and so does re-certification of that edge. Carriers
the SSI search mints are proved one arc at mint time; this gap is the
declared and re-derived path. It is the same silent wrong topology the
P1 row closed on the searches, hence P1.

## The fork (for the designers)

What limb 3 should prove at rest so that OQ2 holds:

1. **One arc, with a band-coincident side as the arc's own.** The walk
   accepts a box edge on the wall's domain side when that side lies on
   the plane within the band and the carrier runs along it. The claim
   becomes "one arc, or the band-coincident side it runs along", which
   is what C3's `Side` region already says for a search. This reads as
   a refinement of C2.3, not a retirement.
2. **One arc at rest, refusing the band-coincident edge class** toward
   C7 or a boundary-curve rung. It keeps the text but refuses real B-rep
   edges (the 9 rows above).
3. **Graph alone at rest.** This would retire half of OQ2's decision,
   so it is Ev's call.
