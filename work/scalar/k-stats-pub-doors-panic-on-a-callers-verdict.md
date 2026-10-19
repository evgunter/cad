---
id: k-stats-pub-doors-panic-on-a-callers-verdict
kind: issue
title: geom_core::k_stats's pub gate_rejection, straddle and decide_magnitude panic on a caller's arguments, which D2 makes input
status: open
opened: 2026-10-19
priority: P3
cost: E
---



Found by the blind-spot pass of ENCL's total-render sweep
(`work/encl/a-hand-built-unminted-refusal-panics-through-pub-render.md`),
which made every `pub` render total. The same shape sits outside the
render class: a `pub` function whose `# Panics` names a value its
caller passes.

`[failure]` (D2): the kernel never panics on any input, and a `pub`
call with a caller's arguments is input. `geom_core::k_stats` is
`pub mod k_stats` (`crates/geom-core/src/lib.rs`), and:

- `gate_rejection` (`crates/geom-core/src/k_stats.rs`, its `# Panics`)
  is `unreachable!` where `decided` is no verdict the band reaches on
  its margin, or one `passes` admits.
- `straddle` (same file) panics where either bound is no verdict the
  band reaches, or the lower's sign is not below the upper's.
- `decide_magnitude` and `decide_magnitude_reported` (same file) panic
  on a definite `Negative` margin.

Each guard reads a kernel invariant at the kernel's own call sites.
The repair is the one PR 5365 took for the span checks: a typed
refusal (or a type that cannot carry the bad combination) at the `pub`
door, and the panic only where the data is known to be the kernel's
(a crate-private wrapper). Census the rest of `k_stats`'s `# Panics`
docs with it.
