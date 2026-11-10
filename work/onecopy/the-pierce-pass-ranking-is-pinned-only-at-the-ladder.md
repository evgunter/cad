---
id: the-pierce-pass-ranking-is-pinned-only-at-the-ladder
kind: issue
title: read_pierces' A-then-B ranking is reached by no fixture, and Rung::refused and rung_or_abort are two public spellings of one rule
status: closed
opened: 2026-11-09
closed: 2026-11-10
pr: 6705
refs: [6598, two-ops-placements-state-a-carrier-held-premise-unheld]
---


Found by the delta review of PR 6598 (head 4ec53e77).

1. **N1, the ranking is unpinned.** `a_later_abort_outranks_a_kept_limit` (`crates/geom-core/src/ladder.rs`, moved from geom-brep by QML T3) feeds hand-built rungs to `climb_evidence`. Each of these mis-wirings of `read_pierces` (topo boolean/mod.rs ~6446-6485) leaves it green:
   - a `?` before the climb;
   - `kept.then` in the wrong order;
   - a Limit classed Aborts in `BooleanError::refusal`.

   The pass's ranking (A, then B) is reached by no fixture. Pin it at the pass with a fixture, or with a seam that reads the pass's own climb.
2. **N3, two public spellings of one rule.** `Rung::refused(e, standing, abort)` and `rung_or_abort(standing, e, abort)` are both public, over one rule. They differ only in whether the standing has already been read. Make one of them the home and route the other through it.
3. **Nits:**
   - `a_refused_reading_stands_by_its_own_standing`'s message says "under either map" but is checked under `identity` only.
   - vtxfac.rs:540 is not rewrapped.

**Unrouted seed found in the same runs:** `topo boolean::conic_quadric::cone_rows::certified_answers_hold_against_the_quadric_form` fails at `CAD_TOLERANCE_EPS=1e-12` with `CAD_FUZZ_SEED=0x3bf35ddc97f968bd`. It is case 41: an ellipse root 2.7e-6 along the arc from the oracle's. It also fails on base fbc704f1. No row names it. This belongs to whoever takes this row only if it shares a cause; otherwise re-file it under green.

## Narrowed: waits on PR 6598 (2026-11-09)

Every item here is about PR 6598's code (`onecopy/ladder-climb-4`, open at 4ec53e77). None of it is on main or on 6576 (`onecopy/boolean-standing`, 1c7d38c3): `read_pierces`, `a_later_abort_outranks_a_kept_limit` and `a_refused_reading_stands_by_its_own_standing` do not exist there, and `Rung::refused` is the two-argument `(e, standing)`. The row is deferred until 6598 lands (orchestrator, 2026-11-09), and is then taken as:

1. **N1, unchanged:** pin the pass's A-then-B ranking at the pass (a fixture, or a seam that reads the pass's own climb), so each mis-wiring listed above reds.
2. **N3, restated:** 6598 widens `Rung::refused` to `(e, standing, abort)`. `rung_or_abort(standing, e, abort)` then differs from it only in taking the standing already read, which lets a caller keep a value other than the one its standing is read off (6598's three remaining callers: `fate.rs`'s `contained(e, …)`, `slit.rs`'s `unread(Some(why))`, census's `Undecided::of_point_in_solid(&e).what()`). Make one the home: route `Rung::refused` through `rung_or_abort` (it is `rung_or_abort(standing(&e), e, abort)`), with one public name, or state in both docs why the two shapes are kept.
3. **The nits, unchanged.**
4. **The unrouted seed** is filed as `work/green/cone-rows-certified-answers-fail-at-1e-12-on-one-seed.md` (PR 6637's review, S7).
