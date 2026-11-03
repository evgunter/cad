# PHASE 0: waiting for the orchestrator (splice/rim-arc-on-cylinder)

Scratch notes, removed before the PR. Base: main 27205d77.

## 1. Klein wall 3 on main, three ε

`demos/tour` `klein::verbs_gate_r1_probes::the_bottles_two_boolean_walls_name_the_pairs_the_scene_claims`:

| ε | wall 3 |
|---|---|
| default (1e-9) | `CurvedPierceUnsupported { operand: B, face: 4v1, edge: 1v1, band: [1e-9, 1e-8] }` |
| 1e-6 | same, band [1e-6, 1e-5] |
| 1e-12 | same, band [1e-12, 1e-11] |

(Wall 4 at all three: `CurvedPairUnsupported { Subtract, RevertRoster, B, 3v3 Nurbs, 3v1 Plane }`.)

**Site** (temporary backtrace on `frontier`): `curved_face_arm`, the uncovered
`(Sign::Zero, Sign::Zero)` arm, its `_ => Err(frontier())` fallthrough
(`crates/topo/src/boolean/reduce.rs:2898` on main), reached via `reduce.rs:1102`.

**Why the lying-on path does not take the arc**: it nearly does.
`wall_crossing` reads the arc **`SpanVerdict::LiesOn`**, so the circle-on-cylinder
reading exists. The arm's guard `parents_distinct_from(x, edge, y, face, band)`
(`reduce.rs:4180`) fails. The arc's two parents are:
- 2v1, the loop's start cap (`Plane`), `carrier_pair_relation` → `Ok(Ok(Distinct))`;
- 3v3, the loop's swept tube wall (`Nurbs`), `carrier_pair_relation` →
  **`Err(PairUnread::OutsideInventory)`**. `face_carrier` has no description for
  `Surface::Nurbs | Approx` (`carrier_pair.rs:238`).

So the gap is not the conic: it is that a **spline parent face is never read
distinct** from an analytic face.

**Probe of what lies beyond** (temporary patch, discarded): with
`OutsideInventory` admitted in `parents_distinct_from`, `lying_on` takes the arc,
including its first vertex on the cylinder's seam, and wall 3 moves on to
**`GermFrameUnsupported { a_face: 6v1 Cone, b_face: 4v1 Nurbs }`**. The seam
endpoint therefore needs nothing new: `lying_on`'s `interior` / boundary pass and
`vertex_on_curved_face` already place it. (Admitting `OutsideInventory` outright
is NOT the proposed fix. It is unsound: a spline face can lie on a cylinder.)

## 2. "Undeclared": a gap, not an owed declaration

- `BooleanError::CurvedPierceUnsupported` (`boolean/mod.rs:2993`) lists "an
  undeclared on-carrier edge or conic" among what the pierce door does not take.
  The `curved_face_arm` doc (`reduce.rs:2066`, "an undeclared on-carrier edge")
  and its `(Zero, Zero)` comment say the arm takes a `LiesOn` arc "only when every
  parent is decided a DIFFERENT carrier", and that every other answer keeps the
  door: "the undeclared cosurface question (CONTACT-DESIGN C2/C4), a parent the
  ladder does not decide distinct".
- History: the phrase arrives in `e99920ba90` (2026-08-29, "wip: fix-pass edits
  before probe adoption"), as a list of what the then-new line × wall pierce lane
  did not yet take ("a CIRCLE carrier against a wall (… no root lane in this tree),
  a SPHERE face, an undeclared on-carrier edge"). "or conic" arrives in
  `f64a00074c` (2026-11-01, the cone-apex fix pass), widening the same list. Both
  are refusal prose in a lane's own commit, describing a frontier. Neither is a
  design ratification.
- The ratified text says the opposite of "owed": DESIGN [verdicts], "A margin
  decided Zero is a coincidence and glues … **Nothing is declared.** The
  `unproven-coincidence` lint ([coincidence]) checks whether each coincidence so
  decided holds structurally" (`docs/DESIGN.md:457–462`), and [intent.checked]
  (`:97`). [coincidence.structural]: a mate is that rung's statement, so the
  REST mate is the lint's business, not the boolean's.
- **Verdict: "undeclared" names a reading the kernel lacks (a gap).** It is
  vocabulary left from the retired declared-pairs era (stage 4 retired the
  declared pairs and the undeclared refusals,
  `docs/doc-ledger/intent-leaves-the-tracker.md:18`). The arc's `LiesOn` margin
  is Zero. What blocks it is that the cosurface question, "is a parent ON
  `face`'s carrier?", cannot be answered for a spline parent.

## 3. Proposed fix

**The rung: a spline parent face is read DISTINCT from an analytic carrier
when every Bézier piece of its surface has a point definitely off that carrier.**

Soundness: on one knot-span cell a NURBS surface is a rational polynomial patch.
Plane, cylinder, cone (as the double cone), sphere and torus are algebraic. So
the carrier's equation along the patch, times wᵈ, is a polynomial in (u, v). If
the patch lies on the carrier over any open piece, that polynomial is zero on an
open set and so identically zero on the cell. Contrapositive: **one point of the
cell decided definitely off the carrier certifies that the cell lies on it along
no piece.** If every cell does, the spline sheet is nowhere cosurface with
`face`, which is the claim `parents_distinct_from` makes ("a curve where two
different carriers meet"). This is the same polynomial-identity argument that
`slit::carrier_on_surface` uses for a conic (`Samples`), applied to a sheet. It
is the converse of that function's spline caveat ("a spline can lie on a surface
along part of its length only"): per Bézier piece that caveat goes away.

Mechanics:
- In `parents_distinct_from` (`reduce.rs:4180`, its only callers are the
  uncovered `(Zero, Zero)` `LiesOn`/`Constant` arm and the covered-arc `LiesOn`
  rung at `reduce.rs:2412`), keep `Ok(Ok(Distinct))` as now. On
  `Err(OutsideInventory)` with a `Surface::Nurbs` parent, run the new reading.
  It is one-way: it returns Distinct or "not read", never "same".
- The reading, beside `carrier_pair` (or in `geom_brep`): for each non-empty
  knot-span cell of the parent's surface, evaluate the cell's centre (with
  fallbacks at a few interior points if the centre reads zero or in band), and
  decide its offset from `face`'s surface through
  `census::points_on_surface` (the same decided offset `RIDER_ON_CURVED`
  uses). Every cell definitely off gives Distinct. Any cell with no point off
  stays unread, so the frontier keeps the pair (conservative). An escalation on
  a point is not an answer for that cell; it tries the next point and refuses
  as now if none decides.
- `Approx` parents stay unread (no knot structure to argue from).
- It does **not** extend `carrier_pair_relation`'s inventory. That door also
  feeds glue, census, merge, holdings and `contact_verify`, where `Distinct` and
  `OutsideInventory` route differently (`contact_verify.rs:94` refuses
  NotCertifiable on `OutsideInventory`), so widening it would move findings
  outside this fence.

**The seam endpoint**: nothing new. `lying_on`'s boundary pass splits or places
an arc crossing `face`'s seam ruling, and the probe above shows it places this
arc's seam end.

**Sibling cases the same rung covers** (all gated by the same
`parents_distinct_from`, so all move together):
- a circle on a cylinder at its ring (this case), on a sphere at a latitude,
  on a cone at its ring, on a torus at a rim/latitude circle or a meridian,
  each with a spline parent;
- a ruling (`Constant`) lying on a cylinder wall with a spline parent;
- the covered-arc `LiesOn` rung (`reduce.rs:2412`) for the same shapes.

**Not covered, named**: a conic lying in a PLANE face with a spline parent goes
through the planar arms, not this guard; to be swept in the build. A spline
carrier edge on a curved face refuses earlier (`EdgeCarrierUnsupported`). A
Villarceau circle answers `Unsettled`, not `LiesOn`.

**Crease class / other sheets**: the reading is per Bézier cell, so a crease
(a knot of full multiplicity) is a cell boundary. Each side is read on its own
and a crease cannot hide a cosurface piece. It asks only "is this sheet on the
carrier anywhere"; it evaluates no foot and no inverse, so there is no
other-sheet foot to mis-place.

**Coupling with PR 6437** (`slit::carrier_on_surface` → `Placed::{On, Off,
Unread}`): that reading asks whether an EDGE's carrier lies on a face's surface
(curve × surface). This rung asks whether a PARENT FACE's sheet does (surface ×
surface), on the same decided point offset and the same polynomial-identity
argument. They are complementary: `LiesOn` (or 6437's `On`) says the arc is on
`face`, and this says the arc's spline parent is not. I will reuse
`census::points_on_surface` and not touch `slit.rs`, so the two do not conflict.

**Witnesses planned**: klein wall 3 re-pinned at its new stop (on the probe,
`GermFrameUnsupported { 6v1 Cone, 4v1 Nurbs }`, to be re-measured on the real
rung at three ε). The kernel row: a cylinder ∪ a body with a spline side wall
whose flat cap sits flush on the cylinder's rim, so the rim arc lies on the
cylinder with a NURBS parent. It goes red on the refusal. A disc capped on an
analytic cylinder alone would not exercise this (its parents are Plane +
Cylinder, read by the ladder), so the row needs the spline parent. Plus a
negative row: a spline parent that IS on the cylinder (a NURBS-exact cylinder
patch) stays at the frontier.
