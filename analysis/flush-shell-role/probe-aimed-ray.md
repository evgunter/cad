# Probe report: shell role from V/A vs from aimed rays (FLUSH)

**Short answer:** on every body that builds, both ε, the V/A read refuses and the far-point ray (a) decides the right role. The normal ray (b) also decides, except at 10ε with ε 1e-9. There it is in band while the census calls the same pair definite. That is a falsifier of "reduces to pairs", but it comes from f64 rounding at the threshold, not from a wider band.

**Branch:** `flush/probe-shell-role-ray`, head `a135ca214`, based on main `8523894dd`. It adds one ignored test, `crates/topo/src/boolean/shell_role_ray_probes.rs`, as a child module of `solid_contain`, so it can call the private `aim_points`, `aimed_side` and `cast_ray`. The one other change is the 4-line `mod` declaration at the end of `solid_contain.rs`. Not for merge. **Time:** about 24 min (15:21 to 15:45 UTC), mostly two release builds.

**How the bodies were built:** each shell was built directly with `cube_into` boxes plus `fixtures::refile_shells`:
- the cavity: brick (−1,2)³ plus an inside-out slab [0,1]²×[0,t], refiled into one solid;
- the union: slab plus brick (5,6)³ refiled into one solid, each order as keeper;
- a lone slab.

The public `subtract` and `union` were also run on the at-rest operands.

## Table
Order gaps and residuals are in ε. "Census" is the `pm_census_vf_residual` decision on the two faces the ray crossed: the least vertex-to-plane residual of that pair, under `Band::linear`.

| case | t | ε | V/A read (`shell_role_read`) | (a) gate far point + `aim_points` | (b) along a face normal, from beyond reach | census on that pair |
|---|---|---|---|---|---|---|
| all four | 9.9ε | both | — | — | — | the slab does not build: the fixture refuses `interval_span_forward` (ParamSpan) with margin 9.9ε |
| cavity | 10ε | 1e-9 | refuses, `positive_volume`, −5.0e-9 | **In**, order gap 19.3–20.3 | **in band**, `bool_point_in_solid_order`, margin 9.99999994e-9 (just under Kε = 1e-8) | **definite apart**, \|r\| 10.000 |
| slab in either union, lone slab | 10ε | 1e-9 | refuses, +5.0e-9 (lone slab's `classify_shells` refuses as `chk_shell_volume_sign`) | **Out**, gap 20.3–21.3 | **in band**, order margin 9.99999994e-9 | **definite apart**, \|r\| 10.000 |
| all four | 10ε | 1e-6 | refuses, ±5.0e-6 | In for the cavity, Out for the rest | decides (In / Out), gap 10.000 | definite, 10.000 |
| all four | 12ε | both | refuses, ±6.0 (units of ε) | decides, gap 23–25.6 | decides, gap 12 | definite, 12 |
| all four | 15ε | both | refuses, ±7.5 | decides, gap 29–32 | decides, gap 15 | definite, 15 |
| all four | 19ε | both | refuses, ±9.5 | decides, gap 37–40.6 | decides, gap 19 | definite, 19 |
| all four | 20ε | both | refuses, ±9.9996 | decides; 6 aims, side faces now offer one | decides: gap 20 on the caps, 1e9 on the sides | definite |

- **Which predicate binds:** in every row the binding margin is the order gap. The levered denom, `bool_point_in_solid_denom`, is at least about 2e9ε throughout.
- **The real booleans:** `brick ∖ slab`, `slab ∪ far` and `far ∪ slab` all refuse `Escalated { ShellRole }` at every t from 10ε to 20ε, at both ε. The certified enclosure is wholly in band (`terminal_sliver: true`).
- **The at-rest gate:** the lone slab passes at every t. The brick with the slab cavity refuses `ShellRoleUndecided` (check 10).

## Band answer
- **The ray's band:** `Crossings::closest` (`ray_walk.rs:346`) decides `bool_point_in_solid_order` on `Margin::of(t_k − t_best)` under the band it is handed. `Band::linear` gives the band (ε, Kε). The gap is measured along the ray.
- **The census band:** `pair_vertex_face` (`census.rs` ~1725) decides `pm_census_vf_residual` on `Margin::of((q−o)·n)`. `census_and_certify` runs under `linear_band(tol)`, so this is (ε, Kε) too, measured along the normal.
- **Same width.** With K = 10, both bands are (1e-9, 1e-8) at ε 1e-9.
- **In terms of thickness t:** a ray at angle θ to the normal sees a gap of t/|cos θ| ≥ t. So the ray's band in t is (ε|cos θ|, Kε|cos θ|): never wider than the pair band, and equal to it along the normal. The (a) rays here sit at about 60° to the normal and see gaps of 2.0–2.1 t.
- **V/A's band is twice as wide.** V/A ≈ t/2, so its band in t is (2ε, 2Kε). Numeric check: at t = 12ε, V/A reads 6.0e-9 (in band) while the pair residual reads 12ε (definite).

## Cases that falsify "the ray is in band only where some pair is in band"
1. **The normal ray (b) at t = 10ε, ε 1e-9, on all four shells.** The ray reads the gap as 9.99999994e-9, just under Kε, so it is in band. The census reads the same top/bottom pair as 1.0000e-8, definite.
   - The cause is rounding, not a wider band. The ray's gap is a difference of two advances of about 3 m (the start is 2·reach + 1 out), so it carries about one ulp of 3 m of rounding. The census reads the vertex coordinate directly.
   - At ε 1e-6 the same pose decides.
   - So the claim holds only up to rounding in the advances, which grows with how far away the ray starts. An implementation would need an exactly-at-the-threshold allowance, or a start point near the shell.
2. **Nothing else.** The (a) rays never came back in band or undecided. The oblique incidence doubles the gap, and that keeps them clear.

## Curved case (not measured; read from the code)
- `aim_points` uses only `stands::planar_interior_point`, so a lens of two spherical caps offers zero aims. `aimed_side` then returns `Undecided { None, None }`, and `missed_side` falls back to the volume's refusal. A ray read would need a curved aim point before it could serve a lens at all.
- A thin spherical-shell cavity is two shells, each a whole sphere with V/A = r/3, so V/A is not thin there and reads fine.
- I did not build either body. The ball fixtures live in `sweep`, which cannot reach `topo`'s private ray functions.