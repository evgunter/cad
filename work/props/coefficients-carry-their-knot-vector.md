---
id: coefficients-carry-their-knot-vector
kind: issue
title: The coefficient↔knot-vector pairing is length-only
status: open
opened: 2026-09-05
---


## The residue

`span-carries-its-knot-vector` closed the span↔vector pairing: a `Span`
borrows the `KnotVector` it is a proof about, every span-restricted door
reads its knots through that borrow, and `KnotVector::admits` is gone
because the state it tested is unrepresentable. It did **not** close the
pairing one level down, and this row is that one.

**The guard, and what it is.** A coefficient array is related to a knot
vector by **length alone**:

- `crates/geom-core/src/spline/hull.rs:113-121` — `span_indices` refuses
  `coeff_len != span.knots().control_count()` and nothing else. Every
  bound door here (`span_hull`, `span_hull_rational`,
  `derivative_span_hull`, `sup_norm_bound_span`, `domain_hull`,
  `domain_hull_rational`) is that check plus arithmetic.
- `crates/geom/src/curves/nurbs.rs` — `NurbsCurve::new` goes through
  `net::validate_counts(knots.control_count(), control.len(), &weights)`,
  once, at construction.
- `crates/geom/src/surfaces/nurbs.rs` — `NurbsSurface::new` checks
  `control.len() == knots_u.control_count() * knots_v.control_count()`.

**What it catches:** an array of the wrong length. That is exactly the
bound that keeps the window `[index − degree, index]` inside the array,
so nothing here can index out of range and no public door panics (D9).

**What it does not catch:** which curve the coefficients came from. A
same-length array from a *different* curve passes every check above, and
the bound or the evaluated point is then computed over the wrong data —
**wrong rather than refused**. The exit that matters is
`sup_norm_bound_span(coeffs, span) <= eps`, the C2.2 honesty limb: a
finite wrong bound there certifies a span whose real coefficients were
never bounded.

Pinned as behaviour, so closing it reds the rows rather than passing
unnoticed:

- `crates/geom-core/tests/span_hull_window.rs::the_coefficient_pairing_is_still_length_only`
- `crates/geom/tests/curves/span_window_pairing.rs::no_cross_vector_pairing_of_equal_control_count_panics`
- `crates/geom/tests/surfaces/span_window_pairing.rs::the_net_is_related_to_the_vectors_by_count_alone`

and stated at `crates/geom-core/README.md` (SPLINE-DESIGN S1, "the one
pairing S1 leaves open").

## The third member of the family

`InteriorKnot` (`crates/geom-core/src/spline/knots.rs:191`) has the same
shape — a value proved interior to one vector's domain, carried without
that vector — and is **deliberately crate-private** for it (CERT-N3's
decision, not reopened here). Its two consumers use the type in opposite
ways, both argued at its doc. Any move that makes it public owes the
borrow `Span` now carries.

## Shape of a fix, not a plan

The span's answer was a borrow. The coefficients' answer is not the same
one: coefficients are `T`-valued payload owned by the curve, not
structure, so a `Coeffs<'a>` borrowing a `KnotVector` would put a
lifetime on every control array in the kernel. The candidates worth
weighing when this is scheduled are (a) making the hull doors take the
`NurbsCurve`/`NurbsSurface` rather than a loose slice — the surface half
of `span-carries-its-knot-vector` did exactly that and it cost one
reference; (b) an invariant-lifetime brand, rejected once already for
`Span` on ergonomic grounds (`span-carries-its-knot-vector`, option B);
(c) accepting it with the docs saying it is a decision. Not urgent: no
live caller mis-pairs, and the failure is bounded to a wrong number
rather than a panic.
