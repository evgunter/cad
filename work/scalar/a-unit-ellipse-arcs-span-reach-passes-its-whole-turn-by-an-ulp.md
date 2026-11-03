---
id: a-unit-ellipse-arcs-span-reach-passes-its-whole-turn-by-an-ulp
kind: issue
title: A unit ellipse arc's span reach from its centre passes its whole turn's reach by an ulp (span_reach_hunt counterexample)
status: open
opened: 2026-11-03
---


Found by the ONEDOOR wedge-fold lane (stage 1 of `the-tier3-wedge-pass-and-the-census-compose-the-edge-wedge-door-again`), whose local `cargo nextest run -p geom-brep --all-features` hit it; the lane's diff does not touch `extent.rs` or the hunt.

`crates/geom-brep/tests/span_reach_hunt.rs` `a_conic_arcs_span_reach_is_never_short_of_its_farthest_point`, its third assertion (`got <= reach.lever_from(p).length()`), reds on:

- carrier `Ellipse { frame: { center: (0, 0, 0), axis: (-0.03969039619027481, 0.7529509146200635, 0.6568786742032877), u_ref: (-0.578512869148579, 0.5186919677397115, -0.6295089378490072) }, semi_u: 1.0, semi_v: 1.0 }`
- span `[3.660488584391903, 3.6604896604454873]` (about 1e-6 wide), pivot the centre
- `Reach::Span`'s `span_reach_from` reads `1.0000000000000002`, past the whole turn's `lever_from` (`1.0`).

Reproduce with `CAD_FUZZ_SEED=0xc543ab8c865d550e CAD_FUZZ_EFFORT=1`. The row's other two assertions carry a `16·ε·scale` tolerance; this one carries none, so either the span reach should be clamped to the whole turn's, or the assertion owes the same rounding tolerance. Which one is the owner's call; the counterexample is pinned as a deterministic row beside whichever fix lands.

## Another seed (2026-11-03, `onecopy/foot-certificate`)

`cargo nextest run -p geom-brep --features test-support` on that branch failed `span_reach_hunt::a_conic_arcs_span_reach_is_never_short_of_its_farthest_point`. The branch touches no reach code. The failing case is a circle of radius 1000 (an ellipse, `semi_u = semi_v = 1000`), over `[-9.77, 0.935]`, more than a turn, read from its centre:

> 1000.0000000000002 passes the whole turn; reproduce with CAD_FUZZ_SEED=0x638e2db0bd762c87 CAD_FUZZ_EFFORT=1
