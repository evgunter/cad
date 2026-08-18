# Inventory: transcendental entry points of the kernel's interval lane

Census date: 2026-07-25, `main` @ 1f3be61. The interval lane is the
`Real` trait's `Interval` instantiation (`crates/geom-core/src/interval.rs`,
behind the `interval` feature). Everything generic over `T: Real` runs at
`T = Interval`, so the consumed surface is: (a) the `Real` trait's
transcendental methods, weighted by (b) which generic code actually calls
them.

## Consumed at interval type (generic `T: Real` call sites)

| Entry point | Kernel call sites (non-test, generic) |
|---|---|
| `sin_cos` (⇒ `sin`, `cos`) | `geom-core/src/linalg/mat.rs:76,148` (rotations); `geom-curves/src/lib.rs:175,200,224` (circle eval + derivatives); `geom-surfaces/src/lib.rs:260,297,307,318,351` (cylinder/cone/sphere/torus eval); `geom-brep/src/implicit.rs:111,163` (cone half-angle); `geom-brep/src/edge_geometry.rs:149` (arc eval); `geom-brep/src/props/curved.rs:299,601` (mass properties); `profile/src/validate.rs:966` |
| `tan` | `geom-brep/src/edge_geometry.rs:122` (bulge re-param); `profile/src/sugar.rs:63,98` (bulge from turn angle); `sweep/src/revolve/axis.rs:477`; `editor-core/src/expr.rs:678` (expression evaluator) |
| `atan` | `geom-brep/src/edge_geometry.rs:148`; `profile/src/validate.rs:1129`; `sweep/src/revolve/mod.rs:585` + `sweep/src/extrude.rs:484` (`arc_span`: angle from bulge); `sweep/src/revolve/axis.rs:321` |
| `atan2` | `profile/src/sugar.rs:62,91,92` (turn angles); `editor-core/src/expr.rs:679` |
| `sqrt` | pervasive (norms, `linalg`, residuals) — exact-family, not transcendental, but part of the enclosure surface |
| `powi` | pervasive; **load-bearing for soundness**: interval squares of zero-straddling quantities MUST route through `powi(2)`, never `x*x`; gated by ci.yml's "interval-square powi(2) allowlist" |
| `pi()` / `tau()` constants | `Real` trait constants; used by revolve/validate angle logic |

## On the trait, implemented by `Interval`, but with NO generic call site today

- `asin`, `acos` — only f64-lane call sites exist (`mesh/src/chords.rs:47`,
  `mesh/src/walk.rs:186`; mesh is f64-only). Provided here anyway: they are
  part of the `Real` surface an adopting kernel must instantiate, and they
  are trivial (monotone, bounded domain).
- `sin`/`cos` individually — defaulted on the trait to project `sin_cos`;
  the bit-identity contract makes `sin_cos` the primitive.

## Exact (non-transcendental) `Real` surface the interval scalar also needs

`+ − × ÷ neg abs min max floor copysign from_f64 zero one lo hi` — provided
in this crate too (an adoptable scalar needs them), with 1-ulp outward pads
on `+ − × ÷` (we cannot set the rounding mode from portable Rust; inari's
correctly-rounded arithmetic is tighter by ≤1 ulp per op — documented
tightness gap, not a soundness gap). `abs/min/max/floor/copysign/neg` are
endpoint-exact.

## Explicitly NOT built (nobody calls them)

`exp`, `ln`/`log*`, `powf`, `sinh`-family, `asinh`-family, `hypot`,
`cbrt`, `exp_m1`/`ln_1p`, gamma/erf. The kernel's `Real` trait does not
even declare them. If a future milestone adds one, it must arrive with the
same pad-derivation + certification discipline as the functions here.
