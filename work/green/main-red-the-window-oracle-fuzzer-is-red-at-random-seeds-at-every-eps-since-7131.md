---
id: main-red-the-window-oracle-fuzzer-is-red-at-random-seeds-at-every-eps-since-7131
kind: issue
title: Main red (slow set, fuzz): curve_pair's window oracle is contradicted at random seeds at every eps since PR 7131, so the nightly and any PR editing oracle.rs go red at random
status: open
opened: 2026-11-23
priority: P1
cost: E
---


`geom-brep curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance`
is in the slow set (`.config/nextest.toml:371`). Since PR 7131
(`9bc9d6517b`), a random draw's `f64` reading is often contradicted by
the dense distance. That is a **wrong answer**, not a refusal, and it
shows at the default ε as well as at 1e-12. The oracle's bands do not
read ε, so a red seed is red on every leg. Three seeds were seen red on
2026-11-23:
- `0x4071461b4c688a45`, from the nightly;
- `0x22eb45feecea31d9`, from GERM, on main at the default ε;
- `0xc960ef101b854219`, from PR 7558's gate, run 45195294885: draw 58,
  `f64` `Recedes` on a circle of radius 523, 405 m out.

The fix and the diagnosis are CPAIR's:
`work/cpair/a-window-reading-at-f64-is-contradicted-by-the-dense-distance-since-7131s-squaring.md`
(P2 there; F1F, 7131's `f64` squaring, alone greens the first seed).
PR 7558 pins that seed's failure as a red, so a fixed seed stays
watched. The random sweep stays red on a fraction of draws, so the
nightly and PR 7359's file-edit selection go red at random until it is
fixed. That is why this is a GREEN P1. A sweep that is red at random
also hides any new red that lands in the same row.

**Done when** the sweep passes at random seeds, which takes one of two
things: CPAIR's fix, or the sweep dropping the needle band's unringed
reading the way its `_at_a_touch` sibling already does. The second
needs CPAIR's agreement, because it narrows what the oracle checks.
