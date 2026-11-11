# Review r2 — PR 6808 (`spiricarm/rotational-lane-torus`), frozen head `f87c193de862a2129cbab57f852870a5a8222cd4`

**Verdict: APPROVE-WITH-FIXES.** No MAJOR. One execution-witnessed MINOR: the torus's level anchor goes unpadded against the lane's own envelope contract. Probes are on `spiricarm-rev6808-r2/probes` (`0576c5dca`): `swap_rows/r2_probes.rs`, the `r2_*` rows at the end of `swap_rows/torus.rs`, `r2_print_oracle_values` in `donut_slices.rs`, and `r2_square_oracle_check.py`.

## Findings

**MINOR-1, `rotational.rs:torus_projected` / `torus_edge_terms` (residual pad). The level anchor `v(t₀)` enters the term unpadded.** Confidence: sure on the execution, likely on how far it reaches in practice.
- The periodic parts are read off the carrier's frame point, but the slope part is `κ·(v_img(t₀) + ∫v′_carrier)`, anchored at the image's end (`start = e.ends[0].1`).
- The module docs' contract says each image tracks its carrier within `e` at every parameter. An image `δ` off in level (`r·δ ≤ e`) moves the area term by `R·r·δ·Δu = R·e·Δu`. The residual pad pays `L·e ≥ ρ·Δu·e`, and on the inner side `ρ < R`.
- Probe `r2_a_band_whose_images_stand_off_in_the_level`: an inner-equator band (`v = π ± 0.05`), both rims swapped, each image's ends moved outward by `δ` in `v`, `env = r·δ`, run over the full schedule.
  - `(R, r) = (1, 0.8)`: the area misses by `−4.0e-7` at `δ = 1e-7` and by `−4.0e-5` at `1e-5`. The flux misses too.
  - `(1, 0.95)`: misses by `−8.3e-7`, scaling linearly with `δ`.
  - `(1.3, 0.45)` and every outer band hold.
- In production a projected image's ends are its carrier's own inverse: enclosed at `T = Interval`, rounded at `T = f64`. So `δ` is rounding-scale and the miss sits far below every pad. Nothing states or checks that the anchor's soundness depends on this rather than on `e`.
- The deck guard (`turns_name`, tolerance `1e-12·(1+k)` turns) also admits a `v_off` up to about `6e-12` rad off a whole turn. That offset feeds the same anchor unpadded.
- Either pad `κ·|Δu|·e/r` per projected edge, or say at the anchor that it must be the carrier's own level.

**NOTE-1, `projected_edge.rs:of_image` (image `R` guard).** The `lo`-only mutant is declared "equivalent, its `hi` comparison refuses the same images". That holds only when both brackets are points. A non-point pair with equal `hi` and unequal `lo` (image `[R, R⁺]`, chart `[R⁻, R⁺]`) passes the mutant and is refused by the guard. The harm is nil, since both enclose one `R`, but "equivalent" is false as stated, and no row can tell the two apart. This is the brief's recurring class. Confidence: sure (inspection).

**NOTE-2, `taylor.rs:primitive_each` (`y` spread).** The rule checks out on paper: `y(x)` comes from left moments, the jet is `[y, y′₀, y′₁/2, y′₂/3, y′₃/4]`, the over-piece value is `y(x) + y′([p,q])·[le, ri]`, and `(t−x)⁴ ≥ 0` justifies the remainder. The surviving spread mutant is structurally hard to witness: the `y₁·by₃` term over the piece carries roughly 8× the dropped `y′·h/2·by₄`, so the Lagrange coefficient keeps slack. I could not make it red by hand. Still, the spread is required for soundness, and no row exercises it. `primitive_each` has no direct unit row; a synthetic integrand with `by₃ ≡ 0` would isolate it. The PR body also names it `primitive_over`, but the function is `primitive_each`. Confidence: likely.

**NOTE-3, `swap_rows/torus.rs` oracles at ulp scale.** An all-harmonic sector whose meridians are split in two (`r2_winding_…`, case `[false, false]`) misses its truth by `4e-16` (`3.9078264390783657` against `lo = 3.907826439078366`). The unsplit one holds by `5e-16`. That is the fixture's own Simpson and f64 rounding of `A⃗` and the closed form against a thin enclosure, not the lane. The PR's rows avoid the all-harmonic winding case, perhaps for this reason. Confidence: likely the oracle.

**NOTE-4, `rotational.rs:torus_face` count loop.** `lo += 1.0` / `hi -= 1.0` are no-ops past `2^53`. With `|X|/A` up to `1e20` it terminates (`r2_count_window_terminates_on_a_huge_x`), because outward rounding makes the end counts meet the window. That rests on rounding rather than on the code's structure. Confidence: unsure whether it can ever hang.

## Claims
1. **Count window: upheld.**
   - Sound for every `X`. The true `(X, A)` is inside `x + N·A` by interval arithmetic, so `meets(N_true)` holds.
   - The skipped starting ends are equivalent: `fl` is monotone, so `⌊fl(−x.hi/A)⌋ ≤ N_true ≤ ⌈fl(1 − fl(x.lo/A))⌉`.
   - Two counts or more read the whole window soundly, and the flux is affine in `N`.
   - Probed at both ends (`r2_count_window_at_the_ends`). A thin `X = 0` reads the whole torus (count 1). A thin `X = A` reads `[0, A]`: sound, though loose. `X = −A` and `X = 2A` were also read.
   - The wrap loop's face never narrows below `[0, A]`. The row says so: the closed form is taken before the lane.
   - Pinched-oval row: not re-executed with the decision restored (I took the PR's run).
2. **Lift and seam: upheld, by execution.**
   - Re-anchoring algebra: moving the start shifts the integral by `κτ·c_j·2πW_u − W_v·κτ·(u_j − u₀)`. The seam term cancels the second part. The first part is a whole `(Φ, A)` pair, which the count absorbs with the right sign under either sense.
   - Probe `r2_winding_loops_read_independent_of_start_and_branch` passes (19 s): a sector (`W_v = ±1`) and a band (`W_u = ±1`), each loop split in two and both of its chains started at each edge, edges named `±τ` up in `u`, in `v` and in both, one loop swapped, both senses and complements.
   - `Junction::levels` and the connector's `ends[j] = c + b` match the chain's level at the edge-before's end.
   - The ring readings are consistent under either sense. A disc-with-hole stored hole first refuses (loud), which is the cylinder's convention.
3. **`primitive_each`: upheld as an enclosure** (NOTE-2).
   - `w′` is right at all three callers (`slope_form(weight)·inv_len`, as `dB/dt` is read), and no caller reads a changed value.
   - C9: `v(t₀)` crosses in through `from_certified`. The `trig_of` used on rims and connectors is the existing algebraic rotation (`trig_at`). The anchor itself is MINOR-1.
4. **Spiric arm against the deleted wall: upheld by inspection** of base `spiric.rs` (`wall_chart_split`, `wall_windings`) and base `quad_lane::spiric_wall`. Every term and refusal is kept or replaced:
   - integrand and `v = v₀ + σt`: the same;
   - radii, mirror and sense refusals: kept verbatim;
   - the old pad `(L_chart·e/min(R−r, r))·r(R+r)` is now the metric `L·e·(R+r)`, consistent with stages 1 and 2;
   - junction lever: from a point to a hull;
   - connectors: added, which closes the filed row;
   - `wall_windings`' `[0, 0]` is now `require_boundary_windings`, which also reads bands.

   I did not diff the ported rows line by line.
5. **Schedule: upheld by inspection.** `span_schedule` bounds `|v′|·h` through `tube_jets`; `piece_reach` uses `R + r`; spirics read a round's pieces. Not probed.
6. **Levers and pads.**
   - Upheld: `ρ`, `r`, the flux density `R·cos v + r` (≤ `R + r` in magnitude), the speed `ρ|u′| + r|v′|`, and Bol–Fiala with `K ≤ 1/(r(R+r)) ≤ 1/r²` (negative curvature only helps).
   - Falsified: the residual pad does not cover the level anchor (MINOR-1).
7. **Guards: upheld.** The ring torus (`min(R−r, r).lo > 0`, certified) and the deck guard check out. The `R` guard's "equivalent" mutant is only conditionally equivalent (NOTE-1).
8. **Oracles: upheld, by execution.**
   - `donut_slices` square rod: oracle `∖ = 9.84309075243496778`. My own reading (the inner `y`-integral in closed form, the `z`-kink at `0.0909059` cut, Gauss–Legendre under the sine map) gives `9.843090752434964–6`, and `∩` agrees to `1e-16`.
   - Off-axis `∩` is `0.122811018352756304`, the PR's polar value.
   - `AlongX`'s `z`-cuts miss the band-edge/circle tangency, but the result is still accurate to about `4e-15`.
   - The cavity: its profile and its steps are cut at the shoulders. Inspection only.
9. **Rows and residue: upheld at inspection level.**
   - The three filed rows meet the bar: they sit past this lane's gate, on SECTTORUS, INSIDE, and a P2 "no volume truth" row with a done-when.
   - The closing rows' done-whens are met: the round trip imports at the sliced volume, and connectors are read and padded.
   - I reran no slow sweep rows (no finding needed one).

## Style
- **Q1:** `rotational.rs:refined_turn` re-spells `entry_turn`'s ladder (`τ/8`, ×4 pieces, `MOST_SPAN_PIECES`), and says so in its doc ("as a projected edge's is"). Two homes for one refinement rule. *sure*
- **Q1/Q4:** the torus learned that a ring reading on a one-round window must read, not decide. The sphere's `require_sphere_sides` still decides every loop (a one-loop face included) at `Run::ringed`, so a wide round at a `RoundWindow::at` end can escalate there, as the torus did. No sibling sweep is recorded. Look at the sphere and cylinder `ringed` closures under the result gate's windowed runs. *unsure (not executed)*
- **Q7:** `torus_edge_terms` builds a throwaway `RotationalChartQ::Torus { outward: true }` only to call `.residual`. *sure*
- **Q7:** `edge_entry`'s spiric arm reads the full `wall_terms` (flux, area and turn) up to 1024 pieces just to get `Δu`. When `Δu` is the one not certified, its refusal text says "no certified positive f". *likely*
- **Q7:** `torus_face` counts whole tori in `f64` (NOTE-4). *unsure*
- **Q5:** the PR body cites `taylor::primitive_over`, but the code has `primitive_each`. *sure*
- **Q3:** `the_wrap_loop_bounds_the_whole_torus` and the pinched-oval row only check containment of a reading that is wide by construction, so a wider (degraded) reading passes them more easily. Their mutant witnesses are the guard here. *likely*
- **Q6:** "the oracle's own quadrature may be off by" `ORACLE_SLACK = 1e-11` (sweep) and `1e-12` (step-import) are measured claims without a mechanical guard. The polar cross-check guards only the `AlongZ` shadow case. *likely*
- **Q2:** the module docs' torus section is a single 40-line paragraph that argues the design (seam, counts, sense) inline. By the style lane's standard that is mild evidence to flag. *unsure*
- **Q8:** I read most of `rotational.rs`'s non-test code (the head docs, lines 240–2355) but not its test tail end to end. *disclosed*
- **Q-sweep:** `rg 'verbatim|re-derived|ported from|mirror of|by hand|kept in step|must match|change both'` over the touched `src` found nothing beyond `refined_turn`'s "as … is". The constants grep found `τ/8` twice (above). The prose pattern cannot match an undisclosed copy. *sure*

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (partial).

## Disclosures
- I read no `spiricarm-rev6808-*` branch but my own, and none of the PR's comments or reviews. I read the PR body through `pull_request_read get`. I posted nothing to GitHub.
- My first nextest filter (`r2_`) also matched other reviewers' old in-tree probe rows (`review_6092_r2_probes`, `ring2_r2_probes`) and ran them (about 10 minutes, all passing). I read only their names and pass lines.
- One cargo run was moved to the background by the harness timeout. I waited for it in the foreground, and nothing is running now.
- `CARGO_TARGET_DIR=/home/user/r2-target`.
