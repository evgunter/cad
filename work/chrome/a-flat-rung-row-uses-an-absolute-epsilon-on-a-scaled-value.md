---
id: a-flat-rung-row-uses-an-absolute-epsilon-on-a-scaled-value
kind: issue
title: A flat-rung row compares a value of magnitude 8e-4 against an absolute f64::EPSILON
status: open
opened: 2026-09-21
priority: P3
cost: E
---



## Finding

`crates/viewer/src/scene.rs`'s `#[cfg(test)]` module,
`a_count_that_does_not_move_is_the_prediction`, closes with

```rust
let last = rungs.last().copied().expect("a rung");
assert!(
    (last - 1.0e-4 * PROBE_FACTOR).abs() < f64::EPSILON,
    "the last rung prices the request: {rungs:?}"
);
```

Two defects in one line, neither of them load-bearing today.

**`f64::EPSILON` is not a tolerance here.** It is `2.22e-16`, the gap
between 1.0 and its successor. The value under test has magnitude
`8.0e-4`, where the spacing of `f64` is about `1.1e-19`, so the
predicate admits roughly two thousand representable neighbours — and
the arithmetic that produces `last` is a single multiply, which is
correctly rounded and lands on the bit or does not. So the assertion
is exact equality wearing a tolerance costume: it neither tests what
`<` suggests (a nearby value) nor says what it means (`==`). An
absolute epsilon against a value of a different magnitude is the
shape that bites when the magnitude changes; if the request under
test were `1.0e-4` times a thousand, the same line would silently
start accepting a genuinely wrong rung.

**And it restates its own input.** The `1.0e-4` is the request
`delta(1.0e-4)` three lines above, written again rather than bound to
a local, so a lane changing the request has to change two lines and
the build cannot tell it to.

## Not the display_budget defect, and why this is separate

This is inside `scene.rs`'s own `#[cfg(test)]` module, which can read
`PROBE_FACTOR` directly and does. The number that is copied is the
row's own fixture input, not a constant that lives elsewhere, so
`work/chrome/display-budget-rows-restate-three-private-constants.md`
does not cover it: there the copy was of production state, here it is
of the row's own argument.

The `1.0e-4` is also numerically `INITIAL_DELTA`, and it is NOT a
copy of it — the row is about how many rungs a flat body pays and
does not depend on the request being the startup δ. CHROME-ONE-NUMBER
disposed of it that way and this row does not reopen that.

## Shape of the fix

Bind the request once (`let requested = 1.0e-4;`), read it in both
places, and compare with `==` or a relative bound — whichever the
author decides the row means. The choice is the point: `==` says the
multiply is exact, a relative bound says the rung may be computed
some other way later.

## Home

CHROME. `crates/viewer/src/scene.rs`. Pre-existing; not introduced by
any unit that found it.

## Found by

CHROME-ONE-NUMBER's style reviewer (PR 3022, 2026-09-21), while
reading the `#[cfg(test)]` modules that unit's own census named as a
blind spot. Filed from that unit under implementer-discipline §6.
