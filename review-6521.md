# Review of PR 6521 (`spiricarm/spindle-partway`): a football revolved part way is its lune

**Frozen head reviewed: `c974bfa84d556d9d49c144cba17c0021569605f9`.** Single FULL reviewer (claims + style lane).
Probes: branch `spiricarm-rev6521/probes` @ `e90b9086`.

## Verdict: APPROVE-WITH-FIXES

No MAJOR. The soundness core holds: the apex reading is this sheet's distance, the certified fallback
cannot certify an off-sheet carrier, and the cone, the torus and every other kind behave as before.
The fixes concern the PR's own evidence. One "equivalent" mutant is not equivalent. One witness row is
blind to the check its doc names. The negative-angle row is blind to the sign of θ. And there is one
user-drawable residue nobody has filed.

## Findings

**MINOR-1 — `pcurve_cache.rs:spans_of`: M7 (`lower > 0` → `>= 0`) is not equivalent.** The PR says a
zero lever "divides to +∞, which reads nothing". That is how M7 fails. A cone ruling from the apex has a
lever enclosure of exactly `[0, 0]`. Under `>= 0`, `over` succeeds with +∞ and the `√|f|` apex fallback
is never taken. Evidence: under M7, `closed_form_limb2::every_closed_form_kind_owes_limb_2_through_the_lane`
goes red: "cone ruling from the apex: … KernelLimit { operand: Cone, bound: 2.36e-8 }". It is green twice
unmutated. The PR's mutant set left that row out. The code (`> 0`) is right; the PR body's claim and its
mutant table are wrong. Confidence: **sure**.

**MINOR-2 — the near-sphere football refuses part way, and nothing records it.** This is the probe
`review_6521_probe_near_sphere`. At `R = ±0.01`, `r = 1`, θ = 2, the build refuses
`AnalyticRung3(KernelLimit { operand: Spindle, bound 5.8e-9 / 1.1e-8 })`, and the full revolve of the
same profile builds. At `CAD_TOLERANCE_EPS=1e-12` the refusing range widens to `|R| ≤ 0.02`, where
`|R| = 0.03` and `0.05` build. A nearly semicircular arc is easy for a user to draw. The PR's headline,
"the football revolved `Partial(θ)` now builds", overstates. Under the parent row's Done-when this
counts as neither "builds" nor "a typed lane-gap reason that names the apex", and no issue file
holds it (§6). `work/lathe/a-sphere-class-arc-centre-…` covers only `R ≈ 1e-8`. I did not trace which
part fails. My guess is that the sheet lever `4|R|ρ` vanishes as R → 0. Confidence: **sure** (it
refuses); **unsure** (why).

**MINOR-3 — `the_football_revolves.rs:a_football_revolved_part_way_is_its_lune` cannot see the sign of θ.**
The oracle uses `|θ|/2π`, and volume and area are the same for ±θ. Mutant S17 makes `Partial(t)` revolve
`|t|` in `revolve/mod.rs`, and all 23 rows of my set stay green. The PR's set plus the football and limb-2
files includes "a small apple at −2.5". I did not run all of `sweep::all` under S17, so some other row
may catch it. This row does not. Confidence: **sure**.

**NOTE-1 — `closed_form_limb2.rs:limb_2_against_a_sheet_reads_that_sheet_up_to_its_apexes` does not
exercise the side check.** Mutant S1 deletes `positive(minor * rho - d * major.abs())?` from
`Meters::over`. The row stays green. Only the fuzz rows go red: `a_spindle_bound_is_this_sheets_distance`
and my probe. The reason is that on the other sheet `M = r`, so the lever `σ(M² − r²)` is zero, and the
lever refuses before the side check is reached. The row does reach limb 2: M1, M4, M5 and M6 each turn
it red, and it pins `KernelLimit` against `Spindle`. The side check's only guard is the fuzz. Confidence: **sure**.

**NOTE-2 — `axis.rs:classify_segment`: the claimed precedence over `ApexEndOffSheet` has no row.**
Mutant S14 moves the pinch guard before `sheet_major`, and every row stays green. It is probably
unreachable, since an interior `Zero` puts the end at the cusp, where the horn reading holds. Then the
PR body's sentence describes nothing. Confidence: **likely**.

**NOTE-3 — the one-segment horn pinch and its four-arc respelling.** These refuse in different classes.
The solid of a horn disc revolved part way is a manifold: the link at the cusp is a lune with its
corners trimmed. Only the single sheet face is pinched. So `NonManifoldAxisContact` for the one-segment
spelling names the face model, while the same solid spelled as four quarter arcs refuses `KernelLimit`
(the filed P2). This follows the ratified stage text in `the-spindle-sheets-apex-is-a-pole.md` ("a face
that reaches it from both sides pinches, and the existing `NonManifoldAxisContact` refuses it"), and the
full revolve refuses the same way. Once the P2 lands, the two spellings will diverge further. Confidence: **likely**.

**NOTE-4 — `limb-2s-floor-on-a-sheet-reads-the-whole-torus.md`: "off the sheet by `√3` at `R = −½, r = 1`"
is the bound, not the distance.** The other sheet's meridian is farthest from the lemon at `(1.5, 0)`,
which is 1 from the lemon's `(0.5, 0)`. `√3` is that point's distance from the apex `(0, √3/2)`, which is
the bound limb 2 reads. Confidence: **sure**.

**NOTE-5 — `a-horns-cusp-…` is banded P2.** `work/README.md` puts "verb breadth beyond the everyday
shapes" at P1, and its parent row, of the same class, is P1. The needle tip revolved part way is
user-drawable. The row says itself that its `KernelLimit` message "reads as the kernel's limit rather
than a lane not written". Otherwise the row is accurate, and it says its mechanism is "not a traced run".
Confidence: **unsure** (P1 vs P2).

**NOTE-6 — `pcurve_cache.rs:limb2_quick` (the first reading) changed, and the PR does not say so.**
`limb2_quick` reads `SpanHull::meters` through `clears(p.hull.meters, band)` and halves on it. A sheet
span that touches an apex used to read +∞ and end the quick reading. It now reads the apex reach, which
halves linearly up to `WORK = 256` spans. This is sound, since it is still a certified upper bound, and
it is part of the cost. The PR body's list of readers omits it. Confidence: **sure** (the change);
soundness **likely**.

**NOTE-7 — the `Indeterminate` on the lever.** Under `apex` a sheet part's band escalation
(`Escalated { check: Sector }`) is now swallowed into the hull's reading. That is the intent, but an
in-band lever on a sheet is no longer visible in limb 2 at all. Confidence: **sure** (description only).

## Claims

1. **Apex reading: upheld.** My probe `review_6521_probe_apex_fuzz_interval` takes the hull's
   `Interval` path (`lower > 0`). It seeds near either apex at scales 1e-12..1e-3, on this sheet and on
   the other, for lemons and apples out to `|R|/r = 1 − 1e-9`. Over 1M draws (EFFORT 1 and 4), about
   99% of them read by the apex reading, no bound under-read. One early red was my own oracle's
   `√(r² − R²)` cancellation. A decimal check showed the exact distance equals the bound, and the
   existing fuzz's `sheet_distance` has the same cancellation; it is harmless only because `|u| ≤ 0.98`.
   M1 (`√|f|`) and M2 go red.
2. **Certified fallback: upheld, with no real tension with [verdicts].** The `lower > 0` reading
   decides whether a division is valid, in certification arithmetic. Its two branches are certified
   upper bounds, and the band decision on the bound (`clears` / the KernelLimit-vs-Bound reading) is
   unchanged. Pre-PR, `limb2_quick` already certified off the same hull reading. It cannot certify an
   off-sheet edge: when the band decides the lever or side negative, the interval cannot be positive,
   and the fallback is then the apex reach, which is ≥ the true distance. Cone (`√|f|` in both paths),
   torus (None → +∞) and plane, cylinder and sphere (never refuse) are unchanged by inspection. One
   remark for Ev: the answer turns on an in-band quantity, which affects how often it refuses but never
   correctness.
3. **Pinch guard: upheld.** S3 (`||`) breaks the needle and two more rows. S4 (no interior test) breaks
   8 football rows. It is consistent with the full revolve: `axis_runs` gives the same variant and
   vertex. The remaining questions are NOTE-2 and NOTE-3.
4. **Lune oracle: partly falsified.** It is independent (Pappus and the segment closed form; the caps
   are exact: `r²acos(d/r) − d√(r²−d²)` and the disc minus that). The sheet count and sign are checked,
   so a wrong sheet goes red. A wrong θ sign does not (MINOR-3).
5. **Witness row: upheld that it reaches limb 2; falsified that the side check is held there** (NOTE-1).
   The build-door claim, that limb 1 refuses first, I did not probe independently.
6. **Mutants.** M1, M2, M4, M5 and M6 are red as stated. M7 is **not equivalent** (MINOR-1). The ninth
   candidates that survive:
   - S13: the side check reads `ρ_hi`. No row catches it, and 400k wide-box Python draws found no
     unsoundness either, so it is probably benign. If so, the check is stricter than it needs to be,
     or uncovered in a way I could not find. **unsure**
   - S14 (NOTE-2).
   - S16: `apex_part` keeps a non-finite reading. This one is equivalent: `certified` always returns
     `Some` on a sheet.
   - S8: positivity read off `upper` instead of `lower`. It differs only at the rounding level on point
     inputs.
   - S9 (apex reading only) is red.
7. **Cost: bounded.** Probe `review_6521_probe_lune_cost`, debug, default ε: 56 lunes. Lemon and apple
   from 0.01 to 1 − 1e-6 times r, r from 1e-3 to 1e3, θ ∈ {1e-3, 2, 6.28, −6.28}. Worst build 1.47 s
   (apple, r = 1e3, θ = 1e-3), typical 0.15–0.85 s. `limb2_quick`'s `WORK` and the rounds' budget bound
   it. Nothing ran long.
8. **Residue:** both rows are accurate apart from NOTE-4 and NOTE-5. The near-sphere refusal is
   unfiled (MINOR-2).

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 only partly: I read `axis.rs:classify_segment` and
`projected.rs`'s Meters/`net_incidence` region (about lines 1540–2110) end to end, not the whole of
`pcurve_cache.rs` (15.5k lines).

- **Q3 — the second assertion in `a_spindle_bound_is_this_sheets_distance` is vacuous for sheets.**
  ("limb 2's apex reading", `envelope_lemma_fuzz/projected.rs`.) The `SpanHull` is built with
  `meters: read.unwrap_or(∞)`, and `apex_part` on a sheet returns exactly that, so the loop checks the
  same number twice. Confidence: **sure**.
- **Q2/Q7 — `apex_part`'s doc is long.** It argues at length for a two-line arm (about the band at an
  apex). That argument belongs in the PR body (implementer-discipline §4). Confidence: **likely**.
- **Q1 — `certified` composes `over` and `apex_root`, and limb 2 re-reads the same composition through
  `SpanHull::meters`.** Limb 2's band path then calls `over` again at T. So there are two parallel
  readings of one bound: band-decided, and enclosure-decided. Which wins is decided by whether the band
  refused. Consider one reading. Confidence: **unsure**.
- **Q5 — `apex_part`'s "None … where the hull's reading is not certified" is unreachable on a sheet.**
  `net_incidence` has already refused a non-finite `f_sup`, `rho` or `z`, and `certified` never returns
  None on a sheet. The same holds for `spans_of`'s `map_or(f64::INFINITY, …)` on sheets. Confidence: **likely**.
- **Q4 — `SectorChannel::Lever`'s doc and `describe()` name only the cone's and torus's levers**
  (`projected.rs`, about line 965). A sheet part refused by the *side check* reports "its metre lever is
  not positive … a spindle torus's inner sheet", which is not what failed. This predates the PR, but the
  PR re-documents `net_incidence`'s errors and leaves this one. Confidence: **sure**.
- **Q4 — the parent row still cites a test that no longer exists.**
  `a-football-revolved-part-way-…-certificate.md` cites the deleted `a_football_revolved_part_way_refuses_typed`
  and stays `open`. The PR body says the status is left to the orchestrator; the citation is stale on
  merge either way. Confidence: **sure**.
- **Q6 — the cost measurement.** The ~0.85 s debug figure is unguarded at the claim site. The rows are
  in the slow set, which re-takes their time nightly, so this is acceptable. Confidence: **likely**.
- **Q1 — the probe oracle's `√(r² − R²)` cancellation recurs.** The fuzz's `sheet_distance` spells it the
  cancelling way, while `geom::sheet_apex` spells it `(r − R)(r + R)`. Two spellings of one quantity.
  Confidence: **sure**.

## Disclosures

- I posted nothing to GitHub. I ran everything locally in debug, mostly at the default ε, plus
  `CAD_TOLERANCE_EPS=1e-12` for the near-sphere probe. I did not check the full revolve of the
  near-sphere footballs at 1e-12.
- Mutants ran on 23–25 rows: the PR's rows, all of `the_football_revolves::` and `closed_form_limb2::`,
  and my probes. They did not run on all of `sweep::all`. A first S17 run over all of `sweep::all`
  exceeded 10 minutes. I stopped it, restored the file and confirmed no processes were left, then re-ran
  S17 on the narrow set.
- I did not trace the near-sphere refusal or the horn row's `√(2r·d)` mechanism, and I did not probe
  the build-door claim in claim 5.
- I did not re-verify the PR's sweep hit list (§5) entry by entry.
