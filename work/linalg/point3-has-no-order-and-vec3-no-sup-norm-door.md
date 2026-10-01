---
id: point3-has-no-order-and-vec3-no-sup-norm-door
kind: issue
title: Point3 has no order and Vec3 no sup-norm door — the two spellings the tour lift sweep could not route through a door
status: closed
opened: 2026-09-11
priority: P1
cost: E
closed: 2026-10-01
design: false
---


(FIX orchestrator) From the tour lift sweep, PR 2341
(`work/fix/tour-scenes-lift-componentwise-not-through-map`). The next
instance of the class `vec3-point3-const-and-conversion-doors` (closed,
PR 1977) named: a spelling the demo could not route through a door,
surfaced by rewriting the corpus through the doors that do exist.

`demos/tour/src/diechamfer.rs:100` reads points back into
`(f64, f64, f64)` tuples, and stayed that way through a sweep that
closed 31 of 31 lift sites around it. Two doors are missing, not one:

- **No order on `Point3`.** `Point3<T>` derives only
  `Clone, Copy, Debug` (`crates/geom-core/src/linalg/point.rs:32`) — no
  `PartialOrd`, and no `PartialEq` either — so a set of points has no
  sort key and the tuple is the only thing that can carry one.
- **No sup-norm door on `Vec3`.** `feet_agreement`'s Chebyshev gap has
  no spelling but a hand-written `.abs().max()` chain.

**The first is a genuine design question and is why this is filed
rather than patched.** Whether a point carries a lexicographic order is
D2-shaped in the same way the `From<Vec3> for Point3` question in the
parent row was: an order on a point is a *presentation* fact, not a
geometric one, and deriving `PartialOrd` on a coordinate triple
publishes a comparison whose meaning is the storage order rather than
anything about the geometry. A separate `fn lex_key()` or an explicit
comparator says the same thing without the type claiming it. `PartialEq`
carries the same hazard one step further — exact float equality on
coordinates is a door this kernel has been careful about elsewhere.

The second looks like a plain missing door, and `Vec3` already carries
`norm` and `norm_squared` beside where it would go.

Neither blocks anything: the one consumer is a demo site that works
today through tuples and says so. Filed so the site's reason has a
durable home, since a demo working around a library gap in silence is
exactly what `memories/demo-purpose.md` forbids.

## Closed (2026-10-01)

**Sup norm.** Closed by #3710: `Vec2`/`Vec3::norm_inf`, the max-abs
norm, NaN-poisoning. It is named apart from `interval::norm_sup`, which
is a bound on the Euclidean norm. Hand-written chains that do not yet
use it are listed in `hand-chebyshev-chains-are-not-yet-on-norm-inf`.

**Order.** There is no order. A designer pair weighed the question
(see the log, 2026-10-01). The two started split, and on
reconciliation they converged on the rule `linalg.rs` already states
under "Deliberate omissions":

- `Point2`/`Point3` carry no order, no equality and no f64 comparator.
- Nothing in production needs a point order. The demo's sort did no
  work.
- The sites that sort clouds and zip them are a hazard, filed as
  `work/tint/sorted-clouds-are-zipped-under-a-tolerance-across-a-sort-tie`.

The omission bullet gains one sentence: clouds are compared by matching
under a tolerance. `diechamfer` now matches by `norm_inf` and does not
sort; its narrated numbers are unchanged.
