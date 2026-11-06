---
id: polish-bound-spells-unit-and-normalises-n-by-hand
kind: issue
title: polish::bound spells unit as a closure beside pose.rs's fn unit and normalises n by hand; past_by's disagreement refusal borrows the straddle's row
status: dispatched
opened: 2026-11-06
priority: P3
cost: E
refs: [6537]
branch: onecopy/curve-pair-tidy
---


Found by PR 6537's delta review, after the polish-meters fix pass.

## What

1. **`unit` is spelled twice.** `crates/geom-brep/src/curve_pair/polish.rs` `bound` (about :117) spells `unit` as a closure, beside `pose.rs:617`'s `fn unit`. It then normalises `n` by hand again (about :125). That is a two-line repeat of a function that already has a home. Route both through the one `unit`.
2. **A borrowed row.** `curve_pair.rs` `window_off`'s `(Ok(_), Ok(_), None)` arm (about :517-520) refuses `Meter { row: "curve_pair_meet_window" }` where `contact` and `place` disagree at rounding. That is the straddle's row, but this arm reads no straddle. Give it a row that names what disagreed, for example `curve_pair_span_end_chord`. Re-run the audit and roster gates.

The arm is reachable only at rounding: 0 calls in both heads' census at either ε.
