---
id: scaled-square
kind: ruling
title: The scaled square — a D9-fixed evaluation order may be reassociated when strictly tighter at Interval and bit-identical at f64
status: closed
opened: 2026-08-20
closed: 2026-08-21
refs: [849]
---

This file is the full statement of the ruling; nothing else carries it.

## Question

**Raised by Track F's F-g (#849), which stopped at the boundary rather
than deciding it.** The interval-square gate forbids `x * x` because the
general multiply must consider four endpoint products and cannot
exploit `x·x ≥ 0`; the tight square is never wider and is **strictly
tighter when the enclosure straddles zero**. F-g converted
`linalg/vec.rs`'s `orthonormal_basis` `b2` (`self.y.powi(2)`) on exactly
that ground — bit-identical at `f64`, tighter at `Interval`, still
containing the truth. **One line above sits `b1`'s
`((s * self.x) * self.x)` — a *scaled* square, invisible to any matcher
of this shape, and deliberately left alone**, because tightening it
means rewriting `(s·n.x)·n.x` as `s·(n.x²)` and the doc says *"each
component exactly as parenthesized"*.

**The question: may a D9-fixed evaluation order be reassociated when
the reassociation is strictly tighter at `Interval` and bit-identical at
`f64`?** It is not a matcher question and must not be answered by
widening one — **it decides two ratified sites**: `orthonormal_basis`'s
`b1`, and `linalg/mat.rs`, whose interval-square allowlist entry is
justified by *"`rotation_about`'s evaluation order"*. A taker who treats
it as a sweep will red both. Note
`memories/output-stability-as-justification.md` does **not** settle it:
it says byte-preservation may choose among equivalent implementations
but never justify keeping code, and the live claim here is that the
*order itself* is the ratified thing.

## Ruling

**RULED: YES — reassociate.** Ev, 2026-08-21, on two grounds.

**(1) Moving output is not on its own a reason not to act.**
`memories/output-stability-as-justification.md` names *arithmetic
association* as exactly the kind of thing output stability may decide,
and is explicit that committed bytes are *"usually a golden, and
regenerating a golden is a chore, not a contract"*. The orchestrator had
offered the `f64` byte-move at `mat.rs` as a downside **while citing
that same memory two paragraphs earlier**, which is the error the memory
exists to prevent.

**(2) The memory's carve-out does not reach this.** It preserves *"the
D2/D9 determinism contract itself (bit-identical replay, byte-identical
export)"* — and Ev: **D9 is determinism at one kernel, not pinning the
same output forever.** So the same document evaluated twice must agree;
it need not agree with last year. `u_ref` is stored as data per D2, so
existing documents keep their frames.

**Three conditions ride with it, none of them a reason to decline.**

- **The `Dual<f64>` tangent changes and nothing tests it** —
  `Dual::mul` is `x'·x + x·x'`, `Dual::powi` is `(2·x)·x'`; 6,388 of
  3,000,000 inputs differ at the last ulp of a subnormal tangent and
  `x = (1e308, 1e-308)` gives old `1.9999999999999998`, new `inf`, while
  the in-tree guard asserts only the **value** channel. Extend the guard
  **with** the change, not after.
- *"Strictly tighter"* has an exception: `powi(2)` is **1 ulp wider**
  below `|x| < 2^-480` (the *"never wider"* claim cited inari, which has
  not been the backend since M5 PR 1) — unreachable in the live regime,
  0 widenings in 3M samples over `|x| ∈ [1e-60, 1e60]`, but do not state
  it absolutely.
- **The gate cannot see scaled squares at all**, so this authorises a
  manual sweep rather than producing one; a taker who reaches for a
  matcher widening will red two ratified sites.

## Gates

The two ratified sites — `orthonormal_basis`'s `b1` in
`crates/geom-core/src/linalg/vec.rs` and `linalg/mat.rs`'s
interval-square allowlist entry — both Track N's; and the `Dual`
tangent guard, Track M's, which the change extends.
