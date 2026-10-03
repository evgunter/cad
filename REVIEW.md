# Review of PR #3962: JOIN edge-edge membership reads a dihedral wedge by its extent

Frozen head `b60ea20c`. Main = the PR base `56345807`. JOIN-3 is not merged in, because the build did not need it.
Probes: `crates/sweep/tests/join_reflex_wedge_review_probes.rs`, with all of its batteries `#[ignore]`d and
`rv_oracles_agree` in the fast set. Both trees were run with `--release` through `differential.rs`'s `outcome`.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 1 · NOTE 4. The fix is the MINOR: a test-side stale premise. The rule itself held under every probe.

## Claims

1. **Union/intersection at every wedge angle: holds for planar flanks. Curved flanks are unreachable. π is unit-only.**
   - Derived by hand. `flankers` (`recl.rs:400-409`) makes `fl[0]` hold the common ray as `start` and `fl[1]` hold it as
     `end`. `BoolSector` sweeps CCW from start to end about the outward normal (`sectors.rs:107-121`). Seen down
     the axis, the material is the CW sweep α from r̂₀ to r̂₁, and n̂₀ is r̂₀ turned +90°. So n̂₀·r̂₁ = −sin α,
     which is positive iff α > π, and that is the reflex sign. The normal is outward (`OutwardNormal`, with its sense folded in).
   - Executed in `rv_planar_battery`, which uses irregular star profiles with every rim vertex at its own radius and
     flanks of unequal length (2.3 vs 1.4, and 0.9 vs 1.6). `b` straddles `a`'s bottom cap. Every pose ran under three maps:
     - identity;
     - a rigid rotation, which tilts the common line;
     - a projective map, under which planes stay planes, so a prism becomes a general polyhedron with non-parallel
       side edges and a tilted common line.

     The closed form is exact: each convex piece's mapped volume by the divergence theorem.
     `rv_oracles_agree` checks it against the kernel on single operands. θa ∈ {2, 40, 100, 179, 180, 181, 230, 300, 358} and
     θb ∈ {25, 170, 200, 290, 350}, so both reflex and near 0 and 2π are covered. On head, θa = 2, 179, 181 and 358 are all SOUND or EMPTY ok.
   - Exactly π: a θa = 180 operand refuses `CoplanarNeighbours` at the operand, in both trees and in all 1 440 runs. The
     π-refusal arm is therefore reached only by the unit test at `recl.rs:1363`.
   - Curved flank: `rv_curved_battery` and `rv_curved_tall_battery` use a keyhole whose flanks along the edge are the plane
     y = 0 and a convex cylinder (270°). `rv_chord_battery` uses a plane against a concave or convex cylinder at
     160–200°. All 3 960 + 3 168 runs refuse `CurvedPierceUnsupported` in both trees: b's corner edge rides a's cylinder
     ruling undeclared. The new rule is not reachable there.
2. **The On ladder under a reflex wedge: holds.**
   - By hand: anti-parallel touch → Out for that plane, and In for the other plane exactly when the wedge is reflex,
     so the union returns In, which is true. An overlapping tie at a reflex wedge reads Out against the other plane,
     so the union returns the lump, as the AND does at a convex wedge.
   - Executed in `rv_coplanar_battery`:
     - b's start or end flank lies along each of a's flank directions or opposite it;
     - θa ∈ {40, 100, 180, 230, 300};
     - each pose ran flush-declared **and** undeclared, in both operand orders, under all three maps.

     Result: 0 BAD over 5 760 lines. Undeclared overlaps refuse `UndeclaredCoincidence`, the same as on main.
3. **No wrong body: holds.** Head and main each produced 22 194 lines, with 0 BAD, EMPTY WRONG or UNMEASURED in either tree. Main → head:
   - 2 508 lines go from `ClassificationInvariant` to SOUND (1 260 generic, 1 248 coplanar);
   - 24 go from `ClassificationInvariant` to `ResultInvalid` (NOTE N1);
   - **0 go from SOUND to a refusal**;
   - every other line is identical.
4. **The rows can go red: holds.** Run in release, with the checkout restored afterwards:
   - **Mutant 1** puts back the convex AND (`Ok(halves[0] && halves[1])`). Two rows go red:
     - `m3_pr6_saddle::prism_reflex_kiss_takes_edge_edge_lane`, with "edge-edge membership disagreement";
     - `edge_edge_sites_at_a_reflex_wedge_build_sound`, at θa 315 θb 300 φb 357 I_ab, with "odd number of surviving crossing records".

     The unit test stays green, as it should: it calls `wedge_is_reflex` directly.
   - **Mutant 2** flips `wedge_is_reflex`'s sign. All three rows go red; the unit test fails with "a quarter-turn is convex".
   - Under **both** mutants, `review_m3_pr4::reflex_edge_touch_benign` and `reflex_edge_crossing_refuses_loudly` stay
     green. See MINOR M1.
5. **The sweep is complete: holds for code. One test-side citation was missed (M1).**
   - My own sweep keyed on the word "wedge", not on `&&` or `side_code`, over topo/sweep/verbs/geom-brep `src`.
     It reached `boolean/rim_wedge.rs`, which the PR's list does not name: a normals and departures table with no AND of
     half-spaces, so not this shape.
   - `resolve_edge_sector` (`recl.rs:1154`) reads one reference plane.
   - `sectors.rs` `build_sectors` takes curved reps from the carrier **tangent** (`sectors.rs:206-215`).
   - My sweep matched only the word "wedge", so a site that ANDs half-spaces without using that word would not show up.
     I did not search pncad, mesh, step-* or viewer.
6. **Re-pin at 40/3: holds.** Checked independently:
   - L = 16 − 4 = 12;
   - the kite's shoelace area is 5/2;
   - kite ∩ L is the kite's y ≤ 2 half (5/4) less its x > 4 tip (1/2 · 1/2 · 1/3 = 1/12), which is 7/6;
   - 12 + 5/2 − 7/6 = 40/3.

   The row asserts tier 2, tier 3′, the certificate and the volume.

## Findings

- **MINOR M1: stale premise and insensitive rows.** `crates/topo/tests/review_m3_pr4.rs:292-296` still says "The
  convex-wedge membership limitation makes the two solids disagree — the PR claims a LOUD typed refusal". Its rows
  at `:250` and `:298` accept `ClassificationInvariant`, `Escalated` or `UndeclaredCoincidence`, and on head they print no refusal.
  Both stay green under both mutants, so neither can go red on this lane. The PR's sweep of who cited the old
  limitation missed this file. Demonstrated by the mutant runs above and by `--nocapture` on head.
  Wanted: re-word the doc, and pin or tighten the rows now that the residue is gone.
- **NOTE N1: an undeclared anti-parallel touch builds a body that only the final validator catches.**
  - Where: 24 runs, for example `RVC id false 100 200 180 S_ba`, in the id, rot and proj maps. S_ba at (θa, θb) = (100, 200) and (40, 200);
    S_ab at (230, 170) and (300, 170).
  - What changed: these go from `ClassificationInvariant` on main to `ResultInvalid { ScaffoldAtRest }` on head, in exactly
    one op and order per pose. The other five runs and the flush-declared twin are SOUND.
  - Assessment: still loud. The odd one-op asymmetry may be worth a row; I did not root-cause it.
- **NOTE N2: the π refusal is defensive.** No single-solid operand reaches it, because a flat edge refuses `CoplanarNeighbours`
  at the operand (1 440 + 1 152 lines). Only the unit test exercises it.
- **NOTE N3: curved flanks along the common line are unreachable today.** The `CurvedPierceUnsupported` refusal comes first
  (claim 1). When that door opens, the reflex reading on curved flanks is unmeasured.
- **NOTE N4: NURBS-edge reps are chords.** `sectors.rs:202-205` gives a NURBS edge its end-to-end chord, not its tangent.
  `bool_wedge_reflex` would then compare a chord with a tangent-plane normal, and a near-π wedge could read on the wrong
  side of π. This is pre-existing for the membership reading too. Unprobed (unsure).

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q7 and Q8 (in part: I read recl.rs's header, its item list and the whole of
`resolve_edge_edge`). Q5 and Q6: nothing found.

- **S1 (Q3, sure).** Same as M1: `review_m3_pr4.rs:250,298` cannot go red on this lane.
- **S2 (Q1, likely).** `wedge_is_reflex` (`recl.rs:839-852`) is a third copy of "decide_nonzero_reported → Positive/Negative
  → bool, Err → coincidence(Coincide::Sectors, Moot)". The others are `bool_dir_same` in the same closure (`recl.rs:965`) and
  `insert.rs:1216-1235`'s `strut_order`, which the PR says it follows. There is no shared home for "a levered sign read
  as a bool or a Sectors refusal". Grep `decide_nonzero_reported(` in `boolean/` for the rest of the class (8 sites).
- **S3 (Q7, likely).** `wedge_is_reflex` takes `fl: &[(usize, Rep<T>)]` and indexes `fl[0]`/`fl[1]`. The membership's
  `halves.iter_mut().zip(other)` (`recl.rs:934`) would silently truncate a shorter slice. A `&[_; 2]` would hold the
  two-flanker invariant in the type, as `a_fl`/`b_fl` already are.
- **S4 (Q7, unsure).** The reflex question is re-decided once per split membership, which is up to twice per solid per site,
  on identical inputs. That is harmless, but a per-solid fact is computed per representative.
- **S5 (Q7, unsure).** A half-turn wedge is one solid's own geometry, yet it refuses as `Coincide::Sectors`, the
  two-solid coincidence vocabulary (`refusal_routes.rs:483-496`). "Coincidence" reads oddly for it.
- **S6 (Q3, likely).** The unit test (`recl.rs:1363-1408`) builds `fl` with both flankers on sector 0, which no real site
  produces. It tests the formula, not the pairing of `fl[0]`'s normal with `fl[1]`'s rep. The pairing is covered only
  by the sweep and topo integration rows, as mutant 2 showed.
- **S7 (Q1, likely).** The new `join_reflex_wedge_probes.rs:57-98` adds a fresh six-run `Pose`/`run`/`OPS` harness beside
  `differential.rs`'s `reflex_pose`/`reflex_run`. Yet `differential.rs:7-10` lists "deliberately not absorbed, and the
  whole of it", and that census does not name the new file. My probe file is a further copy and is not proposed for main.

REVIEW COMPLETE
