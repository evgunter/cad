---
name: local-battery-scope
description: Implementers run only touched-crate local checks; the full matrix is hosted CI's job on the PR
metadata:
  type: feedback
---

Do NOT run the full CI-equivalent battery locally (Evan,
2026-07-30: "we should not be running the same tests as CI
locally"). The full-local-battery discipline dated from the
Actions-budget outage era; hosted CI is restored, tier-aware, and
is THE gate.

**Why:** duplicating the matrix locally costs hours of wall-clock
(two feature lanes × whole workspace on a 9G machine) for zero
added confidence — the PR gate re-proves it all anyway.

**How to apply:** implementer/finisher prompts specify local
checks as: `cargo test -p <touched crates>` in the lanes the
change is relevant to, `cargo fmt --all --check`, clippy on
touched crates (default lane). Everything else is left to CI on
the PR, and the report says so. The dependency-closure filter
(scripts/ci-filter.py, ci-local.sh) is the reference for what
"touched" means when in doubt. Full-workspace local runs only on
explicit request (e.g. debugging a cross-crate bit-replay
failure). Related: [[resume-vs-fresh-subagent]],
[[cad-working-style]].

**Reaffirmed under pressure (Evan, 2026-08-01, PR #152's triple
red):** after a gate cycle caught two lint rounds + a real
Interval divergence that the narrowed battery missed, the
orchestrator proposed standing pre-push CI-row mimicry (full
workspace clippy both feature sets). Evan declined: "i don't
want it running ci before push in general — this seemed like it
worked fine, while doing ci locally was extremely slow." The
red→fix→re-push loop IS the design; targeted local reproduction
of a KNOWN gate failure remains right, standing mimicry does
not.

**Degenerate case (Evan caught it live, 2026-08-01):** for a
CROSS-CUTTING mechanical change (a sweep touching most crates,
e.g. the interval-square conversion), "touched crates" ≈ the
whole workspace and the narrowed rule silently becomes local CI.
For bit-neutral mechanical sweeps the local scope is: workspace
cargo check + the unit's own self-test + ONE spot suite per
class of converted site, default ε only — CI proves the matrix.
Put this scope in the brief whenever dispatching a sweep-shaped
unit.
