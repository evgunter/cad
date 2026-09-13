---
id: inclusive-rule-range-draws-a-line-on-a-nan-count
kind: issue
title: A NaN line count rules one line anyway, at infinity
status: open
opened: 2026-09-12
---

## Finding

`crates/viewer/src/datums.rs`, `rule_patch`'s ruling loop:

```rust
let count = ((last - first) as usize).min(MAX_GRID_LINES);
for i in 0..=count {
```

The range is **inclusive**, so a count of zero draws one line — which
is right when the bounds are real and one line is what fits. It is not
right when the count is zero because it is not a count:

```
inf - inf = NaN     NaN as usize = 0     0..=0 runs once     t = inf * pitch = inf
```

**Every refusal reaches yes and NaN geometry still leaves the module.**
`half_patch_at` answers `Some`, `grid_pitch` answers `Some`, the
normal tick correctly refuses — and the ruling emits positions that
are not positions.

## Measured, and reachable through DOCUMENT DATA

Not a synthetic camera: a datum whose ORIGIN is out at the end of the
number line, seen from an ordinary view (eye `[0, -0.15, 0.1]`,
`look_at` the world origin, 1280 px, 45°). With the datums at
`[f64::MAX, 0, 0]`:

| kind | positions | non-finite | first |
| --- | --- | --- | --- |
| plane | 56 | **2** | `[0.0, -0.26, 0.0]` |
| frame | 56 | **2** | `[-inf, NaN, NaN]` |
| axis | 0 | 0 | — |
| point | 0 | 0 | — |

The axis and the point refuse cleanly, which is what the per-mark
refusal buys; the two plane-like kinds do not, because their ruling
runs past it.

## Why this is filed and not fixed here

It was **mis-dispositioned** in the census that closed
`viewer-grid-pitch-nonfinite-fallback` — cleared as *"a documented
backstop … the conservative direction"*. The cap half is exactly that.
The **inclusive range** half is not: on a NaN difference the
saturating cast is not conservative, it is silent, and it turns a
non-count into one drawn line. The census sentence is corrected in
that row.

The fix is small but it is a behaviour choice in CHROME's house — an
exclusive range plus an explicit `first`/`last` finiteness check
before the loop, or the refusal moved onto the bounds rather than onto
the scale that produced them — and the DOOR row it came out of is one
row, one PR.

**The one thing to preserve when fixing it:** a legitimate count of
zero means one line, and an exclusive range alone would silently drop
it. The two zeros have to be told apart, not merged.

## Fence

`crates/viewer/src/datums.rs` — CHROME's and VIEW's by the territories
table.
