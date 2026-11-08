---
id: a-chains-no-two-in-a-row-rule-compares-read-axes-by-identity-not-by-line
kind: issue
title: the chain's no-two-in-a-row-about-one-axis rule compares a read axis by its read, so two reads of one line, or a read along a target axis, pass as distinct links
status: open
opened: 2026-11-02
priority: P4
cost: M
---


Found by the PR 6394 dual review, filed by its fix pass. Traced, not run.

`mate::solve::written` (`crates/editor-core/src/mate/solve.rs`) names two chain links "in a row about one axis" when their `Coord`s are equal. `Coord::SpinAbout(read)` compares by its read, so these pass as distinct links that turn three times:
- two reads of one line (an edge's axis and the meet of its two faces);
- a read along a target axis beside a spin about that axis.

In truth such a chain turns fewer times than counted, so the bundle's pin is overcounted and its read-back is singular.

**Owed.** The parallel-links check decided at evaluation, typed (`mate/value.rs`, `chain`), as the read slides' independence is (`NoChartWhy::ReadSlidesDoNotSpan`), or a ruling that a degenerate chain is a gimbal pole like the target-axis chain's.
