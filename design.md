# Boolean operands with NURBS or spiric edges — design report

## For Ev

**Recommendation (likely).** Build no edge lane as a unit of its own and no pair-scoped edge gate. Do two things:

- **Now, a small typed fix.** The planar crossing lane silently treats a spiric or NURBS edge as a line (premise 2).
  Make it refuse typed, so the gate stops being load-bearing.
- **Later, retire `gate_operand_edges` inside the first row that has a consumer for it.** Two rows qualify, whichever
  lands first:
  1. the **NURBS-face operand row**, which is what the tour scenes wait on;
  2. **frontier (d)**, the cylinder×sphere join window. Once that lands, the boolean mints NURBS seams on analytic
     faces and must accept its own output.

When the edge is retired, it gets one rung keyed on the carrier:
- a NURBS edge: the existing rational Bernstein composite (`geom_core::spline::compose`);
- a spiric edge: a transfer to the conic section of its cap plane, using the existing conic×torus lanes.

**Terms.** *Carrier*: an edge's underlying curve. *Spiric*: a torus cut by a plane parallel to its axis (the
offset-axial door mints one as a hollowed partial revolve's rim). *Crossing layer*: the sweep that finds where an
operand edge meets the other operand's faces; a *rung* is one certified case of it. *Analytic face*: plane, cylinder,
cone, sphere or torus.

### Premise check (sure, measured)

1. **Lifting the edge gate retires nothing that can be built today: no consumer exists.** I lifted the gate in a
   local build (reverted) and ran both populations of bodies that carry these edges. Every case refused on a *face*:

   | operand (∪ a brick unless noted) | refuses next at |
   |---|---|
   | loft prism, disjoint / clear near a seam | `NurbsExtentUnsupported` (frontier (e)) |
   | loft prism, rod through its planar cap only | `Containment(KindUnsupported{Nurbs})` |
   | loft prism, slab through its walls | `CurvedBooleanUnsupported{Nurbs}` |
   | spiric-rimmed vessel cavity, disjoint / rod through a cap | `Containment(PartialTorusFace)` |
   | that cavity, rod across the rim; ∪ the klein elbow | `CurvedPierceUnsupported` — a *line* edge of B against the cavity's torus face |

   - The loft prism has NURBS walls and NURBS seams.
   - The vessel cavity has analytic faces (torus and planes) and spiric rims. It comes from the public
     `topo::offset_charts_together`, and it reaches the boolean because the operand gate checks tiers 1–2 only. The
     `shell` verb itself stops at tier 3 on that body.
   - The tour joins (teapot spout, lily sheath, klein loop) are NURBS-walled. They belong to the face row.
2. **The gate hides a silent defect (sure).**
   - In the planar crossing lane, `conic_plane_crossing_roots` answers `Err(())` for a line, a spiric and a NURBS
     carrier alike.
   - The arm reads `Err(())` as "a line: the M3 lane below owns it". It then interpolates the crossing point linearly
     from the endpoint distances, and it misses any curve that dips through the plane and back.
   - The body-scoped gate is the only thing keeping that arm sound. A pair-scoped gate would make the defect live.
   - Separately, the join's on-edge frame answers `JoinDesync` for these kinds.
3. **The gate's stated reason is wrong (sure).** It says rung-3 edges "are what the curved zip MINTS, not what it
   consumes". Nothing in the boolean mints them today. The producers are:
   - the loft's seams (the path sweep skins through the loft);
   - the offset-axial spiric rim.

   This is agent-written code doc, not ratified text. It becomes *wrong in the other direction* once frontier (d)
   lands: the cylinder×sphere section's 3-D carrier is the C5 table's rung-3 fitted NURBS
   (`geom_brep::cylinder_sphere_ssi`). From then on the gate would refuse the boolean's own output, against DESIGN's
   "every boolean output is a legal boolean operand". So (d) cannot land before this gate is retired (likely; (d) has
   no scheduled row that I found).
4. **"Re-entry through the germ-chord lanes" does not apply to NURBS edges.** For a spiric edge it is the right idea,
   in the form of the section transfer described below.

### The rung, when its row comes (likely)

- **NURBS edge × analytic face: `spline::compose`.**
  - `implicit_composite` already builds `f ∘ C` in rational Bernstein form, in certification arithmetic, for plane,
    sphere, cylinder, cone and torus. Today it serves the fitted-carrier certificate.
  - The coefficient hull *is* the enclosure: one-signed means the edge clears the face.
  - Knot insertion (`insert_knot_plan` / `apply_certified`) subdivides the hull.
  - A piece whose ends differ in sign and whose derivative hull (`derivative_coeffs`) is one-signed holds exactly one
    root. A piece that shrinks below the band is a tangency and refuses typed.
  - This is the same clear / monotone / split ladder `circle_roots::certified_subdivision` runs, with hulls in place
    of Taylor bounds.
  - A plane face is the linear composite, so the planar lane's arm is repaired by the same rung.
  - The edge's box becomes its control hull (`nurbs_curve_aabb`), which is sound; today it is poison.
- **Spiric edge × analytic face `S`: section transfer.**
  - The edge lies on its torus `T` and its cap plane `Π`. So it meets `S` exactly where the section `Π ∩ S` meets
    `T`.
  - For a plane or sphere face, `Π ∩ S` is a line or a circle, and `line_torus_roots` / `circle_torus_roots` already
    answer it exactly.
  - Cylinder, cone and torus faces refuse typed until their sections have torus lanes.
- **NURBS and `Approx` faces** have no implicit form. They are the face row's problem, not this rung's.
- **Refusals stay pair-scoped.** They come from the rung that lacks an arm, naming `(edge, face)`, which is C12.1's
  "retire per arm, never wholesale".

### Options weighed

- **A. Edge rung as a line item of its first consumer's row; planar-lane fix now (recommended).** Nothing is built
  without a caller, the silent arm is closed first, and it is reversible.
- **B. Standalone edge lane now.**
  - It is unit-testable on hand-built bodies.
  - But no end-to-end consumer exists: measured in both populations above. That is the dead-code pattern (frontier
    (f)'s posture).
  - Not recommended.
- **C. Pair-scoped gate, like the split's.**
  - Retires no row in the table.
  - It re-states the BVH's box pruning at a second site.
  - It is unsound until the planar-lane fix lands.
  - The split could narrow its gate because its only other operand is a plane, and its insertion site re-checks
    clearance. Neither holds here.
  - Not recommended.
- **D. Per-carrier `|C′|,|C″|,|C‴|` bounds with a sampled residual enclosure** (my first draft). It works, but it
  re-derives per carrier what the composite carries exactly, lacks a cone arm, and for the spiric is weaker than the
  exact section transfer. Not recommended.

**Ratified text:** nothing to change. The edge gate lives only in code docs. C12.1 decides the shape. DESIGN's "every
boolean output is a legal operand" is the reason (d) is ordered after the retirement.

**Confidence.**
- Recommendation: likely.
- No consumer today (both populations): sure, measured.
- The planar-lane defect: sure.
- (d) mints NURBS edges on analytic faces: likely.
- The composite as the root rung: likely. The doors exist; the subdivision ladder is new code.

## For the orchestrator

**What changed in round 1, and why.**
1. **"When" — moved; I now agree with the other report.** I measured the population I had argued for: the
   spiric-rimmed analytic body (`vessel_cavity`, via the public `offset_charts_together`). With the gate lifted it
   refuses on its partial-torus face, at containment or at a line-edge pierce, never on its spiric edge. So the edge
   lane has no consumer today. My first draft's step 1 (delete the gate standalone) is withdrawn. I keep only the
   typed planar-lane fix as standalone work.
2. **Where I still differ, slightly.** The other report says the spiric operand "cannot be built past tier 3", so
   never reaches the gate. That is imprecise:
   - `offset_charts_together` is a public kernel door, and its output reaches the boolean, whose operand gate checks
     tiers 1–2. `sweep/tests/spiric_rim.rs` row 11 already pins `CurvedEdgeUnsupported` on it.
   - The conclusion is unchanged (the face refuses next).
   - I also add frontier (d) as a second possible first consumer, with a hard ordering: (d) must not land before the
     gate retires.
3. **"How" — I adopt theirs.** The composite's hull is an exact enclosure in certification arithmetic, already
   written for all five analytic kinds, including the cone, which my draft lacked. My sampled enclosure needed new
   per-carrier derivative bounds. For the spiric, their section transfer is exact and reuses existing lanes.
   - I could not check that `implicit_composite` accepts the operand scalar lanes the boolean runs at (f64 and
     Interval). They assumed it through `certified_coords`.

**Measurement.** All local and reverted; the branch carries only this file.
- An env-var bypass in `gate_operand_edges`.
- Four `loft_prism ∪ brick` cases (round 0).
- In round 1, a throwaway test appended to `sweep/tests/spiric_rim.rs`: `vessel_cavity(1/128)` ∪ four placements
  (far, cap rod, rim rod, klein elbow). The bbox and the two spiric rim midpoints were printed to confirm the
  placements.

**Defects to file** (not filed: this branch carries only the report):
- CLEAVE, P2: the planar-lane `Err(()) => {} // a line` arm in `reduce.rs` (premise 2).
- Doc-only: `CurvedEdgeUnsupported`'s Display says "a spline (NURBS) curve" but fires for spirics. Its doc, and
  `gate_operand_edges`'s, name the zip as the producer.
- `join.rs` `germ_section_frame` on-edge arm: `JoinDesync` for a kind that becomes reachable.
- The work row should be re-scoped into the NURBS-face operand row. Its "no root lane" paragraph should cite
  `spline::compose`.
- If (d) gets a row, it should carry "retire the edge gate first" as a precondition.
