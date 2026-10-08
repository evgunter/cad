# Review r1 — PR #4344, frozen head `bf57b509` (base `f3755cd4`)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 1 · MINOR 4 · NOTE 6.
No wrong body was found. 234 runs that main refuses `SpheresMeet` build SOUND through `differential::outcome`, held against a lens volume I derived independently. 0 lines moved in any battery. All three mutants go red. The MAJOR is the deviation's evidence: the cut-in's bodies cannot be drawn, and the alternative it rejected was measured on half a re-chart.
Lane isolation: I read no other review branch or session, and saw nothing of one.
Method: every build goes through `outcome` (tiers 2 and 3′, the certificate, the legal-operand check, volume to 1e-7 absolute) plus `mesh::tessellate` + `check_mesh`, against `lens()` = π(R+r−d)²(d²+2dr−3r²+2dR+6rR−3R²)/12d. That form never forms the radical plane, whereas the PR's `oracles::lens_volume` shares `sphere_pair_cut`'s formula. Probes: `crates/sweep/tests/review_sphpair_r1_probes.rs`, 14 rows. Logs: `review-r1/` (base vs head probe logs, three ε, mutant diffs and logs, curved suites). Release builds, base and head in separate target dirs.

## Claims

1. **Bodies right — HOLDS (executed).** Base→head transitions over 856 lines: 234 `SpheresMeet`→SOUND; 138 SOUND→SOUND; 0 SOUND→worse. These include:
   - non-parallel charts: 270 runs (small ball turned z30/z90/x45/xy60/xyz110, r ∈ {0.3, 0.7, 1}, three centre lines), 180 SOUND;
   - near either pole, θ−α from +0.3 to +1e-3: SOUND;
   - three balls, the unit ball cut twice by two partners (`+z,−z`, `+z,tilt`, `tilt,tilt2`): every one-lump op SOUND;
   - radius ratio 1e-3 at t = 0.1 and 0.9: SOUND. The relative volume error is ≤1.2e-6, which is ~1e-16 m³ absolute, below ε·area. `outcome`'s absolute 1e-7 is vacuous at this scale, so I checked it with the separate `r1_tiny_ball_relative_volume` row.

   Every remaining non-SOUND line is a typed refusal or the known census refusal (MINOR-1/2/3, NOTE-1).
2. **Right door, sound deviation — FALSIFIED in part (MAJOR-1, MINOR-4).**
3. **`sphere_pair_cut` exact enough — HOLDS (executed).**
   - Near-coincident pairs (r = 1 and 1 ± 1e-7, d from 1e-2 to 1e-5) build SOUND.
   - The near-tangent rows (10⁻⁴ and 10⁻⁶) build at the default ε.
   - `Interval` plain and tilt are SOUND at 1e-9 and 1e-6; at 1e-12 they escalate by name (`pcurve_map_residual`, `Crossing(OnEdge)`), as pinned.
   - Ratio 1e3 refuses throughout (MINOR-2).
   - The form `d² + R² − r²` loses `R² − r²` to cancellation when R ≈ r. The stable `(R−r)(R+r)` would remove that; the cost measured here is below 1e-13 (NOTE-2).
4. **New refusal / narrowed `SpheresMeet` — HOLDS (inspection + executed).**
   - `ops.rs:4127` fires only in a pass that pushed a sphere cut, and main returned `SpheresMeet` there, so no pose main built can reach it.
   - A re-chart on one shell beside a cut-in on another shell of the same operand builds 6/6 SOUND (main: `SpheresMeet`). That executes the "carve keeps keys" comment at `ops.rs:869`.
   - The only `SpheresMeet` raise left is `ops.rs:4042`, bound to `Refused::Zero`. `Negative` survives only in `offer_rows.rs:2087` and the editor-core fixture, as filed on HONE.
5. **Nothing else moved — HOLDS (executed).**
   - `pierce_runs_battery`: 4536 lines, 0 moved (4326 SOUND, 210 EMPTY ok).
   - `rc_wide` shards 0, 30 and 61 of 84: 1440 lines, 0 moved.
   - Curved suites (ring-on-sphere, carved-sphere, germ-sphere, pr9c sphere doors, tilted sphere pair, sphsph/cylsph, reach split gates, m5_s13 incl. interval and review probes, pi-seam, round-tube): base 128/128 pass; head 137/137 (2 renamed, 9 new).
6. **Mutants real — HOLDS (executed; `review-r1/mutant_*`).**
   - M1 (B's cut skipped): 10 red. It also ships tier-3-valid **wrong** bodies at ratio 1e3, e.g. 0.117387760 against 0.117387995. The head refuses those poses.
   - M2 (foot moved 0.2ρ off the circle's centre): 6 red, all loud.
   - M3 (closed-group arm dropped): 10 red. `two_trimmed_spheres_build` stays green, which is the right discrimination.
7. **Sweep complete — HOLDS as stated, but the blind spot is the common case (MINOR-3).**

## Findings

**MAJOR-1 — the deviation rests on an unfair comparison, and its bodies cannot be drawn (executed).** Two measurements:
- `mesh::tessellate` refuses every body the cut-in builds (`UnsupportedCurvedShape{NotIsoRectangle}`): all 234+ in my probes, including the plain witness. A circle off every edge of a full ball cannot contain a pole, so it is always tilted against the kept chart. That is TESS P0 `sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane`.
- The plain pose with **both** balls charted along the centre line (`r1_cut_in_against_both_seams_crossing`) builds all six ops SOUND on **main and head**, F4 E6 V4, and tessellates (`mesh=ok`).

The PR body's "a ring answers less" re-charted only the small ball, so the unit ball kept a ring and hit FLUX. The scan already owns a closed-group re-chart (`apply_recuts`); aligned to the centre line, it is the fair comparator for closed pairs. It cannot serve trimmed groups or the lens double cut, so the cut-in may still be the right door. But the claim that the cut-in "does strictly better" is falsified for closed pairs as measured, and the drawability cost appears in neither the PR body nor the JOIN row.
Fix: state the tessellation consequence in the PR body and the row, add this evidence to the TESS row, and either weigh re-chart-for-closed / cut-for-trimmed as a design fork or record why not.

**MINOR-1 — circles 10²–10⁴ ε from a chart pole refuse without cause (executed).** `apply_cut_ins` orders the meridian hits and the circle crossings by sine of latitude: `ops.rs:4500` (`latitude`) and `ops.rs:4593` (`above`, `Margin::levered(x−y, r)`). A pole hit and a crossing δ apart along the sphere differ by δ²/2r, so the band is effectively √(2rε) ≈ 4.5e-5 at r = 1. 78 head runs refuse "the sphere face's boundary meets the cut's meridian inside the section circle":
- circles 1e-5 and 1e-7 rad from a pole;
- the seam-circle family at ±1e-5 and ±1e-7;
- ratio 1e-3 at mid-depth, where the circle passes 5e-7 m from the tiny ball's pole.

Main refused all of these (`SpheresMeet`), so nothing regresses. But the refusal is typed on a geometry the band decides, and this PR newly routes every closed ball through this code.

**MINOR-2 — ratio 1e3 refuses at every depth (executed).** All 30 runs (unit ball against r = 1000, t ∈ {0.001, 0.1, 0.5, 0.9, 0.999}) refuse `CurvedPierceUnsupported` on the re-entered cut edge (EdgeKey 3). Unsure whether it is justified. M1 shows the same poses can ship silently wrong bodies when only one cut lands, so the area is fragile.

**MINOR-3 — the "same circle with crossings elsewhere" blind spot is most poses, not a corner (executed).** With `y`-poled balls, a centre line tilted ~12° (`census cut-in off`, c = (0.2, 0, 0.93)) falls through to the crossing layer. The unions and a∖b then stop at FLUX `RingOnCurvedFace` on base and head alike. 90 of 270 non-parallel-chart runs end the same way, as do partner balls at ±x.
`a_tilted_centre_line_builds` uses tilts of ≤3°, so the row cannot exercise this (style Q3). The blind spot is typed and filed on GERM `interior-loop-cut-in`, so this is a scope note, not a defect.

**MINOR-4 — the cut meridian survives as a needless face and edge (executed).** For the same plain geometry:
- cut-in: a∖b is F6 E10 (unit sphere 3 faces, small 3), a∪b F4 E6 with the unit sphere at 3 faces;
- both-seams ordinary path: F4 E6 V4 for every op, 2 faces per sphere.

The input face's identity is split (D5) across an edge that bounds nothing geometric. DESIGN.md:524–549 lets a same-surface curved adjacency ship only as a recorded skip. I could not find a record that this cut is one. `a∪b` and `b∪a` agree, so member order holds.

**NOTE-1 (executed).** Two-lump results (three balls: b∖a, a∩b) ship tier-3′-failing bodies, 12 runs (`CensusUndecidable`, lumps 1.3 apart). This is the known `work/contact/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`, as the PR's lens row acknowledges.

**NOTE-2 (inspection).** `sphere_pair_cut` (`ops.rs:4198–4215`) re-derives the radical-plane circle that `geom_brep::sphere_sphere_section` (`intersect.rs:1249–1299`) already computes behind decided branches with a unit-safe `u_ref` (`ss_frame_seam`). The copy's `u_ref: normal.cross(u_ref)` (`ops.rs:4214`) is unit only where the centre line runs along the chart's pole.

**NOTE-3 (executed).** `Interval` with non-parallel charts (z30) escalates at every ε (`Join` Indeterminate `MarginDiag(Invalid)` at 1e-9 and 1e-6; `Crossing(OnEdge)` at 1e-12), where f64 builds. The PR's `Interval` row covers only parallel charts.

**NOTE-4 (inspection).** The JOIN row's `## Built` names `ops::sphere_pair_circle` (line 86); the function is `sphere_pair_cut`. The row stays `status: open` although it records `## Built` and closes ORBIT's duplicate.

**NOTE-5 (executed).** Without tessellation, the PR's suite never meets MAJOR-1. It also never reads the certificate or the legal-operand check. Its volume oracle (`oracles.rs:246`) shares the code's formula. All head rows do pass `outcome`, though.

**NOTE-6 (executed).** Tessellation also fails on main's own off-axis sphere pairs (`ordinary-x`), so the mesh gap predates this PR. MAJOR-1's point is only that this door never produces the drawable case.

## Style

- **Q1, sure:** a second spelling of the radical-plane circle (NOTE-2). I swept `rg radical` and the `d.powi(2) + r…powi(2)` literal: the same form appears three times (`ops.rs:4204`, `intersect.rs:1249`, `oracles.rs:246`).
- **Q1, likely:** `SphereCutIn` now carries two meanings of `u_ref`, "the plane's reference direction" and "`axis × u_ref`" (`ops.rs:3683`, `ops.rs:4214`). Each producer has to keep its own unit-ness by hand.
- **Q2, likely:** the "carve keeps every kept entity's key" comment (`ops.rs:869–872`) asserts an invariant no type enforces. I executed it (6/6 SOUND), but nothing pins it.
- **Q3, sure:** `a_tilted_centre_line_builds` has a premise that excludes the failing mode (MINOR-3). `assert_six` is absolute in neither direction, and its 1e-9 relative bound against a shared-formula oracle cannot catch a radical-plane error common to both.
- **Q4, sure:** the `SpheresMeet` `Display` still says "cross" (`mod.rs:3511`). That is filed (HONE), so it is not a half-fix.
- **Q5, likely:** `sphere_pair_cut`'s doc says the cut's `u_ref` is "unit where `apply_cut_ins` reads it". That holds only through the fallback's geometry, and no assertion backs it.
- **Q6, sure:** the deviation is disclosed. "The ring door is still open to build on top of this" has no work row, and the drawability cost (MAJOR-1) is unscheduled against TESS.
- **Q7, unsure:** I would not have routed closed balls through the trimmed-group cut, given that a centre-line re-chart is drawable (MAJOR-1).
- **Q8, partial:** I read the new suite whole and `sphere_extent_scan` / `apply_cut_ins` whole (`ops.rs:3740–4800`). I did not read all 7118 lines of `ops.rs`.

REVIEW COMPLETE
