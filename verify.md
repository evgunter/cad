# Verify: PR #4046 (reach/carved-sphere-classify)

Lane `reach-verify-4046`, 2026-10-05. Code unchanged, nothing merged, no GitHub comments.

**Head verified: `66818a125e8d1db79ccede853bf6c0866d436d98`.** The brief's `66818a125e3` does not resolve; the branch head and the PR head are `66818a125e8…`, with origin/main merged at `515e79ebfd` as the brief says. The branch has not moved past it. CI on this head: `gate ok` success (test, lint, corrupt-input and mesh green; interval skipped by the change filter).

**Verdict: VERIFIED.** No claim is false. One claim (the azimuth gate) holds by inspection only (below). That point is not blocking.

## Mutants (ε 1e-9; rows = the 19 rows of `carved_sphere_operand` (6), `verbs_sphsph_chart` (12) and `join1_r1_rows::a_pole_struts_halves_face_their_own_meridians`, plus `tilted_sphere_pair` and `full_turn_wall` for context)

Mutants were applied with reviewer r1's `probes/mutants_4046.py`, whose strings match the head verbatim. `oneway` and `nogate` are my own. Each was reverted after its run.

| mutant | edit | red of the 19 | which rows | lane claim |
|---|---|---|---|---|
| none | — | 0/19 (31/31 incl. context) | — | — |
| `flip` (leave/enter swapped) | `Sign::Negative => Inside(false)`, `Positive => Inside(true)` | **14/19** | the strut row, the tilted cut (slab pose: `(-0.943,-0.629,0.314)` reads In), the interval row, the antipode row, both lens rows, 8 of `verbs_sphsph_chart` | 14/19 ✔ |
| `parity` (parity of crossings in place of the closest crossing) | r1's script | **15/19** | the strut row, the tilted cut, the interval row, both lens rows, 10 of `verbs_sphsph_chart` | 15/19 ✔ |
| `noseam` (delete `arcs.retain(\|a\| !seams.contains(&a.edge))`) | — | **1/19**: `a_face_whose_only_edge_is_a_seam_is_the_whole_sphere` (`left: None`, want `Some(In)`) | seam row | reds the seam row ✔ |
| `oneway` (no reverse rays: `both_ways` keeps forward only) | — | 1/19: `a_point_whose_antipode_is_a_face_vertex_is_read`, refusing `Escalated(bool_sphere_region_span)`, the reviewer's own error | antipode row | "red without the reverse rays" ✔ |
| `nogate` (`sphere_chart_trim`'s period gate disabled: `.is_none() && false`) | — | **0**: whole `topo` 2295/2295 and `sweep` 2121/2121 stay green | none | not claimed; see claim 7 |

## ε runs (debug, own `CARGO_TARGET_DIR`)

| suite | 1e-9 | 1e-6 | 1e-12 |
|---|---|---|---|
| `topo` | 2295/2295 | 2295/2295 | 2295/2295 |
| `sweep` | 2121/2121 | 2120/2121 | 2121/2121 |

At 1e-6 the one red is `pocket_ring_steep_ellipse::steep_ellipse_poses_build_sound_or_refuse_typed`. It is **red on `origin/main` 515e79eb too** (run in a main worktree), so it is not this PR's. The PR's new and changed rows are green at all three ε. At 1e-12 the interval row and the ×1e3 antipodal pose stand down loudly through `vacuity::stood_down`, as disclosed.

## Probes widened beyond the rows (release profile, debug-assertions on, three ε)

- **r1 probes** (`probes_4046.rs`: p1 lens family, p2 three balls, p3 ball×box reflex, with reuse) report `SUMMARY wrong 0 vol-mismatch 0` at 1e-9, 1e-6 and 1e-12.
- **r2 `point_classification`** gives **0 wrong** in 196 322 / 176 919 / 136 271 classified queries at 1e-9 / 1e-6 / 1e-12, with 0 OnBoundary far from the boundary. At 1e-12 the ×1e3 scale is skipped: the fixture's plain revolved ball of radius 1e3 is not finished there (`VolumeUncomputable`, `props_band_opposite`), before any boolean. r2 skipped the same case.
- **r2 `reuse_volumes`** keeps its flags: 16 at 1e-9 and 1e-12, 23 at 1e-6. Each flag is the probe's grid quadrature against a body whose exact volume the kernel matches:
  - π/3 = 1.047198 for the quarter ball;
  - pole-corner 0.132961 and 0.371039;
  - lens 0.0038943;
  - at 1e-6 the 0.02 rad "thin-sliver", two perpendicular planes through the centre: a quarter ball π/3 and a quarter of the r 0.45 ball, 0.095426. The kernel gives exactly these. At 1e-9 that fixture refuses `Pcurves(ArcNearPole)` before any boolean, as r2 noted.

  This is r2's own adjudication, and it reproduces. No tier or point failure appears among them.
- **r2 `build_census`** passes at all three ε (with ×1e3 skipped at 1e-12).
- **r2 `antipodal_vertex_refuses`** now fails at 1e-9 because the point answers `Ok(In)`. That is the fix, as intended.
- **My own random points** (`verify_4046_random_points`): 5 carved bodies × 3 500 points each. Per body, 2 000 are uniform over the box and 1 500 sit at ±1e-2…1e-8 off every sphere. The oracle is the CSG signed depth over the primitives, and an answer is wrong when it contradicts the sign at |depth| > 1000ε. The bodies:
  - the item's lens union;
  - the r2 antipodal-pose union, r 1 ∪ r 0.5 along (0.3, 0.9, 0.3);
  - a posed ball ∖ the strut box;
  - ball ∖ a tilted half-space;
  - a three-ball triangle union.

  Result: **0 wrong and 0 far-OnBoundary at all three ε** (52 500 queries). Every refusal is `Escalated(bool_point_in_solid_sphere)`. Most are points 1e-8 off a sphere; the rest (21–67 per body at 1e-9) are in band of a carrier where the face is trimmed away. That is the pre-existing P3 the PR re-filed with a measured witness (`work/cleave/point-in-solid-curved-arms-read-the-band-before-the-face.md`). There were none at 1e-12.
- **Not buildable, so not probed.** A third ball, a box corner or a box edge crossing a carved sphere face refuses at Join with `RingOffCylinderChart`. That is filed (`work/tang/a-ring-on-a-sphere-face-has-no-island-winding.md`).

## Claim checks

1. **Seam row: true.** Deleting the `retain` reds `a_face_whose_only_edge_is_a_seam_is_the_whole_sphere` at 1e-9 and nothing else. The row is built by a `kef` on a revolved ball and asked at the face door, not through a public boolean. It is still a default-ε pin of the mechanism.
2. **Antipodal vertex: true.** Every aimed and schedule direction is cast both ways (`sphere_region.rs:266-272`). The row answers In at ×1 and ×1e3 at 1e-9 and 1e-6, and r2's probe reads `Ok(In)`. Removing the reverse rays restores the reviewer's exact refusal.
3. **Strut row reads a region: true.** It is red under flip and parity at `ball ∪ box (0.0905, 0.4897, 0)`, which reads Out where it should be In.
4. **Tilted-cut row: true.** The added slab pose leaves one sphere face (asserted in the row), and the flip reds it there.
5. **Mutant table: reproduced exactly** (14/19, 15/19, noseam → seam row).
6. **Interval row: true.** `the_lens_union_classifies_points_at_the_interval_scalar` runs in the `sweep` `all` binary (the `test` job), is red under flip and parity, and stands down at 1e-12 as disclosed.
7. **Azimuth gate kept, and the code says why: true by inspection, unpinned by any row.**
   - The comment (`solid_contain.rs:3221-3226`) is right about the mechanism. `latitude_extremes` folds the boundary levels with no orientation, so a pole the loop encloses is invisible to the fold. A rim/meridian loop stepping around a pole unwraps to a whole turn, which the period gate refuses.
   - I tried to construct such a loop through public booleans and could not. The notched cap (cap above y 0.5, minus the quadrant x>0, z<0 below y 0.85) refuses `ResultInvalid(VolumeUncomputable … NotIsoRectangle "props_rim_level")`. A rectangle or C-shaped rim/meridian hole refuses the same way or with `Join(RingOffCylinderChart)`. Cone-wedge cuts refuse `CurvedPairUnsupported` (Cone × Sphere).
   - With the gate disabled, all of `topo` and `sweep` stay green. The gate is therefore currently defensive against bodies the kernel cannot yet build. Not blocking. A hand-built (Euler-op) row would pin it if one is wanted.
8. **Docs and words: true.**
   - `KindUnsupported`'s doc now names the region.
   - `PartialSphereFace` names splines and the quartics left by off-centre tori, cylinders and cones; the recourse text agrees.
   - `bool_sphere_trim` and `bool_sphere_trim_pole` are gone, and every remaining `bool_sphere_trim_*` or `bool_sphere_region_*` word has a producer.
   - `bool_point_in_solid_sphere` is registered and used by both sphere arms.
   - `first_harmonic_roots_in_band` returns the bare `Indeterminate`, and `sphere_region.rs` holds no `unreachable!`.
   - `work.py lint` passes on the head.

## Adversarial read of the fix pass (b793623189..36a9d28a, plus 66818a12)

The reverse rays only add rays. Each is decided by the same closest-crossing rule, so they can turn a refusal into an answer but cannot change a definite answer. The error retyping is behaviour-neutral: `first_harmonic_roots` wraps exactly as before. The rename moves the decision name only; the margin is the same. None of the review findings is reopened by the fix pass.
