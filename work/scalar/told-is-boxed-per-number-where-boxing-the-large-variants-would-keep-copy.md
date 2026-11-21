---
id: told-is-boxed-per-number-where-boxing-the-large-variants-would-keep-copy
kind: issue
title: Told is boxed per number; boxing the large variants or the error would keep it Copy
status: open
opened: 2026-11-20
priority: P2
cost: M
design: true
---


Found building the told quantity (PR 7159); raised as open by its
review. For whoever specs the notation-taking rendering (unit 2), before
more crates build on the type.

`geom_core::Told` is `Box<…>`, one pointer wide. Held inline it is 56
bytes, which took `profile::PathError` from under 128 bytes to 336 and
tripped `clippy::result_large_err` at 660 sites and
`large_enum_variant` in `profile::LiftOutcome` (the PR's first hosted
run, 44725093100).

The cost of boxing each number: `Told` is `Clone` and not `Copy`, so
every payload type that holds one loses `Copy` too
(`PathNoCornerReason`, `LegFit`, `FilletLegCarrier`), which rippled
through `crates/profile` (closures rebuilt per use in
`arc_fillet::resolve`, `.clone()` in `CornerReason::recourse`, borrowed
matches across 25 suite files); and a refusal allocates once per number
it tells.

The alternative not taken: keep `Told` inline and `Copy`, and box the
large variants' payloads, or the error where it is returned
(`Result<_, Box<PathError<T>>>`). That is one allocation per refusal and
no `Copy` loss, at the price of a pattern change at every site that
destructures a boxed variant, and of deciding it again in each crate
whose error grows. Not measured: how many variants across the remaining
crates would cross the 128-byte line with an inline `Told`.

## Measured for this row (PR 7229, the notation unit)

How many error types would cross `clippy::result_large_err` holding a
`Told` inline. **This is an estimate read from source by a survey,
not a compiled measurement**: sizes were summed by hand from field
lists, a generic `T` taken as `f64`, an inline quantity taken as 48
bytes against the box's 8, and the lint's threshold as its default
128 (none is configured in `clippy.toml` or `[workspace.lints]`).
Every scalar field the sweeps will convert was counted as a told
quantity, and each `Told` `profile` already holds.

- **65 error types** are under 128 bytes today and would be at or
  over it: geom-core 2, geom 3, geom-brep 10, topo 21, sweep 4, mesh
  2, step-import 2, profile 12, verbs 1, editor-core 8.
- About 37 of them cross only by nesting, through two chains:
  `geom_brep::CertifyError` held by value in `topo::EulerOpError`
  (which nearly every `topo`, `sweep` and `step-import` error holds),
  and `geom_brep::SsiError`.
- 8 more land within a niche of the line (120 to 136 bytes); 39 stay
  under either way; none that holds a scalar is at or over 128 today.
- Not counted: `geom_core::Indeterminate`, whose `Band` and
  `MarginDiag` are value types. If those are converted, more than
  100 further types cross through it.

A compiled count (`size_of` rows, or the lint run on a branch with
the box removed) is what would turn this into a measurement.
