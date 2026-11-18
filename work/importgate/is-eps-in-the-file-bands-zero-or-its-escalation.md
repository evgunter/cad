---
id: is-eps-in-the-file-bands-zero-or-its-escalation
kind: issue
title: design: [tolerance.recourse] (iii) puts ε_in at ε_input = K·ε (the band's escalation), while file_band puts it at the band's zero
status: open
opened: 2026-11-17
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
priority: P2
cost: M
design: true
---

Found by the dual review of `importgate/interp-layer` (PR 7091).

## The question

`docs/DESIGN.md` `[tolerance.recourse]` says ε_input is K·ε, "a synonym
and not a third dial", and that `[import]`'s ε_in is an instance of
ε_input (clause (iii)). Read literally, a file declaring ε_in reads as a
run whose ε_input is ε_in, so ε_in sits at the band's escalation:
`(ε_in/K, ε_in)`.

`crates/step-import/src/interpret.rs`'s `file_band` (and `chart.rs`'s
before it, since the chart reading landed) puts ε_in at the band's zero:
`(e, K·e)` for `e = max(ε_in, ε)`. Every interpretation reading, its
refusals' endings ("import with ε_in ≥ gap", "ε_in below gap/K") and the
corpus pins (`tier_gate.rs`) read it that way.

## Options

1. ε_in is the band's zero (the tree): two of the file's statements within
   ε_in are one; the file's band reaches K·ε_in. (iii) is reworded: ε_in
   is the file's ε_precision analogue, not its ε_input.
2. ε_in is the band's escalation ((iii) literally): two statements within
   ε_in/K are one, and the band is `(ε_in/K, ε_in)`. Every reading moves a
   factor K finer, and the corpus re-pins.

Ev's call: it is a reading of ratified text.
