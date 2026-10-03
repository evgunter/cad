# Boolean operands with NURBS or spiric edges — design report

## For Ev

**Recommendation (likely).** Do not build a narrower edge gate. Delete
`gate_operand_edges`, and make each crossing site answer a spiric or
NURBS edge itself, through one generic certified lane: "this carrier
against this face's implicit residual". Build it in three steps (below).
Then **re-scope the row**. The three tour joins it names do not wait on
operand *edges*. They wait on spline-walled operands (*faces*), which is
a separate and larger row. The edge row's real consumers are bodies
whose faces are all analytic (plane, cylinder, sphere, torus) but whose
edges are spiric or fitted NURBS.

### Premise check — three corrections, measured

1. **Lifting the edge gate retires none of the three scenes (sure).** I
   admitted spiric and NURBS edges in a local, uncommitted build. Then I
   united `sweep::test_support::loft_prism` (four NURBS walls, NURBS
   seams) with a brick in four placements. Every one refused on a NURBS
   *face*, at three different doors:
   - disjoint: `NurbsExtentUnsupported` (DESIGN frontier (e));
   - 0.3 from a seam but clear: `NurbsExtentUnsupported`;
   - poking through the top cap only: `Containment(KindUnsupported{Nurbs})`;
   - transversal through the walls: `CurvedBooleanUnsupported{Nurbs}`
     (`curved_face_arm` refuses any edge against a NURBS face).

   Behind those doors there is a fourth: `germ_section_frame` has no arm
   for any NURBS face pair (`FrameError::NoArm` →
   `GermFrameUnsupported`). The teapot spout ∪ pot is a lofted
   (NURBS-walled) canal crossing a sphere, so it needs a NURBS×sphere
   *surface* section. The Klein loop is a sweep and the lily sheath is a
   loft, so both reach the same face doors. The pinned
   `CurvedEdgeUnsupported` hides those doors only because it fires first.
2. **Today the gate is load-bearing for soundness (sure).** It is not
   merely conservative.
   - In the planar crossing lane (`sweep_direction`),
     `conic_plane_crossing_roots` returns `Err(())` for line, spiric and
     NURBS alike. Its caller reads `Err` as "a line" and interpolates
     the crossing linearly from the two endpoint signs. Behind the gate,
     a spiric or NURBS edge would get a crossing point that is off the
     plane, and a curve that dips through a face and back would be
     missed entirely. Neither case raises an error.
   - The join's on-edge germ frame answers `JoinDesync` ("the operand
     gates refuse the kinds").

   So a pair-scoped gate is not a gate-only change. The split's twin
   could narrow safely because its crossing site already guards the
   same case (`classify.rs` re-checks `edge_clears` at insertion).
3. **"Rung-3 edges are what the zip MINTS, not what it consumes" is
   false today, and the gate turns into a contradiction later (likely).**
   - Today the boolean's sections are lines and conics. The spiric and
     NURBS edges in the tree come from loft, sweep and the offset-axial
     door (a hollowed partial revolve's torus rim is a spiric).
   - Once frontier (d) lands, the cylinder×sphere join window will mint
     fitted NURBS seams on analytic faces. From then on the boolean's
     own output is refused as its input. That contradicts DESIGN's
     "every boolean output is a legal boolean operand"
     (Maximal-faces paragraph).
   - The claim is agent-written code doc (reduce.rs, REACH/CURVED
     commits), not ratified text. No change to DESIGN is needed.

   "Re-entry through the germ-chord lanes" is not a crossing mechanism:
   (d) is the join's section *window*, which mints an edge. It does not
   locate where an existing edge pierces a face.

### What consuming such an edge actually requires

An operand edge `e` with carrier `C(t)` meets a face `f` of the other
operand where `F(C(t)) = 0`, with `F` the residual of `f`'s surface. The
signed residual is:

| face | residual `F` |
|---|---|
| plane | `n·(p−o)` |
| sphere, cylinder | `(|⊥(p−o)|² − r²)/2r` |
| torus | the existing quartic, with `torus_curvature_bound` |

There is one generic lane, by subdivision of `t`. On each piece:

- enclose `F` by the conic rung's sampled hull plus the chord-dip
  charge `f2·h²/8`;
- enclose `F′` the same way, one derivative down.

The answers are:

- **Clear**: the enclosure of `F` excludes 0 on every piece.
- **Simple root**: `F` changes sign on a piece where `F′`'s enclosure
  excludes 0. The root is unique there, its parameter is certified, and
  the lane splits `e` at that parameter.
- **Endpoint contact only**: `F(t₀) = 0` and `F′` excludes 0 near `t₀`,
  with no other root. This is the lily graft's posture, where seams end
  on the declared rectangle.
- **Unsettled** at the subdivision floor: `CurvedPierceUnsupported`
  naming `(edge, face)` — the pair, as the face gate already does.

Each carrier must supply:

- `s₁ ≥ |C′|`, `s₂ ≥ |C″|`, and `s₃ ≥ |C‴|` (the third bounds the
  dip of `F′`, whose second derivative needs `C‴`);
- for a spiric: from `spiric_rate_bounds` and its trigonometric form;
- for a NURBS: from its hodograph control hulls (a rational carrier
  takes the quotient-rule bound).

Each face kind supplies `|F″|`, from the issue body's formulas for the
quadrics and the torus. Line, circle and ellipse keep their closed-form
lanes as exact special cases. NURBS and `Approx` faces have no implicit
`F`, so they stay outside this lane (they are the face row).

### The options, as final states

- **A. No edge gate; one generic lane at every crossing site
  (recommended).**
  - The crossing lane's own carrier dispatch is the only record of which
    carriers it reads. No second roster has to be kept in step with it.
  - A refusal names the pair that needed an answer. A far edge prunes:
    a NURBS edge takes its control-hull box, which `EdgeBoxRule`'s docs
    already earmark for "the day the gate admits the kind". A near but
    clear edge clears by enclosure.
  - The same lane serves the split, since a split plane is a plane face.
    That retires the split's `edge_clears` box rule as well.
  - Leaves possible: nothing silent, if step 1 lands first.
  - Reversible: yes; a gate can be re-added in one function.
- **B. A pair-scoped edge gate mirroring the split's** (refuse a
  spiric/NURBS edge whose reach box may meet any face of the other
  operand).
  - Cheap, and it fails up front.
  - But it is a second roster beside the lanes. It is strictly weaker
    than A once the lane exists: it refuses at box level where the
    enclosure would clear.
  - It still needs step 1 below to be sound.
  - Defensible only as an interim; I lean against it as a final state.
- **C. Triple-point routing.** For an edge that is the section of two
  analytic faces `g, h`, find where it pierces `f` as the crossing of
  the lower-rung section `g∩f` with `h`, reusing the conic lanes.
  - Exact where it applies.
  - It does not cover cylinder×sphere seams, whose own `g∩f` is rung 3,
    nor spline walls.
  - At most an optimisation under A; not a design.

### Build order (sequencing, not a fork)

1. **Make every site sound without the gate.**
   - The planar lane refuses (or routes) any non-line carrier instead of
     interpolating it.
   - The on-edge germ frame refuses typed rather than `JoinDesync`.
   - A NURBS edge takes its control-hull box.
   - Then delete `gate_operand_edges`. What this admits: analytic-walled
     bodies whose spiric/NURBS edges are box-clear of the other operand.
     For example, a hollowed partial torus revolve drilled away from its
     spiric rim, and later every (d) output used the same way.
2. **The generic residual lane: clearance first.** Clear pairs clear
   against plane, sphere, cylinder and torus.
3. **Then root isolation and the split** — definite crossings for those
   bodies.
4. **Spline-walled operands, as its own row.** The tour scenes need:
   - NURBS extent (frontier (e));
   - point-in-NURBS-face;
   - edge × NURBS-face pierce;
   - a NURBS-face section frame. The SSI tracer that mints plane×NURBS
     already exists in `geom_brep::ssi`; the join has no general-section
     window.

   Steps 1–3 are prerequisites for it, but not the bulk of it.

**Confidence.**
- Recommendation A: likely.
- Lifting the edge gate retires no tour scene: sure (measured).
- The gate is load-bearing for soundness: sure (read).
- (d) will mint NURBS seams on analytic faces: likely. This is DESIGN's
  frontier (d) text read with geom-brep README C1's "cylinder×sphere
  (rung 3)".

## For the orchestrator

- **Measurement.** Local, reverted, nothing committed. I added an
  env-var bypass to `gate_operand_edges` and a throwaway
  `sweep/tests/zz_probe.rs` (registered as its own `[[test]]`), with
  four `loft_prism ∪ brick` placements. Results are as quoted above. I
  did not build a spiric-rimmed analytic operand, so step 1's payoff for
  that population is argued, not measured.
- **Brief errors.**
  - "Three demo joins wait on it" (the issue body) is wrong: they wait
    on the face row.
  - The issue's "Why it is a design row" says lifting the edge gate
    needs "a certificate the crossing layer does not have". That is
    true, but it misses the silent planar-lane fallback (premise 2).
- **Off-question defects to file.** Each is latent while the gate
  holds, and each should be a REACH/CLEAVE issue:
  - The `reduce.rs` `sweep_direction` comment "`Err(()) => {} // a
    line: the M3 lane below owns it`" is the unsound fallback.
  - `CurvedEdgeUnsupported`'s Display says "a spline (NURBS) curve" and
    the variant doc says "rung-3 (`Nurbs`)", but the variant also fires
    for spirics.
  - `join.rs` `germ_section_frame` on-edge arm: `JoinDesync` for a
    reachable kind once the gate goes.
- **Tour/test rows that flip.** Gate deletion turns the pins into other
  refusals, not successes:
  - teapot wall 3, lily wall 8, klein walls 3/4;
  - `review_cleave_nurbs_lane` disjoint union, `s16_box_soundness`,
    `offc_r1_probes`, `spiric_rim`.

  The scene walls should re-pin on the face door they actually reach,
  which is itself the finding.
- I could not find who wrote "rung-3 edges are what the zip MINTS". The
  `-S` search on the reduce.rs phrase lands on REACH/CURVED commits from
  2026-09-30/10-01. Neither commit is an Ev ruling.
