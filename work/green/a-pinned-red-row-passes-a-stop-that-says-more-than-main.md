---
id: a-pinned-red-row-passes-a-stop-that-says-more-than-main
kind: issue
title: pinned_red::row holds a stop by containment only, so a stop that says more than main's (a further failing line, an elided number grown) keeps the pin green
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Found in the STYLE review of PR 6934 (`green/eps-1e-6`), with probes on branch `green/eps-1e-6-review` (`crates/test-utils/tests/pinned_red_pin_reach.rs`, `38b8a3053`); they are not on main.

`test_utils::pinned_red::row` (`crates/test-utils/src/pinned_red.rs`) passes at a pinned ε when the row's panic message CONTAINS every piece of `says`. Nothing holds the message to those pieces, so two shapes of pin stay green while main gets worse:

- **A joined list.** A row that collects its failures and panics with all of them (`bad.join("\n")`) is pinned on the lines main prints. One more failing line, a new wrong answer, still contains them.
- **An elided number.** A pin that stops at `pad=` or leaves out a margin holds nothing about the number, which may be the one that makes the line a failure.

PR 6934 had both, on a P0 row, and fixed its own: `parallel_cylinder_join`'s two rows now print and pin their count, the three cylinder-pair pad pins carry the pad's digits, and `revalue_boss_union` pins both flips' margins. The helper is unchanged.

**The class.** 34 call sites at PR 6934's head (`grep -rn 'pinned_red::row' crates --include=*.rs`). Each whose row panics with a joined list, or whose `says` cuts a number, is a candidate; they are not read here.

**Owed.** Either the helper takes the whole stop (an exact message, or pieces plus a statement of what may follow them), or each call site of the two shapes anchors a count and its numbers. Then the review's four probes become rows that hold it. A guard that cannot go red is P3 (`work/README.md`, "A guard does not inherit the band of what it guards").
