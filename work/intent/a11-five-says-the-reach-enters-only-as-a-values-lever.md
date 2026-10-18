---
id: a11-five-says-the-reach-enters-only-as-a-values-lever
kind: issue
title: ASSEMBLY A11 (5) says a copy's reach enters only as a value chart's lever; it also feeds rest_verdict and the unobservable report
status: open
opened: 2026-10-18
priority: P3
cost: E
---

Found by the review of INTENT stage 3 F (PR #5299, style S7). The imprecision predates F.

ASSEMBLY A11 (5) (`crates/editor-core/ASSEMBLY.md`) says a copy's reach "enters only as the lever a value's chart is decided over". The reach also feeds the at-rest verdict (`rest_verdict`) and the unobservable report (`crates/editor-core/src/mate/unobservable.rs`, which calls `coset::turn_line` with the arm).

**The work.** Re-word A11 (5) to name every reader of the reach, or move those readers onto a lever of their own. The re-wording describes code as it stands, so it lands with whatever change prompts it.
