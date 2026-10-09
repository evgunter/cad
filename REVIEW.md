# Review r1 — PR 4399 (EdgePlane: a tube ending on a ball), frozen head 8e560860

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 7. I found no wrong body: 171 SOUND, 0 WRONG, 0 mesh-BAD over 297 probe lines. Both BAD lines are pre-existing and identical on base (N4).
Isolation: I read no other review branch or session and none of the PR's comments, only its body (API `get`). Base is b1a15ad0.
Probes: `crates/sweep/tests/tube_on_ball_review_r1_probes.rs` (ignored rows). Raw lines, mutant patches and red-row lists are in `review-r1/`.
Builds were release, with separate target dirs for base, head and mutants.

## Claims

1. **The bodies are right: HOLDS (executed).** Closed form, re-derived: for z ≥ z0 = √(R²−a²) the ball's section radius √(R²−z²) ≤ a, so the ball keeps only the cap above the rim plane. Shared = πh²(3R−h)/3, h = R−z0; for len < h it is cap(h) − cap(h−len). This matches the PR's table (e.g. ∩ 0.687856, ∪ 17.443).
   All SOUND, every op in both orders:
   - radius 1.3, 1.4, 1.41, 1.414 (z0 down to 0.020);
   - the tube downward (tilt π; π with turn 0.4), and askew below (tilt π−0.5, turn 0.3);
   - short tubes whose top disc also cuts the ball (len 0.2 up and down; a=1.3, len 0.1);
   - two tubes on one ball, the second against `ball∪up` and against `ball∖up` (down, down turned);
   - a plane through both: the ball halved by a brick at x ≥ 0, 0.3 and −0.4, and x ≥ 0 with the tube turned. These are measured against a z-integrated segment-area oracle.
   - The askew second tube rings one ball face, so ∪ and `ball∖tube` refuse `RingOnCurvedFace`, as the PR's tilt-1.2 row does; the other ops are SOUND.
   - Cones and frusta (narrowing, widening, to an apex, downward, rim near the equator) all refuse `CurvedPairUnsupported` at the operand gate or revert roster, the same on base. That refusal is justified and up front; the lane never sees a cone (N3).
   - The meshes of the untilted poses pass `check_mesh`. The askew, offset-plane and some two-tube bodies refuse the mesh (`NotIsoRectangle`), a mesher frontier and not this PR's.
2. **The one-sided frame is right where it fires: HOLDS (executed, plus inspection for the agreement case).** `Locus::OnEdge` is minted only when the germ runs along the edge, i.e. the edge's far end touches the partner (sectors.rs:984-991). A germ on it lies on the edge, so the edge's conic is the section near the site.
   - In-band rim, d = ±1e-12 and ±1e-10 off the sphere: SOUND, all 6 ops each.
   - d = ±1e-9 and ±1e-8: typed `Escalated`, except `ResultInvalid` for A∪B at +1e-9 (order-asymmetric but typed).
   - d = +1e-6: SOUND through the disc × sphere lane. d = −1e-6: `GermFrameUnsupported{Cyl,Sphere}` (a transverse crossing, honest).
   - Spline rim: unreachable. The operand gate refuses spline edges up front (`CurvedEdgeUnsupported`, boolean/mod.rs:1994), so the frame's desync arm (join.rs:1724) stays dead.
   - A pair that HAS a conic frame: I tried a brick face holding the ball's seam meridians (plane × sphere). It refuses `Join(SectionLoopUndecided)` identically on base, so agreement was not exercised by execution.
   - A box corner edge (a line) on a cylinder wall reads identically on base and head (`straight` both ways).
3. **EdgePlane is sound: HOLDS where reachable (executed for the sphere; inspection for the rest).**
   - A plane meets a sphere, and a cylinder or a cone carrying a conic, in that one conic.
   - The only multi-component carrier is the torus. There `section_case` (chord_join.rs:930) has no torus arm: a torus holder falls to `plane_cylinder_section`'s wrong-lane refusal, so it is typed and nothing is cut. It refuses rather than cutting where the section is not, but the refusal names the cylinder lane.
   - I found no undeclared pose that reaches a torus holder: every tube-on-torus pose first hits a plane × torus or cylinder × torus germ (`GermFrameUnsupported`).
   - Holder-side `Wall(plane)` closures on a sphere route to `RoleLane::QuadricRing` (join.rs:2897), not the chart closure. The seam-crossing and ringed-face poses build SOUND.
4. **The dispatch scope is as claimed: HOLDS (inspection plus a mutant).** `edge_lane` is called only at join.rs:1069 and 1072, both former `Err(no_arm())`. The coaxial `None` arm needs coincident walls: two coaxial r=1 tubes overlapping refuse `UndeclaredCoincidence` upstream in all six ops. I found no undeclared pose that reaches it (N2).
5. **Nothing else moved: HOLDS (executed).** Base vs head, release:
   - `pierce_runs_battery` (4536 lines): 0 moved.
   - `rc_wide` shards 5, 20, 34, 48, 62, 77 of 84 (480 each): 0 moved.
   - `near_tangent_battery` (7200 lines): 0 moved. `join2_r1_tangent_radii` (144 lines): 0 moved.
   - The full sweep `all` suite: the pass/fail sets differ only in the PR's four new rows. That covers `tang_circle_cylinder`, `cylinder_sphere_frame`, `tilted_sphere_pair{,_k_rows}`, and the re-pinned rows on each side.
   - `cone_join_lane`: its three rows fail with the same messages on base and head, at the first assertion (the guard returned `Ok`). They are plane × cone (`PlaneWall` arm), and `EdgePlane` refuses plane holders, so the lane cannot change what GERM's re-pin sees. The one-sided frame could matter only for an along-rim germ there; under M1 (frame reverted) the three rows fail identically.
6. **Design fit: NOTE (N6).**
7. **The mutants are real: PARTLY.**
   - **M1** (frame back to both-sided): 6 red. These are the 4 new suite rows (`every_op…`, `tilted…`, `rim_inside_one_ball_face…`, `witness_brackets…`), `germ_circle_torus::the_lily_stem_glue…` and `mate7a_r1_probes::p1_wall1…`. 126 probe lines go SOUND → `GermFrameUnsupported`.
   - **M3** (`on` swapped): the same 4 suite rows go red, with `ERR Join(..)`.
   - **M2** (`EdgePlane` ahead of every kind arm): **0 rows red, 0 probe lines moved** (N1).
   - **M4** (the planar-holder guard deleted): **0 rows red** in sweep `all`, 0 probe lines moved, and **demos/tour's suite stays green** (MINOR-2).

## Findings

- **MINOR-1 — `AuxDatum::EdgePlane` is keyed per piece, not per edge (executed).** join.rs:302-307 says "the pieces of one split edge share it", and join.rs:272-275 promises "one mint per datum … key-coherent for D6". `split_edge` mints a fresh curve per child (split.rs:366-367).
  - Instrumented split_curve, witness: the one rim mints **2** aux planes (`CurveKey 4v3→SurfaceKey 1v3`, `5v3→3v1`); turned 0.4 it mints **4**, each `new=true` (`review-r1/m4i-aux.log`).
  - Bodies stay SOUND, so no geometry is wrong. Chords on one circle are described against distinct keys of one plane.
  - Fix: key by the plane datum (or the uncut edge), or correct the doc.
- **MINOR-2 — the planar-holder guard is unpinned (executed).** join.rs:981-983: deleting it (M4) turns nothing red in sweep or demos/tour. The PR body says CI's demos caught its absence on a first head; at this head nothing does. A planar holder would go to `section_case`'s plane × plane invariant (chord_join.rs:944) instead of `CurvedBooleanUnsupported`.
- **MINOR-3 — prose the change made false (inspection).**
  - join.rs:877-887: "any other pair refuses typed citing its C5 routing".
  - join.rs:956-958: `no_arm`'s "cyl×sphere's rung-3 fitted chords, a coaxial cylinder pair" now take EdgePlane when a germ runs along an edge.
  - join.rs:188: `GermLane` is "the chord lane a germ face pair joins by", yet EdgePlane is picked by locus, not face pair.
  - reduce.rs:491-492: "the join's germ frame along an edge of both solids" is now either solid.
- **N1 — M2 is an equivalent mutant on every reachable pose (executed).** Where an arm exists and the germ is along a conic edge in a curved face, the arm's datum is the edge's plane: a disc's plane, or a radical plane through a meridian. So the scope claim is true but unpinned. That is evidence for N6, not a defect.
- **N2 — the coaxial `None` → EdgePlane change has no reachable undeclared pose (executed).** Overlapping coaxial same-radius tubes refuse `UndeclaredCoincidence` before the join. Under D10 that arm is dead code until a declared coincidence reaches it.
- **N3 — the cone/frustum/torus holders are unreachable today (executed).** Cone pairs refuse at the operand gate; torus poses refuse in the frame. EdgePlane's live reach is cylinder × sphere, both orders.
- **N4 — pre-existing BAD, not this PR (executed, base identical).** Box [0,2]²×[0,1] ∩ the cylinder about (0.5, 0.5), r = √0.5, z ∈ [−1, 2], whose wall passes through the box's corner edge. A∩B and B∩A build `t2=true t3p=false cert=true`, at the right volume 1.285398163. The body fails tier 3′. Worth an issue file (JOIN or TANG).
- **N5 — in-band asymmetry (executed).** At d = +1e-9: A∪B `ResultInvalid{VolumeUncomputable}`, B∪A `Escalated`, A∩B and A∖B SOUND. All typed, but one pose gets three different answers by op and order.
- **N6 — design fit (likely).** EdgePlane is the planar instance of the ratified section-first design rather than a graft.
  - Its datum is a plane whose cut of the holder is the section, exactly as the radical planes are, and M2 shows it coincides with every planar arm where both apply.
  - It also adds a third `RingClosure::Wall` user and a join-side datum (the aux plane), both of which "The shape to give" retires.
  - It routes the coaxial pair through the join, where that row says coaxial pairs are decided "at the section door, never in the join" (unreachable now, N2).
  - Cheap to retire; it does nothing wrong now.
- **N7 — mesher (executed).** Untilted two-tube and x ≥ 0.3 bodies refuse `NotIsoRectangle`, and line/crossing A∪B refuses `Triangulation`. These are mesher frontiers; only the tilted-circle gap is filed (TESS).

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q7, Q8 partial; Q6 skimmed)

- **S1 (Q1, sure).** `edge_lane` (join.rs:975-1001) and `germ_section_frame` (join.rs:1710-1730) are two spellings of one "conic edge of a one-sided germ" read.
  - They differ in precedence: `(OnEdge, _)` first versus exactly `(OnEdge, InFace)`.
  - They differ in their desync wording: "no longer resolves" versus a folded "carries no certified curve".
  - They differ in the curve match (Line → `None` versus Line → `no_arm`).
  - insert.rs:1855 holds a third locus-to-body dispatch. The one-sided locus read has no home.
- **S2 (Q2/Q4, sure).** The MINOR-3 sites. The PR's own sweep ("Prose moved by the change") updated chord_join and `AuxDatum` but missed reduce.rs:491 and the two dispatch comments. That is the class-not-instance shape: grep "both solids", "any other pair" and "no wired join arm" over boolean/.
- **S3 (Q7, unsure).** `ring_closures` gives the `on` side `RingClosure::Wall(plane)` (join.rs:224), though its chord copies an edge and reads no section. `RingClosure::AlongEdge(on)` is the type that says so. It works because the across-the-edge check (join.rs:2930-2954) decides first. I did not build that mutant.
- **S4 (Q3, likely).** `a_tube_ending_on_the_ball_from_inside…` passes on base too, so it pins a neighbouring refusal, not this change. The suite has no row that fails if the guard (MINOR-2) or the scope (N1) regresses.
- **S5 (Q2, likely).** The PR body argues "This was not a design fork, so the lane did not stop for one" at length. N6 says the lane is consistent with the ratified fork, so that holds, but the coaxial arm sits against that row's text.
- **S6 (Q5, likely).** `JoinLane::AlongEdge`'s new doc (chord_join.rs:817-821) says "a section segment that is an edge of this solid … no section is read". On EdgePlane's holder side the same segment does read a section. The lane is now per-solid, which `SegmentLane::AlongEdge` (join.rs:626, both-sided only) does not express.
- **S7 (Q8, partial).** I read join.rs's module docs (lines 1-120), the whole join-lane region (870-1210) and `germ_section_frame`, but not all 5825 lines. The file accumulates a section per lane, and the 877-887 lane census in prose is the stale one (S2).

REVIEW COMPLETE
