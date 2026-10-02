# Review of PR #3817, frozen head 47b16394c2

Lane `reach-dual3817-r1`. Wall clock 06:52–08:40 UTC, 2026-10-02. **Verdict: APPROVE-WITH-FIXES** — 0 MAJOR / 2 MINOR / 5 NOTE (+ style). No wrong body was found in 230+ executed bodies, every one checked against my own oracle.

**Oracles (mine, none read the kernel's arc choice or props).**
- (a) Boundary Monte Carlo: each result face's sampled loops are put through a stereographic winding test. On each sphere, every sample point must lie in exactly one face of the expected side (set algebra on the two balls), or in none.
- (b) 3-D Monte Carlo of the set algebra on closed-form membership (wedges calibrated against `point_in_solid` on the uncut wedge).
- (c) Monte Carlo area and `∫p·n dA` on hand-built loops for the Gauss–Bonnet arm.

The oracle is live: with its orientation flipped, every carved body fails it (`mc_bad` 1367–7990/7990).

## Claims
1. **Run-side rule — holds (sure).** 24 poses × {∪, ∩, A∖B, B∖A, B∪A} at scales ×1e-3, ×1 and ×1e3, all against oracle (a) at 0 mismatches. They include unequal radii, a large second ball, nearly tangent (d = r₁+r₂−1e-3), nearly inside, a tiny ball (r 0.05) and −y offsets. Also 3 rotated-chart pose families (A or B or both spun), and the meridian section through both poles (oracle (b)). Built: 135 bodies at 1e-9, 115 at 1e-6, 95 at 1e-12. Every other pose refused typed (absolute-band refusals at ×1e-3/×1e3, `SectionArcSide{NoCertifiedRun}`, `SpheresMeet`, `ArcNearPole`). Both operand senses (B∖A) included.
   - Mutants (EXECUTED): `!ccw` at `chord_join.rs:1576` and the sliver flip at `:1567` both turn `tilted_sphere_pair`'s two build rows and my probe red.
   - Inner loops and holes: unreachable (a ring-forming pose refuses at the pierce ring).
   - Interior-left under the outward normal: `face_outward_normal_at` folds the sense bit in (`face_normal.rs:259`), so a reverted operand reads consistently.
2. **Window rule bit-identical — holds (sure, by inspection plus suites).** `azimuth_monotone: true` runs the old window code. `oriented_arc` is the old tail verbatim. A polar sphere decides `Zero` → `true` as before.
3. **Gauss–Bonnet arm — holds (sure, EXECUTED).** 48 faces (caps, tilted lunes, eight 3–6-gons of small-circle arcs, a long-arc face larger than a hemisphere), each under both senses, on a centred sphere and an off-centre sphere (R 2.5). Area and flux match Monte Carlo within 3σ every time. Mutants of the turning-angle sign and of the curvature (×(1+1e-6)) turn the octant, torax and tilted rows red. The area check, though, is not a disambiguator: see MINOR-1.
4. **Planar side refuses `SectionNotPolar` — holds (EXECUTED).** 90° wedge planar caps and the PR's box/pip poses both refuse typed. A tilted `topo::split` of a ball refuses `Reduce(CurvedBooleanUnsupported)` before reaching the shared wall lane, so `split` opens no new surface.
5. **Table and lily — hold (EXECUTED).** Lily wall 7's carve: oracle (a) on both the ball and the zone sphere near the ball gives 0/3386 mismatches under all four ops. The suites (release): topo + geom-brep + editor-core 5386/5386 at 1e-6, 5385/5386 at 1e-12 (only the known `rigid_map_near_eps_plane_nurbs`); sweep 1920/1920 at 1e-9, 1e-6 and 1e-12. The Interval rows ran green at each ε. CI on the head is green (run 36974066004: test, lint, demos).
6. **Moved rows — torax and geom-brep can fail (EXECUTED by mutant). `m5_pr12_fix_pass` f4 cannot** (MINOR-2).
7. **C12 (7) — a description the code moved (likely).** The same sentence already carries the cone's closed-form exception. `git log -S'never a silent Gaussian'` finds only agent commits (a052c47b, the #3504 merge), no Ev ratification.

## Findings
- **MINOR-1** `crates/geom-brep/src/props/curved.rs:2172-2174` (EXECUTED). The doc says the `props_sphere_loop_area` range check fails "a loop whose traversal disagrees with the sense bit". It cannot: flipping either one gives the complement, `4πR² − A`, which is in range whenever `A` is. The PR's own octant row (`props_sphere_circle_loop.rs`, `sense=false`) and my 24 complement rows measure exactly that. The real guard is tier 3: flipping a tilted face's sense with `flipped_face_sense_for_tests` raises `LaminaWedge`, but props returns a wrong volume (3.80 vs 5.94). So the area check is degeneracy-only, and the claim should say that.
- **MINOR-2** `crates/sweep/tests/m5_pr12_fix_pass.rs:256` (EXECUTED). f4's fillet bracket `(clip − r²Σℓ, clip)` is far wider than its spherical corner patches contribute (R 0.08). It stays green with every vertex turning angle's sign flipped and with the curvature scaled. The PR body lists it as a row the Gauss–Bonnet arm moved, but it pins only "does not refuse".
- **NOTE-1** (EXECUTED). Bodies with the wrong arc, from both run-side mutants, pass `validate`, `validate_closed` and `validate_geometric` (e.g. ∪ volume 6.333 vs 5.890). Only the volume-against-oracle rows guard arc choice, so keep them.
- **NOTE-2** The reflex-corner premise (`work/reach/run-side-arc-rule-reads-only-the-run-at-each-end.md`) was not reached. 270°/90° wedge poses refuse earlier (`SectionInvariant`, `NoCertifiedRun`, `CurvedPierceUnsupported`, `SectionNotPolar`), and B through A's pole refuses `ArcNearPole`. Not reached ≠ unreachable.
- **NOTE-3** The tilted rows measure with the props arm this same PR adds. Oracles (a) and (b) close that loop independently (no correlated error found).
- **NOTE-4** `docs/predicate-dimension-audit.md` lists `split_sphere_section_polar` but none of the 8 new predicates (`split_arc_run_*`, `props_sphere_*`). I don't know whether that audit is a register (unsure).
- **NOTE-5** Carved bodies cannot be reused as operands: `PartialSphereFace`, or `SectionInvariant` "no closed-form chart image" for a third ball. Both are typed and both are filed.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q7 and Q8 (`chord_join.rs`, 4020 lines, skimmed rather than read end to end). Q6 was skipped.
- Q1 `chord_join.rs:1108` and `:1493`: `theta_of` is copied verbatim into `select_arc_by_run_side`. The PR extracted `oriented_arc` but left this copy, so the refactor leaves a fresh duplicate behind (sure). Likewise `run_ends` (`:1357`) and `run_is_section_arc` (`:1400`) each walk the run for certified edges with near-identical loops (sure).
- Q7 `curved.rs:2264` and `:2300`: Gauss–Bonnet failures are reported as `PropsError::NotIsoRectangle{what: "props_sphere_loop_*"}`, on the arm that exists for non-iso-rectangle faces (likely).
- Q1 `curved.rs:2216`: the predicate name `props_rim_fit` is reused for "this circle lies on the sphere", which is not a rim (likely).
- Q5 `chord_join.rs:597`: the recourse for `NoCertifiedRun` is "move the geometry", while its actual cause (the pierce ring) has a filed door (unsure).
- Q2 `SphereFluxSide::Sense` is still called "the one home of that fact", but now serves two consumers, and its justification has grown again (unsure).
- Q3 The tilted rows pin `volume_pad == 0.0` and a lens tolerance of 1e-9·V, which is good. The Interval row checks a bracket plus slack, which goes red on a wrong arc (confirmed by mutant).

## Probes and disclosure
Probe sources are in `probes/`: `review_3817_r1_probe.rs` and `review_3817_r1_pole_probe.rs` (sweep tests), `review_3817_r1_gb_probe.rs` (geom-brep tests) and `review_3817_r1_lily_probe.rs` (appended to `lily.rs`). The PR was read via `get` only. Glimpse: my `rg SectionNotPolar` hit one row of `docs/DUAL-REVIEW-LOG.md` (DR-29, PR #3659's past dual), not this pair. No `analysis/reach-dual/*` branch other than this lane's own was read.
