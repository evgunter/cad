# Review of PR #3980, frozen head 5a5c62365b

Lane `reach-dual3980-r1`. Wall clock 2026-10-03 17:09 – 19:03 UTC. **Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 5.
No glimpse: I read the PR body with `pull_request_read get` only, and no other `analysis/reach-dual/*` branch, PR comment or review.
I did not read the PR's check runs either, since the brief allows `get` only. So the verdict rests on my local runs, not on CI:
- every topo + sweep + editor-core test (the `ci` fast set and the 105 slow-set rows) passes at ε 1e-9;
- topo (2164 tests) and the touched sweep suites (72 rows) pass at ε 1e-6 and 1e-12.

## Claims, by execution (probes in `probes/reach_dual3980_r1/`)
1. **Holds.** The oracle is closed form: πr²h, π(R²−r²)L, 4πr³/3.
   - The cells: both seam layouts; radii 0.3 and 2.0 with lengths 1.7 and 0.4; spans through, half-in, blind-top and fully inside; azimuth 37°; scales ×1e-3, ×1 and ×1e3; identity and a rotated-and-translated pose.
   - Each cell runs every op in both orders. Results are reused as operands: (c∖s)∩s, (c∖s)∪s, (s∖c)∖c.
   - `point_in_solid` checks wall, shaft, contact-rim and far points. Ball-in-cavity runs at two radius pairs × three scales × two poses, with points on the contact sphere (`In` under ∪).
   - Result: 852 answers at ε 1e-9 and 1e-6 and 642 at 1e-12, where the ×1e3 fixtures whose own build or pose refuses are skipped. **0 wrong at every ε**; maximum relative error 1.7e-14.
   - The only refusals are full-turn half-in and blind *unions* (`RestZipUnsupported`, already filed in `work/zip/…`), plus `point_in_solid` `Escalated` at ×1e-3 when ε is 1e-6. Neither is this PR's.
2. **Holds.** I declared Rest on pairs where it is false: radius ±1e-3, ±1e-6, ±1e-7, ±2e-8; off-axis 1e-3 to 2e-8; tilt 1e-3, 1e-6, 1e-8; ball ±1e-3 and ±1e-7; ball off-centre 1e-3 and 1e-7. Both layouts, every op.
   - At ε 1e-6, every lie of 1e-6 or more still refuses (for the lies within ε, see NOTE-3).
   - At ε 1e-9 and 1e-12, all 180 answers refuse typed (`ContactContradicted`, or `Escalated` in-band) at the declaration door (`mod.rs:3909`), before the new exemption can read them. No closed-form body was built from a lie.
   - The exemption reads only `verified.one_carrier` with the sense checked (`mod.rs:1135`, `:3928`).
3. **Holds.** I instrumented `touches_only` and the sphere skip and ran every topo + sweep + editor-core test (fast and slow).
   - The exemption fires in exactly the PR's four rows and my probes. No other row reaches it.
   - On my corpus, comparing main's `ops.rs` with head's: verdicts move only on declared-Rest inputs, plus the case in NOTE-2.
4. **Partly.** Mutants against head's `ops.rs` (patches in `probes/…/mutants/`):

| mutant | result |
|---|---|
| M1 `touches_only` → false | red: all 4 PR rows |
| M2 drops the `SameOpposite` test | red: 6 continuation rows |
| M5 swaps the sphere arm's operand order | red: the ball row |
| M3 drops the `peek().is_some()` guard | survives |
| M4 sphere skip `all` → `any` | survives |
| M6 drops the box filter | survives |
| M7 keys on the A face alone, ignoring `fb` | survives |

## Findings
- **MINOR-1: the sphere skip's "every reaching face" qualifier is unpinned.** `crates/topo/src/boolean/ops.rs:3018-3029`. DEMONSTRATED BY EXECUTION.
  - M4 survives the whole suite. My probe `claim4_ball_in_cavity_partly_declared` kills it: with one of the four sphere pairs left undeclared, head refuses `SpheresMeet`, and M4 builds `Empty` / a 2-shell body / a 1-shell body on the under-declared input.
  - M3 and M6 also survive. I found no fixture that tells them apart from head (the half-declared probe refuses identically under M6).
  - The qualifier is the design the PR body describes, and no row holds it.
- **MINOR-2: `touches_only` exempts on the A face alone and nothing notices.** `ops.rs:1280`. DEMONSTRATED BY EXECUTION.
  - M7 drops the partner face from the key and survives everything.
  - My probe of an undeclared rod internally tangent to the same bore (gaps 0 to 1e-6) refuses earlier, at `CurvedPierceUnsupported` / `Escalated`, so M7 is unobservable there too.
  - The partner key carries the soundness argument, and no row exercises it.
- **NOTE-1: the rung-1 arm of `touches_only` is untested.** A "shared recipe source" pair (`ops.rs:1270`) is claimed in the doc, and no row reaches the exemption by that rung.
  - My probe: `brick∖cyl` against `brick∩cyl` refuses `UndeclaredCoincidence` first. Execution plus inspection.
- **NOTE-2: one verdict moved outside the PR's tables, correctly.** A collar against a shaft plus a ball buried in the collar wall (a second B shell):
  - main refuses `FallbackExtentUnsupported`;
  - head answers ∩ = the ball, 2 shells for c∖B, and B∖c = the shaft, all at the oracle.
  - This is right, and it is worth a row. Execution.
- **NOTE-3: at ε 1e-6, lies within ε now build.** Radius or offset 1e-7 and 2e-8, tilt 1e-8 and ball ±1e-7 build the declared closed form, where main refused.
  - The bodies differ from the true geometry by about 3e-7 relative, which is inside the band (D4) and consistent. Execution.
- **NOTE-4: the slow-set count in `.config/nextest.toml:11` is off by one.** It says 190; the filter has 191 `test(=…)` entries. Main has 187, not the 186 the PR body says. Counted with tomllib.
- **NOTE-5: the PR's matrix rows are in the slow set.** After merge they run only on a sweep-touching diff or nightly. Inspection.

## Style (exercised: Q1, Q2, Q3, Q4, Q5, Q6. Q8 partial: `ops.rs` read 700–780, 1190–1380 and 2780–3040, not end to end)
- `ops.rs:741` and `:754`. The no-crossings comment still says "**no escape**: the boundaries are certified disjoint" and lists "sphere faces meeting" as uncertifiable. A skipped settled pair is neither: its boundaries coincide.
  - This is the doc-rotted case of Q4. — likely
- `ops.rs:1371`. `no_crossings_certificates` (the test-support twin of the path) passes `&[]`, so it no longer runs the certificates "as the path runs them" (`ops.rs:1354`) for a declared pair. That makes two spellings of one sequence. — likely
- `rest_mate_every_op.rs:145`. `decls(a, b, r)` is `mate2_common::wall_decls` (`mod.rs:92`) with the radius made a parameter. That module's header exists to say "One copy, one order".
  - The fix minted a fresh copy (Q1). `sphere_decls` (`:363`) is a third, filtered by kind.
  - `ball`/`hollow` (`:344`, `:354`) re-spell `curved_mergedoor.rs:611`, and the full-turn collar (`:86`) re-spells `full_turn_bore_mate`'s. — sure
- `nextest.toml:11`. A hand-written census, corrected by this PR and wrong again at its own head (NOTE-4). That is the Q6 trap. — sure
- `ops.rs:1270-1279`. The doc proves that "touches only" hides no overlap, but its key covers only the pair it names. Whether a sibling unsettled pair of the same A face can still hide an overlap is argued nowhere and pinned nowhere (MINOR-2). — unsure
- `ops.rs:3015`. The comment "nothing of the pair is the scan's" asserts why skipping all sphere questions for a carrier is sound in one clause. The guard has three parts (≥1 reaching face, every one of them, box-filtered) and only one is pinned. — likely
