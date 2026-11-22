---
id: span-past-by-reads-a-refused-place-as-not-past
kind: issue
title: Span::past_by reads a refused place as not past, so an end straddle's escalation refuses as window_off's Meter disagreement
status: open
opened: 2026-11-16
priority: P2
cost: E
---

Found by the walk-stop fork's designers (2026-11-16); checked against main
`2c17deabf`.

`Span::past_by` (`crates/geom-brep/src/curve_pair/span.rs:578`) opens with

```rust
if !self.ended || self.place(q, near, band) != Ok(Place::Past) {
    return PastBy::Not;
}
```

so a refused place reads as not past. `place` refuses an end straddle as an
escalation (`span.rs:556`, `curve_pair_span_end_straddle`). In
`window_off` (`crates/geom-brep/src/curve_pair.rs:637`), with the floor
and the gap both decided and neither side `Near`, the candidate then
refuses `Unsettled::Meter { row: "curve_pair_meet_window_contact" }`
(`curve_pair.rs:679-683`), "the two readings disagree at rounding": an
escalation that a finer reading may settle comes out as a meter
disagreement. Mis-typed.

## Done when

A refused place reaches `window_off` as the escalation it is, and the
`Meter` refusal is read only where every reading decided.

**Read the walk-stop fork first.** The open design fork `work/onehome/the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes.md` may change
the fix; whoever takes this row reads that fork's outcome before writing
anything.
