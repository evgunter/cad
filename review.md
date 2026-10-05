# Review of PR #4046, frozen head b793623189

Lane `reach-dual4046-r2`. **Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 4 · NOTE 5.
Wall clock 2026-10-05 03:45 → 05:15 UTC. Glimpse: none. I read only the PR body (`get`); no comments, reviews or other `analysis/reach-dual/*` branch.
Hosted CI on b7936231 (run 37253406375): test, lint, corrupt-input, mesh all `success`.

## What I executed (probes: `probes/reviewer_4046_r2_probe.rs`, wire with a `#[path]` line in `crates/sweep/tests/all.rs`)
- **Oracle:** a CSG signed-depth bound over the balls and rotated boxes each body was built from (|f| > thr ⇒ really that far from the boundary). Volumes come from an exact-interval z-line slice integral and from closed forms. Nothing reads the kernel.
- **Fixtures** (×1, ×1e-3, ×1e3; ∪ ∩ ∖):
  - lens pairs at separations 0.3, 1.4 and 1.95 (near-tangent);
  - a tilted pair (r 1 and 0.5, axis (0.3, 0.9, 0.3));
  - three-in-a-row (a ring on the middle sphere), and a ball triangle with triple-point vertices;
  - a ball against rotated boxes: a corner inside (reflex vertex), a wedge through the centre (lune), an oblique wedge, a near-tangent slab, a pole-centred box, and a 0.02 rad sliver;
  - a bowl (a sense-false trimmed face).
- **`point_in_solid` probes:** 300 random points; points 1e-3…1e-12·s off every vertex, every pole and every vertex's antipode, both in 3-D and slid along the sphere; and 600 points on or ±1e-6 off each sphere.
  - **0 wrong In/Out in 509 412 classified queries** (196 284 at ε 1e-9, 176 892 at 1e-6, 136 236 at 1e-12). There was also 0 OnBoundary farther than 1000ε.
  - At ε 1e-12 a ball of radius 1e3 cannot be finished (`VolumeUncomputable`, outside this PR), so ×1e3 ran only at 1e-9 and 1e-6.
- **Reuse:** each fixture against a nested, a disjoint and a crossing third ball, every op, both orders: tiers 2 and 3 plus the certificate, and volume against the oracle.
  - Every flag traced to my own quadrature error at grid-aligned walls. The kernel matches exact closed forms to ≥6 digits: π/3; the pole-corner cap integral 0.132961 / 0.371039; and the lens 2·cap(1, 0.025) = 0.0038943.
  - Scale invariance V(s) = s³V(1) holds on all 568 comparable ops, and outcomes are identical across scales.
- **Differential against merge-base 128a6e0c5** (1 380 ops): **0 regressions.** Every op that built on main builds the same volume here. 760 `Containment` refusals and 18 `CurvedPierceUnsupported` refusals now build, and 308 now come back correctly empty.
- **Suites:** topo 2281/2281 at 1e-6 and 1e-12; sweep 2111/2111 at 1e-12. At 1e-6 sweep is 2110/2111, the one red being the known `pocket_ring_steep_ellipse`. The touched files are green at all three ε.
- **Mutant table** (rows: `carved_sphere_operand`, `a_pole_struts…`, `verbs_sphsph_chart`, `tilted_sphere_pair`):

  | mutant | red |
  |---|---|
  | flip leave/enter heading | 11/23 |
  | parity over the half-turn instead of closest crossing | 10/23 |
  | drop the seam exclusion | **0/23**, and in the whole sweep only `full_turn_wall::both_doors_answer_the_bead_by_its_height_window` at ε 1e-12 |
  | panic on any region read | 14/23 |

## Findings
1. **MINOR: liveness gap at the antipode of a vertex.** `sphere_region.rs:253-279`, `:34`. DEMONSTRATED (`reviewer_4046_antipodal_vertex_refuses`).
   - Setup: in the union of the unit ball at (2, 2, 0.5) and an r 0.5 ball along (0.3, 0.9, 0.3), take a point on sphere A 1e-9 from its +y pole. It is 0.063 deep inside B, so it is In.
   - At ε 1e-9 and at ×1e3 `point_in_solid` refuses with `Escalated(bool_sphere_region_span)`.
   - Cause (instrumented): face 2v1 has a vertex at A's −y pole, where two meridian edges meet. Every forward ray meets that antipodal vertex first, at s = π, and is abandoned. The rays aimed at the meridians run along them and escalate. The radical-circle arc 0.08 rad behind the point is never tried: no ray is cast in reverse. The answer is In at 1e-12.
   - Not a wrong answer, and main refused too (`PartialSphereFace`). But "A pole is no point of interest" (`:34`) overstates the claim, and the gap is neither disclosed nor scheduled. Sure.
2. **MINOR: the seam exclusion is unpinned at CI's ε.** `sphere_region.rs:220-222`. DEMONSTRATED (mutant).
   - Dropping the `retain` turns no PR row red at 1e-9. The one witness is a pre-existing row at 1e-12, which reads `None` where it wants `Some`.
   - By construction a kept seam only adds tied twin crossings, which yield refusals and never the wrong side. So claim 3 holds, but nothing in the default gate guards it. Sure.
3. **MINOR: the tightened `join1_r1_rows::a_pole_struts_halves_face_their_own_meridians` never reads a region.** `join1_r1_rows.rs:231-241`. DEMONSTRATED.
   - It stays green under the panic, heading and parity mutants. The far box at [5, 6]³ is placed without any sphere hit, so the row pins only that `sphere_face_region` builds.
   - It went red on main only because `face_geo` refused eagerly. Sure.
4. **MINOR: `carved_sphere_operand::a_tilted_cut_of_a_ball_is_an_operand` cannot see a complemented region.** `carved_sphere_operand.rs:221`. DEMONSTRATED.
   - It is green under the heading flip (red under parity and panic). With the sphere split into two trimmed faces, each face's complement covers the other face, so every exit is still counted once.
   - The lens rows catch the flip, so the claim is pinned. This row is not. Likely the same blindness holds wherever the sphere carries exactly two faces.
5. **NOTE: orphaned decision words.** `"bool_sphere_trim"` and `"bool_sphere_trim_pole"` stay listed (`boolean/mod.rs:251,256`), but their only producer (`point_on_sphere_in_face`) is deleted. Inspection, grep-proven. Sure.
6. **NOTE: `PartialSphereFace`'s doc is wrong about what refuses.** It names "the spiric a torus or a cylinder off the sphere's centre leaves" (`solid_contain.rs:291`). The same file defines spiric as a plane's section of a torus (`:341`), and an off-centre cylinder leaves a different quartic. The doc also omits off-axis cones, which the recourse (`:525`) implies refuse. Inspection. Likely.
7. **NOTE: claim 1's "pole on an arc" premise is not reachable through public builds.**
   - Every body whose tilted circle passes through or near a pole refuses earlier, with `Pcurves(ArcNearPole)`: the lens through A's pole, and the 0.02 rad sliver at ×1 and ×1e3. The same holds on main.
   - So the PR's planted `verbs_sphsph_chart` rows are the only coverage of that case. DEMONSTRATED.
8. **NOTE: the root noise may not cover far-from-origin bodies.** `ray_roots` charges rounding on |C−c|+ρ (`sphere_region.rs:386`), not on absolute coordinates, so a body far from the origin could under-charge. Not exercised. Unsure.
9. **NOTE: a two-ball union refuses at the join, and it is not this PR.** r 1 against r 0.6 at 1.2·(0.6, 0, 0.8) refuses `Join(RingOffCylinderChart)`, identically on main. Worth an issue. Sure.

## Claims
1. Sound: no wrong answer found. One liveness gap (finding 1), and the pole-on-arc case is unreachable through public builds (finding 7).
2. Holds by inspection (grep): `sphere_chart_trim`'s only reader is `classify::sphere_zone_reach`, which falls back to the ball (sound).
3. Holds, but unpinned at default ε (finding 2).
4. Holds, widened as above.
5. Holds in code; the doc wording is wrong (finding 6).
6. Partly false: the seam mutant reds nothing at 1e-9; `a_pole_struts…` is blind to every mutant; the tilted cut is blind to the heading flip.

## Style (questions exercised: Q1 Q3 Q4 Q5 Q6 Q7; Q2 and Q8 only partially)
- **Q1:** `sphere_region` is a third closest-crossing reader, beside `cast_ray` and `splitting::containment`, with its own `TARGET_SHARES` ladder (`:92`) and a reuse of `SCHEDULE`. There is no shared home for "closest crossing plus graze-abandon". Likely.
- **Q4:** `sphere_chart_trim` (`solid_contain.rs:3028`) still computes the azimuth window and period gate only to gate its class for the latitude consumer, so a tilted face still gets only the ball in `sphere_zone_reach`. Unsure whether that is intended.
- **Q4:** the orphaned decision words (finding 5) are what a symbol-scoped sweep misses: the PR swept readers, not names. Sure.
- **Q6:** the liveness gap (finding 1) and the scale/ε floor of the root door (the PR body's "not this reading's") have no `work/` item. Likely.
- **Q7:** I would cast each aimed ray both ways (±t) before giving up, since that closes finding 1 cheaply. Unsure.
- **Q3:** findings 3 and 4 are one class: a row that only builds the region, or reads it through a symmetric pair. Look also at `tilted_sphere_pair`'s pierce rows. Likely.
- **Q8:** I did not read `solid_contain.rs` (≈5 000 lines) end to end. I read only its sphere arms and header, so this question was skipped.

## Disclosure
One 1e-12 run was invalid and is discarded. I built the merge-base in a worktree into the shared target dir, cargo reused the main artifact, and the PR's own lens row then failed spuriously. After a forced rebuild it passes. All figures above are from runs on the true head.
