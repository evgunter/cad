---
id: is-finite-length-homed-in-the-query-seat
kind: issue
title: is_finite_length is a bare-scalar predicate homed in the kernel query seat; geom-core is the argued natural home
status: closed
opened: 2026-09-04
branch: fix/is-finite-length-home
pr: 2349
closed: 2026-09-11
---


## The finding

`topo::query::is_finite_length` (`crates/topo/src/query.rs:434`) is
the one predicate every direction door in `topo` and `editor-core`
asks the length question through — `UnitVec3::new` at the datum door
(SEAT-DV, PR #1564) and `eval::wire::unit()` at the evaluation layer's
own door (PR #1738, which made it `pub`).

It does not fit the module it lives in. `query.rs`'s own header calls
the module *"the kernel query seat — the geometric half of the
selection vocabulary as pure functions of a `Body`"*, and organizes
itself around an EXACT/DECIDED split. `is_finite_length` is neither
half: it takes a bare scalar, touches no `Body`, answers no selection
question and reaches no funnel. The header already spends a paragraph
explaining why `DATUM_UNIT_NORM` is an exception to its own
convention; this is a second exception, and after #1738 it is a
PUBLIC one.

## The argument for `geom-core` (from #1738's style review)

`geom-core` is where `Real` and `is_poison` are defined — the value
channel the predicate is asked through — and where
`Vec3::normalize`'s own doc note about overflow above ~1e154 and
underflow below ~1e-162 already sits
(`crates/geom-core/src/linalg/vec.rs:227-230`). The predicate is a
statement about a scalar, next to the arithmetic whose failure mode it
names, available to every crate that has directions rather than only
to those that depend on `topo`.

## Why it was not moved in #1738

Two reasons, both about scope rather than merit:

- Moving a predicate across a crate boundary is not a one-PR fix, and
  #1738 is a one-line finiteness gate with a red-first row.
- `crates/topo/src/query.rs` is SEAT's territory glob. The predicate
  is SEAT-DV's own; where it lives is SEAT's call, not a passing
  program's.

So #1738 kept it where it is and added a sentence to the module header
naming it as the second exception and pointing here.

## What this asks

**A question for SEAT**, not a scheduled fix: does the finiteness
predicate belong in `geom-core` beside `Real`/`is_poison` and
`Vec3::normalize`'s overflow note, or does it stay in the query seat
with the header sentence carrying the exception? Either answer is
small; the drift is leaving it unstated.

Related: issue 1570 (the direction-family unification, under 1372) is
about the DOORS asking the question. This item is about where the
question itself is written.

## What landed

**The predicate moved to `geom-core`.** `is_finite_length` is now a
free function in `crates/geom-core/src/real.rs`, immediately after the
[`Real`] trait it is generic over and beside the `is_poison` it asks
through, re-exported at the crate root (`geom_core::is_finite_length`).
`topo::query` keeps **no re-export**: its one caller,
`decide_unit_direction`, imports it from `geom_core` like any other
scalar predicate, and the module header's paragraph naming it as the
seat's second, public exception is retired — with it gone,
`decide_unit_direction` is the second thing there that is not a
selection question, not the third.

The reasoning, three parts:

- **It is neither half of what the seat is.** `query.rs`'s header calls
  the module the geometric half of the selection vocabulary as pure
  functions of a `Body`, organised EXACT/DECIDED. This predicate takes
  a bare scalar, reads no `Body`, answers no selection question and
  reaches no funnel.
- **`geom-core` is where the value channel is.** `Real` and `is_poison`
  are defined there, and `Vec3::normalize`'s own note about overflow
  above ~1e154 and underflow below ~1e-162 sits there too. Both
  `normalize` docs (`Vec2` and `Vec3`) now say in one paragraph what
  the overflow end costs a door that decides the length's sign first,
  and point at the predicate — so a reader changing that arithmetic
  meets the question rather than having to know it exists.
- **Reachability is the operative reason.** `profile` depends on
  `geom-core` alone and cannot reach `topo` at all, so two of the five
  doors in
  `work/fix/two-d-director-doors-skip-the-finiteness-question` could
  not ask the question in its one spelling while it lived in `topo`.
  Leaving it there meant a second spelling of the rule — which that
  family's ruling is against — or a layering change.

**No kernel answer moved.** The body is transcribed unchanged
(`#[allow(clippy::eq_op)]` on the self-difference included), every
caller's behaviour is identical, and no refusal text, golden, k-lint
row or render changed.

**The item asked this as "a question for SEAT", and SEAT is closed** —
there is no `work/seat/`, and `work/topo/program.md`'s `keep_out` still
names `topo/src/query.rs` as SEAT's territory. The question had been
parked on a program that no longer exists; that is why FIX answered it.
