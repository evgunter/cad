---
id: interval-square-gate-reads-a-path-prefix-as-a-square
kind: issue
title: interval-square gate reads `f64 * f64::EPSILON` as a square
status: closed
opened: 2026-10-20
closed: 2026-10-26
---


`scripts/gates/interval-square-allowlist.sh:213` (and the same tail on
`:214`, `:240`–`:242`) ends `SQUARE_RE` with the lookahead
`(?![\w.(\[])`. That excludes a following `.`, `(` or `[`, but not `::`.
So `f64 * f64::EPSILON` matches as `f64 * f64`, a "square".

Found by NURBS build A on #5477 (`crates/geom-brep/src/curve_pair/pose/window.rs`,
`up()`): the gate flagged `f64::from(n + 1) * f64::EPSILON`, and A had to
reorder it to `f64::EPSILON * (N + 1) as f64` to pass. It fails loud, so
nothing slips through. But a false positive teaches lanes to reshuffle
correct code to please the gate.

Fix: add `:` to the lookahead's excluded set (`(?![\w.:(\[])`) on each
square regex, with a gate self-test row for `f64 * f64::EPSILON`.

## Closed (2026-10-26, tracker sweep)

**Fixed in this sweep, as the row specified.** All five square
lookaheads in `scripts/gates/interval-square-allowlist.sh` (`SQUARE_RE`
:213–214 and the three `CENSUS_*_RE` at :240–242) now read
`(?![\w.:(\[])`, so a factor followed by `::` is not a square.
`plant_not_squares` gains `n as f64 * f64::EPSILON` (:620) under the
passes case (:878). Mutation-checked: with `:` removed from `SQUARE_RE`
the selftest reds on that fixture. The plain gate passes with the
census register unchanged (0 + 0 + 0 + 5 candidates).
