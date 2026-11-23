## For Ev

**Recommendation, three decisions.**

1. **A body thinner than δ: `tessellate` owes distance and a closed manifold, not volume.** A flat mesh (the two faces' sheets on one set of points, volume zero to rounding) is a correct answer there, and no split rule can stop it. What must not happen is that anything takes that mesh for a solid silently: the door that needs a solid (STL export) reads the mesh's volume and refuses typed, with δ as the lever. (`likely`)
2. **A split point is defined in space, not in the chart:** the foot on the face's own surface of the side's straight midpoint (`geom::Surface::foot`, closed form, already in the kernel). On a sphere a split then cannot refuse, which recovers every body the unit loses. (`sure` on sphere and cylinder, `likely` on cone and torus)
3. **The unit merges with (2) and corrected wording. The flat cells do not block it.** The check in (1) and one README sentence belong to the parked row.

Your "allowing zero volume sounds annoying" points the other way on (1), so the alternatives are laid out below. It is your call.

**Terms.** A *patch* is one face's triangles. An *inside side* is a triangle side that is not a chord of the face's boundary. The *census* is the join's count of sides that two patches both have inside; each such side is *split* in each patch. *Flat* means signed volume at rounding. The *foot* of a point is the nearest point of a surface.

**Premise check.**

- **"Thinner than δ" has a sharp form: δ × the body's area ≥ its volume.** That is where `validate::check_volume`'s bound is vacuous. Below it, distance alone forces positive volume (to first order, the argument in `check_volume`'s doc). At or above it, a single sheet lies within δ of both faces, so the distance promise permits zero or negative volume from any mesher. For the cap of height h the threshold is about h/4; the review's flat and under-half cells sit at or above it as far as the numbers given show.
- **The base does not have the property either.** It returns negative volume on 23 bored-ball cells. Its 0.73 on the cap came from a rule that inserted a point on every side whose two ends one neighbour held (`faces_holding_side`), which the decision retired for cause.
- **The split was never the cure for flatness.** The census finds an edge of four triangles, a combinatorial fact. Collapse is geometric: two patches cover one region with the same points. The two coincide only when both lanes lay the same diagonal. At latitude +60 they do, and the cap keeps 0.59 of its volume; at −60 they lay different diagonals and it is flat. So under the unit, a thin body's volume depends on a triangulation tie-break. The table the decision rested on (+0.078 to +0.092) measured a family where every side is shared.
- **No local rule reaches the class.** Two faces closer than δ that share no point (a hollow ball's two spheres) can cross with nothing for a join to split.
- **The viewer asks for δ far above every body on purpose.** `scene::SCALE_PROBE_DELTA` is 1e9: every body is tessellated there for its cheapest mesh and its extent (`fit_from_probes`, `combine::measured_at_scale`). "Thinner than δ" is a routine call, not an edge case.
- **On the split point, the stretch near the pole is not the defect; defining the point in the chart is.** The cone already needed its development and an apex rule, both chosen by counting folds. The measurement the decision rested on used the foot of the 3-D midpoint as a stand-in; the build then switched to the chart midpoint, and that switch is what loses bodies.

**Decision 1 as three final states.**

*A (recommended). Distance only, stated, and checked where a solid is needed.*
- Makes true: `tessellate` answers every body at every δ with a closed manifold within δ. A public mesh-only check beside `check_winding` reads whether the signed volume is positive above the rounding of its own sum. The STL door calls it and refuses typed. The mesh README says the mesh may enclose nothing once δ × area reaches the volume.
- Leaves possible: a caller that skips the check holds a zero-volume `Ok`. A positive but tiny volume (0.015 of the truth) passes any sign check. Volume at coarse δ still depends on which diagonals coincide.
- Consequence: the thin cap at δ 0.04 is a doubled disc in the viewer, which is what δ = 40 heights means, and a typed refusal at export naming a smaller δ.
- Reversible: yes; a floor or a refusal can be added later.

*B. Keep sheets apart by construction.* Every curved patch splits every inside side whose two ends lie on one other face, at its own surface. This is the base's rule moved to the join, so no lane reads a neighbour, and it replaces the census outright.
- Makes true: the cap at ±60 meshes alike and keeps most of its volume, as on the base.
- Leaves possible: flat or inverted meshes wherever no such side exists (hollow shells, the coarsest meshes). It is not a theorem, so it still needs A's check.
- Consequence: it puts back the density the decision removed (about twice the inserted points on the trimmed lane; strips cut where the neighbour lays a row). It buys volume no reader can rely on: at these δ, 0.73 and 0.015 are equally within the promise.

*C. `tessellate` refuses a mesh that encloses no volume.*
- Makes true: no zero-volume `Ok` exists.
- Consequence: thin bodies fail the viewer's scale probe (`measured_at_scale` passes the refusal on as `StepClearFault::Unmeasured`), so a second door without the check follows. The 27 flat cells become bodies lost against the base.

I lean A. B and C each add a criterion beside distance that cannot be kept in general.

**Decision 2: the split point.**

Rule: each patch splits a side at the foot, on its own surface, of the side's straight midpoint. The new point then stands at most the parent triangle's own deviation from the side, which is at most δ. The chart midpoint has no such bound: the unit file records 5.5·10³ δ on a cone.

- **Plane:** the midpoint itself, as today.
- **Cylinder:** identical to today's chart midpoint.
- **Sphere (centre o):** the point is the radial projection m of (a+b)/2.
  - Orientation: det[a−o, m−o, c−o] is a positive multiple of det[a−o, b−o, c−o], so each piece winds as its parent.
  - Deviation: every point of a piece is a point of the parent pushed radially outward, so the piece is no farther from the sphere than the parent.
  - So a split cannot refuse a sphere face whose patch held. The 29 refusals in the class and the plain-ball cap at δ 1 and 0.3 are all sphere pieces wound against the sphere, by the trace in hand. (`sure` of the mathematics; `likely` that it clears every cell, pending the re-run.)
- **Cone, torus:** the same one rule, replacing the cone's development special case and the apex rule (from the apex the foot is the ruling's midpoint, which is what the apex rule says). No theorem; to be measured.
- **Spindle sheet, spline:** no closed-form foot exists; the chart midpoint stays, by surface kind.
- A side through a sphere's centre or an axis has no single foot, and the face refuses typed.

**Decision 3: what lands where.**

- In the unit before merge:
  - the foot rule, with the review's class re-measured to zero refusals where the base meshes;
  - the unit file and `join`'s docs saying the split cures the edge of four and promises no volume;
  - `signed_volume`'s doc corrected. It says an outward-wound closed mesh has positive volume, which the base's bored ball contradicts.
- In the parked row: the mesh-only volume check, the STL door's refusal, and the README sentence, which waits for your sign-off.
- The unit may merge returning a flat mesh where the base returned most of the volume, if you take A. Under B it may not.

**Ratified text.**

- `[tolerance.chordal]` ("DISTANCE-ONLY") and `[mesh.sizing]` ("as coarse as possible without extra work") are what make the flat mesh correct, and they weigh against B. Your remark on PR 7485 is later and points the other way. A keeps both pages and narrows the remark to "never silently, never into a file". B and C amend the distance-only sentence with a second criterion.
- Proposed addition to the promise paragraph of `crates/mesh/README.md`: once δ times the body's area reaches its volume, a mesh within δ may enclose no volume, and `tessellate` returns it; a reader that needs a solid checks.
- Proposed clarification of `[mesh.winding]`: "outward" is each triangle against its own face, and does not say the mesh encloses the body.
- Provenance, as far as a shallow checkout shows: `[mesh.sizing]` was rewritten under your review (S29, 2026-10-25); `[mesh.winding]` came through an `[ev]` PR (2026-11-14); the distance-only sentence predates the visible history.

## For the orchestrator

**Measurements that would decide.** Same instrument, base `dd8e70198d` and head `24ee9a28fb`.

1. **The coarsest δ.** The snowman cap (h 1e-3, both latitudes) and one sphere–sphere lens at δ 1e9 (`viewer::scene::SCALE_PROBE_DELTA`) and δ 1: per-patch triangles, positions, signed volume. I expect two coincident triangles and zero volume on both trees. If the base is flat there, "the base never returns flat" is false and state C is closed. If the base is positive at every δ, state B's case is stronger than I put it.
2. **`check_volume` on all 435 head cells**, with the body's volume and area from `topo::props`. Every flat or under-half cell should pass. One that fails is a certificate defect (a mesh farther than δ from its face), not this question, and it outranks it. In particular list (h, δ) for each flat cell: a flat cap at δ < h cannot exist under a sound certificate.
3. **The foot rule.** Split point = `geom::Surface::foot` of the straight midpoint on cylinder, cone, sphere and torus, read back to the chart on the branch nearest the chart midpoint for the uv-reading certificates (cone, torus), position = `S(u, v)`. Re-run:
   - the 435 cells and the plain-ball cap (lat −60, h 0.1, δ 1 and 0.3): refusals against the base;
   - the potential-side table of `a-side-the-join-could-split-has-a-piece-that-would-fold-or-fall-under-winding-resolution`. I expect trimmed sphere 297 → 0 and the cylinder unchanged; the cone against the development's 685 / 10 / 0 and the torus's 32 are unknown;
   - the stage-1 table at the three ε.
4. **Only if Ev leans to B:** over the `mesh` and `sweep` suites, the count of inside sides with both ends on one other face per lane, against the census's sides, and the triangle delta. The 997-against-502 figure covers the trimmed arms only.

**What I could not check.**

- I ran nothing. Every number is the brief's or the unit file's.
- Ev's words on PR 7485 are taken from the brief and the unit file; I did not read the thread.
- The checkout is shallow, so `git log -S` stops at grafts. I could not tell whether "DISTANCE-ONLY" is Ev's wording.
- The sphere theorem assumes the pieces are read as `join::held` reads them (`cert::cert_sphere` on positions, `wound_against` with the implicit normal at the centroid). I read both; that holds at this head.
- The cylinder's deviation claim (pieces no worse than the parent) is the same radial argument about the axis. Its fold read is not a theorem: the filed row shows 6 folds on 956,971 potential rectangle-cylinder sides, all on wide sides.
- That the threshold is δ·area ≥ volume rests on `check_volume`'s first-order argument, not on a proof.

**Corrections to the brief.**

- "The base met it by accident" is too generous. The base meets it on the cells measured and fails it elsewhere (its own negative cells; probably the coarsest δ, measurement 1).
- Question 2's premise ("where the point belongs on a face whose chart is stretched") assumes the point lives in the chart. The answer is that it should not.
- "No body that meshes on the base may be lost" is met by the flat cells (they return `Ok`). Whether a flat `Ok` counts as a loss is decision 1, not something the ruling settles.

**Defects off the question.**

- `validate::signed_volume`'s doc ("positive for outward-wound closed meshes (the global orientation invariant)") is false on main: the parked row's table is the counterexample. `check_winding`'s doc sends outwardness to `check_volume`, which is vacuous in that regime. Both belong to the parked row.
- The unit file's section "Why a point of the face's own surface, and in every patch" presents the reopened wall as the reason. It should say what was measured (a family where every side is shared) and that the split's job is the edge of four.
- `join::midpoint`'s doc records a rule chosen by fold counts on sides never split. Under the foot rule the function reduces to the chart read-back.
- The P3 row above now has its witnesses (the review's refusals). Its "what to do" (weigh a second split point) is decision 2; it should be re-prioritised or closed by the unit.
- That row also records 9,937 potential planar sides whose straight-midpoint half falls under winding resolution (`resolved_normal`'s allowance does not halve). If a second patch ever makes one, the plane face refuses `UnresolvedWinding` where the base meshes. No witness yet, and the foot rule does not touch it.
- `combine::measured_at_scale` passes a scale-probe refusal straight on, while `fit_from_probes` falls back to a finer rung. The two probes answer one refusal differently. This matters only if tessellation ever refuses at coarse δ (state C), but it is an inconsistency today; it belongs to the viewer's tracker.

**Sketch for the parked row, if Ev takes A.** A mesh-only check: `signed_volume` exceeds `cert::rounding_allowance` at the mesh's coordinate scale times its total triangle area, the global twin of `resolved_normal`, reading no ε and no δ. It is necessary for a solid's boundary, not sufficient: a locally collapsed fin on a thick body passes, which is correct at that δ. A per-shell read needs the body (a void's shell is rightly negative) and would sit beside `check_volume`, not in the mesh-only check.
