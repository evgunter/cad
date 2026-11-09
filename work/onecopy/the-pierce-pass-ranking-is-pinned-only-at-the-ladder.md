---
id: the-pierce-pass-ranking-is-pinned-only-at-the-ladder
kind: issue
title: read_pierces' A-then-B ranking is reached by no fixture, and Rung::refused and rung_or_abort are two public spellings of one rule
status: open
opened: 2026-11-09
---


Found by the delta review of PR 6598 (head 4ec53e77).

1. **N1, the ranking is unpinned.** `a_later_abort_outranks_a_kept_limit` (geom-brep ladder.rs) feeds hand-built rungs to `climb_evidence`. Each of these mis-wirings of `read_pierces` (topo boolean/mod.rs ~6446-6485) leaves it green:
   - a `?` before the climb;
   - `kept.then` in the wrong order;
   - a Limit classed Aborts in `BooleanError::refusal`.

   The pass's ranking (A, then B) is reached by no fixture. Pin it at the pass with a fixture, or with a seam that reads the pass's own climb.
2. **N3, two public spellings of one rule.** `Rung::refused(e, standing, abort)` and `rung_or_abort(standing, e, abort)` are both public, over one rule. They differ only in whether the standing has already been read. Make one of them the home and route the other through it.
3. **Nits:**
   - `a_refused_reading_stands_by_its_own_standing`'s message says "under either map" but is checked under `identity` only.
   - vtxfac.rs:540 is not rewrapped.

**Unrouted seed found in the same runs:** `topo boolean::conic_quadric::cone_rows::certified_answers_hold_against_the_quadric_form` fails at `CAD_TOLERANCE_EPS=1e-12` with `CAD_FUZZ_SEED=0x3bf35ddc97f968bd`. It is case 41: an ellipse root 2.7e-6 along the arc from the oracle's. It also fails on base fbc704f1. No row names it. This belongs to whoever takes this row only if it shares a cause; otherwise re-file it under green.
