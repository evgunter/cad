# Second verification of PR #4123, frozen head e74ca65a

**Verdict: VERIFIED.** Both of the first verifier's blocking points are fixed.

1. **The Zone arm is sound.** At the 12 ε pad, the face box now contains the face at every tilt, ε and r in the first verifier's table, with the whole pad to spare. At pad 0, no lane under-covers any random sphere face whose axis lies near a box row.
2. **The merge with current main is green.** `origin/main` 8a110e84 merges cleanly, and all three `bounds_census` rows pass on the merged tree.

There is nothing blocking. Two MINOR points and some NOTEs follow; none is a soundness defect.

Scope:
- **Branch:** `reach/split-gate-sphere-azimuth` had not moved past `e74ca65a680c53bc6109cbc5e2901668a515e412`.
- **What I read:** the first verifier's `verify.md` on `analysis/reach-verify/4123`, and the PR body through GitHub MCP `get`. I read no PR comments or reviews.
- **Build:** my own target directories outside the worktrees, `CARGO_INCREMENTAL=0`. The probes ran in a scratch worktree; nothing was pushed to the PR branch.
- **Oracle:** mpmath at 60 digits, fed each face's stored f64 inputs. Rim latitudes are read from each rim plane's own rounded origin, `fl(c + z·fl(r·h))`.
  - Without that step, the oracle charged up to 4.9 ulps of `|c_k| + r` that come from the rounding of the *other* coordinates of `c`, not from the reach.
  - What remains is about 1 ulp of model ambiguity, because the frame vectors are unit only to within an ulp.

## 1. The first verifier's table, re-measured

Fixture: the strut cap (`h0 = −0.3`, strut to the south pole), axis `(sin t, 0, cos t)`. The probe varied:
- `c = (120, −40, c_z)`, with `c_z ∈ {0, 75, −2500}`;
- `r ∈ {1, 1e3}`;
- `t ∈ {1e-9, 1e-8, 1e-7, 1e-4, 1e-3, 1e-2}`;
- three ε values.

That makes 36 faces per ε. All of them read the Zone arm. The face box is `face_box(…, sweep_pad, …)`, compared with the exact support on all three rows.

| ε | r | tilts | worst (exact − padded box), over all rows | first verifier on 3c17e711 |
|---|---|---|---|---|
| 1e-9 | 1e3 | 1e-9 … 1e-2 | **−1.200e-8 m** (inside by the whole pad) | +8.0e-6 to +9.8e-6 m past the pad at t 1e-8 |
| 1e-9 | 1 | 1e-9 … 1e-2 | −1.200e-8 m | — |
| 1e-12 | 1 | 1e-9 … 1e-2 | −1.200e-11 m | +8.0e-9 to +9.8e-9 m at t 1e-8 |
| 1e-12 | 1e3 | 1e-9 … 1e-2 | −1.539e-11 m | +9.4e-11 to +1.2e-10 m at t 1e-4 |
| 1e-6 | 1, 1e3 | 1e-9 … 1e-2 | −1.200e-5 m | covered |

**No deficit remains.**
- At pad 0, the charged reach clears the exact support by at least 15.15 ulps of `|c_k| + r`.
- Without the 16-ulp charge, the shortfall is at most 1.30 ulps.
- `face_box` at pad 0 and `census::face_reach` agree with the reach.

## 2. Soundness sweep (pad 0, exact support, all three rows)

**Faces.** 2400 random faces per ε, at `r ∈ {1e-3, 1, 1e3}` with `|c| ≲ 3.5 r`. Each axis is tilted `t` off a random box row:
- `t = 0` exactly, `2⁻⁵²`, `2⁻⁵³` or `1e-8`, each with probability 1/8;
- otherwise log-uniform on `1e-16` to `1e-3`.

Each tilt has a random sign and azimuth. The seam is rotated randomly two times in three, and otherwise left on a row.

**Families:**
- **cap:** a strut cap, three in four onto a pole and one in four short of it.
- **rect:** a posed chart rectangle, azimuth width 0.05 to 3.0 rad.
- **reflex:** the same, with width 3.3 to 6.2 rad (a reflex loop).
- **zone:** a two-rim zone joined by one meridian strut, so it wraps alone. It is built under each sense: `zone_t` uses sense true, `zone_f` sense false.
  - This is the family the first verifier could not build (`LoopNotClosed`). The fix is to give the second rim the opposite parameter sense, `axis −ẑ`, so that the two rims run opposite ways around the loop.

**Lanes:** `sphere_reach` (charged), the same reach uncharged (charge subtracted), `face_box` at pad 0, `census::face_reach`, and `face_box` at `sweep_pad`.

**Results.** Worst under-coverage is in ulps of `|c_k| + r`; a positive number means the lane falls short of the face.

| ε | family / arm | faces | reach (charged) | reach (uncharged) | `face_box` pad 0 | census | `face_box` at 12 ε |
|---|---|---|---|---|---|---|---|
| 1e-9 | cap / zone | 440 | −13.84 | 1.98 | −14.40 | −13.84 | inside by the pad |
| 1e-9 | rect / rect | 600 | −12.58 | **3.46** | −12.70 | −12.58 | inside by the pad |
| 1e-9 | reflex / rect | 600 | −13.38 | 2.70 | −13.96 | −13.38 | inside by the pad |
| 1e-9 | zone_t / zone | 600 | −13.59 | 2.35 | −14.01 | −13.59 | inside by the pad |
| 1e-6 | cap / zone | 439 | −13.44 | 2.39 | −13.90 | −13.44 | inside by the pad |
| 1e-6 | rect / rect | 599 | −13.43 | 2.70 | −13.88 | −13.43 | inside by the pad |
| 1e-6 | reflex / rect | 600 | −13.96 | 2.10 | −14.13 | −13.96 | inside by the pad |
| 1e-6 | zone_t / zone | 600 | −13.73 | 2.37 | −14.27 | −13.73 | inside by the pad |
| 1e-12 | cap / zone | 430 | −12.61 | **3.13** | −13.50 | −12.61 | inside by the pad |
| 1e-12 | rect / rect | 582 | −13.47 | 2.53 | −13.48 | −13.47 | inside by the pad |
| 1e-12 | reflex / rect | 578 | −13.43 | 2.48 | −13.92 | −13.43 | inside by the pad |
| 1e-12 | zone_t / zone | 582 | −13.24 | 2.56 | −14.07 | −13.24 | inside by the pad |

**Every Rect and Zone read is sound at pad 0, with at least 12.58 ulps of margin.** That holds on every row, at every scale, tilt and ε, and includes `t = 0` and `±1 ulp`.

**Faces that read the Ball:**
- the struts short of a pole (about 160 per ε);
- every `zone_f`, which the side guard refuses (600 per ε);
- 5 rect/reflex faces at ε 1e-12.

For these, the reach is the one-rounding `c ± r` and falls short of the exact support at pad 0 by up to 2.28 ulps. That is main's whole-carrier arm and is unchanged by this PR; see NOTE 1.

**Not built:**
- At ε 1e-12: 19 caps, 15 rects, 20 reflex faces and 18 zones. All are bodies whose rounding the run's ε cannot resolve (`r = 1e3`, or `mint_pcurves` refusing).
- At ε 1e-6: 1 rect.

**The `perp_room` readers**, with the same poses, 2400 per ε. Each was read directly against the exact support of the whole carrier (slab, cone frustum, torus) or of the torus window. The units are ulps of `|c_k| + R` (cone: `|c_k| + |h|·(1 + tan)`).

| ε | `slab_extent` | `cone_frustum_extent` | `torus_extent` | `torus_window_extent` |
|---|---|---|---|---|
| 1e-9 | 0.99 | 0.73 | 0.80 | −1.8e8 (sound by far) |
| 1e-6 | 0.92 | 0.69 | 0.75 | −2.2e7 |
| 1e-12 | 0.97 | 0.66 | 0.74 | −2.3e8 |

- **These are the extents' own last-place roundings, and the 12 ε pad covers them.** These extents carry no charge of their own, and the PR claims none; no near-row tilt makes them worse.
- **The old spelling was far worse.** Under the cancelling `√(1 − a²)` mutants (§3), the committed near-row row reads the slab, cone and torus short by `r·sin t`, up to about 1e-5 m at `r = 1e3`.

## 3. Mutants: the `√(1 − a²)` spelling restored, one arm at a time

Each mutant ran against the PR's committed rows in `topo` (filter `boolean::boxes|census|splitting|separation`, 205 rows) and was then reverted.

| arm | mutant | red rows |
|---|---|---|
| `sphere_reach` Zone arm | `pick_other(a, k)` → `(1 − ax²).max(0).sqrt()` | `sphere_rect_rows::a_zone_axis_near_a_box_row_keeps_its_radial_share`, `…::a_zone_reach_charge_covers_random_poses_near_a_box_row` |
| `slab_extent` | room → `√(1 − abs_min(a_i)²)` | `tests::an_axis_near_a_row_keeps_its_perpendicular_room` |
| `cone_frustum_extent` | same | `tests::an_axis_near_a_row_keeps_its_perpendicular_room` |
| `torus_extent` | same | `tests::an_axis_near_a_row_keeps_its_perpendicular_room` |
| `torus_window_extent` charge | same | **none.** It survives all 2431 `topo` rows (full `-p topo`). |

**MINOR 1: the torus-window charge's fix has no row that pins it.**
- The PR body says that `perp_room`, "read by … the `torus_window_extent` charge", is fixed. That is true, but no row would notice if it reverted.
- The near-row row covers only the slab, cone and torus. The PR's mutant claim ("red under a cancelling `perp_room` mutant") holds only for a mutant of the shared function, not for this arm alone.
- **Not unsound in practice.** My probe re-ran 2400 torus windows per ε *under this mutant*, and every one stayed sound by over 2e7 ulps. The `u` channel's term is `(R + r)·p·h_u²/8` with `p` tiny, and the samples' hull and the `v` channel's charge dominate it.
- **Fix:** add a torus-window case to `an_axis_near_a_row_keeps_its_perpendicular_room`, or a pinning assertion on the charge itself. It is not blocking.

**The first verifier's four mutants, re-run on e74ca65a:**

| mutant | result |
|---|---|
| fix-pass M1 (wrap branch folds every loop level) | **red**: `a_caps_strut_does_not_bound_its_latitude_window`, top `0.30000000000000365` |
| fix-pass M2 (no pole level) | **red**: the same row, `floors = [-1.0000000000000002; 4]`. It also reds both new zone rows. |
| PR M1 (`trim.az` forced to `None`) | **red**: `a_sphere_faces_box_is_its_chart_rectangles_support` and `sweep` `reach_split_gate_azimuth::every_cut_clear_of_a_partial_turn_sphere_face_splits` |
| PR M2 (Rect `az.lo + 0.05`) | **red**: `a_sphere_faces_box…` and `sweep …every_cut_into_a_partial_turn_sphere_face_refuses_at_the_gate` (empty half vs 3.9e-8) |

## 4. The class claim

I grepped `crates/topo/src` and `crates/geom-brep/src` for `1 − x²` (or `1 − x·x`, single- and multi-line) followed by `sqrt`. The production hits:

| site | reading | status |
|---|---|---|
| `geom-brep/src/props/curved.rs:3803` | rim level `(s, √(1 − s²))`, a decision meter | **filed**: `work/flux/sphere-rim-level-cosine-cancels-near-a-pole.md` |
| `topo/src/offset_axial.rs:1419` | `sin_d` of an angle's cosine, for a decided separation | not a unit vector's extent; a cancellation reads Zero, which refuses (fail-safe) |
| `topo/src/boolean/circle_torus.rs:491` | `sin_spread`, a divisor of the root slack | not an extent; a cancellation inflates the slack, which gives `Uncertain` (fail-safe) |
| `geom-brep/src/ssi/march.rs:1366` | `across`: `‖v⊥‖ = len·√(1 − cos²)`, the march's step heuristic | not an extent. It is the same shape, but it only sizes steps (see NOTE 3) |
| `topo/src/splitting/containment.rs:1876` | `1 − |h|²`, a decided disc, no `sqrt` | not this class |
| `geom-brep/src/pcurve_cache.rs:7246` | an eccentricity | not this class |

**There is no unfixed, unfiled, in-class site.** `classify::zone_extent` no longer exists on the branch, so it has no remaining callers. The PR body's classification of each site matches mine.

**MINOR 2: test oracles keep the cancelling spelling.** `boolean/boxes.rs` test module, `:4142`, `:4209`, `:4623` (`(major + minor)·√(1 − a²) + minor·|a|`) and `:4663` (the torus window's expected charge).
- They are test oracles, not code paths, and today no near-row axis reaches them.
- But they would agree with a regressed spelling at the bit if one did. Moving them to the norm of the other two components keeps them independent of the code under test.

## 5. Merge with current main

- **Clean merge.** `origin/main` 8a110e84, merged into e74ca65a in a scratch worktree, has no conflict.
- **No compile break to patch.** #4178 (the `tier3_tests.rs` compile fix) has merged into main, so I applied no local fix.
- **`geom-core` `bounds_census` on the merge, own target dir:** `every_roster_line_states_a_reason`, `the_roster_names_each_door_once` and `every_sole_bracket_bound_door_is_in_the_roster` are all green.
  - The tree holds one `surgery.rs` `worse` line (`Selection`), main's.
- **The PR's rows on the merge, ε 1e-9:** 64/64 green. That covers `boolean::boxes`, `sphere_rect_rows`, `rect_extent_rows`, `reach_split_gate_*` and `splitting::classify`, slow row included.

## 6. ε results on the head

nextest `-p geom-core -p topo -p sweep` on a clean head tree (probe removed):
- **ε 1e-9:** 5543/5543.
- **ε 1e-12:** 5543/5543. `arc_loft` is not in these crates.
- **ε 1e-6:** 5537/5543. The 6 reds are:
  - `pocket_ring_steep_ellipse::steep_ellipse_poses_build_sound_or_refuse_typed`;
  - `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`;
  - `split_across_a_revolve_seam` ×3 (`a_tube_cut_across…`, `a_section_touching_a_rim…`, `a_counterbore_and_a_cone_socket…`);
  - `m5_pr6_pcurves::a_seam_closed_tube_split_mints_clean_halves`.

**Five of the six are already fixed on current main.**
- On `origin/main` 8a110e84 at ε 1e-6, only `pocket_ring_steep_ellipse` is red; the pinch row, the three `split_across_a_revolve_seam` rows and `m5_pr6_pcurves` pass.
- On the trial merge, in its own target dir, the result is the same: `pocket_ring_steep_ellipse` is the only red of the six.
- So the head's six reds come from its base (aeb2bfaa), not from this diff. The PR body says "red identically on `origin/main` aeb2bfaa", which I did not re-check at that commit. NOTE 4: merging main clears five of them.

## 7. Claim checks (verifier fix pass)

| claim | checked |
|---|---|
| Zone arm and `perp_room` read `√(a_j² + a_k²)` | **true** (`boxes.rs` `pick_other`, `perp_room`/`others`; slab, cone, torus and torus-window call sites) |
| `classify::zone_extent` gone; the gate reads `sphere_reach` | **true** (no hits in `crates/`) |
| the verifier's repro row, 5 tilts | **true**, and red under the Zone mutant |
| random-pose zone row, ≥ 600 caps, pad 0, measured ≤ 2.42 ulps, asserting ≤ 4 | **true, as a row.** My independent oracle reads the uncharged caps at up to 3.13 ulps (ε 1e-12) and rects at up to 3.46 ulps (ε 1e-9); see NOTE 2. |
| near-row slab/cone/torus row | **true**; it kills each of those three arms alone. It does not cover the torus window (MINOR 1). |
| `props/curved.rs:3803` filed | **true** (`work/flux/…`) |
| main's `Selection` roster line kept; `bounds_census` green | **true**, on the head and on the merge with 8a110e84 |
| PR body "Last fix pass", 16-ulp correction | **present** (the "Correction to NOTE 3" bullet) |

## NOTEs (not blocking)

1. **The Ball arm's `c ± r` is uncharged at `T = f64`.** It reads up to 2.28 ulps of `|c_k| + r` inside the exact support at pad 0. This is unchanged from main for every kind's whole-carrier arm, and the pad covers it. The `sphere_reach` doc says so ("The ball is not [charged]").
2. **The measured-shortfall figure depends on the oracle.** The `SPHERE_REACH_ULPS` doc cites "under 2.5" from the PR's row. Two things move it:
   - My stored-geometry oracle gives up to 3.13 (zone) and 3.46 (rect).
   - Reading the rims at their intended rather than stored latitudes moves it to 4.9, because rounding in other coordinates of `c` leaks into row `k`. The charge is stated in ulps of `|c_k| + r`, so for a centre far off along another row (`|c_j| ≫ |c_k| + r`), the trim's rim latitudes carry rounding the charge does not count.
   - That stays far inside the 12 ε pad for every body the run's ε resolves, and it is not this PR's regression. But the stated per-step bound ("under 12") implicitly assumes `|c|∞ ≈ |c_k|`.
3. **`ssi/march.rs:1366` `across` loses `‖v⊥‖` when `v` is nearly along the tangent.** It is a step heuristic, and a lost bend only drops the `h_fit` rung (`kappa3d > 0.0`). I did not trace whether the march certifies independently; the PR's classification ("step heuristic") stands.
4. **Hosted CI (per the PR body) is green on e74ca65a.** Merging current main would also clear five of the six local ε 1e-6 reds listed above.

## Probes (not committed)

- **Rust probe:** `mod v2probe`, mounted inside `boxes::sphere_rect_rows` by `include!`. It has three rows:
  - `v2_table` (§1);
  - `v2_sweep` (§2, including the posed rectangle and the two-rim strut-zone builders);
  - `v2_perp` (§2, the `perp_room` readers).
- **Oracle:** an mpmath script (`oracle.py`) reading the probe's JSON lines.
- **Run:** `PROBE_N=2400`, `--test-threads 1`, at `CAD_TOLERANCE_EPS` ∈ {1e-9, 1e-6, 1e-12}.
