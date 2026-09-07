---
id: hand-listed-debug-censuses-in-geom-core-geom-and-topo
kind: issue
title: nine hand-written Debug impls across geom-core, geom and topo list their fields by hand and end in finish(), so a new field is silently unrendered under a completeness claim
status: open
opened: 2026-09-06
refs: [2093]
---

Found by #2093's sweep, which fixed the same class inside
`crates/viewer/`. Filed here on the VIEW orchestrator's direction:
implementer discipline §6 makes reporting the filing act outside a
lane's fence and gives the file to the party with the whole board, and
this program established on 2026-09-06 that a §6 report alone is not a
durable artifact — the same out-of-fence finding was reported twice
through §6 with nothing a later reader could find
(`work/issues/loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed`).

No program obviously owns these three crates' `fmt` impls, which is
what puts the file here rather than on a slate.

## The class

A `Debug` impl that names its value's fields by hand has no compile-time
tie to the declaration: a field added to the struct is silently absent
from every dump, visible only to a reader who reads one and wonders
what is not there. #2093 closed that inside `crates/viewer/` by
destructuring `Self` exhaustively in the impl, so a new field is an
E0027 unbound-pattern error, and by binding a field the dump will not
carry to `_` — a decision a reader can see and the compiler still
forces.

**These are the worse version of it**, because every one ends in
`finish()` rather than `finish_non_exhaustive()`: the rendering claims
to show every field while listing them by hand.

## The hit list

Enumeration rule: `grep -rnE "impl[^=]*\bDebug\b for" crates/`, minus
`crates/viewer/`. **Eight rows, nine concrete impls** —
`crates/geom/src/curves/nurbs.rs:221` is inside `nurbs_curve!`, invoked
twice (`crates/geom/src/curves/nurbs.rs:1492` for `CurveWindow2` and
`:1493` for `CurveWindow3`).

| site | value | fields listed | ends in |
|---|---|---|---|
| `crates/geom-core/src/spline/knots.rs:340` | `Span<'_>` | `kv` (as an address), `index`, `first_control`, `degree` | `finish()` |
| `crates/geom-core/src/spline/hull.rs:262` | `SplineCoeffs<'_, E>` | `knots` (address), `coeffs` (address), `len` | `finish()` |
| `crates/geom-core/src/spline/hull.rs:331` | `RationalCoeffs<'_, E>` | `knots`, `coeffs`, `len`, `weights` | `finish()` |
| `crates/geom-core/src/spline/hull.rs:367` | `CoeffWindow<'_, E>` | `pair`, `span` | `finish()` |
| `crates/geom-core/src/spline/hull.rs:396` | `RationalWindow<'_, E>` | `pair`, `span` | `finish()` |
| `crates/geom/src/curves/nurbs.rs:221` (×2) | `CurveWindow2`, `CurveWindow3` | `curve` (address), `span` | `finish()` |
| `crates/geom/src/surfaces/nurbs.rs:164` | `SurfaceWindow<'_, T>` | `surface` (address), `span_u`, `span_v`, `base`, `stride` | `finish()` |

**One row is NOT this shape and is listed for completeness, not for
fixing.** `crates/topo/src/param_source.rs:84` is
`impl Debug for ParamSource`, where `ParamSource` is
`pub struct ParamSource(Arc<[u8]>)` (`param_source.rs:82`) and the body
is `write!(f, "ParamSource(<{} bytes>)", self.0.len())` — no
`debug_struct`, no `finish()`, and a one-field newtype has no census to
fall behind. #2093's own hit list carried it under a sentence claiming
"every one ends in `finish()`", which was wrong; it is corrected here.

So: **eight rows, nine concrete impls, seven of the eight rows in the
class.**

## Why each one is written out rather than derived

Every one of these is a borrow-carrying token, and each says so in its
own doc: a derived `Debug` would follow the reference and dump the
whole knot vector, control net or coefficient array at every `{:?}`.
That reason is good and is not what is at issue — the same reason
applies to the four walks #2093 rewrote. What is missing is the
exhaustive destructure that makes the hand-listing safe.

## The same files carry the same hazard in `PartialEq`

`hull.rs` and both `nurbs.rs` files write `PartialEq` field by field
beside each `Debug` — `hull.rs:275`, `:344`, `:376`, `:405`,
`crates/geom/src/curves/nurbs.rs:209` and
`crates/geom/src/surfaces/nurbs.rs:176` — on the same address-equality
argument. A field added to one of these values is silently
outside equality too, which is the sharper consequence: a `Debug` that
misses a field misleads a reader, an `Eq` that misses one answers
wrong. Unswept, and named here so the next reader does not have to
re-derive it.
