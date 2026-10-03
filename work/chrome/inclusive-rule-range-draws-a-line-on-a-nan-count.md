---
id: inclusive-rule-range-draws-a-line-on-a-nan-count
kind: issue
title: A NaN line count rules one line anyway, at infinity
status: closed
opened: 2026-09-12
closed: 2026-09-15
branch: chrome/datums-substitution-sweep
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

## Closed

Fixed in `rule_patch`'s `rule` closure, on branch
`chrome/datums-substitution-sweep`. The range is exclusive and the
bounds are asked whether they are bounds before the cast: `first`,
`last` and the cross-direction `lo`/`hi` must all be finite, and the
`last < first` case rules none. The three zeros the old cast merged
now read apart — a NaN difference refuses, a negative difference
rules nothing, and `last == first` rules the one line it always
meant.

**The trap the row named is the `last == first` arm** and it is
asserted in both directions:
`datum_draw::a_datum_at_the_end_of_the_number_line_rules_no_line_at_infinity`
(red on the old code with the row's own measured
`[-inf, NaN, NaN]`, two per plane-like kind) and the pre-existing
ruling rows, which still rule the counts they always did.

**A third zero the row did not name** rode the same cast: a patch
narrower than the pitch and lying between two lattice lines gives
`last < first`, which saturated to the integer zero and ruled one
line at `first` — OUTSIDE the patch, up to a pitch away. Measured at
a four-pixel viewport: a line at `0.02 m`, `0.005 m` outside a patch
`9.1e-4 m` wide. Asserted by
`datum_draw::a_patch_between_two_lattice_lines_rules_neither`.

**The cap's effective maximum MOVED, by one line per direction, and
saying so is the point.** `((last - first) as usize).min(96)` under an
INCLUSIVE `0..=count` drew up to **97** lines; the exclusive
`.saturating_add(1).min(96)` under `0..count` draws up to **96**. The
const is named `MAX_GRID_LINES` and its doc reads "The most grid lines
one plane draws per direction", which was false by one before this
change and is true after it — so the stored behaviour that moved is
the one the doc already claimed. No golden or render baseline covers
it (nothing in the tree asserts 97, and the grid rows count lattice
membership rather than lines), which is why it would otherwise have
moved silently.

**A fourth thing the cast could not say, filed rather than fixed:**
`MAX_GRID_LINES` still truncates a genuine count above 96 and returns
the result as a ruling, with nothing marking it partial — see
`max-grid-lines-truncates-a-ruling-and-calls-it-one`.

**A fifth, fixed in the same closure after review measured it:** a
ruling whose two endpoints coincide. At a datum origin near the end of
the number line the patch's ends `cv ± half` both round to `cv`, so
the extent is lost and every segment comes out zero-length — finite,
in the right plane, and not a line. 27 of them per plane-like kind for
a datum at `f64::MAX`. The bounds check now asks the emitted geometry
whether it is geometry, and the ruling is built and committed whole so
a direction that loses its extent rules nothing rather than something
shorter.
