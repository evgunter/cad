# R1 review record — MESH-6 (PR 1545), frozen head d163719c962caf6616426af89f7f9037e63de5b8

Merge base used: 7e980faf5 (origin/main at review time; the PR page's recorded base 922eb5f is
older and its extra three docs files are main's own movement, not this PR's). Diff against the
true merge base: 5 files, all under `crates/mesh` (+535/−25).

## Files here

- `ab.sh`, `analyze.py`, `ab/` — the in-binary A/B reproduction (temporary `CAD_S65_SKIP` /
  `CAD_S65_CHECK` / `CAD_S65_CENSUS` switch planted at head, release profile, then REVERTED;
  PR-owned files byte-identical afterwards). `ab-summary.txt` is the min-over-rounds table.
- `census-attributed.txt` — the per-face census (`pole`, `ident`, `nu`, `nv`, `tris`) from the
  instrumented head binary over `r2_bytes`' tour.
- `r2_{head,base}_{dev,rel}.txt` — the D9 byte instrument at both revisions and both profiles.
- `mutants.py` — the unit-row mutation battery; `liveness.sh` — end-to-end liveness of both
  guards in the release profile as the manifest ships it and under the post-publish manifest.
- `phase2.sh` / `phase2.log` — the driver and its raw log; `eps_*.txt` — the three-ε battery.
- `base/` is a detached worktree of the merge base (NOT committed; it holds the cost instrument
  copied in for the cross-build corroboration).

## Measurement (reproduced, EXECUTED)

See `ab-summary.txt`. Headline, default ε, min of 4 interleaved rounds, one release binary:

| donut δ | +seam census | +chord census | both | +check_mesh | PR seam | PR chord (table) | PR chord (prose) |
|---|---|---|---|---|---|---|---|
| 0.1 | +5.0% | −7.5% | +13.3% | +31.2% | +12.9% | −2.6% | "+3–8%" |
| 0.02 | +8.7% | +0.6% | +14.0% | +32.6% | +13.1% | −2.0% | "+3–8%" |
| 0.004 | +11.8% | −0.4% | +14.6% | +24.1% | +8.4% | +2.7% | "+3–8%" |

Same-mode spread across the 4 rounds: 4–12% on the donut, 9–40% on rows under 1 ms (other lanes
were building/testing on the box throughout). Seam column on bodies whose identified set is
unchanged (l_prism has no curved face at all): −3.2% … +24.6% — the noise floor, as the PR says.
ε rows (single round each): donut seam +3.5…+25.5% at 1e-6, +6.6…+13.8% at 1e-12; the
check_mesh/tessellate ratio is ε-flat (donut 24.6–30.6% at all three rows).

## Census (EXECUTED) — corroborates the PR's table

ball = 2 hemisphere faces, pole=true, ident=2; cone = 2 half-cones, ident=1; washer walls
ident=2, nv=1, nu=10/15 … 50/71; donut = 2 torus patches, ident 44/96/212, nu×nv 85×43 …
422×211, tris 7310 … 178084. (band_0.1, r2's own body, adds cone walls at nv=1 with ident=2 and a
pole-free sphere band with ident 3–9.)

## CI gate record (API, both heads)

- d163719c9: run 33588098103, 22 check runs = 16 success / 5 skipped / 1 neutral; eps = 1e-6 rows.
- db14af1bc: run 33586798175, only red job `rustfmt + rustdoc (gate) + wasm32`, red at step
  `rustdoc (gate)`; `test (eps = default, 1/2, 2/2)` green.

## Cross-build head vs merge base (EXECUTED, release, min of 2 each; the PR's corroboration figure)

donut +10.4 / +14.0 / +11.2% (PR: +13.9% at δ=0.004). washer −0.8 / +9.8 / −6.5%, l_prism −9.5 / −5.7 /
−6.6% — the PR's "+25–34% washer, +27–31% l_prism" did not reproduce; on this box those rows are
inside the cross-build noise. Everything else −14.5% … +21.3% (cone_wedge 7 µs rows).

## D9 (EXECUTED)

`r2_bytes` merge base vs head: 21/21 identical in dev AND in release (debug-assertions on, guards
executing). Head dev == head release, 21/21. Note the instrument is 7 bodies × 3 δ = 21 rows; the
PR body's "8 bodies × 3 δ" is an arithmetic slip (the 21 is right).

## Mutation battery (EXECUTED, dev; `mutants.py`, files restored from HEAD after each)

| mutant | red rows |
|---|---|
| M1 UV-repeat detection disabled (`p != (u,v)` → false) | identified_ids_takes_the_seam…, a_seam_vertex_fanned… |
| M2 threshold `n > 2` → `n > 4` | a_seam_vertex_fanned… |
| M3 pole entries no longer kept | identified_ids_keeps_a_pole_corner… |
| M4 old pole-only edge filter restored | a_seam_vertex_fanned… |
| M5 chord census `n != 2` → `n > 2` | a_boundary_the_second_face_renumbered…, a_segment_no_face_emitted…, ids_at_or_above_the_mark… |
| M6 `shared_below` mark removed | ids_at_or_above_the_mark… |
| M7 bit-compare instead of spade `==` | a_repeat_at_the_same_uv… |

Every mutant is killed; every red-first row has a mutant that reds it.

## Liveness end to end (EXECUTED, release profile as the manifest ships it)

- L2: one patch withheld from the chord census → `tessellate` panics at tessellate.rs:211 on the first
  tour body ("chord segment Some((3, 4)) is used by 1 face triangles rather than 2"). The cross-face
  guard RUNS in `cargo build --release` from this workspace today.
- L1 (pole floor removed, tour deltas): null — the tour never reaches `nu == 2`; superseded by L1b
  on issue 678's own witness (`revolves::apex_wedges_never_size_to_a_single_azimuth_column`), see
  `liveness1b.sh` and the report.
- L3: with debug-assertions OFF the mesh LIB unit-test target does not compile at head (E0425 ×8:
  `identified_ids`, `overused_identified_edge`, `unpaired_chord_segment` are `#[cfg(debug_assertions)]`,
  the ten tests calling them are not). The merge base compiles clean under the same setting, so the
  break is this PR's. `walk.rs:1664` is the crate's own precedent for gating such a test.

## bounds_census disclosure (EXECUTED)

Green at head (2 passed). Planting `patch_triangles: impl IntoIterator<Item = &'a [[u32; 3]]>` on
`unpaired_chord_segment` reds `every_sole_bracket_bound_door_is_in_the_roster` at
geom-core/tests/bounds_census.rs:279. Disclosure verified.

## Three-ε battery (EXECUTED, dev, `-p mesh`)

default (1e-9), 1e-6, 1e-12: 119 lib + 137 integration green at each row. `the_eps_inventory_is_pinned`
green at head. No `Eps` call and no bare comparison added in the diff (the one `!=` on an `f64` pair
is the spade-equality rule, `#[allow(clippy::float_cmp)]`, same as `trimmed::id_repeats_apart`).

## L1b — the seam/pole census on issue 678's own witness (EXECUTED; `liveness1b.sh`)

Pole floor removed (`pole_columns` returns `nu`), row `revolves::apex_wedges_never_size_to_a_single_azimuth_column`:

- (i) release, manifest as shipped (`debug-assertions = true`): panics at curved.rs:322 —
  "face FaceKey(3v1): identified-vertex fan edge Some((1, 68)) used 4 times in one patch (nu = 2, nv = 128)".
  THE GUARD RUNS IN TODAY'S RELEASE BUILD.
- (ii) release with `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false` (the post-publish manifest): the
  assert is silent; `tessellate` returns `Ok` and the test's own `check_mesh` reports
  `NonManifoldEdge { edge: (1, 68), count: 4 }`. That is the compiled-out posture, and the silent
  non-watertight `Ok` that S65 is about.

PR-owned files byte-identical to HEAD after every mutant (verified `git diff --stat HEAD -- crates docs Cargo.toml` empty).
