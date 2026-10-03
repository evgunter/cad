# Delta review of PR #3977 after fix pass 1, frozen head 742850049c

Lane `reach-delta-3977`. **Verdict: NOT-MERGEABLE-AS-IS.** MAJOR 1 · MINOR 4 · NOTE 7.

I read the brief, `CLAUDE.md`, both prompt files, both dual reviews (r1, r2) and the PR body (`get`). I read no PR comments, reviews or check runs.

Method:
- Worktrees: the frozen head; main at the PR base `c7c71db43`; a clean copy of the head for the K sweep.
- I re-ran the reviewers' own probes on the head, mounted (r1 `probe_r1_slab` ported to the new `shell_role` signature, `probe_r1_booleans`, r2's `probe_r2_inverted`, `probe_r2_e2e` and `probe_r2_reuse`).
- My own probes ran on both trees. Oracles are closed forms: a box is `t·w²`, a 4-arc disc is `π r² h`. Never the kernel.
- The three mutants are env-switched (`DMUT`). `probes/delta-3977/` holds the probes, the mutant and mount diff, and the head and main logs.

## MAJOR

**1. Check 7 still passes inside-out bodies that main refused, whenever the walk has a curved face.** `props.rs:966` uses the world origin as the centre for any walk that is not all-planar. That keeps the far-origin blow-up for every such walk. `validate.rs:4461` and `plus_v_at_target` (`:4987`) then pass a straddle at the target as if it were in band.

DEMONSTRATED BY EXECUTION (`probe_d_disc.rs`, head vs main). The body is a 4-arc disc (one cylindrical wall), radius r and height h, placed d metres from the origin. Each was built upright and reverted.
- **ε 1e-9: 7 inside-out discs pass tier 3 at head, and main refuses all 7 `NegativeVolume`.**
  - r 1 mm × h 100 nm at 1, 5 and 20 km;
  - r 1 mm × h 1 µm at 5 and 20 km;
  - r 1 mm × h 10 µm at 20 km;
  - r 1 cm × h 1 µm at 20 km.
- **ε 1e-12: 2 more** (r 1 mm, h 100 nm and 1 µm, at 5 km).
- In every one, the f64 sum equals the exact volume to 4 digits (−3.142e-12 against −3.142e-12).
- The interval is what fails. With `DPRINT` at r 1 mm × h 1 µm, 5 km, it reads [−1.25e-11, +6.26e-12], a width of 1.9e-11, which is 6× the volume. V/A ≈ 5e-7 m, which is 500·ε at 1e-9: far outside the band.
- These bodies are well inside the representable domain: ulp(5e3) = 9e-13 ≪ ε.

So r1/r2 MAJOR 1 is only PARTLY closed. The reviewers' probes were all planar, and those are all fixed (below). The class is not fixed, for two reasons:
- The enclosure was tightened only for all-planar walks. The `rederive` doc says so, but the PR body says "every inside-out body main refused is refused" and "the far-origin blow-up is gone". Both are false for curved walks.
- Check 7 still reads "the interval straddles" as "exempt". Check 10, after ruling 2, refuses the same reading (`ShellRoleUndecided`). A straddle that comes from the enclosure's own width is not "the sign is in band at this tolerance". Check 7 conflates the two, and that conflation is what lets the body through. sure.

No row in the PR has a curved face, so nothing pins this (MINOR 3).

## MINOR

**2. Valid far, thin curved bodies are newly refused by `classify_shells`, by `point_in_solid` and by a boolean.** This is the same root as MAJOR 1. DEMONSTRATED BY EXECUTION (`probe_d_disc3.rs`, `probe_d_void.rs`, head vs main, ε 1e-9 and 1e-12):
- `classify_shells`: the upright disc (r 1 mm; h 100 nm at 1 and 5 km, h 1 µm at 5 km) reads `Ok([Outer])` on main and `Err(Escalated)` at head. This is r1 MINOR 4, still open for curved walks.
- `point_in_solid` at a point beside the disc: main answers `Ok(Out)` correctly; head answers `Err(Escalated)`.
  - This is **new in this pass**. Ruling 3 now certifies `at_infinity_side` through the loose interval (`solid_contain.rs:5455`). By inspection, 1d4f235 read the f64 sign there.
- **A boolean on in-domain input is refused.** A 8 mm box minus a 1 mm × 1 µm disc wholly inside it, at 5 km, ε 1e-9:
  - main builds the two-shell body, and it passes tier 3;
  - head refuses: "the solids do not cross, and none of the 26 points tried on the first solid…", because its containment probe now escalates.

All of these fail loud, so this is MINOR and not MAJOR. But it is a capability regression on valid input, and the PR's "no upright body is refused or newly escalated" does not hold. sure.

**3. The rows cannot see a curved walk, and ruling 3 has one guard at one ε.** DEMONSTRATED BY EXECUTION, with mutants on the whole `topo` suite:

| mutant | ε 1e-9 | ε 1e-12 |
|---|---|---|
| `oldenc` (world-origin centre, planes through `closed_form_of`) | the three new `tier3_tests` rows red | those three, plus contact9 `a_pierce…` and `a_vertex_pair…` |
| `c10skip` (check 10 skips an undecided shell) | `check_10_reads_every_shell_s_role_or_refuses_the_solid` red | — |
| `inflane` (`at_infinity_side` certifies with no lane) | **0 red** in 2200 | **1 red**: contact9 `a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex` (the 60 m point) |

- `oldenc` changes nothing on a curved walk, which already uses the world origin, so no row anywhere covers MAJOR 1.
- Under `inflane`, r2's reuse probe at ε 1e-12, s = 1 m, goes wrong again: `sliver ∪ C` reads −3e-16 and passes tier 3; `sliver ∩ C` reads 8 m³ and passes; `big ∩ sliver` is refused. The probe is a reviewer artefact, not a row. sure.

**4. The PR body claims more than the code does.** By inspection; Q5.
- "every inside-out body main refused is refused" and "no upright body is refused or newly escalated": falsified by MAJOR 1 and MINOR 2.
- "There is no `VolumeEnclosure { lo = hi }` encoding and no `padded` knob". The backstop keeps `padded: bool` (`boolean/ops.rs:1815`, `:1891`). It hand-builds `VolumeReading::Exact` against `Bracket` (`:1837`), a second home for the choice `MassProperties::reading` makes off `volume_pad == 0.0`. sure.

**5. The `_structural` door's doc now describes behaviour that has changed** (`validate.rs:4263`, `:4271`). Q4.
- "on a body the closed form computes, the verdict is `validate_geometric`'s". The certified door now reads the interval and this door reads the sums. The PR's own 5 km slab is the counterexample (the lane-free item says so). The sentence stays, and is false.
- "Check 10 … is silent on a shell whose sign needed the quadrature". With no lane, `shell_role` now returns `Err(Props)` there, and check 10 pushes `ShellRoleUndecided` (`:4645`). That is a refusal, not silence. Confidence: by inspection, likely.

## NOTE

6. **Valid body newly refused by the always-on re-derivation, out of domain.** The upright r 1 cm × h 1 µm disc at 20 km, ε 1e-12: main passes it; head gives `VolumeUncomputable` (`props_loop_closed` escalates inside the interval closed form). EXECUTION. ulp(2e4) = 3.6e-12 > ε, so the body is below this tolerance's resolution, and I don't count it as a defect. It does show a path the PR doesn't mention: a valid body refused because its interval closed form refuses where the f64 one did not. sure.
7. **The new enclosure is sound on everything planar I could build.** EXECUTION, `probe_d_attack.rs`, ε 1e-9, 1e-12 and 1e-6:
   - bodies: slabs of 1 mm–1 cm, thickness 4·K·ε to 1 µm, rotated 0.7 rad about a skew axis (oblique planes, vertices off them by a rounding), translated 1, 5 and 30 km;
   - result: every upright one passes and every inside-out one is refused, with role and check 7 in agreement.

   On what the enclosure holds (sure, by derivation):
   - The plane arm encloses Σ((o−c)·n)(n·A⃗)/(n·n). That is not the walk's own target Σ o·A⃗. The two differ by Σ(o−c)·A⃗⊥, which is zero only when the loops lie exactly in their planes.
   - The new quantity also depends on `c`, through c·ΣA⃗⊥. The doc's "the volume of the solid those projected faces bound" (`quad_lane.rs:184`) names no well-defined solid: a shared edge projects differently onto its two planes. Both differences are of order A·δ, with δ the vertices' off-plane distance, so they are in band once δ ≤ ε.
   - So this is better conditioned than the old target, but it is a different quantity from what the sums measure. `certify_role`'s "holds the exact value of the stored geometry" should say which value.
8. **Every finding of the dual reviews on planar bodies reproduces as fixed.** EXECUTION:
   - r1 slab family, tiny cubes and check-10 graft: 0 wrong verdicts and 0 classify errors at ε 1e-9 and 1e-12. At 1e-6 the sub-ε slabs don't build.
   - r2 inverted slabs: 0 wrong at 1e-9 and 1e-12.
   - r1 booleans: 48 lines at 1e-9 and 32 at 1e-12. Every result passes, every revert is refused `NegativeVolume`, and `q` is In/Out correctly. The ×1e3 `mapped_cube` panics at 1e-12, as r1 found.
   - r2 e2e: 0 flagged at 1e-9 and 1e-6. At 1e-12 it is 0 of 156 before the same fixture panic.
   - r2 reuse: every case correct at all three ε.
9. **ε runs.**
   - The PR's new and changed rows (`tier3_tests`, `contact9_side_codes`, `review_cleave_farplane`, `dsc_checks`, `edit_refusal_recourse`, the sample roster): 97/97 at 1e-9, 1e-6 and 1e-12.
   - `topo` + `sweep` + `editor-core` at 1e-9: 6899/6899 (5691 + 1208).
   - `pncad-py` cargo rows at 1e-9: 131/131.
   - `topo` at 1e-12: 2199/2200. The one red, `rigid_map_near_eps_plane_nurbs`, is red on main too (I re-ran it there).
   - Not exercised: Python `unittest` and the binding census; `sweep` and `editor-core` at 1e-6 and 1e-12.
10. **The disclosed behaviour changes.**
    - The ~40 % on `validate_geometric` is acceptable as the price of certifying every round. The always-on interval is also what MAJOR 1's fix will lean on. Nothing guards the number, though; it is a PR-body measurement (Q6). Not re-measured.
    - Refusing the in-band cavity at tier 3 is what ruling 2 asked for, and I accept it.
      - It is asymmetric with check 7: the same in-band reading on a lone shell passes check 7 as "exempt", while filed beside another shell it refuses the whole solid.
      - The re-pinned row dropped its predicate assertion (`ind.predicate == "chk_shell_volume_sign"`). It now pins the variant, the band and the ending, but not which predicate escalated.
      - Not a defect; a decision worth stating.
    - The telemetry rename is acceptable: `positive_volume_exact` and `bool_point_in_solid_infinity_enclosure` are new names, and an exact read logs `positive_volume` once. K-lint is below.
11. **The nightly k-lint row (dev-probe).** I ran it the way the nightly does: `scripts/k_probe_sweep.sh`, then `tools/k-lint` on the three CSVs, then the driver's `--gate-rule-1-only`. EXECUTION, on the head and on main (`c7c71db43`).
    - **The sweep exits 101 on both trees**, at the ε 1e-12 demo pass. SHOW's `lily_leaf_b` panics with `QuadratureBudget`, as the PR says. So the merged `k-eps-1e-12.csv` is never written, and the nightly row is red on main before the lint runs.
    - **The large-K lint goes red on both trees, with the same flags.** I ran it on the 1e-6 and 1e-9 CSVs and on the 1e-12 corpus-only CSV.
      - Each tree gives GATE FAILED with 107 flags: 85 `chart_bound_outer_span` NaN, 12 + 8 `props_quad_*` and 3 `volume_backstop` (demo/table, |m| = 1.5e-5).
      - A `diff` of the two flag lists, ignoring line numbers, is empty.
    - The driver lint passes on the head (rule 1 = 0).
    - None of the PR's new predicate names is flagged. The head records about 875 fewer samples per ε, which is the collapsed double read (`positive_volume_enclosure`).
    - So the dev-probe row goes red, but it is red on main identically. This PR adds no flag, and it does not repair main's red either. Logs are in `probes/delta-3977/logs/klint-*`.
12. **The design question** (whether the `_structural` doors may certify through a lane): my reading, not a decision.
    - The door's contract (`validate.rs:4259`) is a statement "about the DOOR and never about the scalar — the f64 caller gets exactly what the dual caller gets".
    - A certifying `QuadLane` needs `CertifiedBounds`, which a `Dual` cannot supply.
    - So a `_structural` door that certified through a lane would certify at f64 and not at a dual. It would answer differently by scalar, which its contract forbids.
    - Two readings keep the contract:
      - (a) the door keeps deciding on the sums, with the divergence from `validate_geometric` stated at the door (today the doc says the opposite, MINOR 5);
      - (b) a lane-free certification that works at every policy scalar exists first.
    - Splitting measuring from certifying in `sign_walk` makes (a′), "certify where the scalar can", mechanically easy. It is the reading that breaks the door's contract.

## Original findings, fix pass 1

| finding | status | evidence |
|---|---|---|
| r1 MAJOR 1 / r2 MAJOR 1: inside-out bodies main refused now pass | **PARTLY** | Planar: CLOSED; the reviewers' probes find 0 wrong at 1e-9 and 1e-12, and so do my rotated, far-translated slabs. Curved walks: OPEN, 9 inside-out discs pass that main refused (MAJOR 1). |
| r1 MAJOR 2: check 10 skips a solid with a straddling shell | CLOSED | r1's graft probe gives `SolidOuterShells`/`ShellWinding` at 0, 1 and 3 km, ε 1e-9 and 1e-12; the `c10skip` mutant turns the new row red. |
| r2 MAJOR 2: tier 3 certifies the sliver; reuse builds wrong bodies | CLOSED (with MINOR 2's new refusals) | r2 reuse and e2e are all correct at all three ε (e2e at 1e-12 up to the fixture panic); `inflane` restores the wrong bodies. |
| r1 MINOR 3 / r2 m1: no row goes red when the enclosure degrades | PARTLY | `oldenc` turns 3 rows red at 1e-9 and 5 at 1e-12. Curved walks: no row (MINOR 3). The PR's argument against a ×2 width row is sound. |
| r1 MINOR 4: `classify_shells` refuses valid bodies | PARTLY | Planar far slabs read `Outer` (0 classify errors). Curved far discs: `Err(Escalated)`, where main reads `Outer` (MINOR 2). |
| r1 MINOR 5 / r2 m2: stale `work/geom/` citation | CLOSED | `contact9_side_codes.rs:132` cites `work/flux/…`, and the file exists. |
| r2 m3: contact9 membership samples only inside | CLOSED | Four points per operand plus a 60 m point; `inflane` turns `a_vertex_pair…` red at 1e-12. |
| r1 NOTE 7: the reverted 1 m sliver passes on both trees | CLOSED | r1 booleans at 1e-12, s = 1: inverted `NegativeVolume`. |
| r1 NOTE 8: `(A∩B) ∪ B` gives no body | as PR says | Still "no body" in the probe; the PR reads it as an undeclared-coincidence refusal, which I did not re-derive. |
| r1/r2 notes 6, 9, n1, n2 | unchanged | The fixture limits reproduce. |
| Style: two role readers (r1 Q1, r2 Q1) | CLOSED | `read_role` is the single reader, used by check 7, check 10, classification and the side at infinity. |
| Style: `lo = hi` encoding and `padded` knob (r1 Q7, r2 Q1) | PARTLY | `VolumeReading::Exact` replaces the encoding; the backstop keeps `padded` (MINOR 4). |
| Style: the "45 km" pass-side paragraph (r1 Q2/Q6, r2) | CLOSED | Deleted; the pass side is certified. |
| Style: `Err(_) => Some(None)` fold (r1 Q4, r2 Q7) | CLOSED | `shell_role` returns `Result`. |
| Style: `Round` doc overclaims (r2 Q5) | PARTLY | Rewritten; `certify_role` still says "exact value of the stored geometry" (NOTE 7). |
| Style: check-7/backstop duplicate (r1, r2 Q1) | OPEN, disclosed | Left for the door unit. |
| Style: `tier3_tests` row stands down at 1e-6 (r1 Q3) | PARTLY | The new `an_inside_out_slab…` runs at every ε; the sign row still stands down above 1e-9. |
| Style: `at_infinity_side` P3 (r2 Q4) | CLOSED | Taken into this unit. |

## Style (exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; not Q8 — `validate.rs` is ~11k lines and I read only the touched regions)

- **Q1** `boolean/ops.rs:1815`–`:1845`, `props.rs` `MassProperties::reading`. The exact-or-bracket choice is made in two places, off two spellings of "padded": a `bool` threaded through `Posture::read`, and `volume_pad == 0.0`. This is the duplication fix pass 1 was asked to remove, minted again in the backstop. likely.
- **Q1/Q7** `RoleNames { high, low }`: five of its six instances set `high == low` (`SHELL_ROLE_NAMES`, `SHELL_ROLE_ENCLOSURE_NAMES`, `PLUS_V_EXACT`, `AT_INFINITY`, `AT_INFINITY_ENCLOSURE`; only `PLUS_V` differs), and `certify_role` takes two of them. The two-field type exists for check 7's historical pair of names. unsure.
- **Q3** `tier3_tests::an_inside_out_slab_just_past_the_band_is_refused` and its siblings build only boxes. The guard ruling 4 asked for covers the arm that was fixed, not the arm that was left. Under `oldenc`, a row with one curved face would be green both before and after the fix. sure.
- **Q2/Q5** `quad_lane.rs:184` (`planar_face_about`'s doc): "a `centre` on the body puts each plane at the body's own distance". That holds only when `rederive` picked the centroid. For a mixed walk the same function runs with `centre` at the world origin, and the doc's conditioning argument does not apply. likely.
- **Q4** `validate.rs:4263` and `:4271`: MINOR 5. The sentences rotted while the code changed. The first is the "code drifted from something meant to hold" kind: the door promised `validate_geometric`'s verdict. sure.
- **Q6** The ~40 % cost and the PR's widths table are PR-body measurements, with no register or guard. unsure.
- **Q7** Check 7 and check 10 now give opposite verdicts on one `Certified::Open` (pass against refuse). One rule decides what an undecided sign means, and it lives in two `last_word` closures with opposite answers. likely.
- The fix-mints-the-defect check (§1 of the style lane) found two instances:
  - the backstop's `padded` (Q1 above);
  - the interval certification, added to stop rounding misreads, now misreads on curved walks in the other direction: a sound but loose straddle passes check 7 and refuses `point_in_solid`.

## Probes (`probes/delta-3977/`, mounted temporarily, not shipped as code)

- `probe_d_attack.rs`: rotated, far-translated planar slabs (NOTE 7).
- `probe_d_disc.rs`: far thin discs through tier 3 (MAJOR 1).
- `probe_d_disc2.rs`: the interval print and the 20 km refusal detail.
- `probe_d_disc3.rs`: `classify_shells` and `point_in_solid` (MINOR 2).
- `probe_d_void.rs`: box − disc (MINOR 2).
- `probe_r1_slab_ported.rs`: r1's probe on the new signature.
- `mutants_and_mounts.diff`: the `DMUT` mutants (`oldenc`, `c10skip`, `inflane`), the `DPRINT` hook and the mounts.
- `logs/`: head and main outputs.
