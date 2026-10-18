---
id: the-slide-charts-indeterminate-arm-is-unreachable
kind: issue
title: MateFault::Indeterminate and PoseDegenerate{Chart} in Translations::slide are unreachable since F: no case split remains
status: closed
opened: 2026-10-18
closed: 2026-10-18
priority: P3
cost: E
---

Found by the review of INTENT stage 3 F (PR #5299, finding 10).

F retired the measured fold's case splits. The only remaining producer of `MateFault::Indeterminate` and `NodeErrorKind::PoseDegenerate { Chart }` in the value solve is `Translations::slide`'s `coset::across` (`crates/editor-core/src/mate/value.rs:287-309`). That call crosses the chart's `x` with its normal, and the two are perpendicular by construction, so it cannot fire.

`MateFault::Indeterminate`'s doc and Display text in `crates/editor-core/src/mate.rs` still read "a case-split predicate" and "a case split could not be decided", but no case split remains.

**The work.**
1. Mint the slide's left line structurally from the chart, and retire the arm.
2. Then retire `MateFault::Indeterminate` if nothing else produces it, restating its tag, class and Python binding with it.
3. Re-baseline any kstats that move.

## Closed (2026-10-18)

The premise holds, and a row now pins it:
`mate::solve::tests::no_admitted_bundle_sets_slides_on_two_mates`. A slide is
its own only beside mates that free every translation, which only a direction
mate does, and a direction mate beside any mate of any kind is refused. So
every slide a bundle sets is on one chart, along its mutually perpendicular
`x`, `y` and `d`.

`value::settle` therefore moves each slide along its own axis by its gap. The
`Translations` machinery and its `coset::across` mint are retired; `across`
had no other caller.

`MateFault::Indeterminate` had no other producer, so it is retired too: its
class `MateIndeterminate`, its tag `mate_indeterminate`, and its binding's
escalation fields (`predicate`, `margin`, `margin_low`, `margin_high`, `zero`,
`escalate`). The frame door's `FrameError` keeps its own. `settle` and
`solve_bundle` no longer take an arm or a band.

**What moved.** `msolve14`'s `MAIN_CORPUS_DIGEST` is re-baselined at all three
ε:
- 1e-9: `0x4096_0556_5341_7a5a`
- 1e-6: `0x13dd_df97_d118_d715`
- 1e-12: `0x36dd_4194_48e8_c0d4`

A slide's move is `axis · gap` instead of the residual's projection of the
axis, and the two agree to rounding. Every pose-asserting row in the crate
still holds at its tolerance. `msolve11`'s decision-log row is restated,
because no slide mints a line any more.
