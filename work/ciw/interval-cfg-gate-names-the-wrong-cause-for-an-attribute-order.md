---
id: interval-cfg-gate-names-the-wrong-cause-for-an-attribute-order
kind: issue
title: check-interval-cfg-additive's tests-half message names block-gating for what is really an attribute order
status: closed
opened: 2026-09-17
closed: 2026-09-24
priority: P4
cost: E
---

## Finding

- **Where**: `scripts/check-interval-cfg-additive.py` — `next_item_line`
  and the `in_tests` violation message in `check_file`.
- **Importance**: low
- **Confidence**: sure — reproduced on a real PR and fixed by reordering
  two attribute lines with no other change
- **Raised by**: the `dup-one-builder` lane (S-DUP), 2026-09-17, on
  PR #2812

A test written

```rust
#[test]
#[cfg(feature = "interval")]
fn the_generic_box_doors_agree_at_an_interval_scalar() { … }
```

**is whole-item gated** — the two attribute orders are identical to
rustc — but the gate reports it as

> an interval cfg here gates a BLOCK, not a whole item. Split it into
> its own `#[cfg(feature = "interval")] #[test]` row …

`next_item_line` walks forward from the cfg line, skipping attribute
lines **unless** `ALLOWED_ITEM` matches them; `ALLOWED_ITEM` matches
`#[test]`, so in the `#[cfg] #[test] fn` order the walk stops at the
`#[test]` and the item is accepted. In the `#[test] #[cfg] fn` order the
walk starts one line later and lands on the `fn`, which matches nothing,
so the block-gating arm fires on a whole-item gate.

## Why this is worth a row and not a shrug

**The remedy the message prescribes is not the remedy that works.** It
says to split the row into its own `#[cfg] #[test]` function; the row
was already its own function, so a reader who takes the message at its
word looks for a `cfg` inside a body that is not there. It cost a round
trip on #2812, and the orchestrator reading the same message reached the
same wrong diagnosis independently — which is the tell that the message,
not the reader, is what misdirected.

The gate is **right to require one order**. The tree is unanimous:

```
git grep -n -B1 '^#\[test\]$' -- 'crates/*/tests/*.rs' | grep 'cfg(feature = "interval")'   # 94
git grep -n -A1 '^#\[test\]$' -- 'crates/*/tests/*.rs' | grep 'cfg(feature = "interval")'   # 0
```

94 sites spell it `#[cfg]` first and none spelled it the other way until
this one. So the ask is small: **the message should name the cause it
actually found.** The `in_tests` arm has one message for two different
defects, and the one it describes is the rarer.

This is not one of the script's STATED RESIDUALS. Those are all
under-reporting — things the syntactic check cannot see. This is
over-reporting on a correct spelling, which the header does not mention
and which `next_item_line`'s own docstring half-conceals: *"`#[test]` /
`#[cfg(test)]` short-circuit as allowed before we get here"* is true only
when the `#[test]` comes after the gate.

## The shape of a fix

Either is cheap and neither weakens the gate:

- **Distinguish the two cases in the message.** If the line the walk
  landed on is a `fn` and some SKIPPED line between the gate and it was
  `#[test]`, the defect is the attribute order — say so, and say which
  order. Otherwise keep the existing block-gating text.
- **Or accept both orders**, by having `next_item_line` remember whether
  it skipped a `#[test]`/`#[cfg(test)]` on the way. The 94-to-0 count
  argues for keeping one order enforced, so the first option is probably
  the better one; a gate that enforces a convention its header never
  states is its own small defect, and naming the order in the message
  closes that too.

Whichever, the script's `--selftest` should gain a case for it: the
existing cases cover prose lines and negation, not this.

## Closed (2026-09-24) — mooted by RING-4, PR 3154

Moot: `scripts/check-interval-cfg-additive.py` is deleted with the `interval` feature (RING-4, PR 3154); there is one build, so there is no additive-cfg gate whose message could name a cause.
