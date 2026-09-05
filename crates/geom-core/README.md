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
| Knot structure | `src/spline/knots.rs` (`KnotVector`, `Span`, `InteriorKnot`), `src/spline/locate.rs` (`SpanLocate`, `SpanSet`) |
| Knot algebra | `src/spline/algebra.rs` (insertion, refinement, removal, degree elevation, the union-and-refine routine) |
| Evaluation and bounds | `src/spline/basis.rs` (basis values and derivatives), `src/spline/hull.rs` (the C2.2 sup-norm mechanism), `src/spline/compose.rs`, `src/spline/net.rs` |

## The spline layer's pairing rule (SPLINE-DESIGN S1)

**S1 — a proof about a structure travels with that structure.** A span
index is only meaningful for the knot vector it was located in; a control
window is only meaningful for the surface whose net it flattens. Both are
therefore **borrows, not plain values**:

```rust
pub struct Span<'a> { kv: &'a KnotVector, index: usize, first_control: usize, degree: usize }
```

and `geom`'s `SurfaceWindow<'a, T>` holds `&'a NurbsSurface<T>` beside its
two `Span<'a>`s. The consequences are the clause:

- **Every span-restricted door drops its knot-vector parameter.**
  `basis_funs(span, t)`, `ders_basis_funs(span, t, n)`,
  `span_hull(coeffs, span)`, `span_hull_rational(coeffs, weights, span)`,
  `derivative_span_hull(coeffs, span)`, `sup_norm_bound_span(coeffs, span)`
  and `geom`'s curve evaluators read their knots through `Span::knots`.
  A door that took `(kv, span)` had two arguments nothing related; a door
  that takes only the span has nothing to relate.
- **The surface doors live on the window.** `SurfaceWindow::eval_in_span`,
  `::ders_in_span` and `::ders3_in_span` read the surface the window
  borrows. `NurbsSurface::window` and `::window_at` are the two mints, both
  tied to `&self`, so the window's two knot vectors, its control net and
  its row-major stride all come out of one borrow.
- **No pairing guard, and no poison route for one.** The state a guard
  would test is unrepresentable, so there is nothing to refuse: these doors
  are total on their inputs, and D9's "the kernel never panics on any
  input" holds by construction rather than by check. `KnotVector::admits`
  and `NurbsSurface::admits` do not exist.
- **Equality on both types is address equality on the borrow**, plus the
  indices. A proof is about *that* structure; two bit-equal knot vectors at
  different addresses are two structures. (Neither `KnotVector` nor
  `NurbsSurface` is `Eq` — their knots and weights are `f64` — so a
  by-value equality is not available in any case.)
- **A span is a borrow, so it cannot be held across a rebinding of its
  vector.** That costs nothing: every knot-algebra door in this crate and
  in `geom` is `&self -> Self`, so a refinement is a new value and a span
  of the old one goes on naming the old one, correctly. Compile-fail
  doctests on `Span` pin all three mismatches (an escaped borrow, a
  rebinding, and the retired `(kv, span)` spelling).

**The one pairing S1 leaves open**, stated rather than implied away: a
coefficient array is related to a knot vector by **length alone** —
`coeffs.len() == kv.control_count()` in `hull`, `control.len() ==
kv.control_count()` at `NurbsCurve::new`, `nu·nv` at `NurbsSurface::new`.
A same-length array from a different curve passes, and the bound or the
value is then **wrong rather than refused**. `InteriorKnot` is the third
member of the family and stays crate-private for exactly that reason: a
knot interior to vector A handed to vector B is representable, so the type
is a guard only in combination with the privacy of its two consumers.

## Related pages

`docs/DESIGN.md` (D2's addendum on refusal design, D4 poison, D9
determinism, Q1 on the comparison-free `Real`); `crates/geom-brep/README.md`
(CURVED-DESIGN C2.2, the sup-norm certificate this layer's hulls feed);
`crates/topo/README.md` (C2's per-knot-span identity clause).

## Open

- The coefficient↔knot-vector pairing above is length-only and open.
