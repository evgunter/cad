---
id: the-canonical-root-trades-one-of-the-brackets-wall-margins
kind: issue
title: the canonical root trades one of the filleted bracket's carrier_on_surface margins: a theorem with rule G shut, numeric with it on
status: open
opened: 2026-11-22
---


Found by BOXFINISH's `boxfinish/signed-radius`, re-cutting
`crates/editor-core/tests/decide_3_split_rows_interval.rs`
`decide_3_no_predicate_loses_a_decision`.

That row compares the shipped rule set against the tier before DECIDE-3
(`before()`: `SymRules::without_canonical_root()` with the decision read
shut), per predicate, at the nominal. Once every arc verb stores its
radius as the literal its construction decided (the fillet arcs their
carrier's radius, no `abs`), the filleted bracket
(`m10_7_r2_probes_interval::bracket`) reads `carrier_on_surface_1` as
`[242, 1]` (symbolic, numeric) under `before()` and `[241, 2]` under the
shipped set: one margin the older tier proves is numeric with rule G and
the decision read on. The same at ε = 1e-6, 1e-9 and 1e-12. It decides
inside the band either way, and the bracket's document total rises
(1361 -> 1370 symbolic), so nothing refuses; the row pins it as a trade
beside the bracket's known `carrier_matches_mapped_source` trade
(`[325, 8]` against `[321, 12]`).

**Not known:** which margin it is, which of the two dials moves it
(rule G's re-keying of a root, as on `carrier_matches_mapped_source`,
is the guess and is not measured), and whether it is another face of
`sym-tier-reach-depends-on-symbol-order`.

**The test that says it is closed.** `decide_3_no_predicate_loses_a_decision`
holds with `("r2_filleted_bracket", "carrier_on_surface_1")` out of its
`traded` set.
