---
name: verification-tools
description: Kani vs Flux for this repo (investigated 2026-08-12, both installed and RUN) — Kani proves the interval crate's discrete layer exhaustively but has NO reals; THREE CBMC float models are unusable (sqrt not a function, fmaf diverges from hardware, concrete playback reports non-witnesses) so FAILED verdicts are not actionable; Flux has no float sort and SILENTLY ACCEPTS false float specs
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

**What Kani DID prove here** (exhaustive over every `f64` bit pattern —
inf/subnormal/±0/NaN included): the `DInterval` representation invariant
is preserved by the exact + set ops (`make`'s `debug_assert` becomes a
theorem); abs/min/max/floor/hull are exactly correct at SYMBOLIC interior
points; poison propagation across the arithmetic surface; poison never
launders through `with_dec_capped`; `powi`'s even-power zero-straddle
floors at exactly `+0.0` (the interval-square-poison contract, but see
the FMA caveat below); Lemma P3's *encoding* half (neighbor step = one
`bitdist` step, subnormals and binade boundaries included); pad soundness
for ADDITION at f32 against the exact sum.

**What it did NOT prove, though the harnesses exist**: the mul/div
containment proofs over two symbolic intervals (`b1 b4 b5 b6`, plus
`a5 a6`) all TIMED OUT at 900 s — including the ones that would have
covered `div_touching_zero`'s four-way sign match. Nothing about `sqrt`
is machine-checked (the model is unusable, below).

**Tier C trick worth remembering (and what happened to it).** The exact
product of two `f32` is always `f64`-representable, so `f64` is an EXACT
oracle for `f32` arithmetic — the pad-encloses-the-true-value property
becomes statable one precision down. Division works too by
cross-multiplying. ADDITION VERIFIED (`c3`, 5/5). MUL and DIV did NOT:
their harnesses fail, and the failures are CBMC's float models, not the
algorithm. Withdrawn, not proven false.

**THE BIG ONE — Kani's float FAILURES are not actionable here.** Four
diagnoses, all run: (1) `f64::sqrt` is not modelled as a FUNCTION (two
calls on the same value can differ; monotonicity fails; but `sqrt(4)==2`
and `sqrt(x)>=0` pass — partially constrained, i.e. the worst case);
(2) `fmaf` DIVERGES from hardware — `c1`'s witness pinned as a constant
still fails in CBMC while holding under native `rustc -O`, so it is a
WRONG model, not just a coarse one, and a wrong model can make a false
property PASS; (3) **concrete playback reports values that are not the
witness** — every `c7` witness passes when pinned back into CBMC, so a
FAILED verdict cannot be turned into a fact about the code; (4) the
systematic cross-check (`f32` mul vs `f64`-then-round, exhaustive) does
not terminate in 40 min. RULE: pin a reported counterexample back inside
the checker BEFORE replaying it natively; replaying natively alone is
not enough and led this investigation to a wrong intermediate
conclusion. One localisation survived: with the divisor pinned, `c7`
verifies for all NORMAL numerators and fails only for subnormal ones —
unresolved.

**Read per-check, never the top-line verdict.** `VERIFICATION:- FAILED`
folds in Kani's automatic checks (NaN, FP exception, overflow,
unwinding), which fire on correct code here: `b8` reports FAILED with
all 3 of its own assertions SUCCESS (the failures are `feraiseexcept`
and `fma.NaN` inside CBMC's builtin fma library, reached legitimately
via `mul_add(a,b,-r)` with `r = ±inf`). The type encodes Empty/NaI as
NaN bounds, so NaN-valued ops are the design. No flag disables them.

**What actually stands**: `a1 a2 a3 a7 b2 b3 b7 b10 c3` — clean of every
model defect (no `sqrt`, no FMA on their paths), <10 min total. `b8`
(interval-square-poison, 3/3) passes but its path reaches FMA. SEVEN
harnesses gave NO VERDICT at 900 s, including `b4`/`b5`, the flagship
mul/div containment proofs — that case analysis stays fuzz-only.

**Frictions.** (1) `rust-version = "1.97.0"` in the crate blocks Kani
outright (bundled nightly-2025-11-21 = rustc 1.93); true MSRV is 1.87.
Flux is in the same boat (pins nightly-2026-02-05). (2) Symbolic loop
bounds do not work — CBMC unwinds structurally and ignores
`assume(k <= 4)`; every harness needs a fixed trip count +
`#[kani::unwind(n)]`. (3) `f64` FMA bit-blasting is the cost ceiling.
CBMC only engages the SMT float theory with **bitwuzla** (z3 still gets
`QF_AUFBV`); bitwuzla could not be built in the sandbox (its configure
fetches CaDiCaL from codeload, 403 through the proxy), so that lever is
untested — and it is THE open lever: if the FP theory also fixes the
`sqrt`/`fmaf` models, most of what was withdrawn comes back and the
report should be re-run rather than believed.

**Where Flux would earn its keep in the kernel** (not done, just
established as feasible): index-safety arguments that today live in
prose, e.g. `geom-curves/src/nurbs.rs::eval_in_span`'s "Indexing
justified: span valid ⇒ i = span − p + j ∈ [span − p, span] ⊆
[0, control_count)". That exact pattern checks in 50 ms and the negative
control (drop `p <= span`) is caught as a `usize` underflow. NOT a fit:
`geom-brep/src/props/quad.rs`'s exact `i128` Newton–Cotes headroom claim
— nonlinear integer arithmetic, where liquid-fixpoint's encoding stops
being reliable.
