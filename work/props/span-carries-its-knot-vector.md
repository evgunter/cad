---
id: span-carries-its-knot-vector
kind: unit
title: Consider giving Span its KnotVector — close the unbranded-pairing hole structurally
status: review
opened: 2026-08-13
github: 475
refs: [447, 463, 468]
pr: 1952
branch: props/span-knot-vector
---

## From GitHub issue 475

Opened 2026-08-13; 0 comments.

`Span` is validated against a `KnotVector` it does not carry, so the pairing is prose. Same for `SurfaceWindow` and its surface. This is the one obligation the newtype sweep (#447, #463, #468) left with the caller, and each PR in the sweep has restated it rather than closed it. Filing the option so the decision is made deliberately rather than by accumulation.

## What breaks today

A `Span` proves "in range and nonempty **for the vector it was drawn from**", and then travels separately from that vector:

```rust
pub fn basis_funs<T: Real>(kv: &KnotVector, span: Span, t: T) -> Vec<T>
pub fn span_hull<E: Enclosure>(kv: &KnotVector, coeffs: &[E], span: Span) -> RingInterval
```

Nothing relates the two arguments. Hand a `Span` from vector A to an evaluator holding vector B and:

- **B at least as long as A:** in-range but wrong window — a silently wrong answer. This is what #447 documented.
- **B shorter:** indexes past B's arrays. Before #463 the entry points' own range guard caught it and returned poison; #463 deleted those guards (that was the point), so it is now a **panic** — a worse failure than the poison D4 asks for, and one clippy's panic-family gate cannot see because it is an index expression.

`hull`'s surviving `coeff_len != kv.control_count()` check does not help: it relates `coeffs` to `kv`, not the span to either.

No live caller does this. Every construction site in the tree draws its span from the vector it evaluates, usually one statement apart, and #468's `SurfaceWindow` narrows the surface case further by owning the stride. The hole is real but currently theoretical, which is exactly why it is worth deciding on rather than drifting.

## Option A — the `Span` holds its vector

```rust
pub struct Span<'a> { kv: &'a KnotVector, index: usize, first_control: usize, degree: usize }
```

and the entry points **drop their own `kv` parameter**:

```rust
span.basis_funs(t)                    // or basis_funs(span, t)
hull::span_hull(coeffs, span)
```

The mismatch stops being an obligation and becomes unrepresentable — there is no second vector to disagree with. `Span` stays `Copy` (a `&` is), the runtime cost is one pointer, and no new lint or macro machinery is involved.

Cost: a lifetime parameter propagates through `Span`, `SpanSet`, and the `SpanLocate` trait (`fn locate_spans<'a>(self, knots: &'a KnotVector) -> SpanSet<'a>` — a method-level lifetime, so the trait itself stays plain). It also makes a `Span` a borrow of the knot vector, so nothing can hold one across a mutation of the curve it came from — probably fine, since every current use is within one evaluation, but it is the constraint to check before committing.

## Option B — an invariant-lifetime brand

Keep `Span` a plain value; give `KnotVector` a scoped constructor that stamps both the vector and its spans with an invariant `'brand`, so only spans from *this* vector typecheck against it. Values stay small and un-borrowed; the price is a `with_knots(|kv| …)` scope at every entry into the spline layer, which is a real ergonomic tax on an API-first kernel.

## Option C — accept it

Leave the prose, and treat the panic as acceptable because the pairing is always local. Cheapest, and defensible — but then the docs should say it is a decision rather than a deferral, and the sweep should stop promising a brand.

## Scope if A or B is chosen

`geom-core` (`basis`, `hull`, `knots`, `locate`, `interval`), `geom-curves`, `geom-surfaces` (`SurfaceWindow` has the same shape one dimension up — a window from surface A on surface B is in-range-but-wrong or a panic, per #468), `geom-brep`, `mesh`. Roughly the same call-site set the newtype sweep already touched three times, which is an argument for deciding before a fourth pass lands rather than after.

Not urgent, and not a blocker for #463 or #468 — both state the gap in their type docs. This wants Ev's call on whether the lifetime is worth the unrepresentability, and if so a DESIGN.md revision rather than a drive-by refactor.

Refs: #447 (deferred the brand), #463 (deleted the guards that were masking it), #468 (same shape for `SurfaceWindow`).

## Home

`work/issues/`: the scope is the `geom-core` spline layer and its consumers, ground no open program's `paths` covers, and the decision is a DESIGN.md question rather than a code-quality row.

## Question for Ev (PROPS orchestrator, 2026-09-05)

**Which of A, B or C?** This is the plan's `[ev]` ruling for the item,
asked now because the answer gates an L-class sweep and nothing else
about the item waits on code.

What changed since the filing: CERT-N3 (#1879) adds two more consumers
of the unbranded pairing (`spline::algebra::union_refinements` and
`NurbsCurve::refine_to_union`, where a knot crosses vectors by design
and `refine_plan` re-validates it against the vector it lands in), and
`SurfaceWindow` (#468) still carries the same hole one dimension up.
No live caller mis-pairs today; the hole is a panic-shaped obligation
carried in prose across four PRs.

**Recommendation: A** — `Span<'a>` holds `&'a KnotVector`, the entry
points drop their `kv` parameter, and the mismatch becomes
unrepresentable (the D2 addendum's row 0, available and local: one
lifetime through `Span`, `SpanSet` and a method-level lifetime on
`SpanLocate`). Cost is one pointer and the constraint that a `Span` is
a borrow — nothing may hold one across a mutation of its curve. That
constraint is checked FIRST in the sweep: if any live site holds a span
across a mutation, the unit stops and reports, and the answer falls to
C with the docs saying so. B (an invariant-lifetime brand) is
rejected for the reason the item gives: a `with_knots(|kv| …)` scope at
every entry into the spline layer is an ergonomic tax on an API-first
kernel that A does not charge. C is the fallback, not the
recommendation: "the pairing is always local" is true today and
unenforced, which is the accumulation the item was filed to stop.

If A: the unit lands the sweep (`geom-core` spline layer, `geom`
curves and surfaces incl. `SurfaceWindow`, `geom-brep`, `mesh`; the
same call-site set the newtype sweep touched three times) and a
companion note beside the code (`crates/geom-core/README.md`'s spline
clause, present tense), and closes this item. Dispatch waits for
CERT-N3's `spline/` edits to merge.

**RULED: A** (Ev, in-chat, 2026-09-05: "A and B both sound ok, so if
you recommend A then that works"). The item is a unit: the sweep per
§Scope, the mutation-hold check first, `crates/geom-core/README.md`'s
spline clause as the companion note. CERT-N3 (#1879) merged 2026-09-05 04:08; the unit is cut against the
post-N3 `spline/` layer: spec `docs/PROPS-SPAN-SPEC.md` (binding; L,
block PROPS-B1 slot 1). The census (2026-09-05) found no span held
across a mutation, no storage beyond `SurfaceWindow`, no serialization
or FFI crossing, ~82 src + 112 test sites over five crates; the
coefficient↔vector pairing stays open and is filed by the unit.
