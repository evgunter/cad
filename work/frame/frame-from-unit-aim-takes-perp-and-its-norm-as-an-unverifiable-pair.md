---
id: frame-from-unit-aim-takes-perp-and-its-norm-as-an-unverifiable-pair
kind: issue
title: frame_from_unit_aim takes perp and cross_len as a pair the callee cannot verify — a witness for 'this length is that vector's norm' is the next shape
status: open
opened: 2026-09-15
priority: P1
cost: E
---



## Where this came from

The fix pass of `unit-vector-witness-in-geom-core` (SCALAR).
`frame_from_unit_aim(origin, aim: UnitVec3<T>, perp, cross_len)` in
`crates/geom-core/src/linalg/frame.rs` takes the aim as the witness,
so "the aim is unit" is the type's. The other relation the same
boundary carries is still prose.

## The site

`frame_from_unit_aim` divides `perp / cross_len` into the frame's x
axis. `perp` is `reference × aim` and `cross_len` is `perp.norm()`,
evaluated once by the caller (`point_at`'s roll rung,
`path_start_frame`'s ladder) so that the decided quantity and the
divisor are one evaluation and one rounding — the doc says so, and
the callee cannot check it: any `(perp, cross_len)` pair a caller
passes builds a frame, and a mismatched pair builds one whose x column
is not unit and whose columns are not orthonormal, with no refusal.
The private signature is the only guard — the function is private and
both callers are in the file — which is also why the witness take has
no runtime pin: a mutant reverting `aim` to a bare `Vec3` survives
every test, because the type is the guard and a private function
cannot carry a `compile_fail` doctest.

## The next shape

A witness for "this length is that vector's norm" — the pair minted
together by the gate that decided it (`definitely_positive` on
`perp.norm()`) and consumed as one value, so the divide inside
`frame_from_unit_aim` reads the length off the pair rather than
trusting the caller to have evaluated the same expression. The
narrower move is `frame_from_unit_aim` taking `perp` alone and
dividing by `perp.norm()` itself — the same divide `UnitVec3::new`
performs (`Vec3::normalize` is `self / self.norm()`), bit for bit,
since the norm is deterministic — at the cost of the gate's decided
quantity and its divide being two evaluations of one expression rather
than one. PROPS' ground; filed rather than changed because the filing
unit was pinned to bit-identity and that D9 argument is this row's
owner's to make.
