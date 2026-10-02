# PR #3805 third delta review: fix pass 5ebc15ef73..7b8f75faf6 (checkout 7b8f75faf6)

**Verdict: APPROVE-WITH-FIXES.** The MAJOR is fixed in general, by execution through the rung (0 wrong clearances in 12 931 certified, over 4 pose sets). The fixes asked are: pin the load-bearing sample charge with a row (MINOR-1), and finish the reader sweep (MINOR-2/3). PR head is now 337d85fbe (a later `main` merge), which I did not review.

## Findings
**MINOR-1, the per-sample charge is load-bearing but no row can redden it** (execution; *sure*). Location: `implicit.rs:791`.
- The PR body says dropping it leaves the rows green "out to 1000 km" and that "the chord-dip charge dominates". The first part is true; the second is false for short arcs far out.
- Short arcs (±3e-4 rad, metre conic, ball 0.1–1 m, centre 1000 km out, ε 1e-12, 3000 poses per kind) with `sample_rounding` zeroed: 18 crossings and 4 in-band certified clear, out of 1220.
- At head: 0 of 832.
- Pin: `probes/reach_3805_short_arc_pin.tsv`, true least −5.3e-12 m (replay with `reviewer_rung_replay`).
- At 100 km the same mutant makes 0 wrong (1408 certified), so the in-tree fuzz's arc of ±0.5 rad cannot see it.
- The harmonic `f2`'s `4·noise` (`:506`) stays unreddened by anything I ran either (fuzz + full geom-brep/topo, 2908 green with both zeroed). It is probably honestly dominated, as the body says. *likely*

**MINOR-2, three ordered/signed readers the sweep missed** (inspection; *sure*):
- `topo/src/split.rs:252` and `topo/src/splitting/classify.rs:373`: `InfSpeed::new(minor)`. For `minor > major` (passes tier 3) this OVER-states the speed floor, so an interiority claim can be certified short. This is the class `certify`/`param_rate` were fixed for.
- `mesh/src/sizing.rs:426` `ellipse_step`: `R_eff = major·(major/minor)²`, documented for `major > minor`. Swapped, it is ≪ the true `sup|C″|` and the step hits the angular cap. For a = 0.1, b = 4 the chord sag is about 4·step²/8 against δ. A negative `major` gives NaN, which also takes the cap.
- The work item's sweep record (`an-ellipse-stored-minor-over-major…md`) and the audit row "the same mint `param_rate` and `splitting::classify` make" list none of them.

**MINOR-3, the span meter fix WIDENED the certify gate** (execution; *sure*). Location: `certify.rs:2260`. `probes/reach_3805_signed_minor_certify.rs` puts the plane∩tilted-cylinder ellipse through `EdgeCurve::certify` (the gate `set_edge_curve` runs):
- at 653e7d5c, `minor = −1` is refused with `IntervalNotForward(−π/2)`;
- now it certifies.
- The new `Conic` doc (`implicit.rs:597`) says `set_edge_curve` "checks neither" sign. That sentence was false before this commit and is made true by it.
- A negative `minor` now reaches the signed readers of MINOR-2, which see a negative meter.
- Either intended (then say so in the doc and the item), or the floor should refuse a non-positive stored semi-axis.

**NOTE-1, the charges are valid bounds** (derivation + execution; *sure*).
- `quadric_harmonics` (`implicit.rs:1022`) is an exact identity for any frame: `cos²`/`sin²`/`cos·sin` folded, no orthonormality assumed.
- Each coefficient is a few roundings of quantities ≤ `terms = (|C₀−o|+speed_hi)²+r²`. Charging 8u·terms/2r covers them. Extremes add a₁/a₂ sqrt and final-sum roundings ≤ u·terms/2r.
- Mutant `HARMONIC_NOISE_ULPS = 2` (8× less): still 0 wrong in 2114 certified, so the margin is at least 8×.
- `sample_rounding`: the point error ≲ 3u·(|C₀|+√2·a)·√3 levered by D/r. That is within its 8u·(2D·at)/2r, tight in the worst case (~10% room); the empirical errors are far below it.

**NOTE-2, claim 1 by execution** (fresh probe through `super::conic_clearance`, mounted in `reduce.rs`; door per `wall_crossing`'s dispatch). 60-digit oracle `probes/reach_3805_rung_oracle.py`, which reads the stored f64 values as exact binary and judges the true margin `max(least, −most)` over the arc.
- **Poses:** circles (radius sign drawn) and ellipses in 8 orders/signs, eccentricity ≤ 40, size and centre each 1 m / 1 km / 100 km; sphere, wall, torus, cone; normal in-plane or tilted; gap ±40ε; ε 1e-12/1e-9/1e-6.
- **Head:** 0 of 901 (36k poses), 0 of 3631 (144k), 0 of 7567 (short arcs at 100 km), 0 of 832 (short arcs at 1000 km).
- **653e7d5c (old charge):** 160/140/48 crossings + 7/9/6 in band, of 2095.
- **Circle half on main's arm:** circle × sphere 27/29/11 → 0/0/0.
- **Mutant "extremes bare":** 600+ wrong; it reddens both in-tree `clearance_rows`.
- **Readers:** only `reduce::conic_clearance` reads `conic_residual_extremes`/`conic_arc_residual_range`. The `circle_*` wrappers and `circle_residual_curvature_bound` are test-only. No bare harmonic bound is left in the curved-face arm. `tangent.rs`'s `chord_dip_charge` reads a line's own `f2`, a different class.
- **Torus and cone:** cone never certifies (`None`); torus certified 7 at 1e-6, all correct.

**NOTE-3, over-refusal** (execution; clear poses, gap 30–10⁴ ε, refused = rung not clear and door not `Miss`).
- Metre scale (size and centre 1 m), ε 1e-9 and 1e-6: no change.
- ε 1e-12 metre: S circle 4→15, S ellipse 41→60, C ellipse 62→64, all of them 1 µm–0.4 mm balls at 30–7400 ε (≤ 7 nm). That is where the true rounding (u·a²/r ≈ 1e-10) is past the band; no ordinary pose was among them.
- Large scales refuse more (e.g. 1e-6 S circle big 97→209).
- Side observation (pre-existing, `main`): the circle × sphere door answered `Uncertain`, never `Miss`, on every clear pose that reached it, so circle × sphere clearance rests on the rung alone. *likely* worth a look by TANG.

**NOTE-4, claims 3–5.**
- **`curve_reach` (`measure.rs:517`) is no under-reach on certified geometry** (inspection, *sure*). A negative circle radius fails certify's span meter (`span·r < 0`, or with `span < 0` the winding headroom), as well as tier 3. The ellipse row reddens on the old read. The speed floors and `edge_extent` have NO row red on the old read (inspection).
- **Straddle row:** the mutant (∪ answers the slab) is red at 1e-6, 1e-9 and 1e-12, on `volume 0.37522 vs 0.37575 (pad 0)`; the head is green at all three ε.
- **Slow set:** `no_certified_clearance_on_a_graze` is excluded from `--profile ci`, selected by the workflow's slow expression, and red under the bare extremes.

**Suites** (`--profile ci -p geom-brep -p topo -p sweep -p editor-core`): 7253/7253 at ε 1e-9, 1e-6 and 1e-12. `rigid_map_near_eps_plane_nurbs` is in the ci-excluded slow set, so it did not run.

## Style (exercised: Q1, Q2, Q3, Q4)
- **Q1, a fix mints a copy.** "One home for the conic test oracles", yet `clearance_rows` (`reduce.rs` ~4920) writes a fourth distance oracle (sphere/wall/torus, anchor-relative) beside `oracle::distance`. And the shared oracle lives in a consumer (`ellipse_roots::oracle`) imported by `reduce`. *sure*
- **Q2/Q7.** `ladder_roots` now computes a full quartic ladder whose only possible effect is an escalation; it adds refusals and nothing else (HONE's filed item). *likely*
- **Q3.** `the_reach_bounds_an_ellipse_in_any_stored_frame` and the in-tree clearance fuzz use f64 oracles; the latter's "`C₀ − anchor` is exact" holds only while the two are within a factor 2 (Sterbenz). It is fine for its poses. *likely*
- **Q4.** See MINOR-3: a doc sentence about the gate postdates the change that made it true. *sure*
- **Reviewer's own trap, recorded.** My first oracle read `Decimal(repr(x))`, which is up to half an ulp off at 1000 km (6e-11 m). It reported 103 false defects until fixed; the probe now reads `Decimal(float)`.

Probes: `probes/reach_3805_rung_fuzz.rs` (mount in its header), `reach_3805_rung_oracle.py`, `reach_3805_signed_minor_certify.rs`, `reach_3805_short_arc_pin.tsv`.
