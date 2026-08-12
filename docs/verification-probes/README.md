# Flux probes

Five one-file probes run against a real Flux build during the
verification-tools investigation (`docs/VERIFICATION-TOOLS.md`). They are
kept because the third one records a **footgun**, not a limitation, and a
footgun needs evidence.

## How they were run

Flux is a rustc driver on its own pinned nightly, so it is not part of
any cargo build here. Setup used:

```
curl -fsSL https://raw.githubusercontent.com/flux-rs/flux/main/install.sh | bash
```

which needs `rustup`, a `liquid-fixpoint` binary (prebuilt nightly from
liquid-fixpoint's GitHub releases), and `z3 >= 4.15` (pip's `z3-solver`
ships a usable `z3` binary; Ubuntu's `z3` 4.8 is too old). Flux itself is
built from source (`cargo xtask install`) and pinned
`nightly-2026-02-05`. Each probe:

```
flux --crate-type=lib <probe>.rs
```

Version used: flux `main` @ 2026-08-12, liquid-fixpoint nightly, z3 5.0.0.

## The probes

| File | Intent | Result |
|---|---|---|
| `span_ok.rs` | the real NURBS span-indexing pattern, index safety as a precondition | **checked**, 2 constraints, 50 ms |
| `span_bad.rs` | negative control: drop `p <= span` | **rejected** — `arithmetic operation may underflow` |
| `float_sort.rs` | can a refinement constrain an `f64` value? | "checked" — see below |
| `float_vacuity.rs` | vacuity test: claim `id(x) > x` | **"checked"** — false spec ACCEPTED |
| `float_vacuity2.rs` | claim `next_down(x) >= x`, plus an `i32` control | f64 claim ACCEPTED; `i32` control correctly rejected |

### The finding that matters

`float_vacuity.rs` is:

```rust
#[spec(fn(x: f64) -> f64{v: v > x})]
pub fn id(x: f64) -> f64 { x }
```

`id` returns exactly `x`, so `v > x` is false for every input. Flux
reports:

```
summary. 1 functions processed: 1 checked; 0 trusted; 0 ignored.
1 constraints solved. Finished in 21.52ms
```

The identical shape at `i32` (`float_vacuity2.rs`) is correctly rejected
with "a postcondition cannot be proved". So Flux does not *reject*
float refinements — it **silently accepts them regardless of truth**,
because `f64` carries no refinement sort and the index is unconstrained.

That is worse than a plain absence of support: a green Flux run over
float-carrying code says nothing about the floats, and says it in the
same words it uses when it has actually proved something. Any future
Flux adoption here must treat every float-mentioning refinement as
unchecked, by convention and preferably by lint.

### The finding that encourages

`span_ok.rs` mirrors `crates/geom-curves/src/nurbs.rs::eval_in_span`,
whose index safety is currently a prose comment:

```rust
// Indexing justified: span valid ⇒ i = span − p + j
// ∈ [span − p, span] ⊆ [0, control_count).
let i = span - p + j;
```

Flux checks that argument in 50 ms with the loop invariant inferred and
no annotation beyond the function signature, and `span_bad.rs` shows the
check has teeth: weaken the precondition and the `usize` underflow is
caught. This is the one pattern in the kernel where Flux would convert
existing prose obligations into machine-checked ones.
