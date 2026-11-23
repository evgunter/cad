PR 7573 — MAJOR (merge-blocking, DEMONSTRATED from hosted CI on the frozen head 24ee9a28fb, run 45189870448, job 137746716462 "lint (fmt, clippy, gates)"). Besides main's known NodeErrorClass red, two gates fail on code this diff adds, both in crates/mesh/src/join.rs `midpoint`:

1. scripts/gates/interval-square-allowlist.sh:
   `crates/mesh/src/join.rs:422: [theta + turns * period, nappe * Real::sqrt(x * x + y * y)]`
   `##[error]use powi(2): ... Convert, or ratify this file into the allowlist.`
2. scripts/gates/period-spelling.sh:
   `ROSE crates/mesh/src/join.rs midpoint: 0 -> 1`
   `##[error]a whole turn is spelled in production code outside its homes — read it from where it is stated (... Surface::angle_periods ...)` — the line is `let period = core::f64::consts::TAU / k.abs();` (join.rs:420).

The PR's Verification lists "mesh clippy with all features, fmt, work.py lint" locally and never names scripts/gates; it calls the lint job's red main's. That is not so: these two are this unit's own. Fixes look mechanical (powi(2) / hypot; read the cone's period from its home), but they move `midpoint`'s arithmetic, so the cone row and the shell row need re-running after them.

Other jobs on that run: test (kernel) is red only on a fast-set time bar, sweep::all shell8_r2_probes::r2_three_solids_on_one_sphere_chart_open_additively 1.056 s > 1.0 s, with 7632/7632 passed. That is not one of the three known reds the brief named. test (app) is red only on check-slow-set-names: "slow-set entry matches no test: viewer: app::properties_pane_tests::refused_slot_entry::the_range_buttons_grey_from_the_admission_door". Its tests passed: 606/606 plus 58/58 slow. The diff does not touch .config, so this looks like main's, but I have not checked it against main. The python suite and the mesh budget meter are green.
