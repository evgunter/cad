# Kani and Flux: what they can and cannot prove here

Investigation, 2026-08-12. Question asked: can [Kani] or [Flux] be used
(a) to prove the interval side crate correct, and (b) to prove useful
things in this project generally?

Both tools were **installed and run against real code in this repo**, not
just read about. Everything below that says "verified" or "rejected" was
executed; the harnesses live in
`interval-transcendentals/src/verify.rs` and the Flux probes in
`docs/verification-probes/`.

[Kani]: https://model-checking.github.io/kani/
[Flux]: https://flux-rs.github.io/flux/

## Verdict up front

**(a) The interval crate.** Kani: **yes, for a well-defined and
surprisingly large slice** — but not the slice that the crate's own
`docs/derivations.md` spends its pages on. Flux: **no, and worse than
no** — it has no floating-point refinement sort, and rather than
rejecting a float refinement it **silently accepts false ones**. Probed
and confirmed: `#[spec(fn(x: f64) -> f64{v: v > x})]` on `fn id(x) { x }`
type-checks green. The same shape at `i32` is correctly rejected. Any
Flux adoption here has to treat every float-mentioning refinement as
unchecked.

The decomposition that matters:

| Layer of the crate's soundness argument | Machine-checkable by | Status |
|---|---|---|
| L0 libm's ≤1/≤2 bit-distance accuracy | neither (external assumption) | stays an assumption |
| L1 Rounding lemmas P1/P2 (real-arithmetic content) | neither — **Flocq/Gappa** territory | stays paper |
| L1′ Lemma P3's *encoding* half (bit-distance = neighbor-step count) | **Kani** | ✅ verified |
| L2 Enclosure of the exact SUM by the pads | **Kani, one precision down** (f32 model) | ✅ verified |
| L2′ Enclosure of the exact PRODUCT/QUOTIENT by the pads | attempted the same way — **blocked by CBMC's float models** | not established |
| L3 Representation invariant of `DInterval` (never emit an invalid interval) | **Kani** | ✅ verified |
| L4 Case analysis: mul corners, div's zero-touching four-way split, powi parity, poison propagation, decoration cap | **Kani** | ✅ verified |
| L4′ Anything involving `sqrt` or FMA | **neither** — CBMC's `sqrt` is not a function and its `fmaf` diverges from hardware (below) | stays paper + oracle |
| L5 π-family constant enclosures, grid-test conservatism | neither (needs reals) | stays paper + oracle |

L3/L4 is where implementation bugs actually live, and for the paths that
avoid `sqrt` and FMA, Kani covers it **exhaustively over every `f64` bit
pattern** — strictly stronger than the existing seed-pinned differential
harness, which samples. That qualifier is not decoration: three of
CBMC's floating-point models turned out to be unusable, and separating
the results that depend on them from the results that do not took most
of this investigation.

**(b) The project generally.** Kani is a poor fit for the kernel's
geometry (it is a bounded model checker: every loop needs a trip bound,
and the kernel's marching/fitting loops have none). Flux is a **good**
fit for one specific, real, recurring pattern: index-safety arguments
that today live in prose comments. See "Where Flux earns its keep".

Neither tool is a candidate for the thing an outsider might hope for —
proving the *geometry* (that a certified span really encloses the true
curve). That is real-analysis content; no Rust verifier reaches it.

---

## Why the two tools split the way they do

**Kani** compiles Rust to a GOTO program and hands it to CBMC, which is
**bit-precise in principle**: an `f64` is 64 symbolic bits and the core
IEEE operations are modelled exactly, subnormals, signed zeros and NaN
payloads included. (In practice several of the non-core operations are
not — see "Kani's floating-point counterexamples are not actionable
here"; that section is load-bearing and should be read before trusting
anything here.) What CBMC does **not** have, in principle or practice,
is any notion of a real number. So:

- `next_down(x) <= x` — statable, and proved.
- `next_down(RN(t)) <= t` (Lemma P1) — **not statable**, because `t` is a
  real that no expression in the program denotes.

That single gap is the entire reason the crate's pad derivations cannot
be discharged directly, and it is not a limitation that will be fixed by
trying harder.

**Flux** is a refinement type checker: it decorates Rust types with
logical indices and discharges the resulting Horn constraints through
liquid-fixpoint/Z3. It is modular (specs compose, no unwinding bounds,
unbounded loops fine) and roughly typecheck-fast. Its refinement sorts
are `int`, `bool`, `bitvec<32>`, `char`, `Set`, `Map`, and user-declared
sorts. **There is no float sort, and no real sort.** In Flux's own
tutorial, `f32`/`f64` appear only as inert payloads of length-indexed
vectors.

The probes confirm this against the real binary, and turn up something
sharper than a missing feature. Flux does not reject a refinement that
mentions an `f64` value; it accepts it, unconditionally:

```rust
#[spec(fn(x: f64) -> f64{v: v > x})]   // false for every input
pub fn id(x: f64) -> f64 { x }         // => "1 checked; 1 constraints solved"
```

The `f64` index is unconstrained, so the constraint is discharged
vacuously — and the success message is word-for-word the one Flux prints
when it has actually proved something. The identical shape at `i32` is
correctly rejected ("a postcondition cannot be proved"). Evidence and
exact outputs: `docs/verification-probes/README.md`.

### What Flux does that Kani cannot

The tools are not ranked; they are disjoint. Flux is strictly better at
four things Kani structurally cannot do:

1. **Unbounded loops and recursion.** Flux infers loop invariants and
   proves the property for all trip counts. Kani must unwind to a fixed
   bound — and does so *structurally*, not by using your assumptions:
   `step_down(x, k)` with symbolic `k` unwound past iteration 763 before
   being killed, despite `assume(k <= 4)`.
2. **Modularity.** A Flux spec is enforced at every call site and
   composes; Kani proves exactly the harnesses you write and nothing
   about their callers.
3. **Whole-crate coverage at typecheck speed.** Probe 1 checks in 50 ms.
   The `f64` harnesses here run in minutes to hours.
4. **Unbounded data.** `&[T][@n]` for arbitrary `n`; Kani has to bound
   every container.

The catch, for *this* crate, is that the intersection of those strengths
with its actual obligations is almost empty, because every obligation is
about a float value. The one place the two nearly meet is instructive:
`powi`'s binary exponentiation loops over `m: u64`, so Kani can only
cover the concrete exponents a harness pins (`b8` takes n ∈ {2,4,6} with
`#[kani::unwind(8)]`), whereas Flux's loop reasoning would cover all `m`
— except that the property to prove is "the lower bound is exactly
`+0.0`", which is a statement about an `f64`, which Flux cannot express
and would silently pretend to check.

---

## Part (a): what was actually verified in `interval-transcendentals`

Harnesses: `interval-transcendentals/src/verify.rs`, behind `cfg(kani)`,
so they do not exist in any cargo build.

Reproduce:

```
cargo install --locked kani-verifier && cargo kani setup   # ~1 GB bundle
cd interval-transcendentals && cargo kani                  # all harnesses
cargo kani --harness b5_div_encloses_every_point_quotient  # just one
# the two mutation controls, which MUST fail:
RUSTFLAGS="--cfg kani_mutation_is_normal --cfg kani_mutation_no_pad" \
  cargo kani --harness c1_mut_is_normal_gate_is_unsound \
             --harness c4_mut_unpadded_product_is_unsound
```

Versions used: Kani 0.67.0 / CBMC 6.8.0 / CaDiCaL 2.0.0, bundled
toolchain nightly-2025-11-21. Machine: 4 cores, 15 GB.

### Results

Clean sequential run, nothing else on the box. "own" counts the
harness's own `assertion.*` checks; "auto" is Kani's automatic checks
(see the section on reading them). Times are CBMC's own reported
verification time (symex + solver), excluding cargo and codegen.

**Proved, and independent of every model defect found below** — these
reach no `sqrt` and no FMA:

| Harness | own | auto | time |
|---|---|---|---|
| `a2` neighbor step = one `bitdist` step (Lemma P3's encoding half) | 2/2 | 3 | 0.2 s |
| `a3` `step_up(x, k)` is exactly k neighbor steps | 9/9 | 31 | 1.2 s |
| `a7` Kahan corner convention `0 · anything = 0`, inf included | 3/3 | 13 | 0.04 s |
| `b10` `with_dec_capped` only lowers; bounds bit-identical | 3/3 | 127 | 0.7 s |
| `b3` poison propagation across the arithmetic surface | 7/7 | 280 | 2.5 s |
| `a1` `step_down`/`step_up` move outward, k ≤ 4 | 4/4 | 113 | 8.8 s |
| `b2` exact + set ops preserve the representation invariant | 6/6 | 135 | 24.8 s |
| `b7` abs/min/max/floor/hull are exactly correct at symbolic points | 5/5 | 159 | 37.7 s |
| `c3` add pads enclose the exact sum (exponent spread ≤ 28, f32) | 5/5 | 13 | 501.1 s |

**Proved, but the path reaches CBMC's FMA model** — reported with that
caveat attached:

| Harness | own | auto | time |
|---|---|---|---|
| `b8` even `powi` of a straddling interval floors at exactly `+0.0` | 3/3 | 296 (2 fail: `feraiseexcept`, `fma`) | 845.0 s |

**Failed, not actionable** (see the counterexample section):

| Harness | own | time |
|---|---|---|
| `c1` mul pads enclose the exact product (f32) | 0/2 | 0.6 s |
| `c2` div pads enclose the exact quotient (f32) | 0/4 | 2.0 s |
| `c7` div padding logic with an exact `f64` witness | 0/4 | ~3 s |

**No verdict** — killed at the 900 s per-harness budget (`c6` at 1800 s).
Every one of these puts two unconstrained symbolic `f64` intervals
through multiplication or division:

`a5` (additive primitives ordered) · `a6` (multiplicative primitives
ordered) · `b1` (arithmetic preserves the invariant) · `b4` (mul
encloses every point product) · `b5` (div encloses every point
quotient) · `b6` (add/sub enclose every point) · `c6` (mul padding
logic with an exact witness)

That `b4`/`b5` — the two flagship containment harnesses, the ones that
would have covered `div_touching_zero`'s four-way sign match — are in
this list is the single biggest gap in the result. The case analysis
they target remains covered only by `tests/review_fuzz_div.rs`.


### Negative controls

Every claim above is worth exactly as much as the controls next to it,
so those were run too (all behind `--cfg` flags; all are *expected* to
fail, and a pass means something is wrong):

| Control | Expected | Result |
|---|---|---|
| `c4_mut` pad removed ⇒ the enclosure property must break | FAIL | ✅ FAILED, 0.06 s — Tier C's `c3` is not vacuous |
| `x1_ctl` `sqrt` is not modelled as a function | FAIL | ✅ FAILED — tripwire for a future Kani that fixes it |
| `x2_ctl` the pinned `fmaf` pair still mismodels | FAIL | ✅ FAILED — the divergence, captured as a standing check |
| `c5_ctl` `mul_add` is genuinely fused (asserting it is not must yield a counterexample) | FAIL | ⏱ no verdict in 40 min |
| `c1_mut` restore the historical `is_normal()` gate ⇒ the bug class reappears | FAIL | ⏱ **no verdict**, even narrowed to the region the bug lives in |

`c1_mut` was the intended showpiece — proving Kani would have caught the
subnormal-residual bug that `round.rs`'s `TWO_PROD_VALID_MIN` comment
records the differential harness catching. It does not terminate, and
even if it did, its counterexample would come from the `fmaf` model now
known to be wrong. Claim withdrawn: **there is no evidence here that
Kani would have caught that bug.**

### Tier A — the discrete lemmas (fully rigorous, no model gap)

`a2` is the interesting one. Lemma P3 in `docs/derivations.md` §1 is what
lets `PAD_ULPS = 4` follow from libm's CI bound, and it rests on a claim
about the *encoding*: within a sign class, one `next_up`/`next_down` step
changes libm-test's `bitdist` by exactly 1, subnormals and binade
boundaries included, with the ±0.0 pair as the only sign-crossing
exception. That claim is exactly the kind of thing that is obvious,
load-bearing, and occasionally false. Kani proves it over all finite
`f64`, and `a3` ties it to the code by proving `step_up(x, k)` really is
`k` encoded steps for the `k`s the crate uses.

### Tier B — the representation invariant and the case analysis

`b1`/`b2` turn `DInterval::make`'s `debug_assert!("make() got invalid
bounds")` into a theorem: over **every** pair of intervals in the type
(all three poison shapes; unconstrained `f64` endpoints, so ±inf,
subnormals and signed zeros included) no operation on the surface can
construct a value violating the documented invariant.

`b4`/`b5`/`b6` are containment with **symbolic interior points**: for
arbitrary `p ∈ x` and `q ∈ y`, the result encloses `mul_lo(p,q)` /
`div_lo(p,q)` / `add_lo(p,q)`. This is the property the corner-selection
code exists to provide, and stating it with symbolic points rather than
the four corners means the proof does not assume the very monotonicity
the implementation relies on. `b5` covers `div_touching_zero`'s four-way
sign match and the zero-straddling split — the case analysis that
`tests/review_fuzz_div.rs` was written to attack by fuzzing.

`b8` pins the contract from `memories/interval-square-poison.md`: an even
`powi` of a zero-straddling interval has lower bound **exactly `+0.0`**,
so a downstream `sqrt` can never be poisoned. `b9` proves `sqrt`'s lower
bound is never negative for any input interval. `b10` proves
`with_dec_capped` only ever lowers the decoration and leaves the bounds
bit-identical — "poison is never laundered", as a theorem.

### Tier C — pad soundness against exact arithmetic: ATTEMPTED, FAILED

This tier was meant to recover part of what L1 loses, and the idea still
looks right: the exact product of two `f32` values is **always**
representable in `f64` (24+24 = 48 mantissa bits, exponent range fits
with room to spare even for subnormal factors), so `f64` is an exact
oracle for `f32` arithmetic and the enclosure property becomes statable:

```rust
let exact = f64::from(a) * f64::from(b);   // EXACT
assert!(f64::from(mul_lo32(a, b)) <= exact);
```

`c3` (addition, whose `two_sum_err` is adds and subs only) **verified**,
5/5, in 501 s. Everything touching multiplication or division did not,
and chasing why is what produced the tool findings below. The status:

| Harness | Result |
|---|---|
| `c3` add pads enclose the exact sum (exponent-spread ≤ 28) | ✅ own 5/5 |
| `c1` mul pads enclose the exact product | ❌ own 0/2, counterexample **not reproducible on hardware** |
| `c2` div pads enclose the exact quotient | ❌ own 0/4, same |
| `c6` mul padding logic, FMA replaced by an exact `f64` witness | ⏱ no result in 30 min |
| `c7` div padding logic, FMA replaced by an exact `f64` witness | ❌ own 0/4, witness **not extractable** |

`c6`/`c7` exist because `c1`/`c2` route through `f32::mul_add` and the
first hypothesis was that CBMC's `fmaf` was at fault; they replace the
FMA exactness witness with an exact `f64` comparison, removing `mul_add`
from the harness entirely. `c7` still failed — so that hypothesis was
incomplete, and the investigation moved to the tool itself.

The Tier C claims are therefore **not established**. `c3` stands.

## Kani's floating-point counterexamples are not actionable here

Four separate diagnoses, each run rather than reasoned:

**1. `sqrt` is not modelled as a function.** Two calls on the same value
can return different results.

| Property of `f64::sqrt` under Kani 0.67 / CBMC 6.8 | |
|---|---|
| `sqrt(4.0) == 2.0` | ✅ passes |
| `x >= 0` ⟹ `sqrt(x) >= 0`, not NaN | ✅ passes |
| `a <= b` ⟹ `sqrt(a) <= sqrt(b)` | ❌ **fails** |
| `x.sqrt().to_bits() == x.sqrt().to_bits()` | ❌ **fails** |

This is how a harness over `sqrt` "failed" on the singleton
`[5.180654e-318, 5.180654e-318]`: the harness's own assertion passed;
what failed was `DInterval::make`'s `debug_assert!("make() got invalid
bounds")`, because `sqrt_lo` and `sqrt_hi` had been handed unrelated
roots of the same number. Read cold, that is a subnormal bug in
`ops.rs`, and someone would have "fixed" correct code.

**2. `fmaf` diverges from hardware.** `c1`'s reported witness
(`a = -6.604977e9`, `b = 7.398497e-39`), pinned back into CBMC as
concrete constants, still fails there — while holding under native
`rustc -O`. CBMC implements `fma`/`fmaf` as a builtin software library
(`<builtin-library-fma>`), not a native operation. Note this is a
*wrong* model, not merely a coarse one, so it can in principle make a
false property pass, not only a true one fail.

**3. Concrete playback reports values that are not the witness.** This
is the one that breaks the debugging loop. Every `c7` witness Kani
printed **passes when pinned back into CBMC** as a constant — three of
them, including two from a search narrowed to a single pinned divisor.
So a Kani `FAILED` here cannot be turned into a fact about the code:
you cannot get the input that caused it.

The rule this forces, and it is stronger than "replay natively":
**pin a reported counterexample back inside the checker first.** If it
passes there, playback lied and you have learned nothing. Only if it
fails in the checker *and* holds natively do you have a real divergence
(case 2). Replaying natively alone is not enough — it was not enough
here, and it led this investigation to a wrong intermediate conclusion.

**4. The systematic cross-checks do not terminate.** The obvious
diagnostic — is `a * b` at `f32` equal to `(f64 * f64) as f32`, over all
pairs — ran 40 minutes without a verdict. So the model cannot be
characterised in bulk either.

One thing was localised before the trail went cold: with the divisor
pinned, `c7` **verifies for all normal numerators** and fails only for
subnormal ones. Whether that region holds a genuine divergence or
another phantom is **unresolved**, and it is recorded as unresolved.

### What this means for the results that DID pass

A wrong model can make a false property pass. So the passes are graded
by whether their code path reaches an operation now known to be
mismodelled:

- **Independent of the broken models** — pure bit/order operations and
  the exact endpoint ops, reaching no `sqrt` and no FMA: `a1`, `a2`,
  `a3`, `a7`, `b2`, `b3`, `b7`, `b10`, `c3`. These are the results this
  document stands behind.
- **Passes that route through `mul_exact`'s FMA**: `b8` (the
  interval-square-poison contract, own 3/3). Its `powi` path reaches
  `f64::mul_add`. The result is reported, and it is conditional on
  CBMC's double-precision `fma` model — which was not shown wrong, but
  whose single-precision sibling was.

## Read the per-check results, never the top-line verdict

Needed to use any of this: Kani's `VERIFICATION:- FAILED` folds together
*your* assertions and the automatic checks it enables by default (NaN
production, FP exceptions, arithmetic overflow, unwinding). On this
crate the automatic ones fire on perfectly correct code:

- `b8` reports FAILED with **all three of its own assertions SUCCESS**.
  The two failing checks are `feraiseexcept` ("floating-point
  exception") and `fma.NaN.5` ("NaN on division") inside CBMC's
  `<builtin-library-fma>`. They are reached legitimately: `mul_exact`
  evaluates `mul_add(a, b, -r)` with `r = ±inf` for infinite bounds, and
  `inf - inf = NaN` is the correct answer, which `mul_exact` then
  correctly reads as "not exact".
- The type itself encodes Empty and NaI as NaN bounds, so NaN-valued
  operations are the crate's design, not a defect.

There is no flag to turn these off. A harness is proved when **its own
`verify::<harness>.assertion.*` checks are all SUCCESS**; everything
else is triage. The results table reports the two columns separately.

## Part (b): the rest of the project

### Where Kani does not fit

The kernel's hot code is marching, fitting, subdivision and knot algebra
— loops whose trip count depends on geometry, not on a constant. CBMC
must unwind every loop to a fixed bound; past the bound you get either an
unsound "verified" or (with unwinding assertions, Kani's default) an
honest failure. Proving a marcher correct is out of reach in principle,
not just in practice.

Kani does fit small, closed, combinatorial code where exhaustiveness over
machine values is the point. Candidates in this repo, in order of
value-per-effort:

1. `crates/geom-core`'s `Interval` scalar wrapper (the `interval`
   feature) — same shape as the crate proven here, and it is the
   adapter that the whole kernel's interval lane runs through.
2. The exact-order band in `topo` (null-edge sort over dyadic
   geometry) — a comparator-total-order proof is finite and discrete.
3. `crates/quantity` — unit/dimension algebra, tiny and total.

### Where Flux earns its keep

Flux's target is the argument the kernel already writes down in prose.
From `crates/geom-curves/src/nurbs.rs::eval_in_span`:

```rust
// Indexing justified: span valid ⇒ i = span − p + j
// ∈ [span − p, span] ⊆ [0, control_count).
let i = span - p + j;
```

That comment is a refinement type. Probe 1 (`docs/verification-probes/`)
encodes exactly this pattern and Flux checks it — including that
`span - p` cannot underflow — with the loop invariant inferred, no
annotation on the loop. Probe 2 drops `p <= span` from the precondition
and Flux rejects it, so the check is not vacuous.

Scale of the opportunity, measured: ~162k lines of kernel `src`, **1850**
`unwrap`/`expect` sites and **2047** direct index sites. Flux is not
something to point at all of that. The realistic lane is one subsystem
whose invariants are index-shaped and already documented in comments —
`geom-curves`' knot algebra (3.9k lines) is the natural pilot, `topo`'s
arena indexing the natural second.

**Where Flux will not help, despite looking like it might:** the exact
`i128` rational quadrature in `geom-brep/src/props/quad.rs`. It is
already written with `checked_mul`/`checked_add` and a documented
"`m ≤ 12` fits in `i128` headroom" claim — precisely the claim one would
want machine-checked. But the quantities are products of Newton–Cotes
numerators, i.e. **nonlinear** integer arithmetic, which is where
liquid-fixpoint's Z3 encoding stops being reliable. Expect this one to
need hand annotation at every step or to fail outright.

---

## The tool that actually fits layer L1

Nothing in the Rust verification ecosystem proves P1/P2. The tools that
do are the floating-point formalisations in Coq:

- **Flocq** — a Coq library formalising IEEE 754 formats with `ulp`,
  `succ`, `pred` and rounding predicates. P1
  (`pred(RN(t)) <= t <= succ(RN(t))`) is a short derivation in its
  vocabulary; P2's binade-boundary counting argument is the kind of thing
  it exists for.
- **Gappa** — proves bounds on rounding errors of *specific* expressions
  and emits Coq proofs. The right shape for the per-function pad
  compositions in `docs/derivations.md` §2–§5, less so for the
  format-level lemmas.

This is a genuinely different project (Coq, not Rust; no connection to
the source) and is **not** recommended now. It is recorded so the
frontier is named rather than blurred: the paper proofs in
`docs/derivations.md` stay the authority for L0/L1/L5, and the Kani
harnesses do not pretend otherwise.

---

## The one that bites: CBMC has no usable `f64::sqrt`

This was found the hard way and it is the most important caveat in the
document.

A harness over `sqrt` came back **FAILED** on the singleton interval
`[5.180654e-318, 5.180654e-318]`. Precisely: the harness's own assertion
(`sqrt`'s lower bound is never negative) *passed*; what failed was
`DInterval::make`'s own `debug_assert!("make() got invalid bounds")`,
reached through `sqrt` — i.e. `sqrt` had produced `lo > hi`. That reads
as a live subnormal bug in `ops.rs`. It is not. Probing the tool
directly:

| Property of `f64::sqrt` under Kani 0.67 / CBMC 6.8 | Result |
|---|---|
| `sqrt(4.0) == 2.0` | ✅ passes |
| `x >= 0` ⟹ `sqrt(x) >= 0`, not NaN | ✅ passes |
| `a <= b` ⟹ `sqrt(a) <= sqrt(b)` (monotone) | ❌ **fails** |
| `x.sqrt().to_bits() == x.sqrt().to_bits()` (same call, same input) | ❌ **fails** |

**`sqrt` is not modelled as a function.** Two calls on the same value can
return different results, which is exactly how `sqrt_lo` and `sqrt_hi`
came to be handed unrelated roots of the same number and produce
`lo > hi`. The generated checks (`sqrt.NaN.1` … `sqrt.NaN.8`, "NaN on
division/multiplication/subtraction") show CBMC running a software
Newton-style routine that the default unwinding truncates into something
partially unconstrained.

Partially is the worst case: it is constrained enough that the obvious
smoke tests pass, and unconstrained enough that real properties fail
with plausible-looking subnormal counterexamples. Without the
determinism probe, the natural reading of that failure is "Kani found a
bug in the crate", and someone would have gone and "fixed" correct code.

So: **no result in this document covers `sqrt`.** Its enclosure contract
(`docs/derivations.md` §3, the FMA exactness witness) stays on the paper
proof plus the inari oracle, and the `sqrt` assertions were removed from
`b2` and `b8`. The defect is preserved as
`x1_ctl_sqrt_is_not_modelled_as_a_function` (behind
`--cfg kani_tool_control`, expected to fail) so that a future Kani which
fixes this announces itself.

The same question is moot for the `libm` transcendentals, and for a
second reason: probing `libm::sin` under `#[kani::unwind(64)]` produces
a wall of

```
Not unwinding loop ...rem_pio2_large... iteration 64
  libm-0.2.16/src/math/rem_pio2_large.rs:268
```

— Payne–Hanek reduction is a loop CBMC cannot finish, so `sin` is
truncated into something partly unconstrained exactly as `sqrt` is.
L0/L5 were already out of reach for the real-arithmetic reason; this is
the independent second reason.

## Read the per-check results, never the top-line verdict

Related, and needed to use any of this: Kani's `VERIFICATION:- FAILED`
folds together *your* assertions and the automatic checks it enables by
default (NaN production, FP exceptions, arithmetic overflow, unwinding).
On this crate the automatic ones fire on perfectly correct code:

- `b8` (the interval-square-poison contract) reports FAILED with **all
  three of its own assertions SUCCESS**. The two failing checks are
  `feraiseexcept` ("floating-point exception") and `fma.NaN.5` ("NaN on
  division") inside CBMC's `<builtin-library-fma>`. They are reached
  legitimately: `mul_exact` evaluates `mul_add(a, b, -r)` with `r = ±inf`
  for infinite bounds, and `inf - inf = NaN` is the correct answer, which
  `mul_exact` then correctly reads as "not exact".
- The type itself encodes Empty and NaI as NaN bounds, so NaN-valued
  operations are the crate's design, not a defect.

There is no flag to turn these off. The practical rule: a harness is
proved when **its own `verify::<harness>.assertion.*` checks are all
SUCCESS**; everything else is triage. The results table below reports
those two columns separately for exactly this reason.

## Frictions found (all real, all hit during this investigation)

1. **MSRV pin blocks Kani outright.** `interval-transcendentals`
   declares `rust-version = "1.97.0"`; Kani 0.67.0 bundles
   nightly-2025-11-21 (rustc 1.93.0-nightly) and cargo refuses before
   compiling anything:
   `error: rustc 1.93.0-nightly is not supported by the following package`.
   Flux is in the same position (it pins nightly-2026-02-05, rustc
   1.95.0-nightly). The crate's *true* MSRV is 1.87 (edition 2024 needs
   1.85, `next_up`/`next_down` 1.86, `u64::is_multiple_of` 1.87); 1.97.0
   is the pinned-toolchain number, not a requirement. Lowering
   `rust-version` to the real floor unblocks both tools and costs
   nothing. **This is the one change the investigation asks for.**

2. **Symbolic loop bounds do not work.** `step_down(x, k)` is
   `for _ in 0..k`; with a symbolic `k: u32`, CBMC unwinds structurally
   and never terminates (`kani::assume(k <= 4)` kills paths but does not
   cut the unwinding — observed: still unwinding at iteration 763 after
   ten minutes). Harnesses must fix the trip count and carry
   `#[kani::unwind(n)]`.

3. **FMA is expensive.** `mul_exact`/`div_exact` call `f64::mul_add`, and
   bit-blasting a double-precision FMA costs CaDiCaL ~20 s per query.
   This is the practical ceiling on `f64`-level harnesses and the reason
   Tier C is stated at `f32`, where the same reasoning is cheap.

   There is a lever here that went **untested**: CBMC engages the SMT
   floating-point theory (`QF_AUFBV with FPA`) only under `--solver
   bitwuzla`; `--solver z3` still bit-blasts (`QF_AUFBV`, no FPA), so z3
   is not the answer. Bitwuzla could not be built in this sandbox — its
   `configure.py` fetches CaDiCaL from codeload.github.com, which the
   proxy 403s — so whether native FP reasoning rescues the `f64`-level
   FMA harnesses is an open question, and a cheap one to settle on a
   machine with unrestricted egress. (Amusing footnote: bitwuzla wants
   GMP and MPFR, the exact C/LGPL dependencies this crate exists to
   escape. As a dev-time verifier that is fine — it links nothing.)

## Recommendation

The honest summary is that Kani earns a **narrow, manual** place here,
much narrower than it looked three hours in, and Flux earns none in this
crate.

1. **Keep the nine clean harnesses, run them by hand.** `a1 a2 a3 a7 b2
   b3 b7 b10 c3` prove real obligations — the representation invariant,
   poison propagation, the decoration cap, exact-op correctness at
   symbolic points, P3's encoding half, and pad soundness for addition —
   exhaustively over every `f64` bit pattern, which is strictly more
   than the seed-pinned differential harness gets. Total runtime is
   under 10 minutes. Trigger: the same one as the `oracle-inari` tier,
   i.e. when the rounding layer, the case analysis or the invariant
   changes.
2. **Do not gate CI on them.** Two independent reasons now, not one: the
   toolchain skew (friction 1), and the fact that a FAILED verdict here
   is frequently not about the code. A red CI job nobody can action is
   worse than a documented manual tier.
3. **Treat every FAILED verdict as unproven, not as a bug**, until the
   reported witness has been pinned back inside CBMC *and* replayed
   natively. Three of the four failures this investigation chased were
   the tool; the fourth is still unattributed.
4. **Do not adopt Flux for the interval crate** — it cannot express the
   properties, and it will say green while not expressing them.
5. **A Flux pilot on `geom-curves`' knot algebra is worth considering**
   on its own merits, scoped to turning existing prose index-safety
   comments into checked specs. This investigation establishes only that
   the tool handles the pattern (50 ms, negative control caught).
6. **Leave L0/L1/L5 to the paper proofs**, with Flocq/Gappa named as the
   tool class that would actually fit, so nobody re-derives that Kani
   "should" have been able to do it.
7. **One cheap open lever**: build bitwuzla on a machine with
   unrestricted egress and re-run `b4`/`b5`. CBMC engages its SMT
   floating-point theory only under `--solver bitwuzla` (z3 still
   bit-blasts), and those two harnesses — the div/mul containment
   proofs, the most valuable ones in the set — timed out under CaDiCaL.
   If the FP theory also fixes the `sqrt` and `fmaf` models, most of
   what is withdrawn above comes back, and this document should be
   re-run rather than believed.

## What this cost

Roughly six hours of wall clock, most of it solver time, on a 4-core
box. The tool findings — not the proofs — were the bulk of it: three
CBMC floating-point defects and one Flux soundness footgun, each of
which first presented as a plausible bug in correct code. That ratio is
the most useful single number in this document for deciding whether to
go further.
