---
id: a-lone-slide-refuses-the-chart-it-never-reads
kind: issue
title: A value's chart refuses at the antipode band even for a lone slide that never reads the swung axes
status: open
opened: 2026-10-18
priority: P3
cost: E
---


Found by the second review pair on PR #5267 (R4, style, `likely`), left unfixed in that PR.

**What.** A value is charted on each body's frame swung onto the mated direction. `Chart::of` (`crates/editor-core/src/mate/value.rs:71`) builds the whole chart for any mate a value names: both swings (`reference_about` on the target's direction and on the copy's), each refusing `PoseDegenerate { Chart }` when its direction sits in the antipode band. A value that reads no swung axis still pays for them: a lone `Slide(Z)` on an axis mate slides along the mated direction itself, and a slide with no spin beside it reads only the target's swing. Either refuses whenever the copy's own direction (or, for `Slide(Z)`, either direction) is within the band of its antipode.

**Why it was left.** The spec says "the chart refuses" (INTENT stage 3 §1, Values; ruling C-1), and the refusal is honest: the band cannot decide which way the swing goes. The cost is a refusal a value did not need, at a measure-zero set of poses.

**To decide.** Whether a value's chart is built lazily, swinging only what its coordinate reads (`Slide(Z)` neither swing; `Slide(X|Y)` the target's; `Spin` both), so a refusal names only the swing the value depends on. Belongs with F's measure-free door, which re-states when a value's refusal is decided.
