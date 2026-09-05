# geom-core

`geom-core` is the scalar substrate: the [`Real`] ring the kernel evaluates
in and its instantiations (`f64`, `Dual`, `Interval`, `Sym`, the `Probe`
recorder), the certified decision door (`Decide`, `Band`, `Tol`), linear
algebra, and the **B-spline/NURBS structure layer** in `src/spline/`.
Nothing here knows about topology; nothing here decides anything a caller
did not ask it to.

## Where in the code

| Area | Modules |
|---|---|
| The ring and its instantiations | `src/real.rs`, `src/dual.rs`, `src/interval.rs`, `src/sym.rs`, `src/k_stats.rs` (the `Probe` recorder), `src/ring_interval.rs` |
| Decisions and tolerance | `src/predicate.rs`, `src/tolerance.rs`, `src/bit_identity.rs` |
| Linear algebra | `src/linalg.rs`, `src/linalg/` |
| Knot structure | `src/spline/knots.rs` (`KnotVector`, `Span`, `InteriorKnot`), `src/spline/locate.rs` (`SpanLocate`, `SpanSet`) — the S1 clause below |
| Knot algebra | `src/spline/algebra.rs` (insertion, refinement, removal, degree elevation, the union-and-refine routine) |
| Evaluation and bounds | `src/spline/basis.rs` (basis values and derivatives), `src/spline/hull.rs` (the C2.2 sup-norm mechanism), `src/spline/compose.rs`, `src/spline/net.rs` |

## The spline layer's pairing rule (SPLINE-DESIGN S1)

**S1 — a proof about a structure travels with that structure.** A span
index is only meaningful for the knot vector it was located in; a control
window is only meaningful for the curve or surface whose net it selects
from. Each is therefore a **borrow, not a plain value**:

```rust
pub struct Span<'a> { kv: &'a KnotVector, index: usize, first_control: usize, degree: usize }
```

and, one level up in `geom`, `CurveWindow{2,3}<'a, T>` holds
`&'a NurbsCurve{2,3}<T>` beside its `Span<'a>`, and `SurfaceWindow<'a, T>`
holds `&'a NurbsSurface<T>` beside two of them. The consequences are the
clause:

- **A door restricted to a span takes exactly one structure, and reads
  everything from it.** In this crate that is the span:
  `basis_funs(span, t)`, `ders_basis_funs(span, t, n)` read their knots
  through `Span::knots`. In `geom` it is the window: evaluation lives on
  `CurveWindow::{eval_in_span, ders_in_span, ders1_in_span,
  deriv_in_span, deriv2_in_span}` and `SurfaceWindow::{eval_in_span,
  ders_in_span, ders3_in_span}`, each reading its basis from its own
  span and its control net from the curve or surface that span was drawn
  from. A door taking `(structure, proof)` has two arguments nothing
  relates; a door taking only the proof has nothing to relate.
- **The mints are `&self`, and they are the only ones.**
  `KnotVector::{span, span_at, span_range}` for a `Span`;
  `NurbsCurve::{span, span_at}` and `NurbsSurface::{window, window_at}`
  for a window. So a window names the curve or surface that minted it,
  and that is the one it answers for.
- **No pairing guard, and no poison route for one.** The state a guard
  would test is not representable, so these doors are total on their
  inputs and D9's "the kernel never panics on any input" holds by
  construction rather than by check. There is no `admits` predicate on
  `KnotVector` or on `NurbsSurface`.
- **Equality on all three types is address equality on the borrow**,
  plus the indices. A proof is about *that* structure; two bit-equal
  knot vectors at different addresses are two structures. (None of
  `KnotVector`, `NurbsCurve` or `NurbsSurface` is `Eq` — their knots and
  weights are `f64` — so a by-value equality is not available in any
  case.) `Debug` prints the borrow as an **address** and never follows
  it: the alternative dumps a whole control net at every `{:?}`.
- **A borrow cannot be held across a rebinding of what it borrows.**
  That costs nothing here: every knot-algebra door in this crate and in
  `geom` is `&self -> Self`, so a refinement is a new value and a proof
  about the original goes on naming the original. Compile-fail doctests
  pin it — on `Span` (an escaped borrow, a rebinding, and a second knot
  vector that has no parameter to arrive through) and on
  `geom::curves::nurbs` (a span of one curve against another curve, and
  a window outliving its curve), each with a legal twin.

**The one pairing S1 leaves open** is coefficients against knots, and it
is now confined to `hull`'s **free functions**: `span_hull`,
`span_hull_rational`, `derivative_span_hull` and `sup_norm_bound_span`
take a loose `&[E]` beside a span and relate the two by **length alone**
(`coeffs.len() == kv.control_count()`). A same-length array from another
curve passes, and the bound is then **wrong rather than refused** — which
matters most at `sup_norm_bound_span(..) <= eps`, the C2.2 honesty limb.
The curve and surface windows do not have it: they read their
coefficients from the borrow. The count relation survives at
`NurbsCurve::new` and `NurbsSurface::new`, where it is what keeps every
window in range. `work/props/coefficients-carry-their-knot-vector.md`
carries the row.

`InteriorKnot` is the third member of the family — a value proved
interior to one vector's domain, carried without that vector — and stays
crate-private for it: the type is a guard only in combination with the
privacy of its two consumers, argued at its doc.

## Related pages

`docs/DESIGN.md` (D2's addendum on refusal design, D4 poison, D9
determinism, Q1 on the comparison-free `Real`); `crates/geom-brep/README.md`
(CURVED-DESIGN C2.2, the sup-norm certificate this layer's hulls feed);
`crates/topo/README.md` (C2's per-knot-span identity clause).

## Open

- The coefficient↔knot-vector pairing at `hull`'s free functions is
  length-only and open (`work/props/coefficients-carry-their-knot-vector.md`).
