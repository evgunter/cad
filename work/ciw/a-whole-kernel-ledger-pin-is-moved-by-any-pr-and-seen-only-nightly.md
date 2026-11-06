---
id: a-whole-kernel-ledger-pin-is-moved-by-any-pr-and-seen-only-nightly
kind: issue
title: A whole-kernel ledger pin in the slow set is moved by PRs that never run it, and the nightly that does is red in five jobs, so the move lands on main unseen
status: open
opened: 2026-11-06
priority: P2
cost: M
---


Filed by `onecopy/main-reds-k-and-m10`, at the ONECOPY orchestrator's direction: the policy question below is CI's, not that PR's.

## The class

A test that pins what the whole kernel builds or decides on a document — a walk ledger, a digest, a census count, a K-margin scaling claim — is moved by any PR that changes anything the document meets. When it costs ≥ 0.5 s it sits in the `ci` profile's slow set (`.config/nextest.toml`), so the PR gate never runs it, and a `gated_to!` marker cannot route it: no path list names everything the document meets. Only the nightly sees it, and the nightly has been red every night since at least 2026-11-01 (run 83: rustdoc, gate self-tests, `--all-features`, `k-lint (dev-probe)`, `k-lint (release-budget)`, the full suite at two ε), so one more red there reads as noise. A lane re-pins days later "at what main measures, not explained".

## Measured instances (bisected first-parent over main, 2026-11-06)

| row | moved by | what moved |
|---|---|---|
| `m10_sym_profile_interval` plate ledger + `Plain/Report` | PR 6504 (`143d99ec29`) | every plate line; `Early/Decision` 396 → 412 calls; blocked residuals 66 → 94 |
| same | PR 6527 (`3fef8d7ade`) | `Plain/Decision` and `Early/Decision` 9581 forms fewer each |
| `ray_wall_margin_twins` (probe) | PR 5781 (`4e0ef4858b`), PR 6349 (`096817df51`), PR 6482 (`a6f0b5e99c`) | 62 → 32 pairs; then the pin passed; then two new non-scaling predicates |
| `m4_pr8_k_probe` corpus dump (probe) | PR 6341 (`449f6f6097`), PR 6376 (`47b0763a4a`) | probe/f64 key splits off their pin; 116 (now 138) `<unnamed>` samples |

Neither of the m10 movers touched a path the suite's `gated_to!` named (both are geom-brep pcurve incidence), and all three probe rows ran only in `k-lint (dev-probe)`, whose compile step had itself been red since `72f10792` (2026-11-02).

## Reported to this row by another lane (origin/main ~`c80a1e8c`), not bisected here

- `editor-core` `m10_bulge_interval` ×2: `pcurve_chart_radial_moving` reads `[22, 20]` → `[18, 24]`.
- `editor-core` `sym11_the_exact_channel_never_contradicts_past_the_ceiling`: `r2_filleted_bracket` 1325 vs 1313, `r2_link` 4104 vs 5161.
- `scripts/gates/loop-boundary-discards.sh --selftest` fails on case `plant_extra_site census.rs face_cycles wrapped-return` (`work/guard/loop-boundary-discards-selftest-fails-on-a-clean-tree.md` is closed and was a different case).

(The needle-cavity and `seat7_sweep_lowering` reds from the same report already have open rows on FLUSH and GREEN.)

## Options

- **A. Run such rows on the PR gate** whatever they cost. Catches every move at its PR; breaks the slow-set policy (`check-fast-set-times.py`). The two m10 rows are ~13.5 s locally.
- **B. Status quo**: nightly only, a re-pin lane after each mover.
- **C. Widen `gated_to!`** to the kernel's sources. Does nothing for a slow-set row, which no PR runs whatever its marker says.
- **D. A fast sentinel per ledger**: a sub-0.5 s row over the cheapest document the ledger covers, ungated, on the PR gate. Taken for m10 in that PR (`the_forms_the_slabs_walks_build_are_pinned_per_eps_row`, 0.32 s locally); it covers what moves the slab and not the plate's curved walls — it would have stayed green on both 6504 and 6527, which moved the plate alone.
- **E. A nightly that is green**: the slow set's reds are only invisible because the nightly is red elsewhere. A nightly that reports each *new* red against the previous night's set (and pings) would make B work.

The question for CIW: which of A/D/E is the rule for a ledger pin, and does it go in `.config/nextest.toml`'s header beside the 0.5 s rule.
