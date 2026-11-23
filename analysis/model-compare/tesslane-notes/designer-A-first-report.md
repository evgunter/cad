## For Ev

**Recommendation (likely).** Build one chart-generic lane for every curved face. The trimmed lane's machinery becomes that lane, and the curved (iso-rectangle) lane is retired into it. Don't add a cone arm, a torus arm, a ring arm and a notch route one at a time.

Definitions:
- **Chart lane:** what `trimmed.rs` does today, made chart-generic. It builds a UV polygon from every loop's stored lift, inserts a lattice, runs a CDT with even-odd classification, and checks a per-triangle certificate.
- **Box bound:** for a chart, the worst certificate any triangle can have when its UV extents fit inside a box `Δu × Δv`.

A face then needs three things from its chart: eval, its certificate, and its box bound (with the steps that put that bound at δ). It also needs the chart's singular rows (poles, apex) and its normal. From each boundary loop it needs only the lift and a UV speed bound for each half-edge's image.

**Premise check (sure):**
- The routing facts in the brief hold. `lane_of` reads only the outer loop's carrier kinds. Both curved lanes refuse a ring before reading any loop. `trim_frontier` is where torus × spiric and cone × ellipse refuse. The notch is refused at `require_iso_rectangle_face`, although the trimmed cylinder arm exists and every edge of the face stores a `Harmonic` image.
- The question is one level up. "Rectangle vs not" shouldn't be a routing split at all. The curved lane is the trimmed lane's special case: a polygon that happens to be its own bounding box. Both build a CDT of boundary plus lattice, checked per triangle. The real differences are two:
  1. **Where the polygon comes from.** The curved lane inverts the 3-D chord points through the chart and unwraps them (`walk::loop_polygon`). The trimmed lane reads the stored lift (`trim_polygon`). DESIGN `[one-way.places]` says a loop's position in its face's chart has one home, `topo::pcurves::Lift`, and that a second place is a defect. The tier-3 clause of `[topology]` says the mesher's domain "reads the loops as stored … through the lift's whole turn" at a singular vertex. That clause came through the `[ev]` tier-3 PRs of 2026-10-30/31. The `[one-way.places]` row is agent-written (2026-11-12), and I could not find your own words on it.
  2. **Which sizing theorem applies.** It is a cell-half when every boundary point sits on the lattice, and a sliver beside a point the lattice did not place otherwise.
- A separate arm for each (chart, boundary) pair is not forced. Only one thing in the lane depends on the boundary's form: the UV speed bound the chord pass needs. That can be one rule: the carrier's 3-D speed times the chart's inverse-Jacobian bound over the image's window. The closed forms we already have (Harmonic, IsoLine, IsoArc, General) stay as tighter special cases.
- Every certificate the lane checks is already chart-local and does not care how the lattice is shaped: `cert_cylinder`, `cert_sphere`, `cert_cone`, `cert_torus`, `cert_spindle`, and the NURBS cell bound. Only sizing is missing.

**The final state (what the code is):**
1. **Routing.** `lane_of` sends Plane faces to the planar lane and every other face to the chart lane. These retire:
   - `has_trim_carrier`, `has_tilted_circle`, `takes_sphere_arm`, `sizes_at_headroom`;
   - `curved.rs`, the chart-inverse walk in `walk.rs`, and the five-question doors (`require_iso_rectangle_face`, `MeridianFreeCurvedFace`, `SingleColumnCurvedFace`);
   - `RingOnCurvedFace`, together with its Display that blames a kernel bug.
2. **The polygon.** It is built from every loop, rings included, read from the lift. A ring is placed in the outer loop's window by one integer deck element, decided in `topo` as a method on the lift. A valid ring cannot cross a seam edge, so exactly one placement fits. Chart singularities (sphere poles, cone apex, spindle apices) are sides of the polygon, through the lift's whole turn there. That is today's sphere pole side generalised over `Chart::poles()`, and it absorbs `curved::pole_columns` and `pole_side_columns` into one #678 rule.
3. **One chart table**, one entry per chart: cylinder, cone, sphere, torus, spindle, NURBS. Each entry gives eval, the certificate, the box bound and its inverse steps, the singular rows, and the normal used by the fold census (the census reads only the cylinder's and sphere's radial direction today).
4. **One lattice.** It has lines at every iso coordinate the boundary has (each rim's `v`, each meridian's `u`), subdivided down to the chart's steps. This is the NURBS per-cell grid generalised. An iso-rectangle, a notch or a keyway therefore has every boundary point on the lattice, and meshes at the curved lane's present density. On a ruled chart (cylinder, cone) the box bound does not read `Δv`, so rows are added only where the face has a point off the lattice lines.
5. **One theorem**, written once and pinned per chart in the shape of `torus_grid_steps_meet_the_bound_with_equality`. It has two cases:
   - every boundary point on lattice lines: a cell-half, or a strip on a ruled chart;
   - otherwise: the constrained-Delaunay sliver beside an off-lattice chord point, in the lattice metric, given chord UV steps within the lattice steps. The NURBS lane's `LATTICE_SHARE` lemma is the template.

   Steps are the box bound solved at δ for whichever case applies. `UNPROVED_HEADROOM` is deleted.
6. **One chord tightening door.** It replaces `grid_boundary_step`'s circle arm, `sphere_arm_tighten`, `nurbs_tighten` and `edge_target`'s headroom. Each adjacent face that is a chart-lane face raises the count to `⌈s·Δt/h⌉` per axis. That makes the four-caller list in `chords`' docs, which nothing enforces today, a single door.

**What it closes:**
- The three P0 rows: torus × spiric, cone × ellipse; rings on a cylinder, cone, sphere and torus; the notch.
- Two P2 debts: the trimmed cylinder and trimmed sphere have no theorem.
- Three P3 sizing rows: a cone split side, the thin-torus aspect, an ellipse skipping the lattice step.
- The three lattice emitters of `one-element-grid-axes-drop-schedule` become one.
- The sphere-bounded-by-spline row: it needs nothing more once its carrier is C¹.

**What it does not close:** the near-pole chord cost of a tilted circle (P3). The `1/cos v` speed belongs to the chart, and stored rows live in the surface's chart. A face-local chart is a separate question, and I would not open it here. Nothing here touches the interior-pole form you tabled, (E). By `[topology]`, a singularity is always a vertex with a wrap edge, and the lane reads it as a pole side.

**Alternative, also defensible: keep the curved lane for iso-rectangles and route everything else to the chart lane by props' rectangle predicate.**
- *Pros:* rectangle meshes don't move, and no replay re-baseline is needed for them.
- *Cons:*
  - two readers of a loop's chart position, which is a `[one-way.places]` defect;
  - two pole rules and two lattice emitters;
  - a routing predicate that the chord pass must also read, in step with the dispatch;
  - the fragile walk code (#653, #678, issues 896 and 1571, the five doors) kept for faces the general lane would mesh at the same density.

I lean against it. Choose it only if the lattice of item 4 cannot match the curved lane's density on rectangles; tess-meter can measure that before the curved lane is deleted. Either way is reversible: the alternative is the recommendation's midpoint.

**Rejected:**
- *One arm per (chart, boundary) pair.* That is the current path, roughly ten arms.
- *Normalising rings and notches away at construction.* `[topology]` says "a wall keeps the cuts its author wrote: the kernel adds none". It would also leave the trimmed-chart cells unmeshed anyway.

**Worked example: a pipe wall holding a ring of four ellipse arcs.**
- The outer loop lifts to `[α, α+2π] × [z0, z1]`, with the seam walked at both `α` and `α+2π`.
- The ring's lift is shifted by the one deck element that puts it inside that window.
- Lattice columns are uniform at the cylinder's step. Rows every `r·h_u` appear only because the ring puts points off the lattice.
- The ellipse chords are tightened so their UV steps fit within `(h_u, h_v)`. Even-odd classification drops the hole.
- Every triangle is checked by `cert_cylinder`. The sliver case of the theorem sizes the steps.
- The same wall without the ring takes no rows, as today.

**Confidence:**
- One lane (recommendation): likely.
- Lift-only polygon, rings placed by one integer: likely.
- Lattice of item 4 matching curved-lane density: likely; it is measurable.
- The constrained sliver lemma holding at a constant that is not ruinous: unsure. It is proved nowhere. The NURBS pin (`a_strip_lattice_off_row_sliver_stays_inside_the_box_bound`) only samples one constraint segment.

**Ratified text:** none needs changing. `crates/mesh/README.md` `[mesh.sizing]` stays as it is in substance. Its list of lanes and its rim-chord sentence get reworded with the code, which follows from the change rather than being a new decision.

## For the orchestrator

**Corrections to the brief:**
- It says "Five rows at the top band". Three open P0 rows are instances, holding seven measured cells. The other two P0 rows from that measurement are the two the brief excludes.
- The two debt rows and the NURBS theorem text disagree. `the-trimmed-{cylinder,sphere}-…-no-worst-triangle-theorem` say a CDT triangle beside a trim chord is unbounded. `nurbs_cert::LATTICE_SHARE`'s doc claims its sliver lemma covers trim chord points ("adjacent chord points of a slanted trim edge"). Its pin samples one constraint with lattice on both sides; it is not a proof. Settle this once, chart-free: the lemma lives in the CDT plane, so it is the same for every chart. That proof is the riskiest piece of the design.

**The two excluded defects:**
- **C⁰-knot section spline:** not the same problem. It is a carrier mint defect under the PR 5913 ruling. It does block the sphere × spline and cylinder × ball cells from ever reaching the chart lane.
- **Ulp-tilted ball:** not the same problem, but its raising site is in code this design retires. The pole entry's `u = −4.838e−49` is below spade's `MIN_ALLOWED_VALUE = 2^−142 ≈ 1.79e−43` (spade 2.15.1 `validate_coordinate` returns `TooSmall`), so `insert` refuses it. Every CDT door (planar, curved, trimmed's `bulk_load_cdt`) can meet this. Fix it at the chart lane's one insertion, or at the conditioning of the chart frame (`[simple.conditioned]`), with or before unit 3. Confidence: likely on the mechanism. I did not run it.

**Units, in landing order.** The P0s close at unit 3, under the README's allowed named-headroom debt. Unit 4 removes the debt everywhere; unit 5 removes the second lane.
1. **`topo`, the lift (E–M).** Place a face's rings in the outer loop's window by an integer deck element, as a lift method beside `window`/`azimuth_window`. Also widen `sphere_face_widest_cos` to read every loop; today it reads the outer loop only and requires circles.
2. **`mesh::chords` (M).** One tightening door, with the speed bound "carrier speed × chart inverse-Jacobian over the image window" for Projected, FocalSection, Spiric and Ellipse images on every analytic chart. Keep the existing closed forms. Closes `an-ellipse-edge-on-a-sphere-or-torus-face-skips-the-lattice-step`.
3. **`mesh::trimmed` becomes the chart lane (H).**
   - Read every loop.
   - Add the cone, torus and spindle table entries, with generic singular sides from `Chart::poles()`.
   - Use the chart normal in the fold census.
   - Interim `lane_of`: Plane → planar; rings, a non-rectangle, or a trim carrier → chart lane; otherwise curved. The new charts carry `UNPROVED_HEADROOM`, filed as one debt.
   - Retire the `RingOnCurvedFace` path and `trim_frontier`.
   - Closes the three P0 rows.
4. **The theorem (M–H).** Box bounds per chart, the constrained sliver lemma, and pins per chart. Delete `UNPROVED_HEADROOM`. Closes the two P2 debts, `a-cone-face-with-a-split-side…` and `a-thin-torus-lattice…`.
5. **Retire the curved lane (H).**
   - Iso-aligned lattice lines.
   - Route every non-plane face to the chart lane.
   - Delete `curved.rs`, the chart-inverse walk and its doors, and `grid_counts`. Fold `pole_columns` into the generic rule.
   - Measure rectangle density against the current curved lane with tess-meter before deleting. If it loses, stop at unit 4; that is the alternative in the Ev section.
   - Re-baseline `docs/TESS-BUDGET.md` and the D9 replay pins once, here.
   - Closes `one-element-grid-axes-drop-schedule` structurally.

**Not checked:**
- Whether every chart-lane face's `Lift::read` succeeds on today's corpus. The topo README says a missing row is loud at rest, so I assumed full coverage.
- Whether the lift's turn at a cone apex reads as a polygon side exactly as at a sphere pole.
- The P3 row `a-trimmed-sphere-face-holding-a-pole-inside-it-refuses` may be partly stale. Under `[topology]`, a pole in a face's closure is a vertex with a wrap edge, which the lane reads as a pole side. Re-read that row.

**Provenance:**
- The `[topology]` mesher's-domain sentence: commits `f0bc8efc03` and `ed48558d90`, both `[ev]` PRs.
- The `[one-way.places]` lift row: commit `ccd4828d6d`, agent-authored, ONEDOOR, 2026-11-12. I did not find Ev's words on it.
- The checkout had the history these needed, so no extra fetch was required.

No files were changed. I did not run cargo.
