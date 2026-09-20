---
id: the-boolean-joins-shell-processing-order-is-unasserted
kind: issue
title: Reversing the order the boolean join walks an operand solid's shells reds nothing
status: open
opened: 2026-09-20
---

## Finding

- **Where**: `crates/topo/src/boolean/finish.rs`, the
  `movefac`-every-shell loop (~:230).
- **Importance**: low
- **Confidence**: sure. Measured by mutation, with its own control.
- **Raised by**: the `Body::shells_of_solid` fold, 2026-09-20.

The join distributes an operand solid's shells by `movefac`-ing each in
the order the solid lists them. **No row distinguishes that order from
its reverse.**

Measured at `230c46738`, baseline **733 lib / 566 integration**:

| planted | direction argued before the run | lib | integration |
| --- | --- | --- | --- |
| `for shell in shells` → `for shell in shells.into_iter().rev()` | permutes: neither grows nor shrinks the set distributed, so it reds only where a consumer downstream depends on the solid's list order | 733 / 0 | 566 / 0 |

**Its control is the fold's own first plant**, not a second run here:
shrinking `Body::shells_of_solid`'s answer by one shell reddens this
site broadly — `bool4r1_probes` ×5, `bool4r2_probes` ×5,
`issue93_nested_islands` ×5, `issue86_double_subtract`, `merge_skip`,
among 190 integration reds. So the loop is live and heavily asserted
about WHICH shells it walks; only the ORDER is dark.

## Why this is not the door's problem

`Body::shells_of_solid`'s rustdoc states the order it answers in — the
solid's own list order, which no arena determines — as a contract a
caller may reason with. It makes no claim that any caller does. This
row records that at this caller, today, none does; that is a fact
about the suite, not about the door.
