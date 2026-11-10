---
id: the-at-rest-gate-certifies-embedding
kind: issue
title: The at-rest gate certifies that a finished body's boundary is embedded off its recorded contacts, at every scalar with certification rights (Ev, PR 6301)
status: parked
opened: 2026-11-02
priority: P0
cost: H
blocked_on: [clearance-cell-enclosures-are-typed-as-the-interval-scalar]
---


Ruled at PR 6301 (`where-the-embedding-certificate-is-paid`), now in
`docs/DESIGN.md` `[topology.tiers]`. A finished body passes tier 3′
*and* its boundary is embedded off its recorded contacts, certified by
the clearance engine's body-level injectivity question at the at-rest
gate. This runs at every scalar with certification rights: f64, Probe,
Interval and Sym. Duals answer `NotRunAtThisScalar`.

What the unit carries, from the three designers' converged reports
(PR 6301's body):

1. **Split the clearance lane** (`AtRestPolicy::clearance_lane`,
   `topo/src/props.rs`). The minimum-clearance measure keeps `None` at
   f64. A yes/no verdict door reads certification rights.
   `topo::shell`'s cavity gate reads the same door, so its f64 window
   ("a curved crossing shells silently wrong") closes.
2. **The gate asks the certificate.** It has one fixed pair budget,
   PR 6102's rule. `Violated` names the two faces and a witness, and an
   engine refusal refuses typed and never reads Ok. For a loft, the
   editor maps the faces through `Lofted`'s face roles to walls and
   section ranges.
3. **Exclude recorded contact pairs.** This is a face-pair exclusion
   on `topo::clearance::Side`, since a tier-3′ body is not injective at
   its records.
4. **Keep the gate's verdict per node by content key**, which is
   PR 3870's "once, at the door that built it". Today
   `finished_operand` and the product gather re-gate on every
   evaluation or landing. Where the memo lives is ONEHOME's at-rest gate
   question (`onehome/the-at-rest-result-gate-and-its-typing-live-in-five-homes`);
   announce the seam.
5. **The viewer's preview landing shows construction state, badged,
   and does not finish the product.** Only a commit finishes it
   (`viewer::session::land`, G1's preview/commit split). This seam is
   the viewer's.
6. **CARVE's fold retires.** The fold-back arm and the far-normal
   interim go, and the hood and oblique-arc rows in
   `bool6_per_slab_stacking.rs` flip to building. The Zero arm stays
   before the skin as its own decide. The reversed list builds, with
   inside taken from slab 0's sign.
7. **The shell's pre-insertion cavity check** becomes a second reading
   of the same question: it retires, or stays as an early typed arm.

**Measure before the spec:**
- the certificate's price on planar and boolean-chain bodies, since
  every finished body now pays it;
- the refusal price after NETCACHE on the 9 rad and 13 rad arcs, the
  closed spine and CARVETAIL's joints;
- what a drive leaf pays at `Sym<Interval>`.

D5 is unchanged. One residual is named: dragging a loft that a boolean
or shell consumes pays the certificate once per new value (up to 10 s
on the Klein loop). If that bites, it is a D5 question of its own.

CARVE's `self-overlapping-spines-build-and-validate` is parked on this
row.

## Found by ONECOPY's branch-band fork (PR 6625), 2026-11-09

The fork on `a-carriers-branches-within-the-band-of-each-other` (closed
into this unit by PR 6618) put this gate as the home of "an edge or face
that comes within the band of itself". Its designers and its lane read
three defects in the engine this gate runs on. All were read from code
and none was run.

- **(a) A trimmed analytic face is never paired against itself for
  distance.** `Sweep::run` discharges a trimmed analytic face against
  itself by `by_kind` (`topo::clearance`, ~1580; `topo::injective`,
  ~518–538). That proves the chart `S` one-to-one. It never pairs the
  face's own non-touching cells for the distance test, which is
  ERROR-DESIGN's condition (2). The chart decides touching
  (`injective::rects_meet`), not the trimmed region in M. So two cells
  either side of a sub-band slit or waist in one face are certified,
  not measured. Exhibit: a plane cutting a ring torus in a spiric with
  a sub-band waist; no door builds it today. Two fix shapes:
  - pair a trimmed face's cells for distance anyway;
  - or read each face's boundary cycles clear of themselves at the band
    (tier 3's "simple and disjoint", at the band).
- **(b) Cross-face touching is decided by edge key over the whole span,
  not by the shared edge's parameter.** `injective::Local::of` keeps an
  edge whenever a cell's ball is not `clear_of_arc` over the edge's
  whole span (`topo::cell_region`). `Near::of` keeps a shared edge
  wherever neither ball clears it, and SLIVER's `near_edges.contains`
  then concedes a sub-band strip beside it. A cell beside one branch of
  a hairpin edge is thereby "near" it through the other branch. This
  flips no verdict found, but it is where one could be conceded. Fix:
  `Near` keeps the parameter run of the edge that the ball is not clear
  of.
- **(c) Residue:** a same-surface spline split refuses `Uncertified`,
  because the spline pcurve is unpresentable.

**Named residual.** A closed-form section read on the wrong branch
mid-op, before its result is gated. No instance is known. If one is
shown, the remedy is a uniqueness certificate at the section's mint.

**Pins for when this unit lands:**
- a hairpin edge with a NURBS wall, on a fold whose speed stays above
  `certify`'s span meter: a semicircle tip of radius below the band,
  over a short knot span between legs on long ones. The cubic hairpin
  of `geom`'s on-carrier rows does not serve here: its tip speed is
  `1.5·h`, and `certify` refuses it `SpanMeterCollapsed`;
- the torus waist of (a).

**Optional ordering:** run embedding before the census's
branch-resting reads (`census::curved::edge_tangent_at` and its
callers).
