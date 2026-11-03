# R1 review: PR 6486 (`spiricarm/spindle-football`), frozen head `49d88cc0f11bc1cce1ef9ef4e8a479263af7fd92`

**Verdict: APPROVE-WITH-FIXES** (0 MAJOR, 2 MINOR, 2 NOTE; style below).
Probes: `spiricarm-rev6486-r1/probes` (`crates/sweep/tests/review_6486_r1.rs`, the
`r1_spindle_meters_probe` module in `projected.rs`, and `review-6486-r1-mutants.patch`, the
env-gated `R1_MUT=<name>` mutants cited below; all default off).

## Findings

**MINOR-1. `sweep::revolve::axis::classify_segment`: `SheetApex` is decided before the arc's interior, against the ratified arc rule.**
The `va.pinned || vb.pinned` return comes before `interior_clear()`. `crates/topo/README.md`
says an arc whose interior crosses the axis refuses, *and otherwise* the centre distance picks
the kind. Executed at head:
- P1: a full circle centred `(−1, 0)`, `r = 1`, vertex on the axis: wholly across the axis, and
  with `R = −r` no sheet exists. P2: a full lemon circle (`R = −0.5`) from an apex. P3: the
  lemon's long arc between its apexes, through `x < 0`. Each refuses `SheetApex`.

All three should be `ArcCrossesAxis`. The recourse ("end the arc off the axis and close the
profile to the axis with a line") cannot be followed for P1. For P3 it leads the user to
`ArcCrossesAxis` next. Only the type and text are wrong; no body is built wrongly.
Confidence: **sure** (rows `r1_p1..p3`, red at head).

**MINOR-2. `geom_brep::pcurve_cache::projected::Meters::over`, `Spindle` arm: the second lever is load-bearing and nothing goes red without it.**
I fuzzed `over` against the exact distance from *this* sheet's meridian arc. The draws were
400k point boxes and small boxes, weighted toward the axis, both sheets' sides, and every
`R ∈ (−1, 1)`. Result: 0 under-reads, with the least `d/d_true` at 1.0000015 (sound, and tight).
With `positive(minor·rho − d·|R|)` removed (`R1_MUT=lever2`), the fuzz finds 1271 under-reads,
e.g. `R = 0.90, (ρ, z) = (0.142, 0.049)`: `d = 0.268` against a true `0.411`.
That mutant survives the PR's football rows, `bool3_torus_doors` and the revolve rows. No
geom-brep row has a `Spindle` case: `each_conversion_reads_its_stated_floors` has none, and
`a_spindle_toruss_bound_asks_its_lever_positive` is the ring arm. `Meters::Spindle::over` *is*
reached by the football (a panic mutant there fails 5 sweep rows), so the "written, not fuzzed"
gap is real but now closed by the probe. A committed row is still owed. Confidence: **sure**.

**NOTE-1. `classify_segment`, `ArcReach`: the near-horn band refuses only in the escalate half; the zero half mints `R = rc` as it stands, past `r`.**
`R = r + 1e-13` builds `Spindle { major_radius: 1.0000000000001, minor_radius: 1 }`, passes tier 3,
tessellates and measures exactly (rel 3.7e-12); `R = r − 1e-9` builds; `R = r + 1e-9` refuses
`SliverAxisClearance { ArcReach }`.

Consistent with piece 1's horn accepted in band (clamped radicand), but the brief's premise holds
only for `(zero, escalate]`, and the stored datum exceeds `R ≤ r` by up to the band.
Confidence: **sure** (executed); design-consistent.

**NOTE-2. A sphere-class centre at `−1e-9` refuses `Op { Certification … pcurve_map_residual }`, while `+1e-9` refuses `SliverAxisClearance { ArcCenter }`.**
The `ArcCenter` path is unchanged by this PR, so this is pre-existing. I record it because
the brief asked about a sphere-class arc with `rc` in band. At `±1e-13` both build a sphere
and pass tier 3. Confidence: **likely** pre-existing (not bisected).

## Claims
1. **Partly falsified** (MINOR-1).
   - Otherwise upheld. The new `ArcInterior` is equivalent to the old `ArcSpan` + `ArcApex` on
     sphere arcs: both ask whether the arc reaches `c − r·ê_r`, given endpoints at `r ≥ 0`. I
     checked the semicircle pole-to-pole case by hand (margin `π/2·r`).
   - The ring-torus path is untouched. Near tangency: in band refuses, the zero half builds a
     horn (NOTE-1). Exact horn `R = r` builds and measures exactly; a sphere-class arc at `±1e-13`
     builds a sphere.
2. **Upheld** (the fuzz in MINOR-2). The algebra checks:
   `M² − m² = 4Rρ`; the lemon takes `r² − max M²` over the box (a convex max sits at an
   endpoint); the apple takes `min`, because `ρ + R > 0`; `|m − r|·|R| ≤ rρ` suffices for the
   circle's nearest point to have `ρ ≥ 0`.

   `apex_root` is cone-only, so no fallback bypasses the lever. `distance_floor`'s `Spindle`
   divisor bounds the distance from both sheets, which is still a sound floor for one.
3. **Upheld.** Each of the six fixes, reverted alone, turns the football row red. Each reverted
   pair (`spine`, `bstar`, `alpha`, `fit`, `orient`, `core`) also fails `r1_p4`. My second-pass
   reading found no miss: `torus_grid_steps` and `cert_torus`'s `abs_radial_max` are sign-safe,
   the Placement latitude lever is right, `chart_incidence` is span-only (`R1_MUT=turn` reds 3 rows).
4. **Upheld.**
   - `R1_MUT=walk` (walk unwired) reds all four re-pinned `door_roster` rows and the football.
   - `g(v) = r(Rv + r sin v)` has `g′ = rρ ≥ 0` on the sheet, so a winding-0 loop reads by its
     area sign as on a sphere.
   - `dv ≤ span/r` follows from `ds² ≥ r²dv²`.
   - An apex inside a face reads `NotAVertex`, an unread one `SheetApexReached`; neither passes.
5. **Upheld, independently.**
   - My probe checks props against a 400k-step midpoint quadrature, not the closed form: 7 bands
     (horn, apple `R = 0.9, v = 2.0`, `R = ±0.05`, lemon `R = −0.9`, and the PR's two), plus 12
     bands at `v*(1 − ε)` for ε down to 1e-7 (rim ρ ≈ 1e-7). Every one is within rel 2.3e-11.
     `volume_pad` and `area_pad` are 0, so the 1e-10 test tolerance is the real one.
   - The mesh certificate's second partials are `ρ`, `r|sin v|` and `r`, which hold for either sign.
6. **Upheld.**
   - I re-derived both closed forms by hand (`πr∫(R² cos v + 2Rr cos² v + r² cos³ v)dv`,
     frustums); quadrature agrees to about 1e-11. The four bands assert what they claim (rims at
     `r ≥ 0`, tips above the rims, apple `v = 1.6 > π/2`). The `δ·A` bound is right; deficits
     measured 20–25% of it.
   - A wrong-sheet mint (`R1_MUT=wrongsheet`) dies inside `revolve` (the pcurve mint) before the
     `R` assertion is read (style S5).
7. **Upheld by reading.**
   - The `bool3`, `one_segment`, `revolve_errors`, roster, Display and `door_roster` re-pins
     follow the change.
   - The envelope fuzz's wrong-stored-scalars cone rows went with the signature that carried
     them: an inexpressible case deleted, not a loosening.
   - No assertion was weakened except where the classification moved.
8. **Upheld by reading.** `meters_at` takes the cone's nappe through the same `nappe_of`
   decision (same predicate, same `Margin::of(mid)`) as `chart_at`, and every other analytic
   arm of `chart_at` was infallible. `limb2_quick`'s memo name renders the same `Opening`/`Mirror`.
9. **Upheld.**
   - The lathe row is closed with a reason and the plan drops it. The tour row exists (P3).
   - The wide-apple evidence holds. Horn bands at `v*(1 − ε)`, past `π − 1`, build and pass tier
     3, then the mesh refuses `UnsupportedCurvedDomain` typed (P6). Nothing built was silently
     mis-handled.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. For Q8 I read `sweep/src/revolve/axis.rs` end to
end (705 lines), not the largest touched file (`pcurve_cache.rs`, ~15k lines).

- **S1 (Q1). The question "does the arc hold `m = c − r·ê_r`" is computed two ways in one file** (`axis.rs`).
  - `radial_extent` (≈ :212–240) answers it with a chord half-plane sign folded by `copysign`.
  - `classify_segment`'s `interior_clear` (:518–537) answers it with a turn angle `π + atan2` less
    the span.

  Two spellings of one predicate, which can disagree in band. **likely**
- **S2 (Q1/Q4). Five new `T::one().copysign(major)` sites choose a sheet** (`pcurve_cache.rs`
  meridian `alpha` and `b*`, `projected.rs` σ, `props/curved.rs` `about_core` and
  `meridian_orient`). The open row `work/lathe/copysign-stands-in-for-a-nappe-and-a-max-fold-and-hulls.md`
  names copysign-as-a-choice as the defect class, and its plan is edited by this PR. They are
  safe only because `R ≠ 0` is decided at rest, and no site says so. **likely**
- **S3 (Q1). The torus-or-sheet split is re-made inside the shared or-pattern arms by
  `matches!(surface, Surface::Spindle { .. })`** (`pcurve_cache.rs` ≈ :6432, :10409, :11519).
  These are the non-exhaustive kind tests piece 1's census swept, re-minted inside arms the
  compiler no longer splits. **likely**
- **S4 (Q4/Q5). Stale text after `UnsupportedToroid` retired:**
  - `sweep/src/revolve/mod.rs:1075` ("unsupported toroids" in `# Errors`); `revolve/tube.rs:157`
    (`TubeError::Revolve`, "ring-torus convention");
  - `sweep/tests/m6_tube.rs:273`, "The ring-torus convention (R > r > 0) refuses through the
    SHARED revolve decide". That row's `Err(TubeError::Revolve(_))` passes on any refusal, so
    it cannot see that the reason changed to `ArcCrossesAxis` (also Q3);
  - `bool3_torus_doors.rs` ≈ :704, "if both are mintable through public doors. They are not".

  **sure** for the first three, **likely** for the last.
- **S5 (Q3). The football's "wrong-sheet" assertion cannot go red alone.** A flipped `R` never
  builds (the pcurve mint refuses at `a_conical_ended_football_revolves.rs:132`), so the row the
  PR body calls the wrong-sheet row is redundant. Its volume half is red only on an oracle flip,
  not on a code flip. **likely**
- **S6 (Q2). `Meters::over`'s doc carries the whole soundness argument for two levers.** No row
  executes it (MINOR-2). It is the long justification beside unguarded code. **likely**
- **S7 (Q7). The `Spindle` arm reads `z` as `f64` (`from_computed`) and `ρ` as `S`.** **unsure**
- **S8 (Q5). `chart_singularity.rs` ≈ :841 reuses `R` for the sheet's minor radius** ("`R` a
  spindle sheet's minor radius on a sheet") in a doc where `R` is the major radius everywhere
  else. The line also runs past the wrap. **likely**
- **S9 (Q6). The demo deviation is disclosed and scheduled** (`the-conical-ended-football-has-no-tour-stop`).
  Scheduled, so nothing is owed; I flag only the P3 priority. **unsure**
- **S10 (Q1). `torus_grid_steps`' `a = R + r` → 0 on a thin lemon, so `β = √(r/a)` grows and the
  share of δ shrinks: sound, needlessly fine, unpriced. **unsure**

## Disclosures
- No other `spiricarm-rev6486-*` branch, PR comment or review read; body via `get` only; no glimpses.
- I did not run the workspace suite, admesh or `export_acceptance`; I rely on the PR's CI run
  41733776655 for those. I ran the sweep rows named above, geom-brep's whole lib suite under the panic mutant (1052 passed;
  the sheet arm is not reached in geom-brep), and topo's `door_roster` sheet rows.
- Mutants were env-gated and reverted. The report branch is the frozen head plus this file.
