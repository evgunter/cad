---
id: limb3-at-rest-proves-the-graph-not-the-arc
kind: issue
title: at rest, limb 3 proves its chain a graph, short of the ratified one arc; a declared plane x NURBS carrier joining two separate arcs certifies
status: closed
opened: 2026-10-03
closed: 2026-10-04
pr: 4012
branch: ssi/limb3-at-rest
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

## Design (converged, 2026-10-04; `[ev]` PR for the C2 sentence)

Two designers weighed it over four rounds. The record is `docs/DESIGN-FORK-LOG.md` and the SSI log; probes are on `analysis/design-fork/limb3-at-rest-{a,b}`. The converged design: limb 3 proves one theorem at every door, `certify_rung3`'s at-rest lane included.

1. **One piece per window.** Each window, cut to the wall's knot rectangle (and to the ℝ³ slab where a search clips to one), holds exactly one piece. Either its boundary has two simple zeros (#3999's count), or a stretch of its boundary lies on a wall side within ε of the locus and the rest of the boundary has one certified sign. The second case is the *side cover*. "Within ε" is the side's certified reach, `boundary.rs`'s `strip_reach`, lifted into one door that the boundary pass's `side_region` and limb 3 both call.
2. **Coverage along the carrier.** There is a zero on the slice through each shared knot (#3999's `holds_zero`) and on the carrier's two end slices (new). An end also counts where the carrier's end lies within ε of the locus along its slice. The end slices close a gap #3999 leaves at rest: a "tail fold" (wall `z = c·x + a·x² + β(y/L)²`, a = 0.24c²/β, carrier `(0,0,0)→(0,L,0)`) certifies a carrier whose end is about 2 mm from the locus at β ≤ ε, under the count alone.
3. `Banked`/`Lane` select no proof. The proof takes only the region it is cut to.
4. `edge_nurbs::refusal` gains a `TubeNotOneArc` arm; today an at-rest refusal surfaces as `Unsupported`.

Measured on the probes: the 9 band-flush rows certify. The fold, the half fold and the tail fold refuse. A correct branch that leaves through a side parallel to its slices certifies; the section theorem, the rejected alternative, refuses it at every rung.

Unprobed by both designers, so the build must cover them:
- linking a side-cover window to a crossing window;
- ℝ³ end-face existence (Krawczyk on the end faces);
- the lifted `strip_reach` door.

**Done when:**
- the at-rest door runs the theorem;
- the fold, half-fold and tail-fold rows refuse at rest, and the 9 band-flush rows certify;
- the rail-end row certifies;
- each new arm has a row that goes red when it is removed;
- `edge_nurbs` has a typed arm for the refusal;
- C2 reads as on this PR.

Not in scope: whether a carrier must lie within ε of the *locus* rather than within ε of each surface. Both designers name this as a separate D4 question. The certified bound 2·residual/margin reaches 10⁴ε on correct shallow carriers the search mints.

## Closed (PR 4012)

Ev took the converged design ("sounds good, then!"), and it landed on
the same branch:

- `one_arc` (`ssi/one_arc.rs`) proves one piece per window: the count
  of two simple boundary zeros, or the side arm. A window's edge on a
  wall side holds the side's piece where the boundary pass reads that
  stretch (`boundary.rs`'s `read_stretch`, over the side's own
  Bernstein section, `section.rs`'s `SectionReader`) within ε of the
  plane with no piece of it clear by `clears`, the one test #3862's
  rule decides a side by in `side_region`, and the side's cover
  (`side_cover`, the door `side_region` reads too) holds every zero of
  the window within ε of the side. `beyond_reach` screens a side
  before the stretch is read.
- It links consecutive windows: two stretches of one side where their
  overlap reads within ε and no piece clear, a side to an arc or two
  arcs through a shared solution on the knot's slice.
- It reaches both ends of the carrier. An arc's end counts where the
  end slice holds a zero, or a zero is certified within ε of the end on
  a slice beside it (`near_end`, read as `Near::{Found, Far, Unknown}`;
  a chord whose wall lies wholly farther than ε reads `Far`). A side's
  end counts where the end, moved across onto the side, lands on its
  stretch at a point read within ε and not clear. `Short` only where
  that is certified not so, including a box that resolved no piece when
  an end is certified unreached by its own box; `Undecided` otherwise.
  `one_arc_r3` reaches both ends by Krawczyk on the end box's slice, or
  a box of diameter ε about the end.
- `certify_branch` runs it at every door; `Lane` selects no proof, the
  at-rest ℝ³ chain is cut to nothing. `SsiCertificate::tube_one_arc`
  retired, always true now.
- `OneArcRefusal::{Count, Unlinked, Short, Undecided}`;
  `PlaneNurbsRefusal::TubeNotOneArc`, and `pcurve_cache::ssi_refusal`
  names it. Each door ends by its own one recourse (`TUBE_ONE_ARC` at a
  search, `REST_ONE_ARC` at rest), the cause carried as data.

The fold, half fold and tail fold refuse at rest; so do a phantom
side, a fold on a side, a carrier past where a side's locus ends, and a
side whose slope across it dips (`ssi_limb3_one_arc.rs`). The 9
band-flush rows certify, and flush sides of cone and twisted walls; the
rail-end branch certifies on the search lane. The mutant table is in
the PR body.
