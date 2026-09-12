# Inventory: transcendental entry points of the kernel's interval lane

The interval lane is the `Real` trait's `Interval` instantiation
(`crates/geom-core/src/interval.rs`, behind the `interval` feature).
Everything generic over `T: Real` runs at `T = Interval`, so the consumed
surface is the `Real` trait's transcendental methods, weighted by which
generic code calls them. This document fixes the SCOPE of the crate; it
is not a call-site census, which drifts and is a grep away.

## Consumed at interval type

| Entry point | Consuming roles |
|---|---|
| `sin_cos` (⇒ `sin`, `cos`) | rotations (`geom-core::linalg`); circle and analytic-surface eval and derivatives (`geom::curves`, `geom::surfaces`); cone half-angle and curved mass properties (`geom-brep`); profile validation |
| `tan` | bulge re-parameterization and bulge-from-turn-angle (`geom-brep`, `profile::sugar`); revolve axis logic; the expression evaluator (`editor-core::expr`) |
| `atan` | `arc_span` — angle from bulge (`sweep::revolve`, `sweep::extrude`); profile validation; revolve axis logic |
| `atan2` | turn angles (`profile::sugar`); the expression evaluator |
| `sqrt` | pervasive (norms, `linalg`, residuals) — exact-family, not transcendental, but part of the enclosure surface |
| `powi` | pervasive; **load-bearing for soundness**: interval squares of zero-straddling quantities MUST route through `powi(2)`, never `x*x`; gated by ci.yml's "interval-square powi(2) allowlist" |
| `pi()` / `tau()` | `Real` trait constants; used by revolve/validate angle logic |

## On the trait, implemented by `Interval`, with no generic call site

- `asin`, `acos` — only f64-lane call sites exist (`mesh` is f64-only).
  Provided anyway: they are part of the `Real` surface an adopting
  kernel must instantiate, and they are trivial (monotone, bounded
  domain).
- `sin`/`cos` individually — defaulted on the trait to project
  `sin_cos`; the bit-identity contract makes `sin_cos` the primitive.

## Exact (non-transcendental) `Real` surface the interval scalar also needs

`+ − × ÷ neg abs min max floor from_f64 zero one lo hi` — provided here
too (an adoptable scalar needs them), with 1-ulp outward pads on
`+ − × ÷` (we cannot set the rounding mode from portable Rust; inari's
correctly-rounded arithmetic is tighter by ≤1 ulp per op — a documented
tightness gap, not a soundness gap). `abs/min/max/floor/neg` are
endpoint-exact.

**`copysign` is on the `Real` surface but NOT in this crate.** Sign
transfer on an interval is not endpoint selection — a `sign` operand
containing zero has no sign to transfer, and the result is a hull of
`±|self|` with the decoration capped at `Def` — so the implementation
lives with the trait impl, `crates/geom-core/src/interval.rs`, built out
of `abs`, `hull` and negation from here. It is listed because it is part
of the exact surface an adopting kernel gets. Where it should ultimately
live is part of the open `RingInterval`-vs-`Interval` question, not a
thing this document settles.

## Set operations — inventoried separately, and not on `Real`

`hull` and `intersection`. Neither is a `Real` method and neither is a
function evaluation, which is why they are not in the census above and
why the crate's scope sentence names them apart from it.

| Entry point | Callers |
|---|---|
| `hull` | `crates/geom-core/src/interval.rs`'s `copysign`/`min`/`max` tangent-hull paths |
| `intersection` | none today; it is the 1788-strict reference point that `docs/semantics-diffs.md` §D7 defines `hull`'s deliberate divergence AGAINST, and its `Trv` cap and empty/NaI taxonomy are pinned by a unit test in `src/ops.rs` |

## Explicitly NOT built (nobody calls them)

`exp`, `ln`/`log*`, `powf`, `sinh`-family, `asinh`-family, `hypot`,
`cbrt`, `exp_m1`/`ln_1p`, gamma/erf. The kernel's `Real` trait does not
declare them. If a future milestone adds one, it must arrive with the
same pad-derivation + certification discipline as the functions here.
