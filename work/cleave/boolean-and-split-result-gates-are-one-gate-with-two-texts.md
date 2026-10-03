---
id: boolean-and-split-result-gates-are-one-gate-with-two-texts
kind: issue
title: BooleanError::ResultInvalid and SplitFinishError::ResultInvalid are one tier-2 gate with two texts, and boolean's gate runs tier 1 twice
status: closed
opened: 2026-10-02
closed: 2026-10-03
---

Found by the review of PR 3797 (TQUERY), which added split's
tier-2 result gate.

- **Two texts for one gate.** `topo::boolean::ops::gate` refuses
  `BooleanError::ResultInvalid { errors }`, and its `Display`
  (`boolean/mod.rs`, the `ResultInvalid` arm) prints the first finding
  as `{:?}`, a Debug struct the refusal-shape standard bans. It also
  says "kernel bug", though an operand that is not a closed solid
  reaches it as well. The boolean takes its operands unvalidated, as
  split does. Split's twin, `SplitFinishError::ResultInvalid`
  (`splitting/finish.rs`), renders the first finding through
  `ValidationError`'s `Display`. One shared rendering, or one shared
  error, would keep the two from drifting.
- **Tier 1 runs twice.** `gate` calls `validate(body)` and then
  `validate_closed(body)`, and `validate_closed` already runs tier 1
  (`validate.rs`, `validate_closed`, which opens with `tier1(body)`).
  The first call is redundant. On a tier-1 failure it also returns
  before tier 2's findings exist, which is a different report from the
  one `validate_closed` alone would give.

## Closed (2026-10-03, REACH `boolean-door-adopts-the-finished-body-type`)

- **Tier 1 runs twice:** gone. `ops::gate` is the kept tier-3 gate
  (`AtRestPolicy::gate_at_rest_kept`), whose first act is tiers 1 and 2,
  once.
- **Two texts:** `BooleanError::ResultInvalid` renders its first finding
  through `ValidationError`'s `Display`, as split's twin does, with no
  Debug struct; "kernel bug, unless an operand carried the finding in" is
  gone because the operands are finished bodies, so the findings are the
  door's own.
