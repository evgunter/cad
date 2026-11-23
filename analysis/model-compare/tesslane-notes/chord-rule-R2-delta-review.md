DELTA RE-REVIEW — PR 6943 `tesslane/chord-rule` @ d63033b457 (from 9d3a0bce57)
    
    **Verdict: APPROVE-WITH-FIXES.** No MAJOR: every bound dominated in floats on every pose I attacked. Two MINORs are new (one a hang on a pub entry), and one row gap of mine is only half-closed. CI on d63033b457, checked by SHA: lint, test (kernel), test (app), mesh meter, gate all success.
    
    ## Claims
    1. **HELD.** Exact-formula truth (closed-form `u′`/`v′`, not difference quotients) against `speed_bound`, strict (no allowance).
       - Conics:
         - Sphere circles of angular radius ε = 1e-3…1e-9 centred k·ε off the pole, k ∈ {1.001, 1.05, 1.5, 2, 10}.
         - Cylinder circles ε from the axis; torus circles ε from the core.
         - Spans [0,2π], [0.3,1.9], [−5,9], [0,3π], [2, 2+1e-6].
         - Worst truth/bound 0.9999999999999991. The first head's −3.2% under-bound at ε = 1e-7 is gone.
       - Huge parameters: sphere pole circle at t₀ ∈ {1e6, 1e9, 1e12, 1e14}, widths 1e-3…1. Never under. This was my attack on the f64 extremes decision: `periodic_window_may_hold` is fed TAU and an unwidened phase, so the drift is ~4e-17·t. The z under-read stays below the enclosure's own rounding until t ≈ 4e8, and even past that it never surfaced as an under-bound.
       - Spiric wall and cap:
         - Tubes R=1 with r = 0.99999, 0.999, 0.99, plus R=3 r=1, R=100 r=0.01, R=1 r=1e-6; |d| up to (1−1e-9)(R−r).
         - Spans: a whole period, [π±1e-3], [3, 3.2], [0,1], [−7,12], [π, π+0.5]; truth refined around t = π.
         - Worst 0.9999999937 (wall), 0.9999999999999996 (cap).
    2. **HELD.**
       - `window_rates`/`piece_speed` read only `e.major/minor/offset` (grepped both bodies), so `Radii` is a mechanical change; `span_rates` and `spiric_length_bound` pass `e.radii()`, no answer moves.
       - `image_rates` reads `piece_speed`'s paired speed and derives `|f′| ≤ √(S² − r²·inf cos²)` and `|u′| ≤ √(S² − r²)/ρ_lo` from `|P′|² = f′² + r²cos²t`. I checked both identities.
       - Its "apart" azimuth reading `r|d|·sup|sin|/(ρ_lo·f_lo)` re-derives `geom::spiric_rate_bounds`' form locally (style S2). Not a fourth pairing.
    3. **PARTLY FALSIFIED** (MINOR-2). 25 mutants on speed.rs / props/spiric.rs.
       - **Red:** wall du ×0.9/×0.97/×2; cap df ×0.9/×0.97/×2; grading 0 and 10; extremes never / swapped; z ignored; net lever ×1.2 (survived on the first head; closed now); conic speed ×0.97; line vertex ×1.03; line ends ×1.03; focal ×0.97; sphere v ×0.97; apart-only; paired-only; torus tube ×0.7.
       - **SURVIVED:**
         - **torus tube-angle bound ×0.97 and ×0.9, both UNSAFE.**
         - cylinder/cone v ×2 and torus tube ×2 (loosening).
         - `cos_sq` forced to `min|c|²` across a zero (unsafe in form, harmless at piece width π/32).
    4. **HELD, resting on CI's goldens.** `reach_probes.rs` changed between the two heads, but only through main's lift-probes commits (`340f118707`, `3ff00f761b`: visibility and rustfmt); d63033b457 itself does not touch it. `sphere_image` restores the old sphere-arm refusal of projected spline/line rows, which closes my D. Curved and spline lattices now read `face_target`, which is δ for both, so no value moves. CI's mesh suite is green with goldens unmoved, which shows consistency, not correctness.
    5. **FALSIFIED** (MINOR-1, NOTE-1).
       - Correct: `NotASpan` (reversed, NaN, ∞); `OffRegime` (r=R, r=0, r&lt;0, R&lt;r, NaN R, ∞ R); `NonFinite` (∞ va, NaN sense, NaN conic); `OutsideDomain` on nets.
       - **MINOR-1, DEMONSTRATED:** the spiric arm's work grows with the span:
         - `speed_bound(0, 1e4)` takes 15 s and `(0, 1e5)` 166 s (debug build).
         - `speed_bound(1e15, 1e15+1)` never returns. RSS reached 4.9 GB and the kernel killed it.
         - Cause: `spiric_breaks`' `k += 1.0` / `m += 1.0` stall once `k·step` passes 2⁵³. The image is periodic, so no span needs more than one period's pieces.
         - Unreachable from the chord pass, whose edge spans are sane, but it is a pub entry that hangs on a finite forward span.
       - **NOTE-1, DEMONSTRATED:**
         - Harmonic (and iso-line, arc-rim) with a NaN coefficient answers `Ok((NaN, 0))`, not `NonFinite`.
         - A projected torus with NaN `major` refuses `ThroughSingularSet`, not `NonFinite`.
         - `Levers::tube` is plain f64 (`(lo−x).max(x−hi)`, `hypot`), so the torus v bound is not outward-rounded as the module doc claims. Off by an ulp at most.
    6. **HELD.** The README sentence re-words the code: iso circle at unit speed on the curved lane, the row's `speed_bound` on the sphere and NURBS lanes. The trimmed-cylinder sentence is gone. It decides nothing.
    7. My first-review findings:
       - **Closed:** A (pole under-bound), B (reversed-span panic), D (sphere arm admits spline rows), E (net domain, refusal doc), NOTE-2 (README), NOTE-3 (knots history line), S1 (spiric now reads props), S3 (Iso reader cites its retiring unit), S5 (one target), S8 (rule rows).
       - **Half-closed:** C. The azimuth legs are pinned ≥0.99 and an unsafe net lever goes red, but no v channel off the sphere is pinned (MINOR-2).
       - **Open:** NOTE-4 (tightness claims are sweep-local: wall reads 0.68 of its bound on one full-period pose I swept, against the pinned 0.75; safe direction). S2 and S4, below.
    
    ## Findings
    - **MINOR-1** `speed.rs` `spiric_breaks`: unbounded work and an infinite loop at |t| ≳ 6e14. DEMONSTRATED (claim 5).
    - **MINOR-2** `speed_rows.rs` `a_projected_images_bound_dominates_it_on_every_chart`: a torus tube-angle bound 10% too small passes every row. DEMONSTRATED by mutant. The projected torus has no mesh reader today, but the bound is pub and stated certified.
    - **NOTE-1** NaN coefficients and the f64 `Levers::tube` (claim 5).
    - **NOTE-2** `image_rates`: the `cos_sq` branch is unpinned (mutant survives). Harmless at the current piece width.
    
    ## Style (delta)
    - **S1 `likely`** `speed.rs:538/558/579`: a third spelling of a harmonic's range. `harmonic_enclosure` (intervals) now sits beside `harmonic_range` (f64, kept pub for `mesh::sizing`) and `pcurve_cache.rs:991` `harmonic_span_box`. Two of the three share `harmonic_extremes`.
    - **S2 `likely`** `props/spiric.rs` `image_rates`:
      - Its apart azimuth reading re-derives `geom::spiric_rate_bounds`' `r·|d|·sin/f`.
      - The dependency runs from `pcurve_cache::speed` into `props` (a properties module) for a chart-image fact.
      - The open `work/germ/the-whole-period-spiric-speed-bound-reads-sin-and-f-apart.md` is still neither cited nor touched.
    - **S3 `unsure`** `speed.rs:500-532`: private interval `dot`/`hyp`/`root_down`/`root_up` beside `patch_bound.rs:391`'s interval `dot`.
    - **S4 `likely`** `chords.rs` `sphere_image` plus `spline_image` plus `speed_bound`'s match: still three per-form lists (class unchanged; the restored refusal adds its own arm).
    - **S5 `sure`** `speed.rs` module doc says the projected bounds dominate in floats via outward rounding; `Levers::tube` is f64 (NOTE-1).
    
    Questions exercised: Q1, Q3 (mutants), Q5, Q6.
    
    ## What I executed
    - 4 probe tests: conic/pole/axis/core sweeps, huge-t extremes, spiric extremes and sub-spans, input domain and timing.
    - 28 mutants (25 + 3 depth runs on the torus and sphere v).
    - The PR's check runs, read by SHA.
    
    I did not re-run the mesh suite or reach_probes locally on this head; CI covers both. Read only: the speed.rs and props/spiric.rs diffs, the chords.rs diff and its new rows, the README diff, the knots counts.
    
    I saw no other review's material. My workspace is clean apart from a scratch copy of my probes, kept outside the repo.
</cross-session-message>

0 notifications remain queued.
