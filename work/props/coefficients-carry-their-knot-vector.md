---
id: coefficients-carry-their-knot-vector
kind: issue
title: The coefficient↔knot-vector pairing is length-only
status: open
opened: 2026-09-05
---



## The residue

`span-carries-its-knot-vector` closed the span↔structure pairing: a
`Span` borrows the `KnotVector` it is a proof about, a `CurveWindow`
borrows its curve and a `SurfaceWindow` its surface, every
span-restricted door reads everything from that one borrow, and both
`admits` predicates are gone because the state they tested is
unrepresentable. It did **not** close the pairing one level down, and
this row is that one.

**Where it lives, exactly: `geom-core`'s free `hull` functions.** They
are the only doors left that take a coefficient array *beside* a span
rather than reading it through a borrow:

| Door | `crates/geom-core/src/spline/hull.rs` |
|---|---|
| `span_hull(coeffs, span)` | :123 |
| `span_hull_rational(coeffs, weights, span)` | :193 |
| `derivative_span_hull(coeffs, span)` | :296 |
| `sup_norm_bound_span(coeffs, span)` | :339 |
| the private `span_indices(coeff_len, span)` they all route through | :108-113 |

`span_indices` refuses `coeff_len != span.knots().control_count()` and
nothing else; `span_weights_positive` (:172) is the same check on the
weights.

**What it catches:** an array of the wrong *length*. That is exactly the
bound that keeps the window `[index − degree, index]` inside the array,
so nothing here can index out of range and no public door here panics
(D9).

**What it does not catch:** which curve the coefficients came from. The
review lane measured the three shapes the retired `admits` used to
refuse, all at equal control count, and all three are answered finitely
by these doors:

- **(a)** same degree, different interior knots — the disclosed shape;
- **(b)** a span whose index is **empty** in the vector the coefficients
  belong to (`admits == false` at the merge base);
- **(c)** a span of a **different degree** (`admits == false` at the
  merge base) — the sharpest, because the basis row that would pair with
  it is a different length from this vector's.

The exit that matters is `sup_norm_bound_span(coeffs, span) <= eps`, the
C2.2 honesty limb: a same-length foreign array yields a *finite* bound
over data that was never bounded.

**What is NOT in scope, and this is what changed:** `geom`'s curve and
surface doors. `CurveWindow::{eval_in_span, ders_in_span, …}` and
`SurfaceWindow::{eval_in_span, ders_in_span, ders3_in_span}` read their
control points and weights from the curve or surface the window borrows,
so they have no coefficient argument to mis-pair. The count relation
survives at `NurbsCurve::new` and `NurbsSurface::new`, once, at
construction — and there it is load-bearing: it is what makes every
window's `first_control + j` a construction fact.

Pinned as behaviour, so closing it reds the rows rather than passing
unnoticed:

- `crates/geom-core/tests/span_hull_window.rs::the_coefficient_pairing_is_still_length_only`
- `crates/geom/tests/curves/span_window_pairing.rs::the_free_hull_doors_relate_coefficients_by_length_alone`
- `crates/geom/tests/curves/span_window_pairing.rs::the_free_hull_doors_answer_every_shape_the_guard_refused` — shapes (b) and (c), adopted from the review lane's probe
- `crates/geom/tests/surfaces/span_window_pairing.rs::the_constructor_relates_the_net_to_the_vectors_by_count_alone` — the constructor half, falsifiable in both directions

and stated at `crates/geom-core/README.md` (SPLINE-DESIGN S1, "the one
pairing S1 leaves open").

## The third member of the family

`InteriorKnot` (`crates/geom-core/src/spline/knots.rs`) has the same
shape — a value proved interior to one vector's domain, carried without
that vector — and is **deliberately crate-private** for it (CERT-N3's
decision, not reopened here). Its two consumers use the type in opposite
ways, both argued at its doc. Any move that makes it public owes the
borrow `Span` now carries.

## Shape of a fix, not a plan

The span's answer was a borrow, and the curve and surface halves took
the same answer one level up — the door reads the net from the structure
it already borrows. The free `hull` functions are the case where that
does not obviously apply: their `coeffs` are not a curve's control net
but whatever coefficient brackets a fitting or composition pass
produced, and they are `RingInterval`/`CertifiedEnclosure` values with
no owner to borrow. Candidates when this is scheduled:

- (a) give the hull doors a *pairing* argument of the same shape — a
  `Coeffs<'a>` minted only against a `KnotVector`, which is `Span`'s
  trick applied to the payload;
- (b) an invariant-lifetime brand, rejected once for `Span` on ergonomic
  grounds (`span-carries-its-knot-vector`, option B);
- (c) accept it with the docs saying it is a decision.

Not urgent: no live caller mis-pairs, and the failure is bounded to a
wrong number rather than a panic.
