# Review of PR #4272 (roof-cross valley), frozen head 99b2f983

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 5.

The outer-lane fix is sound on everything I could run. I found no wrong body, and nothing outside the claimed lines moved. The fixes asked for are small: a guard for the tier, a row-status and row-wording fix, and a decision about the two forced-order lanes.

## Method (all release, own target dir)

I built four binaries of `sweep --test all`:
- **head**: the PR as frozen;
- **old**: head with `join.rs` set to main f7e17b0f's, so it runs the old rule. It is main plus the ported probes;
- **untier**: `worst = Capture::Clean` in place of `RingHeld`;
- **flip**: when both arcs are RingHeld, take `ra → ea`.

My own probe family is in `crates/sweep/tests/review_r2_vv_probes.rs` on this branch:
- `R2_SWEEP=rv` adds five placements, `rv-e1` `rv-e3` `rv-in` `rv-out` `rv-mix`, that slide valley4's corner along the cube's edge or push it generically off it. That is 540 poses and 3 240 runs, every op in both orders;
- `R2_MESH=1` also tessellates each body and runs `mesh::validate::check_mesh`.

## Claims

**1. Ring re-homing premise: holds (likely).** I executed probes and read the code.
- **Why it holds.** The chord is a section segment that ends on the split loop. It meets no ring, so every ring lies wholly on one side. A loose half on arc A (other than at the chord's own end vertices) bounds only region A, so its segment enters A. The segment cannot cross the chord, so the partner's ring lies in A. `rehome_rings` tests that ring against the run (`chord_join.rs:3410`).
- **Ring whose other end is on the split loop.** The `held == split` check excludes it (`join.rs:2681`), so it still reads Separates.
- **Rings on opposite arcs.** This is the witness itself. Each ring re-homes to its own side.
- **A ring holding partners from both arcs.** Geometrically it would have to straddle the chord, which is impossible unless it touches the chord or the run. That is a coincidence, D10 ground.
- **A pending pierce ring.** It stays in the old face, but `place_pending` (`chord_join.rs:701`, `join.rs:818`) moves it to its partner's face before that pair joins. A pending ring holding partners on both arcs would refuse typed at the second join, not ship wrong.
- **What I could not construct.** No case put a ring on the wrong side. The rv family reaches the arm: 78 runs that refuse on old with "every chord arc separates" become SOUND on head, and all 3 240 head runs are SOUND.
- **Coverage gap.** Curved faces, which use `chart_ring_side` and `sphere_ring_side`, are not exercised by any battery that reaches the arm (unsure).

**2. Tie-break principled: holds at verdict level (sure); face identity unseen (unsure).** The flip mutant is byte-identical to head on valley4 `grid` (5 040), `psi` (2 880), `tilt` (2 592) and `rv` (3 240). Both arcs are RingHeld at the witness, according to the PR's trace, so the flip does fire there. Caveat: `outcome` lines carry the verdict and volume, not face keys. So "face identity is order-independent" (D9) is not observed. What is observed is that neither choice changes the verdict.

**3. No wrong body: holds (sure, sampled).** I ran `outcome` plus `check_mesh` on the row's three named witnesses (grid, psi and tilt; 18 runs). All are SOUND, `MESH ok`, and volume equals the oracle.
- Every valley4 grid and psi run on head is SOUND or `EMPTY ok`.
- Tilt, old against head, moves exactly 48 lines, all `JoinDesync` → SOUND.
- Head's 33 tilt BAD lines are byte-identical on old, so they pre-exist (NOTE-1).
- I did not re-run mnotch.

**4. Nothing else moved: holds (sure).** Old against head is byte-identical on:
- `corner_pairs_battery` (16 380 lines);
- `pinch_runs_battery` (3 024);
- `rc_wide` shards 1/12 and 7/12 (two shards, distinct from the PR's 0, 3, 6 and 9).

**5. The pin can go red: half holds.**
- With the old rule, the pin fails on `sc U: ERR JoinDesync "every chord arc separates…"`.
- With **untier, the pin passes.** Only the ignored `corner_pairs_battery` catches it. It reproduces the PR's 21 lines exactly (13 SOUND→BAD, 8 BAD→SOUND, all `t3p` flips at ψ ∈ {0, π/2, π}).
- So nothing CI runs goes red on the untiered mutant (MINOR-2).

**6. The sweep: holds for "off the loop = separated" (likely).** `clean_dir` is the only reader of the `loose` map (`join.rs:647-654`, `2340`, `2411`, `2498`). `splitting/join.rs` decides by leave-side and has no arc rule. But the PR's own change has a sibling shape in the two forced-order lanes (MINOR-1).

## Findings

**MINOR-1. The forced-order lanes inherit the best-of-two tier, which their premise does not call for (inspection, sure on the logic; reachability unknown).**
- **The lanes.** `ring_order` (`join.rs:2498`) and the across-edge lane (`join.rs:2411`) do not choose an order. Winding or the plan fixes it, and they only ask whether `clean_dir` returned that order.
- **The tier.** `clean_dir` now returns the *better* arc (`join.rs:2689-2693`).
- **The effect.** A forced run that is RingHeld, whose other arc happens to be Clean, still refuses "derived ring role order separates…". By the PR's own premise that run is safe, so the rule is stricter than its rationale there.
- **What moves.** The relaxation can also move these lanes from refusal to build with no witness. No old or head output in my runs or the PR's carries either lane's message, so the "serves all three callers" claim is measured for the outer lane only.

**MINOR-2. The tier's reason is a measurement with no guard (executed).** The untiered mutant passes `a_roof_cross_valley_on_a_cube_edge_builds` and every non-ignored test. The 21-line justification lives in the PR body and in an `#[ignore]`d differential battery. Nothing goes red if the tier is dropped, and there is no reason at the claim site (`join.rs:2643-2645`) saying why it cannot be guarded (Q3/Q6).

**MINOR-3. The row misdescribes and mis-states its class (executed).**
- `work/join/a-roof-cross-valley-on-a-cube-edge-refuses-every-chord-arc.md` says "All of them are edge placements". But valley4 with its corner generically *off* the cube's edge (`rv-in` 30, `rv-mix` 30, `rv-out` 18) refuses the same desync on old, and head fixes those runs too. The class is "a pinched face with pierce rings either side of the chord", not edge placement. `## Measured` undercounts what moves.
- The row's `status: open` stays while `## Built` and a PR exist. `work/README.md:108` gives `review` or `closed`.

**NOTE-1 (executed).** valley4 `tilt` has 33 BAD runs, identical on old and head:
- the `eps=0` ones are StaleContactDeclaration or UndeclaredContact (D10);
- the `eps=1e-6` ones are CensusEscalated (filed under `near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`).

They are not this PR's, but the row's `## Measured` does not mention them.

**NOTE-2 (executed).** The untier result shows that the RingHeld arc choice *can* flip a tier-3 verdict at exact ties where a Clean arc existed. Tiering keeps those poses off the arm. It does not show the arm is identity-neutral at coincidences.

**NOTE-3 (inspection).** `clean_dir`'s doc (`join.rs:2606-2608`) still says it returns the arc that "avoids every `unused` half". It now returns a RingHeld arc that captures halves whose partners are elsewhere, and `unused` is not a parameter name.

**NOTE-4 (inspection).** The comment says "A direction is BAD iff" (`join.rs:2634`). That reuses the battery's outcome word for a three-way rank whose middle tier is not bad.

**NOTE-5.** I did not re-run mnotch (6 lines). Budget.

## Style
- **Q1, parallel roles** (likely). The ported probes carry declared copies, `/// As join_pierce_runs_sweep::frame` and `::convex_volume` (`review_r2_vv_probes.rs:58,99`), now in-tree beside their originals. That is two more homes for the oracle half-space volume.
- **Q2, comment doing the code's work** (likely). The safety of RingHeld rests on "section segments in one face do not cross" (`join.rs:2642`). Nothing local enforces it, and it fails exactly at coincidences (NOTE-2). The comment is longer than the check it defends.
- **Q3, can it fail** (sure). The pin cannot see the tier (MINOR-2).
- **Q4, invalidated premise** (likely). `choose_roles`'s doc (`join.rs:2294-2297`) and `loose_partners`'s doc ("splitting a pair walls one side off") still describe the binary rule. They are true for the outer partner, stale for ring partners. The doc rotted; the code is right.
- **Q5, Q6** (likely). The tier is a disclosed deviation justified by measurement, with neither a guard nor an unguardable note at the site (MINOR-2).
- **Q7, how I would do it** (unsure). I would split `clean_dir` into `capture_rank(arc)`, used directly by the two forced lanes, and `best_arc`, used only by the outer lane. Returning an order the caller has to re-check is awkward (MINOR-1).
- **Q8, whole file** (unsure). I did not read `join.rs` end to end (≈4k lines), for budget. Not exercised.

REVIEW COMPLETE
