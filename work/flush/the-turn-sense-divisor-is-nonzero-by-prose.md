---
id: the-turn-sense-divisor-is-nonzero-by-prose
kind: issue
title: Turn.sense is a bare T: the nonzero divisor turned_past and off_conic_slack read is held by call order, not by type
status: closed
opened: 2026-10-09
priority: P3
cost: E
branch: flush/e-tail
pr: 4714
closed: 2026-10-11
---


These items were left open by PR 4396's review and its fix-pass verification. The PR merged; none of them changes a result.

## What

- **The nonzero divisor is held by call order.** `Turn.sense` (`crates/topo/src/boolean/join.rs`, `Turn`) is a bare `T`. The fix pass added `Sense`, which carries the radial-germ refusal, but `turned_past` and `off_conic_slack` still take any `Turn`. The rule that the divisor is decided nonzero before ranking is held by call order and prose, not by type. Carrying `Sense` (or a decided-nonzero wrapper) in `Turn` would make it structural.
- **Repeated code.** The arm test opens both `nearer_along` and `nearer` (join.rs ~1298-1336), and five `let escalate` closures are defined separately.
- **An invariant that is argued, not scheduled.** The completed-null-faces row says "the reason is argued, not proven… the assertion is what holds the line". No row or schedule asks for the proof or for a witness search.

## Fix (branch `flush/e-tail`)

Measured on `main` at `121741a2b`: unchanged since the row was filed.
`Turn` (`boolean/join.rs`) carried `sense: T`, and the divisor
`turned_past` and `off_conic_slack` read, `|turn_of(site, tangent)|`,
was nonzero only because `germs_face_each_other` had decided the
partner's rotational sense before `partners` built the `Turn`;
`germ_arm` rebuilt one by struct update with the germ's own site.

- **The divisor.** `Turn` now lives in a submodule (`join::turn`) with
  private fields, and `Turn::decide` is the one way to make one: it
  decides both the germ's and the site's rotational sense
  (`rotational_sense`) against the one frame it stores, so a `Turn`
  whose divisor is zero cannot exist. It keeps the germ's `Sense` (not
  a `±1` scalar) and the germ's site, and `Turn::read_from_germ` gives
  `germ_arm` the same turn read from the germ's own site, so
  `germ_arm` no longer takes the germ separately.
- **The repeated code.** `nearer_along` no longer opens with the arm
  test `nearer` repeats: it reads the turn only when the arms agree
  and falls through to `nearer`. The five identical `let escalate`
  closures are one `join_escalate`.
- **The argued invariant** (the third bullet) is not this file's:
  the guard is in `chord_join.rs`, and it is filed on HONE as
  `a-kef-of-a-completed-null-face-is-ruled-out-by-a-debug-assert`.

## Closed (2026-10-11, PR 4714)

Fixed in the E-tail batch, PR 4714 (single FULL review, APPROVE-WITH-FIXES; every fix taken). The PR body has a section for this row.
