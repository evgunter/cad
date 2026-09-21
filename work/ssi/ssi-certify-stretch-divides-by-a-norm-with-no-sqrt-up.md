---
id: ssi-certify-stretch-divides-by-a-norm-with-no-sqrt-up
kind: issue
title: ssi/certify: the chart stretch divisor is a raw sqrt of an f64 fold, so it is not an upper bound
status: open
opened: 2026-09-15
priority: P0
cost: H
---


## The finding

In `crates/geom-brep/src/ssi/certify.rs`, the plane×NURBS chart-form
margin divides by the chart's stretch along `e⊥`:

```rust
let stretch =
    (vt.x.mag() * vt.x.mag() + vt.y.mag() * vt.y.mag() + vt.z.mag() * vt.z.mag()).sqrt();
...
let margin = zero_free_lower_bound(phi_u * ex + phi_v * ey) / stretch;
```

The comment two lines above states the soundness argument: *"An UPPER
bound on the stretch is used, which can only shrink the margin: the
safe direction."* The expression does not produce one. Every step
rounds to NEAREST — three multiplies, two adds and the square root —
with no `sqrt_up` and no ring arithmetic anywhere in the chain, so
`stretch` can land BELOW `‖S_u·ex + S_v·ey‖`. Where it does, the
margin is over-reported, which is the unsound side of a transversality
decision: the trilean can read a sliver band as clear.

This is worse than the two siblings it was found beside, which at
least carried `sqrt_up`'s single `next_up`.

## Scale

On the sibling site, an `f64` fold whose square root WAS rounded up
still landed below the ring reading on 306 of 308 cells of a real
grid, worst deficit 4.70e-16 relative. Without the `next_up` the
deficit is the fold's ulps plus the square root's, and it is one-sided
only by luck.

## The fix shape

`offset_meters::norm_sup` is the spelling the kernel already owns: the
per-component square and both sums in the ring, then `sqrt_up`. It is
`pub(crate)` in `geom-brep` as of PR #2469. The operands here are `T:
Real` magnitudes rather than `RingInterval`s, so the fix is either to
lift them into point intervals and call it, or to give this site the
same rounding discipline in place. A bare `.sqrt()` where the comment
claims an upper bound should not survive either way.

## Why it is filed rather than fixed

Found by PROPS's mignitude-floor sweep (PR #2469), which fixed the two
instances on its own ground in `offset_fit.rs`. `ssi*` is TRIM's
ground behind PCURVE P-2 (PROPS's `keep_out` says so explicitly), and
this is a change to a transversality decision rather than a spelling
tidy, so it goes to the owner rather than riding a bound unit.

Note that the sweep table in an earlier revision of #2469 classified
`certify.rs`'s `zero_free_lower_bound` as *"genuinely scalar — keep"*,
which it is. `stretch` is two lines above that call and is a different
shape: a norm assembled from three channels and used as a divisor. The
first sweep's patterns (`mig(`, `mignitude`, `sqrt_down(`,
`hi() < 0.0`) match none of it.

## The class

*An upper bound on a norm assembled by an `f64` fold of ring
endpoints, used as a divisor.* Sibling filed on SHELL's slate as
`offset-meters-cell-normal-midpoint-direction-is-an-f64-fold`.

## Still live (2026-09-15)

Re-read at `37dce8287`: `crates/geom-brep/src/ssi/certify.rs` is
unchanged on `main` since this was filed, and `stretch` is still the
bare `.sqrt()` over three `mag()` products quoted above.

## Re-read after the `speed_sup` extraction (2026-09-15, supersedes the section above)

The spelling this row quotes is gone; the arithmetic is not. `stretch`
is now `vt.speed_sup()`, and `Box3::speed_sup` in
`crates/geom-brep/src/ssi/enclose.rs` has the identical body —

```rust
(self.x.mag() * self.x.mag() + self.y.mag() * self.y.mag() + self.z.mag() * self.z.mag())
    .sqrt()
```

— three multiplies, two adds and a square root, every one of them
rounded to NEAREST. Two things changed and neither is the fix.

**It has three callers now, not one**: `ssi.rs`'s chart-tube speed
(`nan_propagating_max(du.speed_sup(), dv.speed_sup())`),
`certify.rs`'s `stretch` at the plane×NURBS margin, and `certify.rs`'s
`m` in the tube fold. Whether each of them is on the unsound side is a
per-caller question — `stretch` demonstrably is, because it divides a
`zero_free_lower_bound` and the comment above it claims an upper
bound — but the arithmetic is now in ONE home, so the fix is one line
where it used to be three.

**The extraction's own doc raises the rounding and settles it the
wrong way.** It says: *"`offset_meters::norm_sup` is the same shape
over a different operand and a different arithmetic — it rounds the
square root outward — so the two are siblings, not copies, and folding
them into one would move bits."* That is the correct READING of the
difference and the wrong CONCLUSION from it. "Folding them would move
bits" is not a reason (`memories/output-stability-as-justification.md`,
and implementer discipline §3: a change justified by output staying
identical has not been justified). The question is which of the two
arithmetics each caller needs, and a caller that divides a lower bound
by this number needs the outward one. Where the bits move, they move
because the old ones were not certified.

So the finding is unchanged in substance and better placed: it is one
method, with a doc that already names the sibling it should have been.
What it needs is a decision per caller — outward-rounded where the
result is a bound, round-to-nearest only where it is a diagnostic —
rather than one arithmetic chosen for stability.

**Sibling row, not a duplicate.**
`work/trim/limb-3-chart-tube-speed-has-neither-guard-its-sibling-site-has.md`
is about the same method's GUARDS (what `0`, `NaN` and `+∞` mean at
each caller), which `speed_sup`'s doc also points at. This row is about
its ROUNDING. The two want settling together and neither subsumes the
other.
