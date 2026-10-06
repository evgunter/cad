# Review of PR #4128, frozen head 4ec0d4b25c

**Verdict: NOT-MERGEABLE-AS-IS** (one MAJOR, and its fix is small). **MAJOR 1 · MINOR 2 · NOTE 3.** Wall clock: 09:54–11:05 UTC, 2026-10-06. No glimpse: I read the PR body through `get` only, never its comments or reviews.

## Correctness

**MAJOR-1: `carrier_touch` assumes an ellipse stores `major ≥ minor`. Nothing guarantees that, and with the order swapped it clears real crossings.** (`crates/topo/src/boolean/carrier_touch.rs:122`, `:363`) — *likely*.
`speed_bound` takes `major` as the top speed and `major/minor²` as the curvature. The ellipse doc says readers past the constructor "take the semi-axes as magnitudes in either order" (`crates/geom/src/curves.rs:156-161`), and STEP import stores `ELLIPSE` semi-axes as written (`crates/step-import/src/entities.rs:939`). When `minor > major`, the bound under-states the arc length by `minor/major`, so the Lipschitz and second-order bounds clear pieces that hold roots. `edge_clear_of_ball`'s annulus `max(ρ−major, minor−ρ, 0)` has the same flaw: inside the true annulus it reports a positive gap. The shared home already does this right: `geom_brep::implicit::Conic::speed_hi` (`implicit.rs:705`, `max(|major|, |minor|)`).
DEMONSTRATED BY EXECUTION at function level: `probes/carrier_touch_localization.rs::a_swapped_ellipse_keeps_its_crossings`. 60 poses, each a swapped ellipse crossing a small sphere twice transversally (0.01 deep in r = 0.02). `clusters` returns **no cluster in 60/60**. The control stores the SAME geometric ellipse ordered (`u_ref` turned, `t` shifted π/2) and keeps both crossings in 60/60. Through `off_face`, that becomes `OffFace` → `CurvedEvent::None`, and the crossing is lost.
NOT built end to end. That needs a STEP-imported swapped ellipse whose conic door answers `Uncertain`, so reachability rests on inspection of the import path.

**MINOR-1: the necessity paragraph argues from the wrong failure.** (`crates/geom-core/src/real.rs:1251-1266`, `scripts/gates/bounds-allowlist.sh:461-467`) — *sure*.
DEMONSTRATED BY COMPILE:
- `Decide + CertifiedBounds` in `carrier_touch` does fail at `reduce.rs:3157`.
- I then propagated it through 16 signatures: `wall_crossing`, `curved_face_arm`, `settle_deferred`, `sweep_direction`, `sweep_and_settle`, `boolean_reduce_declared_strategy`, `ops::through_the_join`…`union/intersect/subtract(_with)`, and `boolean_reduce(_declared)`. They all compile.
- The first real failure is `Dual<f64>: CertifiedEnclosure` at `crates/topo/tests/inside_out_operand.rs:103/141`.

So necessity holds, but because the public boolean is instantiated at `Dual`, not because "the sweep driver is `Decide + Bounds`". A caller's bound is an edit, not a failure.
The paragraph also says "Every verdict it returns is a `Decide` call". It is not: `edge_clear_of_ball` returns `Ok(true)` on box non-overlap (`carrier_touch.rs:326-328`). That is a terminal `Bounds` grant, allowed by the #571 direction rule (disjointness), but the ledger text says otherwise.

**MINOR-2: the rows' module doc is stale.** (`crates/sweep/tests/pierce_tangent_off_face.rs:11-16`) — *sure*.
It cites `reduce::CarrierTouch`, which existed only in 387ada72 and was replaced by `carrier_touch` in 645d69d9. It also presents `√(2r·(zero+escalate))` as the ball's radius, but the kernel's radius is `ℓ + |d(m)| + escalate` (`carrier_touch.rs:89`).
MEASURED: for the item's touch, the kernel radius / rows' radius = 1.03 (ε 1e-9), 1.04 (1e-6), 1.02 (1e-12) (`the_touch_ball_against_the_rows_radius`). So the near-miss rows do straddle the kernel's real threshold, and "outside" is exactly outside: the touch's placement is geometric, not band-dependent. Claim 4 holds in substance.

**NOTE-1: circle × torus and ellipse × torus are a verdict move with no observable build.** — *sure*. DEMONSTRATED (`probes/e2e_pierce_tangent.rs`):
- A coin whose rim touches the 270° donut in its mouth, and an obliquely cut rod whose ellipse is its support point toward the torus, both pass the pierce on head.
- Both then refuse `FallbackExtentUnsupported` in all 6 runs at all three ε.
- With the kernel files reverted to the merge base, both refuse `CurvedPierceUnsupported` in all 6 runs, so the new path is reached.
- The on-face controls (touch at azimuth 135°) refuse `CurvedPierceUnsupported`.

No wrong body ships, but "fixed" is not observable for these pairs.

**NOTE-2: K telemetry.** — *likely*. The 0 samples are consistent with a curated corpus: `scripts/k_probe_sweep.sh`'s header calls new shapes "a geometry conversation". So this is acceptable for the merge. Still, the four decisions' margins are uncalibrated, and the rows here are cheap candidates for a probe set.

**NOTE-3: one red at ε 1e-12 in my ×1e3 probe, not this PR's.** The ×1e3 lens is already not a finished operand there (`VolumeUncomputable`, an escalation in the operand's volume, raised before the boolean). The ×1e-3 and ×1 poses built.

## What was falsified and held
- **Localization soundness** (claim 1): `clusters_hold_every_band_meeting`. 1620 random tangent and near-tangent poses (Δ ∈ ±{0, 2e-9, 1e-8, 1e-6}·scale), covering sphere/cylinder/torus × line/circle/ordered ellipse at ×1e-3/×1/×1e3, torus inner side included. The oracle is a closed-form signed distance written independently, with 2·10⁵ samples per span. Result: **0 misses**, every sign change or band-touching sample inside a cluster. By inspection the bound is a true lower bound: Hess ≤ 1/(κ−|d|) holds on the torus inner side with κ = min(r, R−r), and the premise is a `decide`. A span midpoint exactly on the torus axis localizes both touches (`a_midpoint_on_the_torus_axis`, and e2e G builds).
- **Mutants** (claim 4): all three turn red exactly the rows the PR names (face check → both refusal rows; edge clearance only → the 0.3-ball row; Lipschitz-only → the 3-balls-outside row).
- **Callers** (claim 2): by inspection, an ON endpoint inside the face is never cleared. A piece with |d| ≤ zero is never cleared, so its ball holds the face point and the reading fails.
- **End to end** (all ops, both orders, validate tiers 1–3, volume vs closed form): ×1e-3/×1/×1e3 item and turned-brick poses; three rigid rotations × three poses; torus inner equator in the mouth; two touches over a quarter donut's axis; a union reused against a second touching brick; U ∩ slab with `point_in_solid`, 4000 samples, 0 disagreements against my predicate, volume within MC error. Near-rim refusals hold at every scale.
- **Suites** at this head (geom-core + topo + sweep, 5418 rows): 1e-9 all pass; 1e-6 two red, both on the known-red-on-main list (`pinch_faces_tessellate…`, `pocket_ring_steep_ellipse…`); 1e-12 all pass. Default features only. I did not re-run `per-op-postcondition`.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 only partly: I read `carrier_touch.rs` whole, but `reduce.rs` (5831 lines) only in its changed arms.
- **Q1** `carrier_touch.rs:118-125` re-derives the conic speed bounds of `geom_brep::implicit::Conic::speed_hi/speed_lo` and `pcurve_cache::param_rate` (`pcurve_cache.rs:3618`), and it is the copy that drifted (MAJOR-1). Also sweep `mesh/src/chords.rs:126` and `certify.rs:1927`. — *sure*
- **Q1** `distance` (`carrier_touch.rs:129`) is a second closed-form sphere/cylinder/torus distance beside `carrier_eq::distance_to` (`carrier_eq.rs:1075`). Neither names the other. — *likely*
- **Q1** `PIECE_BUDGET = 4096` repeats `circle_roots::SUBDIVISION_BUDGET` and `spiric_arc::MAX_PIECES` with no sentence tying them together, and `clusters` is a third budgeted bisection beside `certified_subdivision`. — *unsure*
- **Q2** The ledger's "every verdict is a `Decide` call" is contradicted by the box grant three functions down (MINOR-1). — *sure*
- **Q3** No row reaches an ellipse span, an elliptic boundary edge, the torus inner side, or a midpoint on the axis. A mutant on any of those branches (e.g. ellipse speed = `minor`) stays green. My e2e F and G are ready-made rows. — *likely*
- **Q4/Q5** The test doc cites the removed `reduce::CarrierTouch` (MINOR-2). — *sure*
- **Q4** In the ON-endpoint arms (`reduce.rs:2290`, `:2347`) `OffFace` still places the ends through `vertex_on_curved_face`. If that answers In/On for an end `OffFace` cleared, the two reads disagree and no contradiction door catches it. — *unsure*
- **Q6** The straddle-arm and spline/spiric residues are filed as items: scheduled. — *sure*
- **Q7** The floor `√(κ·escalate)/8` and `CLUSTER_BUDGET = 8` are knobs with no derivation at the site. — *likely*

## Probes
`probes/carrier_touch_localization.rs` (mounted into `carrier_touch.rs` via `#[path]`) and `probes/e2e_pierce_tangent.rs` (mounted into `crates/sweep/tests/all.rs`). Not part of any crate; each file's header gives its mount line.
