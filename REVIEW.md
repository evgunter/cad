# Review r2 — PR #4344 (sphere pair crossing off every edge)

Frozen head `bf57b509`, base `f3755cd4` (the main it merged). Probe rows:
`crates/sweep/tests/review_r2_sphere_pair_probes.rs` (P1–P15, `#[ignore]`d, one `differential::outcome` line plus face
census and `check_mesh` per run), run on base, head, and a mutant tree (env-switched M1/M2/M3). No glimpse of the
other lane; PR comments not read.

**Verdict: APPROVE-WITH-FIXES** · MAJOR 0 · MINOR 6 · NOTE 4

## Claims

1. **Bodies right — holds.** No build anywhere had a wrong volume (independent lens formula
   `π(R+r−d)²(d²+2dr−3r²+2dR+6rR−3R²)/12d`, not the suite's cap sum). P1, non-parallel charts (4 centre lines incl.
   `(1,1,1)`, chart turn 0…2.5 rad, r ∈ {0.05,0.3,0.7,1.5}, 6 ops): 480/480 SOUND; base 480 `SpheresMeet`. P12 (pole
   tilted 0–60° toward the centre line): 264 `SpheresMeet`→SOUND, rest identical. P4 three balls (one face cut by two
   partners: two longitudes, one longitude, tilted, neighbours): 12 SOUND, 12 BAD only by tier 3′ census (m3), "z and
   −x" identical to base. P5 ratios 1e-3…1e3: 69 SOUND, 3 BAD (m5), rest typed. Poles: a circle holding a pole crosses
   the seam, so P2/P3 reach the crossing layer and are line-for-line identical on base; near-pole refusals are m4.
2. **Right door, sound deviation — holds, costs unweighed.** No scan-path run of mine (~1100) stopped at FLUX's ring
   gate; M1 (one side cut) rings 160/480, confirming both sides are needed. But the cut leaves the user more faces
   than the same point set gets via the crossing layer (m2), no build tessellates (m1), and two-lump results lose tier
   3′ (m3); the ring comparison in the PR body counts none of these.
3. **`sphere_pair_cut` exact enough — holds** (n1).
4. **New refusal / narrowed `SpheresMeet` — holds.** `FallbackExtentUnsupported` ("rename") fired only on poses main
   refused `SpheresMeet` (P7 18/18). `SpheresMeet` is built only on `Refused::Zero` (`ops.rs:4044`); `Negative`
   survives only in fixtures (`offer_rows.rs:2085`, `mod.rs:6534`), filed on HONE.
5. **Nothing else moved — holds.** `pierce_runs_battery` 4536 lines, 0 moved (4326 SOUND, 210 EMPTY ok); `rc_wide`
   shards 7/84 and 41/84, 960 lines, 0 moved. Full sweep binary: base 2446 passed, head 2455; pass sets identical but
   the renamed m5_s13/verbs_sphsph rows and the 9 new rows. The PR suite with `verbs_sphsph_opening` and `m5_s13_pips`
   passes at ε 1e-9, 1e-6 and 1e-12.
6. **Mutants real — holds.** M1 skip B's cut: 10 rows red. M2 cut 0.2ρ off the centre: 6 red. M3 closed groups refuse
   `SpheresMeet` again: 10 red. On P1 each fails typed (M1 160 `ResultInvalid` ring, M2 480
   `FallbackExtentUnsupported`, M3 480 `SpheresMeet`), never silently. Survivors are expected:
   `two_trimmed_spheres_build` under M3; near-tangency and trimmed rows under M2.
7. **Sweep — holds; blind spot also has a pure sphere-pair witness** (n2).

## Findings

- **m1 MINOR — no body this PR builds can be tessellated** (executed). Every scan-path build fails `mesh::tessellate`:
  `UnsupportedCurvedShape { NotIsoRectangle }`. The P6 crossing-layer twins of the same point sets mesh `ok`. By
  construction the scan is reached only when the circle misses both seams, so the circle is tilted against both
  charts. This is TESS's P0 `sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane`, but the PR does not say
  so and adds no evidence there.
- **m2 MINOR — the cut meridians survive as extra faces** (executed, P6/P14). On the same point sets, the r 0.7 / d
  0.9 ∪ gives 6 faces and 8 edges between distinct faces of one sphere through the scan, against 4 and 4 through the
  crossing layer. P4's ∪ gives 10 faces. Two of the extra faces are `Mef`-born (the cut-in). So the output complex
  depends on where the operands' charts sit, which is against DESIGN's maximal-face form ("a union's body does not
  depend on member order"). D5 records are present.
- **m3 MINOR — tier 3′ census fails on two-lump results as a class, not one pin** (executed, P4/P8). With one ball
  holding two cuts, the scan path gives `CensusUndecidable` on all 12 two-lump runs (∩ and b∖a, both orders). The same
  shape through the crossing layer certifies `Ok`. The PR pins one lens op (`spheres_crossing_off_every_edge.rs:52`).
- **m4 MINOR — refusals near a chart pole, where the geometry is decidable** (executed, P10/P15). `apply_cut_ins`
  orders latitudes through `Margin::levered(x−y, r)` (`ops.rs:4593`, `:4743`). A latitude is quadratic in the distance
  δ to the pole, so a circle passing δ from a pole refuses whenever δ ≲ √(2εr). Examples:
  - r 1e-3 at d = 1: δ ≈ 5e-7, which is 50× the band → `FallbackExtentUnsupported` ("inside the section circle").
  - r 3e-3 → `Escalated bool_sphere_cut_order`.
  - `Interval` r 0.01 at d = 1, ε 1e-6 → refuses.

The pose is the common one: a small ball centred on the big sphere, with the centre line ⟂ the poles. Main refused it
too.
- **m5 MINOR — a sliver that builds but is not a legal operand** (executed, P5/P8). Unit ball against `r = 50`, 2e-6
  from either tangency: a∖b and ∩ build, and pass t2, t3′ and the certificate. A union with a far brick then refuses
  `ShellWitnessExhausted`, and ∩ measures −4.1e-12 against +1.23e-11. Main refused this pose; DESIGN says every
  boolean output is a legal operand.
- **m6 MINOR — the re-pinned rows can't see m1/m5** (inspection + m5 run). `m5_s13_pips.rs:317` and
  `verbs_sphsph_opening.rs:152` assert volume only. `assert_six` asks neither the legal-operand question nor
  `check_mesh`.
- **n1 NOTE — `sphere_pair_cut` (`ops.rs:4198`) is exact enough.**
  - `s = (d²+R²−r²)/2d` is geom_brep's own form. `R²−r²` cancels only absolutely (≈ εR²/d, harmless), and ρ comes from
    the factored `(R−s)(R+s)`.
  - Near tangency, f64 error in ρ ≈ εR²/ρ, ≪ band at 1e-6 depth.
  - `Interval` at 1e-4 from both tangencies and at r 0.01: tier 3, with the caps inside the bracket, at ε 1e-9 and
    1e-6.
  - At ε 1e-12 everything escalates or refuses by name (`CurvedPierceUnsupported`, pcurve `MapResidual`).
- **n2 NOTE — the blind spot in sphere-pair form.** When exactly one seam crosses the circle, the crossing layer gives
  the other face a ring, which stops at FLUX's gate. P13 (seam on the cut's own great circle): 48/144 runs
  `ResultInvalid { RingOnCurvedFace }`; P2: 63/126. All are identical on base. This is evidence for FLUX
  `sphere-face-with-a-hole-has-no-closed-form`.
- **n3 NOTE — refusals that moved kind** (main → head):
  - r 1e3: `SpheresMeet` → `CurvedPierceUnsupported` (30).
  - r 1e-2 at 2e-8 from tangency → `Join(SectionInvariant "tangent plane×sphere germ pair")` (12), which is an
    invariant-named variant carrying a frontier refusal; the cut plane's `Intersection` reaches it.
- **n4 NOTE — the PR's own near-tangency poses** (96) are SOUND through `outcome`, the legal-operand check included.

## Style

- Q1 *sure*: `sphere_pair_cut` writes out the radical-plane formula again (`ops.rs:4204`). Other copies:
  `geom-brep/src/intersect.rs:1249` (the owner), `section_cert.rs:1145` and `:1164` (which the same scan has just run
  on this pair), `census/curved.rs:227`, `validate.rs:7955` and `carrier_cross.rs:447`. None says it is a copy.
- Q2 *likely*: `sphere_pair_cut`'s `u_ref` "is unit where `apply_cut_ins` reads it" (`ops.rs:4191`). That invariant is
  held by prose alone.
- Q4 *unsure*: `ops.rs:870` says cut-in names hold because re-charts graft "other shells". That was prose until the
  new refusal at `:4127`; the comment does not cite the guard.
- Q3 *sure*: m6. The near-tangency row stands down off the default ε, so two of three ε are vacuous for it.
- Q5 *likely*: `SpheresMeet`'s doc (`mod.rs:2571`) now says "touch", while its payload still admits `Negative` (filed
  on HONE).
- Q6 *likely*: the deviation is better than the brief and needs no schedule. The tessellation consequence (m1) is
  undisclosed and unscheduled for this class.
- Q7 *likely*: I would not cut a closed ball pole-to-pole and ship the cut. The join or the output stage should
  consume it (m2).
- Q8 *sure*, partial: `ops.rs` is 7118 lines. I read `through_the_join`'s fallback, the whole of `sphere_extent_scan`,
  `sphere_pair_cut` and `apply_cut_ins` (~1100 lines), not the whole file.

REVIEW COMPLETE

