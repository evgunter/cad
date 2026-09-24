---
id: tour-walk-runs-the-certified-narrations-in-the-render-and-budget-lanes
kind: issue
title: The tour walk runs the certified narrations in the render lane and the tess-budget sweep since RING-4, minutes per walk
status: open
opened: 2026-09-24
priority: P4
cost: D
refs: [ring-4-interval-feature-dropped]
---

## What

`demos/tour/src/main.rs`'s walk narrates two certified cells —
`tolerance::narration` (the two-hole plate) and `chaintol::narration` (the
chain, one E6 drive per link count). Until RING-4 both sat behind the
`interval` feature, which only `ci.yml`'s `demos tour suite` row passed, so
every other tour walk skipped them. RING-4 deleted the feature and the walk
now runs them everywhere it runs: the kernel render lane
(`render.yml`, `cd demos/tour && cargo run --release -- ../out`), the
`k-lint (gate, release-budget)` leg's tessellation-budget sweep
(`scripts/tess_budget_sweep.sh`, the `tess-budget` mode walks the tour), and
`local-scripts/ci-local.sh`'s rows that walk it.

The narrations print; they draw no scene and write no CSV row, so no frame,
`scenes.json` or `docs/tess-budget-data/` row moves (RING-4 re-took the
sizing sweep at its base and its head: bit-identical). What moves is wall
clock. One local reading, release profile, 4 vCPU, light load: the
`--sizing-only` sweep's run step went from ~9 s at the base to ~9 min at the
head, the build excluded. The `demos tour suite` row always ran them (three
`eps_regression` walks, feature on), so `k-lint (gate, release-default)` does
not move.

The question for CIW: whether a walk whose subject is scenes (the render
lane, the budget sweep) should narrate at all, or whether the narrations'
coverage belongs to the `demos tour suite` rows alone — a tour CLI switch, or
a render/budget mode that skips non-scene cells. Hosted readings of the two
steps on RING-4's PR run are the first measurement of record.

## Evidence

- `demos/tour/src/main.rs`: the two `narration(tol)` calls in the walk.
- `scripts/tess_budget_sweep.sh`: `cargo run --release --features budget --
  tess-budget`.
- `.github/workflows/render.yml`: the kernel lane's tour step.
