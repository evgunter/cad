---
id: path-error-numbers-below-1e-9-render-as-zero
kind: issue
title: every PathError number below ~2e-9 renders as 0 (and a negative one as -0): num()'s absolute tolerance swallows exactly the margins these messages exist to report
status: review
opened: 2026-09-11
branch: fix/num-absolute-floor
pr: 2366
---



Found while filing `underflow-gate-owed-at-five-more-doors` from PR
2359's sweep. The 2359 lane reported row 5 as *"its rendered text
reads `got (0, 0)`* — the `num()` formatter prints `1e-180` as `0`".
That is true and it is **not an underflow defect**. Executed here, the
threshold is not the format's limit but a hard-coded `1e-9`, and the
class is every number a `PathError` message carries.

## The defect

`crates/profile/src/path.rs:1395`, `fn num`, the shared number
formatter behind **38 call sites** across three `Display` impls in
that file — `CornerRefusal` (`:672`), `CornerReason` (`:690`) and
`PathError` (`:1418`), so the title understates the reach by two
types.
It searches for the shortest decimal spelling that round-trips within

```rust
let tol = 1e-9 * x.abs().max(1.0);
```

For `|x| <= 1` that `max(1.0)` pins the tolerance at an **absolute**
`1e-9` regardless of the magnitude being rendered, and `prec = 0`
wins immediately for anything smaller. Executed (a standalone binary
over the function's exact body):

| value | rendered |
|---|---|
| `1e-8` | `0.00000001` |
| `2e-9` | `0.000000002` |
| `1e-9` | **`0`** |
| `1e-10` | **`0`** |
| `1e-12` | **`0`** |
| `1e-30` | **`0`** |
| `1e-180` | **`0`** |
| `5e-324` | **`0`** |
| `-1e-30` | **`-0`** |

## Why this is live and not cosmetic

The kernel's length unit is the metre, so `1e-9` is a **nanometre** —
and the quantities these messages exist to report sit below it by
design. `num` is applied to `margin` and `arm` at
`path.rs:1430-1431, 1443-1444` among others: a junction-tangency
refusal whose whole content is *"turn margin {margin} m on a {arm} m
arm"* renders a picometre-scale margin as `0 m`. The message then
reads as a statement that the margin **is** zero, which is the one
thing it is not — a decided-zero margin would have taken a different
arm.

`-0` is worse than `0`: it reports the sign of a quantity it has
just rendered as having no magnitude.

This is the same shape as the two units this program has already
landed at the arm level (`direction-underflow-reports-zero-length`,
`two-d-director-doors-skip-the-finiteness-question`) — *a refusal
that tells the author their value is zero when it is not* — one layer
over, in the formatter rather than in the arm. Fixing the arms and
leaving `num` in place means a correctly-typed refusal still renders
a false number.

## What the fix has to preserve

`num`'s purpose is real and should survive: `format!("{:?}")` on an
`f64` produces `0.30000000000000004` at ordinary scales, and the
shortest-round-trip search is what keeps refusal prose readable. The
defect is only the **absolute** floor. A relative tolerance
(`1e-9 * x.abs()`, no `max`) preserves the behaviour at every scale
where `num` is doing its job and stops the collapse below it, at the
cost of deciding what to do at exactly `0.0` — where the relative
tolerance is `0` and only an exact spelling round-trips, which is
`0` and correct.

A taker should also check the **fallback**: the `for prec in 0..=17`
loop falls through to `raw` for a value no 17-decimal spelling
round-trips, which is every subnormal. `{:?}` on `1e-180` is
`1e-180`, so the fallback is right and readable — worth a row, since
it is the path the fix newly puts these values on.

## Fence

`crates/profile/src/path.rs` is `profile`'s. `num` is private to that
module and has no consumer outside the three `Display` impls named
above, so the change is contained to the one file; the visible surface is refusal text, so
the rows are text assertions and any existing golden that pins a `0`
where a small number belongs is a **wrong pin that moves**, per
`docs/prompts/implementer-discipline.md`.
