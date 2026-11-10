# Review of PR 6806 (`spiricarm/horn-cusp`): reviewer R1

**Frozen head reviewed: `0c3420d4cb983431637bb3dfbe16586c2a30ee40`** (checked out; the PR head matched).
Probes: branch `spiricarm-rev6806-r1/probes` (`projected/r1_probe.rs`, `probes-r1/r1_oracle.py`,
two `#[ignore]` probes in `closed_form_limb2.rs`, one in `a_conical_ended_football_revolves.rs`).

## Verdict: APPROVE-WITH-FIXES

The new reading (`Meters::at_height`) is sound, and so is the least-of fold. Every guard
reaches its gate, and the lunes' oracles are independent. One MINOR is falsified by
execution: the sheet's `v` window fix is half done. The pcurve placement still names a
wide apple's `v` in the latitude window. The PR's sweep says that reader does not exist.

## Findings

**MINOR 1: `geom-brep` `pcurve_cache.rs` `chart_image` (torus/sheet arm, `RevolvedTube::Sheet => Along::Latitude`) and `Along::window`. A sheet's `v` now has two readers that disagree.**
`chart_point` reads a sheet's `v` about zero. Closed-form pcurve placement (parallels:
`v0 = place.phase(tube_angle, …)`; meridians: `delta = place.phase(tube_angle, …)`) still
places it in `Window::Latitude`, `(1 − π, 1 + π]`. Executed (`r1_probe_wide_apple_pcurve_v`,
`R = 0.8, r = 1`, band 1e-9):
- The parallel at `v = −2.4`: `chart_pcurve` gives `v = 3.883`, `chart_point` gives `−2.4`. The same holds at −2.45 and −2.2.
- The meridian arc `[−2.4, −2.2]` at its midpoint: `3.983` against `−2.3`.
- Every `v > 1 − π` agrees.

This falsifies the PR's sweep line "`ChartAngle::SheetTube` is the one name for it, and
`chart_point` was its one reader … `pcurve_cache.rs` … read a sphere's latitude only". Its
grep keyed on `Window::Latitude` and so missed `Along::Latitude`. It also leaves these texts
false: `Window::Latitude`'s new doc ("A sphere's latitude."), the `RevolvedTube` doc ("the
sheet's tube angle in the latitude window"), the arm comment ("its tube angle read in the
latitude window") and the `Levers` doc. The row's done-when ("A sheet's points read their
`v` in `[−v*, v*]`") is met by the letter of `chart_point` only. No user-visible failure
found: revolving bands wholly below `1 − π` (`[−2.45, −2.2]`, full and θ = 2) meshes
watertight (`r1_probe_a_band_below_the_latitude_cut`). It is latent wherever a face's
placed pcurves meet chart reads: `topo` `chart_window`, `chart_region`, the pcurve mint.
Confidence: sure (the mismatch); likely (that it bites a later sheet op).

**NOTE 1: `closed_form_limb2.rs` `limb_2_against_a_horn_reads_the_sheet_up_to_its_cusp`. The off-circle half never reaches the height reading.**
It refuses `Limb` with margin ≈ the offset. That is the distance floor, an earlier gate, at
every offset tried (±1e-6 … ±1e-8, five spans, both bands). Under the grossly unsound
mutants MJ (`level(far) − ρ_lo` only), MK and MUfar the row stays green. Only the fuzzes
catch them. The row's `own` half does reach its gate: it is red under MC (no height reading),
MWz (+ the z width) and MU0, and green under ×10 and `max(ρ_hi, ρ_s(far))`, which are sound
and still under the band. `KernelLimit | Limb` is accepted, but only `Limb` occurs.
Confidence: sure.

**NOTE 2: `projected.rs` `net_incidence` vs `SpanHull::meters`. "The least that reads" holds in one of two sheet readers.**
Limb 2 reads `hull.meters` (the least). `net_incidence` takes band-decided `over` first and
falls to `apex_part` only on refusal. So where `over` reads beside the cusp, that reader
keeps `|f|`/z² and not the height reading. That is sound but looser. Confidence: likely (by
inspection; not shown to cost a build).

## Claims
1. **Upheld: `at_height` is a sound upper bound.** Independent oracle: 60-digit `decimal`,
   the exact distance to the arc `{ρ ≥ 0}` (circle foot, else the nearer apex). It ran on
   121,508 Interval readings (seeded `R1_SEED` default) at the corners and a 5×5 grid. The
   draws covered horns, near-horns 1e-14…1e-2, `R` up to r·(1+1e-13) (horn snap), apples and
   lemons. Boxes beside the cusp, straddling h, near `|z| → r`, on the outer branch and on
   the other sheet's circle, at scales 1e-17…1e-1. Result: 0 misses, worst d/bound = 1.000000
   (tight at the cusp, as expected). The `(z² − h²)/(R + √(r² − z²))` algebra and the monotone
   max were checked by hand.
2. **Upheld.**
   - ME (no `|z| > h`): 24,553 misses on my oracle.
   - MF (no `|z| < r`): at Interval it yields NaN (3,612 uncertified), not a misread. So the
     guard is load-bearing for liveness at Interval and for soundness at f64, where
     `max` drops NaN.
   - MG (no `R > 0`): the proof holds (`r ≤ h + |R|`, so the height reading ≥ ρ_hi + |R| ≥
     the apex reading). Numerically, on 5,847 lemon readings the height reading ≥ the apex
     reading, least relative slack 15.6%, 0 misses.
   - MG over every row of `geom-brep` and `sweep`: see "MG run" below.
3. **Upheld.** `over`/`over_at` are byte-identical to the merge base. For non-sheet kinds,
   `over.or(apex)` equals main's `over.ok().or_else(apex_root)`; `apex_root` is now eager,
   which has no effect.
4. **Upheld, with MINOR 1.**
   - In `[−v*, v*] ⊂ [−π, π]`, `near(0)` is the angle read renamed. It differs from the
     latitude window only for `v ≤ 1 − π`, which is the fix.
   - The `±π` tie meets only a horn's cusp, which `chart_point` already reads singular.
   - The cut decision moves from the caller's band (logged) to `Band::least` (F31, unlogged).
   - No other `ChartAngle::window` reader exists: `projected.rs` `raw` has `unreachable!`.
   - The sphere's latitude readers are untouched.
5. **Upheld.** `Chart::poles` yields ±π/2 (sphere), 0 (cone) and ±v* (sheet). `v* = π`
   exactly only on the horn (`atan2(+0, −R)`), so the branch is horn-only. `ids[1]` is the
   leaving edge's first sample, so the sign is the leaving meridian's.
6. **Upheld.**
   - Lune oracles re-derived by hand: needle V = π(5/3 − π/2), band 2π(π/2 − 1), disc π,
     profile 1 − π/4; the horn by Pappus 2π², 4π², profile π. The lower needle mirrors the
     needle.
   - The rows' green (needle, lower needle, four-arc horn, wide apples) was not re-run
     separately: they ran inside the MG suite.
   - The limb-2 horn row reaches its gate (the `own` half); its off half does not (NOTE 1).
7. **Upheld.** The new `m − r` is algebraically exact
   (`m² − r² = ρ(ρ − 2R) + z² + (R − r)(R + r)`). The side `r·ρ + R(m − r) ≥ 0` is
   equivalent to the foot's `ρ ≥ 0` for `m > 0`. It is f64, not exact, and shares
   `geom::sheet_apex` with the code under test; my decimal oracle closes that gap.
8. **Partly falsified.** `limb-2s-floor-on-a-sheet-reads-the-whole-torus` exists on
   spiricarm's slate. `implicit.rs` 227 is unfiled, disposed "read by nothing at a cusp" in
   the body only. MINOR 1 is residue of the wide-apple row itself and is unfiled. The
   horn-cusp row's done-when is met. The wide-apple row's done-when is met only for
   `chart_point`.

**MG run (claim 2, every row).** I ran MG (`positive(major)` dropped):
- Over all 5,001 rows of `--profile ci -p geom-brep -p sweep`: 5,001 passed.
- Over the 394 slow-set rows of both crates whose names touch a sheet
  (`football|horn|spindle|sheet|lemon|apple|lune|needle|cusp|apex|limb_2|wedge`): 393 passed,
  including both lune rows.

The one failure is `a_plane_near_a_needle_cones_apex::a_needle_cone_cut_off_its_apex_lands_its_closed_forms`
(a cone). It fails identically at the unmutated head and is outside the diff. It cites
`work/splitdoor/tier-3-prime-cannot-settle-…`; that is for the orchestrator, not this PR.

Not run under MG: the remaining slow-set rows, which have no sheet and cannot reach the
mutated arm. A first full run was abandoned because debug spline rows (none on a sheet)
ran 20+ minutes. MG survives every row, and it is equivalent (proof checked plus numeric slack).

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8: partly; I read `projected.rs`'s
`Meters` impl, `net_incidence` and the module header, not all 3,300 lines.
- **Q1/Q4 half-fix:** the class "a sheet's `v` named in a window" has a second home
  (`Along::Latitude`) that the PR neither swept nor re-documented, while it rewrote
  `Window::Latitude`'s doc to deny it (MINOR 1). This is the brief's "fix mints the defect
  it closes" shape. `pcurve_cache.rs` `RevolvedTube` doc, the torus/sheet arm comment, the
  `Levers` doc. sure.
- **Q1:** horn-ness is spelled twice: `topo` `Chart::poles` (`sheet_apex(..).height == 0.0`)
  and `mesh` `walk.rs` (`vp == PI && !chart.v_domain_wraps()`). The second infers the kind
  from a float value instead of asking the chart. likely.
- **Q1:** the z-straddle predicate `z.0 <= 0.0 && 0.0 <= z.1` is written in both `at_height`
  and `over_at`'s Spindle arm (`projected.rs`), and `apex_root` reads the same z box a
  third way. likely.
- **Q7 / [simple.uniform]:** `Meters::certified` takes "least" on a sheet and "first" on
  every other kind, though the cone's `over` and `apex_root` are both upper bounds too.
  Two policies keyed on kind. unsure.
- **Q2/Q7:** the fold `fold(apex, |least, d| least.map(..))` returns `None` whenever
  `apex` is `None`, discarding a reading `over` made. It is correct only because
  `apex_root` "always reads on a sheet", an invariant held by a caller's comment
  (`pcurve_cache.rs` `spans`). likely.
- **[unrepresentable]:** `ChartAngle::window() -> Option<Window>`, where `None` means "read
  about zero", is an in-band meaning on `None`, not a tagged variant. likely.
- **[verdicts]:** `walk.rs` branches on `vp == core::f64::consts::PI`, a raw float
  equality on a control path. It is defensible as a kind-fixed singular point ([surfaces]
  locus sentence), but it is spelled as a value test. unsure.
- **Q3:** the limb-2 horn row's off half cannot go red under an unsound height reading,
  since the floor refuses first (NOTE 1). sure.
- **Q3:** `limb_2_against_a_horn…` accepts `KernelLimit | Limb`, and only `Limb` is
  reachable at both bands. A disjunction no run exercises. sure.
- **Q2:** `at_height`'s 14-line doc argues the tangency and the digit-keeping form at
  length for 20 lines of code; the PR body already carries that argument. unsure.
- **Q6:** "No `Interval` row reads a horn's cusp `v`" (PR body, Unproven) is disclosed
  without a schedule. unsure.

## Disclosures
- I read no other `spiricarm-rev6806-*` branch and no PR comments or reviews. I had no
  glimpses.
- Two background waits were mine: a cargo run the harness auto-backgrounded, and an
  until-loop I wrote wrongly. Both finished or were killed (my own PID) before I went on.
  The MG suite ran `setsid`-detached with its PID recorded and was polled in the
  foreground. No background work remains.
- Probes are on the probes branch; nothing was posted to GitHub.
