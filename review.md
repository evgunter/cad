# Review — PR #3853 (`reach/dev-probe-red`, head `32b84e46`)

**Verdict: APPROVE-WITH-FIXES.** Every claim survived execution. Geometry is unchanged, and so is every decision except the three margins. The fixes are one false sentence on the new door, which its own unit test contradicts, and a miscount in a filed item. Hosted CI run 37037630510 is green on exactly `32b84e46` (I resolved it myself).

## By execution
- **C3, no geometry moves (sure).** I ran `k_probe_sweep.sh` on the head and on main `2ea5c9b9`. On main I had to stop the plain loop aborting, or it writes no CSVs. Then I diffed every row (`probes/review_3853_kdiff.py`) across the corpus+demo, M2 and driver dumps at ε 1e-6/1e-9/1e-12, about 4.4M rows. **0 rows differ** outside the margin column of the three names: 1461 germ, 144 cylinder and 59 torus rows per ε, plus 6 torus rows in M2. All stay `positive`. The new margins run from 0.031 to 5.06 m (minimum: `demo/heatsink` cylinder at 0.031). My own pipe-bores-plate poses (`probes/review_3853_arm_poses.rs`) give the same verdicts and face counts on both trees for sub/uni/int, 1e-3/1 scale, far sketch origins and sub-band radii.
- **C1, the door (sure).** `probes/review_3853_levered_door.rs` sweeps |v| ∈ {0, subnormal, 1e-300 … MAX, ∞, NaN} against arms {−1, 0, 1e-13 … 1e300, ∞, NaN}.
  - Wherever both doors certify, the normalized bits are identical.
  - The finiteness and underflow gates fire as in `new`. A NaN arm escalates; an ∞ arm with a zero vector gives NaN and escalates.
  - **Two departures from `new`.** (a) A *unit* vector is refused `Degenerate` once the arm is ≤ ε. Arm 0 or a negative arm is refused too. (b) An arm over 1 m certifies vectors `new` refuses as `Degenerate`: at ε 1e-9, |v| = 1e-11 passes with arm 1e3, and 1e-14 passes with arm 1e6.
  - Neither departure is reachable through the public paths I tried. Profile refuses every pipe radius < 2e-8 at ε 1e-9 (sweep 5e-11 → 2e-8), so the cylinder arm stays at or above twice the escalation threshold.
- **C2, the arms (likely).** The measured arms are the radius (cylinder), R+r (torus; `4·scale` in the pin) and the two-site ball's reach (germ: 1.0, 1.207, 1.415 on my poses). Moving the sketch origin to 1e3 or 1e6 changes nothing: extrude re-anchors the cap plane, so I could not build a far-origin carrier through the public API. None of the arms over-states the reach. See the germ NOTE.
- **C4, the ring row (sure).** It prints `[0.9,0.9]`, `[0.4,0.9,0.9]` and `[0.408,0.408]`, which matches the closed forms read off `surgery.rs:2667`. The drop-the-mates mutant (`.take(0)` on the mates chain) reds it with `[0,1,2]` against `[2,3,2]`.
- **C5, the pins can fail (sure).** Each `T::one()` mutant reds its own row:
  - germ: `bool_germ_plane_normal (worst rel dev 9.990e-1)`;
  - cylinder: ten margins of `1e0`;
  - torus: `1e0` against `4e-3`.
  
  Renaming the germ predicate reds `rim_dim_boolean_twins.rs:278` with "pin vacuous".
- **C6, the CI diagnosis (sure).** Nightly job 110813608440 ran on main `1b802294`, and its sweep step failed at `r1_ring_clearance…` (2 vs 0), so the lint steps were skipped. With the plain loop made non-aborting, main has exactly two hidden reds: that row and `topo::rim_dim_boolean_twins`. k-lint on the head's CSVs prints "75 margin(s)": 27 `chart_bound_outer_span`, 8 `bool_circle_torus_root_slack`, and the lily-wall clearance. C3 makes those rows identical on main, so the flags are main's and the CLEAVE filing is right in substance. One count is wrong (MINOR 2).
- **C7, suites (sure).** `nextest --profile ci` on geom-core, geom-brep, topo and sweep passes 5601/5601 at 1e-9, 1e-6 and 1e-12. `k_probe_sweep.sh` exits 0. At 1e-12, `rigid_map_near_eps_plane_nurbs::the_certificate_re_derives…` reds when selected (a slow-set test outside the ci profile). That red is the known one.

## Findings
- **MINOR 1 — the door's safety sentence is false** (`geom-core/src/linalg/unit_vec.rs:381-383`; PR body: "a shorter arm only escalates"). Shown by execution. The doc says an under-stated arm "escalates more and never decides a length the full reach would not". In fact an arm ≤ ε decides `Degenerate` on a unit vector, and the PR's own row `ask(1e-10) == Degenerate` (`:430`) pins exactly that. The claim that holds is weaker: a shorter arm never certifies *positive* where the full reach would not. In `join.rs` a spurious `Degenerate` would surface as the lie "a broken plane carrier". It is unreachable on admitted bodies today (C1), but the text is what future callers will rely on.
- **MINOR 2 — the filed count conflates flags with margins** (`work/cleave/lily-walls-curved-clearance-crowds-the-band-under-k-lint.md`, title and block). Shown by execution. The 46 is a count of FLAG rows; they sit on **40** distinct margins (12 at 1e-6, 14 at 1e-9, 14 at 1e-12). Six margins carry two flags each, so 27+8+40 = 75, while 27+8+46 = 81.
- **NOTE — the germ arm is a proxy, not the consumed reach** (`topo/src/boolean/join.rs:447-458`). Inspection. The lanes consume the normal as `(mid − origin)·n` at between-edge midpoints (`chord_join.rs:1848`), which can lie outside the two-site ball. The arm can therefore only under-state (conservative per MINOR 1's weaker claim), but the doc text calls it the reach "the direction is consumed over".
- **NOTE — the witness weakens for long arms** (`unit_vec.rs:8-10` against C1(b)). Execution. A `UnitVec3` can now carry a vector whose norm is below ε when the arm is over 1 m, while the module header still says its "length DECIDED positive under a band".
- **NOTE — the site lookup is eager** (`join.rs:447-458`). Inspection. It runs for every germ pair before the kind match, so pairs that never call `germ_normal` (the typed C5 refusals) gain a new `desync` door that answers first.
- **NOTE — the ring row doesn't assert the dome rim's margins** (`review_ring_clearance_r1_probes.rs:543-553`). Execution: they read 0.408 each, and only the count pins them.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6; Q7 in passing; Q8 not done for `join.rs`, read only around the diff)
- Q1, **sure**: the half-edge → start-vertex → point chain is now written a fourth time in topo: `join.rs:448-452` repeats `shell_witness.rs:309-311`, `offset_nappe.rs:172` and `merge_faces.rs:1011`.
- Q1, unsure: `levered` takes a bare `T` arm, while editor-core already has a typed `Arm` (`mate/coset.rs:378`). Two spellings of "the lever".
- Q1/Q7, unsure: `new` has a free twin, `decide_unit_direction`; `levered` has none. The asymmetry will invite the next caller to reach for `norm3`.
- Q2, likely: MINOR 1 is a comment carrying a guarantee the code does not have.
- Q3: the pins red under their mutants (C5). The cylinder pin's `!is_empty()` check plus the per-margin equality is sound.
- Q4/Q5, sure: the audit rows and K-REPORT were updated in step. K-REPORT's "none lands near the band for a carrier the at-rest rule admits" holds on the corpus (minimum 0.031 m).
- Q6, sure: the masking diagnosis's "owed" (a PR-gate probe leg, a loop that collects every red) sits in an existing CIW item rather than a new schedule. That is acceptable.
