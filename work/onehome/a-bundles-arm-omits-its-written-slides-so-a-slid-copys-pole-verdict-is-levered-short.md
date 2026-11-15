---
id: a-bundles-arm-omits-its-written-slides-so-a-slid-copys-pole-verdict-is-levered-short
kind: issue
title: A placement's arm omits its written slide values, so the least turn's pole verdict for a copy slid L is levered over its reach alone and the copy lands about L·u/δ off inside the admitted band
status: open
opened: 2026-11-15
priority: P1
cost: E
---


Found by fork 270's designers (pole conditioning, `analysis/design-fork/pole-conditioning`, approved in PR 6964) and recorded as owed in `work/onehome/log.md` (2026-11-14, fork 270's entry).

**What the arm is.** `bundle_arm` (`crates/editor-core/src/mate/solve.rs`) forms the length a placement's angle verdicts are levered over: the copy's reach in its own coordinates plus, over the bundle's mates, the farthest its two poses sit from their coordinates' origins, through `Arm::of` (`crates/editor-core/src/mate/coset.rs`). Its one caller, `crates/editor-core/src/eval/wire.rs` (the bundle's chart), hands it to `Chart::of` (`crates/editor-core/src/mate/value.rs`), whose pole verdict (`mate_chart_pole`) levers the chord from the pole by it.

**What it omits.** A placement's written slide values. A slide runs along one of the target's axes as the least turn carries it (`[placement.values]`, `docs/DESIGN.md`; `Chart::slide`), so a copy slid `L` sits up to `L` past the reach `bundle_arm` sums, and its points ride the carried axes over that whole length. Near the pole those axes turn by about `u/δ` under a direction's rounding (`work/onehome/a-mates-carried-axes-near-the-least-turns-pole-are-conditioned-u-over-delta.md`), so the slid copy lands about `L·u/δ` off, while the verdict that admitted it was levered over the reach alone.

**Why it matters.** `[placement.values]` (fork 270, Ev 2026-11-15) states the band's width as the chord "levered by the bundle's arm (written slides included)". With the slides left out, a slide long against the copy's reach admits a chart the band should refuse.

**Owed.** `bundle_arm` sums the bundle's written slide values (their magnitudes, at their upper bounds at a lane scalar) into the arm, and a row pins a slid copy's verdict moving with its slide. Done with the decided least-turn mint (fork 270's Decision 3).
