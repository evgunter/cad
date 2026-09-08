---
id: gate-findings-name-no-columns-and-recoverable-has-two-aggregations
kind: issue
title: the gate's finding lines render figures through a helper and name no columns; Row::recoverable and SceneTotals::recoverable are one name over two aggregations
status: open
opened: 2026-09-08
---



## What

Unit 9 gave the report's cell totals their column names and printed
the two factors as their formulas. **It did not reach the gate's
output, and the gate is where the same quotient appears under a third
English name.**

`tools/tess-lint/src/main.rs:233` `fn line(prefix, o) -> String`
renders every observation, figures included:

```
Kind::Slack { face, was, now } => format!(
    "{prefix} {scene} face {face}: recoverable slack {was:.1}x -> {now:.1}x — the \
     sizing schedule got wastefuller"
)
```

and is invoked as `println!("{}", line(" ", note))` (`:452`) and
`println!("{}", line("  FINDING", f))` (`:465`) — **no figure in
either format string.** `was` and `now` are `Row::recoverable()`
(`lib.rs:1683`), which is `grid_cells / span_opt_cells`: the identical
quotient the report now spells at `main.rs:361` and `:426`. Three
names for one ratio, and the third is the one that reddens CI.

## Finding

Two things, and the second is why this is a row rather than a
one-line edit.

**The finding line should name its columns**, as the report block now
does. It is the output a reader meets when the gate fires, which is
the worst moment to owe them a lookup.

**`recoverable` is one name over two aggregations**, and unit 9
attached the formula to the sweep-level one only. So do `span_held`
and `total_slack`: all three exist on both `Row` (`lib.rs:438`, `:444`,
`:461`) and `SceneTotals` (`:1251`, `:1258`, `:1296`). `recoverable`
is today the only one of the three with BOTH aggregations reaching
print — the per-face one through the gate's finding lines, the
per-scene one through the report — so it is where a reader can meet
the same word meaning two different things in one run. Deciding how
the two are told apart in output is the substance here, and it is not
a wording pass.

## Next

A `tools/tess-lint/src/main.rs` lane. `CELL_TOTALS` and
`tests/report_columns_pin.rs` are the shape to extend; note that
`line`'s output is asserted by `tests/cli_contract.rs`, so a wording
change there re-baselines those assertions and the lane should say
what moved.

## Was

disclosed by METER unit 9's style review, which found the site as a
live instance of a blind spot that unit's PR had claimed was empty —
"a figure rendered through a helper rather than a literal format
string". The claim was wrong; this is the instance.
