---
id: span-carries-its-knot-vector
kind: issue
title: Consider giving Span its KnotVector — close the unbranded-pairing hole structurally
status: open
opened: 2026-08-13
github: 475
refs: [447, 463, 468]
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
