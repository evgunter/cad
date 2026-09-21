---
id: sketch-headings-guard-zero-length-but-not-an-infinite-one
kind: issue
title: sketch.rs's two 2-D direction sites guard a zero length and not a non-finite one
status: open
opened: 2026-09-16
priority: P1
cost: E
---


## Finding

Found by the class sweep of
`datums-unit-helper-normalizes-with-a-silent-x-fallback`, re-derived
from the PROPERTY that fails — *a length is divided by without being
asked to be a number* — rather than from the `sqrt()` spelling that
surfaced it. Both hits are `hypot`, which the `sqrt(x² + y²)` pattern
cannot see, and both are in `crates/viewer/src/sketch.rs`.

**`heading`** (`crates/viewer/src/sketch.rs`, the `length > 0.0`
`then`) answers `Option<[f64; 2]>` — the type says *a unit vector, or
there is none*. `length` is `dx.hypot(dy)`, which is infinite for a
pair of points past about `1e308`, and `length > 0.0` is true of an
infinite length, so the door answers `Some([0.0, 0.0])`: a zero vector
delivered as a heading, through a door whose `None` arm exists to say
there is no heading.

**`arc_points`' centre** (same file, the `half == 0.0 || sin_half ==
0.0` `continue`) takes `half = dx.hypot(dy) / 2.0` and builds the left
normal as `(-dy / (2.0 * half), dx / (2.0 * half))`. The same infinite
`half` passes the guard, and `apothem = half / tan(θ/2)` is infinite
beside it, so **`centre` comes out `[NaN, NaN]`** and the arc's points
are all `NaN` — it is not placed anywhere, and a `NaN` polyline is
pushed into the drawn output.

Both routes were executed rather than reasoned, because this row first
claimed the wrong shape (see the correction below): with `dy` finite
and `dx` overflowing, `nx = -dy / inf = -0.0` and `-0.0 * inf = NaN`;
with both overflowing, `nx = -inf / inf = NaN` directly. Either way
`centre = [NaN, NaN]`, where the chord's midpoint would have been
`[0, 0]`.

Both are the shape `datums.rs`'s `unit` had before that row closed —
*a guard that admits everything except zero is not a bound* — and in
both the recourse already exists in the signature: `heading` has a
`None`, `arc_points` has a `continue`.

**Reachability**: neither is reachable inside the session box (D4 ¶4),
which is why this is filed rather than folded into the datums unit. It
is the same prose-unreachability the datums row was closed over: an
argument about callers, not a property of the door.

**Where**: `crates/viewer/src/sketch.rs`, `heading` and `arc_points`.

**Confidence**: sure (the arithmetic reads as quoted; the infinite-
length arm was not executed).

## Corrected before this row was ever worked (orchestrator, 2026-09-17)

As filed, the second bullet said `centre` *"becomes the chord's
midpoint — an arc whose centre is silently placed ON its own chord."*
**It does not.** The VIEW review of #2783 caught it and the arithmetic
was then executed under `rustc -O` on both input shapes: the centre is
`[NaN, NaN]`.

The correction makes the defect **worse**, not milder — a centre on
its own chord is a wrong arc, and `NaN` coordinates in a drawn
polyline are not an arc at all — so the row is strengthened rather
than withdrawn. It is recorded here because this program's own rule is
that **a filed row that is wrong is worse than no row**: the next
reader would have gone looking for a misplaced centre and found
nothing of the kind.
