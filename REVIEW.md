# Review r2 — PR 4399 (tube ending on a ball; `GermLane::EdgePlane`)

Frozen head `8e56086`, base `b1a15ad0` (the main it merged); release builds, separate
target dirs. Probes: `crates/sweep/tests/r2_probes.rs` (12 families, 332 head lines, each
through `differential::outcome` + `check_mesh`); raw lines in `review-r2/`. Lane
isolation kept: no other review branch or session read; PR comments not read.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 5. No wrong body anywhere
(0 BAD / 0 WRONG / 0 UNMEASURED in 332 head lines); the fixes are a scope line and a pin.

## Claims

1. **Bodies right — holds (executed).** Cap re-derived as `πh²(3R−h)/3`, `z0=√(2−a²)`;
   the cap's radius `√(2−z²) ≤ a` above `z0`, so it is the shared volume (matches the
   PR's 0.687856162 / 11.159831674 / 17.443016981). Head: 187 SOUND lines (base 49).
   - Near the equator, a = 1.2, 1.4, 1.41, 1.414, 1.4142: every op, both orders, SOUND
     and meshed; tilted a=1.4 SOUND (mesh refuses typed, `NotIsoR…`, TESS's filed gap).
   - Downward (tilt π; a = 1, 0.5, 1.4; turned π/2; tilt π−0.4): all SOUND.
   - Rim offsets dz = ±1e-12, ±3e-10, +1e-7: SOUND at the offset's own closed form;
     dz = −1e-7 refuses `GermFrameUnsupported{Cylinder,Sphere}` (truly transverse — right);
     tilted dz = ±3e-10 refuses `Escalated{Crossing(OnEdge)}` at margin 1.2e-9 (justified).
   - Two tubes on one ball (up+down; up + an r=0.3 tube across a seam meridian): 20/20
     SOUND. A plane through both: `(tube∪ball) ∩/∖` x≥0, y≥0 (the seam plane), z≤2:
     9/9 SOUND; a tube on a half-ball whose flat face its rim crosses: 6/6 SOUND, meshed.
   - **Cones/frusta (F3, F4: widening, narrowing, to apex, tangent to the ball, downward,
     a cone face holding the ball's seam meridians; × 2 turns) all refuse
     `CurvedPairUnsupported{Cone}`** at the operand gate / revert roster, identically on
     base. They never reach the join (N4); not this PR's refusal.
2. **One-sided frame — holds (executed).** Where an existing arm also reads the germ,
   base and head lines are identical: line edges in plane × cylinder (a square prism
   inscribed in a rod, two turns; a tube's seam line in a y≤0 half-space), and a circle
   edge whose face pair HAS a conic frame (plane × sphere: a pipe whose inner rim lies
   on the ball, F12, 24/24 SOUND on both). The ball's seam meridians in a y≤0 face refuse
   `SectionLoopUndecided` on both (pre-existing). A NURBS-carried seam (loft prism edge
   at (1,1,z) in a face x=1) is refused at the gate on both: the new arm is unreached (N3).
3. **`EdgePlane` sound — holds (executed + inspection).** A plane meets a quadric in a
   degree-2 curve; containing the edge's conic it IS that conic, so sphere, cylinder and
   cone carriers have no second component. A torus can have one, but `wall_section`
   refuses torus faces typed (`chord_join.rs:1479-1487`); F8 (tube on a torus whose rim
   plane cuts it in a second circle) refuses typed. Cylinder holder across its seam: F10
   (a dome's sphere-face rim in a rod's wall, the rod's whole turn split by the rim plane,
   rod seam on and off the dome's) is 12/12 SOUND and meshed.
4. **Dispatch scope — holds for existing arms; coaxial arm unreached.** By code
   `edge_lane()` is called only at `join.rs:1069,1072`; batteries moved 0 lines; mutant
   M2 (EdgePlane first for EVERY pair) moves no probe line and no row — where both lanes
   apply they compute the same circle (so the restriction is also unguarded, m2).
   Coaxial `None`: three coaxial poses (F9: rod in a pipe's bore of its radius, through /
   ending inside / ending at its rim) refuse `CurvedPierceUnsupported` earlier (m1).
5. **Nothing else moved — holds (executed).** `pierce_runs_battery` (4540 lines) and
   `rc_wide` shards 5, 20, 34, 48, 62, 77 /84: 0 lines moved, 0 BAD. Whole `sweep::all`:
   base 2491 pass / 3 fail, head 2496 / 3 (+5 new rows), the same three `cone_join_lane`
   rows, panic text identical ("the interior-loop guard refuses the cone pair…", plane ×
   cone poses): EdgePlane is not on their path, so GERM's re-pin sees the same on either.
   The full run covers `cylinder_sphere_frame`, `tilted_sphere_pair*`,
   `conic_edge_curved_face`, `pi_seam_and_kiss_*` (TANG's abutting-rim rows): identical.
6. **Design fit — NOTE N1, likely: the planar instance, not a graft.** For the witness the
   face pair (tube wall × sphere) meets in two circles z = ±z0; the component through the
   sites is the rim, and its plane is EdgePlane's. 4313's row already says the `AlongEdge`
   chord "copies an edge whose curve is known" and the planar lanes are closed-form
   instances. To re-home when frames retire: the datum is keyed by edge
   (`AuxDatum::EdgePlane{on,curve}`), not face pair; and the coaxial routing (m1) runs
   against the row's "coaxial … decided at the section door, never in the join".
7. **Mutants — real (executed;** rows: the five new + `pi_seam`, `germ_circle_torus`,
   `mate7a_r1`, `cylinder_sphere_frame`, `tilted_sphere_pair`, `conic_edge_curved_face`;
   `review-r2/rows-m*.txt`, `probe-m*.txt` (mutant probe runs predate F10–F12)**).**
   - M1 revert the frame: **6 red** (four a_tube rows,
     `germ_circle_torus::the_lily_stem_glue…`, `mate7a_r1_probes::p1_wall1…`).
   - M2 EdgePlane for every pair: **0 red, 0 probe lines moved.**
   - M3 swap `on`: **4 a_tube rows red**, typed `SectionInvariant` ("curved section chord
     outside the split/wall lane") — loud, never a wrong body. The inside row survives
     all three (no-crossings path).

## Findings

- **m1 MINOR — the coaxial `None` arm routes to EdgePlane** (`join.rs:1069`; inspection +
  executed-unreached). `parallel_radical_plane`'s doc (`join.rs:1986-1987`): coaxial
  walls "meet nowhere or everywhere", so an `OnEdge × InFace(cylinder)` germ there is a
  coincident wall — D10 ground, and the ratified row puts coaxial pairs at the section
  door (refuse until INTENT E). No pose named, no row pins it. Keep `no_arm()` or pin one.
- **m2 MINOR — the scope claim has no guard** (executed: M2 survives every row and
  probe). "Taken ONLY at the fallthrough" is true by code; nothing goes red if it widens.
- **N1 NOTE** — design fit (claim 6).
- **N2 NOTE — the `on` side's ring closure is `Wall(plane)`** (`join.rs:224`) though its
  chord is `AlongEdge`; the both-sided case uses `RingClosure::AlongEdge`
  (`join.rs:1084-1088`). A planar `on` face in the ring lane (not across the edge) goes
  `RingFace::Wall` → `chart_island_winding` (`join.rs:2898, 3077`). Unreached (F12's
  ring rim took `PlaneWall`); unsure it is reachable.
- **N3 NOTE** — a one-sided germ on a non-conic edge is now `JoinDesync`
  (`join.rs:1725-1729`), where the face-pair read refused typed; unreachable today (F11).
- **N4 NOTE** — cone × sphere is gated before the join, so no cone pose exercises
  EdgePlane (F3, F4).
- **N5 NOTE, pre-existing (not this PR)** — `rod ∖ inscribed square` (F7) builds SOUND,
  then `mesh::tessellate` **panics** (`tessellate.rs:592`, "edge of 4 face triangles":
  four pieces meet along lines), identically on base. Class of
  `work/tess/tessellator-panics-on-a-self-slit-face-every-validator-passes.md`; worth filing.

## Style (exercised Q1 Q2 Q3 Q4 Q6 Q7; Q5 partly; Q8 not — read join.rs 900–1220, 1650–1780, 2800–3110 of 5825)

- Q1 · `join.rs:985-1001` vs `1716-1729`: the edge curve is read twice (`get_edge` →
  `get_curve_geom` → `certified` → Circle|Ellipse) with diverging desync texts — one rule
  ("a one-sided germ's datum is its edge's conic"), two homes. **sure**
- Q4 · `join.rs:1683-1693`: the frame doc still says the match is "EXHAUSTIVE over kinds
  by construction" and an unwired pair must refuse `GermFrameUnsupported`; every one-sided
  `OnEdge` germ now bypasses it. Doc rotted, code right. **likely**
- Q4 · `join.rs:956-958`: `no_arm`'s list ("cyl×sphere's rung-3 fitted chords, a coaxial
  cylinder pair") no longer says their along-edge forms now take a lane. **likely**
- Q3 · `a_tube_ending_on_a_ball.rs:186`: the inside row survives all three mutants — it
  pins an unrelated path and cannot go red on this change. **sure**
- Q3 · no row distinguishes EdgePlane from the arm beside it (m2). **sure**
- Q6 · PR Deviations: the 47 × 6 × 3-ε "0 BAD" family ran from an uncommitted scratch
  example; CI's ε loop (`ci.yml:341`) re-runs only the five rows. The headline
  measurement has no register. **likely**
- Q7 · the `on`-side closure (N2): the edge side reads no section, so its closure should
  say so, as `AlongEdge` does. **likely**
- Q7 · `edge_lane` refuses a planar holder by kind (`join.rs:982`) after the dispatch
  sent planes to `PlaneWall`: it only bites when the germ face's kind differs from the
  edge-holding face's. Correct but subtle, pinned only by the demos. **unsure**
- Brief check: base, head, CI red and re-pins match the tree. **sure**

REVIEW COMPLETE
