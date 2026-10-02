# Review of PR #3805, frozen head 9b6fe0843d

Lane `reach-dual3805-r2`. Started 05:59 UTC, finished 07:17 UTC (2026-10-02). Glimpse: none. I read no other lane's branch, scratchpad, report or PR comment. I read the PR body (`get`) and its check runs and job log.

**Verdict: NOT-MERGEABLE-AS-IS.** The head does not compile against its base (MAJOR-1, a one-line fix). I found no wrong body and no wrong verdict. Once that line is fixed and CI is green, I would put it at APPROVE-WITH-FIXES. **Counts: 1 MAJOR · 2 MINOR · 3 NOTE.**

## Findings
**MAJOR-1: the head's merge with `main` does not compile, and CI is red on 9b6fe0843d.** Demonstrated by execution. The cause is `crates/sweep/tests/conic_edge_curved_face.rs:39`, which passes a `Vec3` where `main`'s `SplitPlane::normal` is now a `UnitVec3` (67ff4791). That gives `E0308`, and the `sweep` test binary does not build.
- CI run 36968237415 on this head: `lint` and `test` both fail with exactly this error (job log read); `gate ok` fails.
- I reproduced it in a local merge of `origin/main` and the head: `merge-tree` is clean, but the build fails.
- The PR body's "re-run … after merging main … 7270 passed" does not describe this head's merge.
- With `UnitVec3::new(…, topo::DATUM_UNIT_NORM, band)` at :39, the 105 touched rows (ellipse/circle roots, `conic_tests`, the five sweep suites) pass on the merge. *sure*

**MINOR-1: the `Conic` frame premise is false, and an enclosure fails to enclose on frames the mint certifies.**
- `crates/geom-brep/src/implicit.rs:526-527` says `major ≥ minor > 0` holds "as every `Curve3::Ellipse` minted in this tree is".
- But `crates/topo/src/tier3_tests.rs:1332-1344` and `loop_winding.rs:139` record that the mint certifies `minor > major` and negative-`major` ellipses, and only tier 3 refuses them at rest.
- `gate_operand_edges` (`reduce.rs:430`) admits them, and `Conic::of` copies the frame unchecked.
- Demonstrated at unit level: in `probes/probe_r2_ellipse.rs::probe_r2_swapped_frame`, 610 of 6000 torus arc ranges fail to hold my own dense residual (e.g. 4.1913260 above the range's top of 4.1913253). That is the bound read at the stored `major`, which here is the smaller axis. Swapped frames also invert `ellipse_roots`' speed bracket.
- I did not build an end-to-end wrong body. Circles with `r < 0` share the class from before this PR. *likely*

**MINOR-2 (Q3): the speed-bracket choices are sound but unpinned.**
- By inspection each direction is right: `τ` and the conditioning margin at `b` are lower bounds; the slacks at `a` are upper bounds; `|C′| ∈ [b, a]`, `|C″| ≤ a`.
- But no row goes red when they degrade. Four mutants survive the PR's rows and my fuzz at ε 1e-9 and 1e-6 (`probes/mutants.sh`), all in `ellipse_roots.rs`:
  - `speed_lo` → `major` (:163);
  - `speed_hi` → `minor` (:164);
  - the first-harmonic `speed` → `minor` (:149);
  - dropping `+ second` from the first-harmonic noise (:147).
- The `lever` → `2a` mutant goes red at 1e-6, as the PR says. *sure*

**NOTE-1:** at the head, the first-harmonic coaxial branch decides `c₀` alone. `main`'s fdc49f05 (#3752's fix pass, `constant_residual_roots`) charges `a₁ + noise`, and with it this door's `A₂`, once merged. Merging `main` is needed anyway (MAJOR-1). *sure*

**NOTE-2:** the ladder certified two crossings of depth 1e-11 at ε = 1e-9 (`probe_r2_near_tangent`, δ = −1e-11). Both roots are genuinely on the surface. This is the filed GERM class `the-half-angle-ladder-certifies-in-band-configurations`, not new here. *sure*

**NOTE-3, scope from the end-to-end runs:** "builds" holds only for bodies held inside the drum.
- Every disjoint pose (a ball or rod above the cut inside the carrier, a ball below the floor, a ball 0.02 outside the wall near the rim) and every wrapping rod (coaxial and offset) refuses typed. They stop at `Containment(VolumeUncertified)` or at the plane-boundary `FallbackExtentUnsupported`, so the two new extent-scan arms open doors that these poses still cannot pass.
- `mass_properties` refused one result twice: `props_quad_converged` at ψ = 2, and `QuadratureBudget` at ×1e3 with ε 1e-12. Both are in props, not this PR. *sure*

## Claims (attacked by execution unless noted)
1. **Root cause holds.** Inspection of the `reduce.rs` diff, plus every probe passing on the head (the base refused the drum's ellipse rim, edge 12v1).
2. **Enclosures hold.** I checked the algebra by hand: the harmonics, `f2 = A₁ + 4A₂`, and the torus terms at `a` and `a_h`. Then I fuzzed 1500 ellipses (b/a down to 0.01) × sphere/wall/torus/plane × 4 spans (whole turn, near the major and minor vertices, random), against my own residual at 20 001 samples each: 0 failures. Reading the torus bound at `b` turns `conic_tests` red, as claimed.
3. **Roots hold.** I ran 3000 random eccentric poses (sphere, tilted wall, near-section wall) and 1200 near-tangent poses (δ from 1e-3 to 1e-11, at minima and maxima), at ε 1e-9, 1e-6 and 1e-12, against a bisection of the TRUE distance. There were 0 wrong counts, 0 wrong misses and 0 misplaced roots. The 16 count mismatches were my 40k grid missing root pairs about 1e-5 rad apart; I checked each one: the door's roots are on the surface and its pair midpoint has the opposite sign. Exact sections read `OnSurface` (400/400), and walls offset by ±1e-3·b read `Miss`.
4. **Wall placement holds.** Five cuts (plane turned by ψ ∈ {0, 0.7, 2.0, −0.9}, steep cuts crossing the floor and top), 60 azimuths, and heights including 1e-7 and 1e-4 either side of the rim: 5400 verdicts, 0 wrong, 0 `None`.
5. **The extent-scan cylinder arm holds.** "Inside or clear of the whole carrier ⇒ meets no face on it" is sound (a face is a subset of its carrier). Mutating `nested` to accept a straddling ball turns `a_ball_straddling_a_notched_walls_carrier…` red.
6. **Bodies hold.** The PR's held balls and rod, plus a new ball at (0.25, −0.25, 0.3), at ×1e-3, ×1 and ×1e3, identity and a rigid re-pose, both operand orders, every op, at three ε. Every volume is within `volume_pad`. Membership: 9555 `point_in_solid` samples at 1e-9 agree with my analytic oracle. The 1e-6 / ×1e-3 disagreements were points inside ε, correctly `OnBoundary`. Results reused: (A∖B₁)∖B₂ and (A∖B₁)∩B₂ are correct. (A∖B₁)∪B₁ refuses on a great circle lying on B₁, a documented door.
7. **The diff is this PR's own.** The three-dot diff from `4ba454b3` is 27 files, and nothing of #3752 is reverted (`merge-tree` is clean). But the branch is stale against `main` (MAJOR-1).

**Suites:** `geom-brep` and `topo`: 2841 rows each at 1e-9, 1e-6 and 1e-12, all green except the known `rigid_map_near_eps_plane_nurbs` at 1e-12. `sweep` touched suites plus probes: green at all three ε.

## Style (Q1, Q2, Q3, Q4, Q5 and Q8 exercised; Q6 and Q7 lightly)
- S1 (Q2) `reduce.rs:1329-1330`: an orphan fragment, "// unconditional door.", is left after the edit. *sure*
- S2 (Q1/Q5) `reduce.rs:1865`: the row `bool_circle_curved_clearance` now decides ellipse clearances too; the name is stale. *likely*
- S3 (Q1) `reduce.rs:2033-2055`: the ellipse arm restates the circle arm's `CircleRoots → SpanVerdict` mapping (`reduce.rs:2006-2023`) almost verbatim, including the `CountDisagrees` message. That is a third copy beside `:2225`'s torus form; look for others in `wall_crossing`. *sure*
- S4 (Q4) `implicit.rs:526`: the false premise of MINOR-1 is now cited by all three enclosures and both harmonics functions. It is a doc that rotted while the code relied on it. *likely*
- S5 (Q5) PR body: the local-run claim after merging `main` is contradicted by this head's CI. *sure*
- S6 (Q6) `ops.rs:2478-2489`: the carrier gap and nested margins carry no rounding charge, unlike the harmonics. This is harmless at these scales but asymmetric. *unsure*
- S7 (Q1) the first-harmonic arm choice is decided by `A₂` here and by tilt in `circle_cylinder`. Self-filed (`conic-quadric-doors-choose-their-first-harmonic-arm-two-ways`), so it is scheduled. *sure*
- Q8: I read `ellipse_roots.rs` whole and `circle_roots.rs` in large part; nothing further.

## Probes (`probes/`)
`probe_r2_ellipse.rs`: a topo unit module, mounted with `#[cfg(test)] mod probe_r2_ellipse;` in `boolean/mod.rs`. `probe_r2_conic.rs`: a sweep suite, mounted via `tests/all.rs`. `mutants.sh`: the mutant loop. The torus-`b` and `nested` mutants were done by hand with `sed`.
