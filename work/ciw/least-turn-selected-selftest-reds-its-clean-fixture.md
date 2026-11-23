---
id: least-turn-selected-selftest-reds-its-clean-fixture
kind: issue
title: least-turn-selected.sh --selftest reds its own clean fixture: CALLERS pins crates/editor-core/src/mate/value.rs, which the fixture does not plant, so the nightly gate-selftests job is red
status: open
opened: 2026-11-22
---


Found by the CIW gates-latency lane (2026-11-22) while taking the
`scripts/run-gates.sh --selftest` baseline on `7709b0eac4` (origin/main):
`least-turn-selected.sh --selftest` fails with `SELFTEST FAILED: the gate
FAILED on a clean fixture` / `crates/editor-core/src/mate/value.rs: pinned
at 1, names least_turn_selected nowhere`.

`scripts/gates/least-turn-selected.sh` `CALLERS` pins three files (the
home, `crates/sweep/src/skin.rs` and `crates/editor-core/src/mate/value.rs`,
the last added by `91a30948a4`), and a pinned file that names the
identifier nowhere reds; its `gate_plant_clean` plants only the first two.
The plain gate is green on the tree, so PRs do not see it; nightly.yml's
`gate-selftests` job (`run-gates.sh --selftest`) does.

The repair is the third caller in `gate_plant_clean`
(`crates/editor-core/src/mate/value.rs` calling
`RigidMotion::least_turn_selected`), and with it a look at whether the
other gates whose pin lists grew since their fixtures were written plant
every pinned file.
