# BOXFINISH model-compare draws, 2026-11-14

Protocol: e1c428efbb2. One /dev/urandom byte per draw: even = Opus, odd = Fable.

| unit | arm | byte | model | date |
|---|---|---|---|---|
| `cylinder-and-torus-edge-misses-are-read-back-about-the-axis` | implementer, letter M, single tier (FULL) | 136 | Opus | 2026-11-14 |
| `the-side-reader-cannot-read-a-solids-orientation-over-a-box` (the aimed ray) | implementer, letter M, dual tier (sequential) | 133 | Fable | 2026-11-14 |
| `check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends` (the chart ends) | implementer, letter M, single tier (FULL) | 164 | Opus | 2026-11-14 |
| `the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box` (the sweep doors keep their frames) | implementer, letter H, dual tier (concurrent) | 202 | Opus | 2026-11-14 |
| `the-profile-drops-the-unit-directions-its-verbs-made` | implementer, letter M, dual tier (sequential) | 122 | Opus | 2026-11-14 |
| `cylinder-and-torus-edge-misses-are-read-back-about-the-axis` (PR 6946) | reviewer, single tier M (FULL), frozen head e010b7d7ad20 | 145 | Fable | 2026-11-14 |
| `the-side-reader-cannot-read-a-solids-orientation-over-a-box` (PR 6958) | first reviewer, dual tier M (sequential), frozen head e51e52c99f6a | 215 | Fable | 2026-11-14 |

## Row notes (for the rows; kept off the unit branches until each row lands)

- **PR 6946, edge-turns.** Implementer: session_013QoYWYSv7TNyVMatNJygov, configured claude-opus-5-5. Dispatched 2026-11-14 03:26 UTC; PR opened 05:55; first pushed head with CI: b9c4c937 (kernel red on the two inherited ε 1e-6 rows only; an earlier head failed lint on `raw-measure-door-sites`, re-pinned in b9c4c937); INTERRUPTED 06:45–16:33 UTC by the account usage outage while polling CI (nothing lost, per its report); report delivered 17:02 on head e010b7d7 (main merged). Hosted CI on e010b7d7: lint, test (app), mesh green; test (kernel) red on the same two inherited 1e-6 rows; the 1e-12 bead row absent from the failure list, where main's own run (PR 6928) listed it. Reviewer: session_013QoYWX5YSpdCXb6hSs38mc, configured claude-fable-5-2, dispatched 17:04 UTC on frozen head e010b7d7ad20.
- **All four units dispatched before 06:40 UTC** (edge-turns, aimed-ray, chart-ends, sweep-frames) had their implementation phase interrupted by the same outage (about ten hours of wall time not the agent's); profile-directions was running across it.
- **PR 6958, aimed-ray.** Implementer: session_013QoYWYPCrikZXnZMyUm3oa, configured claude-fable-5-2. Dispatched 03:50 UTC; INTERRUPTED 06:40–16:33 UTC by the outage (temporary edits discarded, nothing lost, per its report); PR opened 17:05; report delivered 17:38 on head e51e52c9, `gate ok` green on that head (first PR head with CI: to confirm from the PR's runs at row time). First reviewer (drawn): Fable, dispatched after this note.
