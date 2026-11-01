---
id: main-red-a-prism-tilted-within-the-zero-escalates-at-the-sector-reach-since-arm-vs-reach
kind: issue
title: main red: a prism tilted within the zero escalates at bool_sector_within instead of building as upright, since PR 6053 read sectors at the reach (topo three_solids_on_one_line, every ε)
status: closed
branch: splitdoor/prism-tilt-reach
opened: 2026-10-30
priority: P1
cost: M
refs: [arm-readers-left-at-the-sector-arm-after-the-reach-reading]
closed: 2026-11-01
pr: 6212
---


**What fails.** `topo::all` `three_solids_on_one_line::a_prism_tilted_within_the_zero_builds_as_upright` (`crates/topo/tests/three_solids_on_one_line.rs:468`, slow set) is red at every ε in nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467) (main `39bbdfc777`), and was green on 2026-10-29 (run 40823227622, `5648e577eb`). At default ε:

> the 240° prism tilted 0.2ε towards 25°, and the plate, member order [3, 2, 1, 0]: step 1 refused: `Escalated { decision: Coincidence(Sectors, Moot), diag: Indeterminate { margin: 1.3898e-9, band: { zero: 1e-9, escalate: 1e-8 }, predicate: "bool_sector_within" } }`

A tilt inside the zero band must build exactly as the upright prism does. It now escalates at the sector test.

**Attribution (bisected).** `git bisect run --first-parent` ran this row alone at default ε, over `5648e577eb..39bbdfc777`. It passes at `0c58bd6e`, which is main after `ed7e40b`, and fails at `712dadf2`, the merge of PR 6053 (`splitdoor/arm-vs-reach`: read a direction's side of a bound at the bound's reach, not the sector's arm). The margin 1.39e-9 sits just over the 1e-9 zero, in the escalation band. A likely mechanism, read but not verified: the reach reading in `crates/topo/src/boolean/sectors.rs` levers the tilted prism's sector over a longer run than the arm did, so a within-zero tilt now reads as in-band. The row is in the slow set, which the PR gate does not run.

**Reproduce.** `cargo nextest run -p topo --test all --run-ignored all -E 'test(=three_solids_on_one_line::a_prism_tilted_within_the_zero_builds_as_upright)'`

**What closing it takes.** Decide what lever the reach reading should carry for a within-zero tilt, so the tilt decides as upright, or explain why the contract no longer holds and re-state the row. `arm-readers-left-at-the-sector-arm-after-the-reach-reading` is the neighbouring work.

## 2026-10-30 (GREEN, lane `green/nightly-77-pins`)

Pinned: `a_prism_tilted_within_the_zero_builds_as_upright` runs under `test_utils::pinned_red::row` at every ε (`EVERY_ROW`). It must stop on the 0.2ε-towards-25° case with the plate, order `[3, 2, 1, 0]`, step 1, `Escalated { decision: Coincidence(Sectors, Moot)` at `bool_sector_within`. That is what main `2e1c1bff` does: margin 1.3898e-9 at default ε, 1.3906e-12 at 1e-12, and the same stop at 1e-6. Un-pinning owes a re-measure of everything past that stop: the remaining orders of that case, 85° and 265°, and the prisms-alone folds. The nightly has not run them since PR 6053.

## 2026-11-01 (SPLITDOOR, `splitdoor/prism-tilt-reach`)

Root cause: the tilted 290° face meets the 120° prism's upright face 10° away, so `pair_search` read their line as `(n_a × n_b).normalize()`, which divides the tilt by `sin 10°`. At the −z bound's reach of 1.23 m that line read 1.39ε off, though the bound's far end lies within the zero of both planes. The short arm hid the same factor before PR 6053. Fix (orchestrator ruling): two faces' line is read un-normalized (`sectors::face_line_rays`), so each reading is the bound's far end's offset from the other face's plane, which is what its code reads. It applies at `pair_search`, `insert.rs` `germ_dir` and `vtxfac::pierce_germ_dir`'s membership check. The row is un-pinned.
