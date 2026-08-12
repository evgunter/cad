---
name: verification-tools
description: Kani vs Flux for this repo (investigated 2026-08-12, both installed and run) — Kani proves the interval crate's discrete layer exhaustively but has NO reals so the pad lemmas stay paper; Flux has no float sort and SILENTLY ACCEPTS false float specs, so it is only ever a candidate for index-shaped invariants
metadata:
  type: project
---

Full report: `docs/VERIFICATION-TOOLS.md`. Harnesses:
`interval-transcendentals/src/verify.rs` (`cfg(kani)`). Flux probes and
their verbatim outputs: `docs/verification-probes/`.

**The one-line split.** Kani = bit-precise bounded model checking (CBMC),
models `f64` exactly including FMA, but has **no model of the real
numbers**, so `next_down(RN(t)) <= t` (Lemma P1) is not even statable.
Flux = refinement types; sorts are `int`/`bool`/`bitvec<32>`/`Set`/`Map`
— **no float sort, no real sort**.

**The Flux footgun (verified, do not forget this).** Flux does not reject
a refinement mentioning an `f64` value; it accepts it whatever its truth
value. `#[spec(fn(x: f64) -> f64{v: v > x})] fn id(x: f64) -> f64 { x }`
type-checks green ("1 checked; 1 constraints solved"), while the same
shape at `i32` is correctly rejected. A green Flux run over
float-carrying code proves nothing about the floats and says so in the
same words it uses when it has proved something.

**What Kani DID prove here** (all exhaustive over every `f64` bit
pattern — inf/subnormal/±0/NaN included): the `DInterval` representation
invariant is preserved by the whole surface (`make`'s `debug_assert`
becomes a theorem); mul/div/add enclose the padded value at every
SYMBOLIC interior point (not just the four corners), which covers
`div_touching_zero`'s four-way sign match; `powi`'s even-power
zero-straddle floors at exactly `+0.0` (the interval-square-poison
contract) and `sqrt`'s lower bound is never negative; poison never
launders through `with_dec_capped`; Lemma P3's *encoding* half
(neighbor step = one `bitdist` step, subnormals and binade boundaries
included).

**Tier C trick worth remembering.** The exact product of two `f32` is
always `f64`-representable, so `f64` is an EXACT oracle for `f32`
arithmetic — the pad-encloses-the-true-value property becomes statable
one precision down. Division works too by cross-multiplying. Addition
only under an exponent-spread bound. This verifies the algorithm's
shape, NOT the `f64` instance or the `2^-960` constant.

**Frictions.** (1) `rust-version = "1.97.0"` in the crate blocks Kani
outright (bundled nightly-2025-11-21 = rustc 1.93); true MSRV is 1.87.
Flux is in the same boat (pins nightly-2026-02-05). (2) Symbolic loop
bounds do not work — CBMC unwinds structurally and ignores
`assume(k <= 4)`; every harness needs a fixed trip count +
`#[kani::unwind(n)]`. (3) `f64` FMA bit-blasting is the cost ceiling.
CBMC only engages the SMT float theory with **bitwuzla** (z3 still gets
`QF_AUFBV`); bitwuzla could not be built in the sandbox (its configure
fetches CaDiCaL from codeload, 403 through the proxy), so that lever is
untested.

**Where Flux would earn its keep in the kernel** (not done, just
established as feasible): index-safety arguments that today live in
prose, e.g. `geom-curves/src/nurbs.rs::eval_in_span`'s "Indexing
justified: span valid ⇒ i = span − p + j ∈ [span − p, span] ⊆
[0, control_count)". That exact pattern checks in 50 ms and the negative
control (drop `p <= span`) is caught as a `usize` underflow. NOT a fit:
`geom-brep/src/props/quad.rs`'s exact `i128` Newton–Cotes headroom claim
— nonlinear integer arithmetic, where liquid-fixpoint's encoding stops
being reliable.
