---
id: the-walks-polish-drops-its-steps-at-interval
kind: issue
title: The curve-pair walk's polish drops its steps at Interval and keeps them at f64: F35 counted 17,883 drops at Interval and none at f64
status: open
opened: 2026-11-09
priority: P3
cost: E
---

Filed from `onecopy/census-rosters` (row `the-least-band-gates-door-list-is-a-hand-copy-of-k-stats-doors`, its last bullet).

**The measurement.** PR 6602 instrumented `curve_pair::walk::step_taken` (ledger row F35, `k_stats::choose` on `16·slack − moved`) across the `geom-brep` and `topo` suites: 17,883 dropped polish steps at `Interval`, every one decided, and none at `f64`.

**What it overcounted.** `curve_pair::polish::pair` used to `continue` past a step its `keep` refused. The iterate had not moved, so each later pass recomputed the same step and was refused again, up to `STEPS` (4) times. `onecopy/census-rosters` makes that a `break`, so a refused step is now counted once. The distinct drops are therefore between about 4,500 and 17,883.

**The question.** At `Interval` the polish seems to keep almost none of its steps, where at `f64` it keeps them all. That suggests `moved`, an enclosure at `Interval`, sits above `16·slack` for a reason the geometry does not give, perhaps the iterate's width again (`curve-pair-polish-at-interval-reads-the-iterates-width`, closed by PR 6516, fixed the same shape on `pose`'s meter). If so, the walk's candidates at `Interval` are its seeds, unpolished. That is sound, because the window read at an unpolished point escalates rather than loses a meeting (F35), but it costs escalations a polished point would not.

**To close it:** re-measure after the `break`, then read one dropped step's `moved` and `slack` at `Interval` against the same step at `f64`.
